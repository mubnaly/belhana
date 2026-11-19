//! Deterministic meal-planning engine.
//!
//! Design principles:
//! - Hard constraints (allergies, exclusions, hard rules, equipment) filter the
//!   candidate set and are **never** violated by scoring.
//! - Soft objectives are scored explainably: preferences, variety, repetition,
//!   budget fit, stock usage, legume targets.
//! - The engine is fully deterministic: stable ordering, no randomness.
//! - If no feasible candidate exists for a slot, the engine reports a
//!   structured conflict instead of silently violating constraints.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Row, SqlitePool};

use crate::{costing, households::Household, inventory, map_db, members, pricing, recipes, rules};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LockedSlot {
    pub day_index: i64,
    pub period: String,
    pub recipe_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryUse {
    pub product_id: String,
    pub name_ar: String,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingItem {
    pub product_id: String,
    pub name_ar: String,
    pub quantity: f64,
    pub unit: String,
    pub price_known: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlannedMealOut {
    /// Planned-meal row id when the plan is saved (empty for in-memory runs).
    #[serde(default)]
    pub id: String,
    pub date: String,
    pub day_index: i64,
    pub period: String,
    pub recipe_id: String,
    pub recipe_name_ar: String,
    pub servings: i64,
    pub cost_minor: i64,
    pub shopping_cost_minor: i64,
    pub inventory_used: Vec<InventoryUse>,
    pub missing: Vec<MissingItem>,
    pub explanation: String,
    pub locked: bool,
    /// Estimated calories for one serving of the chosen recipe.
    pub kcal_per_serving: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PlanOutput {
    pub slots: Vec<PlannedMealOut>,
    pub total_cost_minor: i64,
    pub total_shopping_cost_minor: i64,
    pub budget_minor: Option<i64>,
    pub days_cost: Vec<(String, i64)>,
    pub conflicts: Vec<String>,
    pub unknown_price_products: Vec<String>,
}

struct Candidate {
    recipe: recipes::Recipe,
    ingredients: Vec<recipes::RecipeIngredient>,
    cost_minor: i64, // total for `servings`
    cost_known: bool,
    legume: bool,
    pref_score: f64,
    liked_reasons: Vec<String>,
    disliked_reasons: Vec<String>,
}

impl Clone for Candidate {
    fn clone(&self) -> Self {
        Candidate {
            recipe: self.recipe.clone(),
            ingredients: self.ingredients.clone(),
            cost_minor: self.cost_minor,
            cost_known: self.cost_known,
            legume: self.legume,
            pref_score: self.pref_score,
            liked_reasons: self.liked_reasons.clone(),
            disliked_reasons: self.disliked_reasons.clone(),
        }
    }
}

struct EngineContext {
    hh: Household,
    excluded_products: Vec<String>,
    disliked_products: Vec<String>,
    liked_products: Vec<String>,
    liked_dishes: Vec<String>,
    hard_rules: Vec<rules::HouseholdRule>,
    soft_rules: Vec<rules::HouseholdRule>,
    min_legume_per_week: Option<i64>,
    use_freezer_first: bool,
    /// (period, product_id) pairs: rice at lunch, bread at breakfast/dinner, etc.
    staples: Vec<(String, String)>,
    equipment: Vec<String>,
    policy: String,
}

async fn build_ctx(pool: &SqlitePool, hh: &Household) -> Result<EngineContext, CommonError> {
    let all_rules = rules::list_rules(pool, &hh.id).await?;
    let hard: Vec<_> = all_rules
        .iter()
        .filter(|r| r.priority == "hard" && r.enabled)
        .cloned()
        .collect();
    let soft: Vec<_> = all_rules
        .iter()
        .filter(|r| r.priority != "hard" && r.enabled)
        .cloned()
        .collect();

    let mut excluded = Vec::new();
    for e in members::list_exclusions(pool, &hh.id).await? {
        excluded.push(e.product_id);
    }
    for r in &hard {
        if r.rule_type == "allergen_ban" {
            if let Ok(params) =
                serde_json::from_str::<serde_json::Value>(r.params.as_deref().unwrap_or("{}"))
            {
                if let Some(pid) = params["product_id"].as_str() {
                    excluded.push(pid.to_string());
                }
            }
        }
    }

    let mut disliked = Vec::new();
    let mut liked = Vec::new();
    let mut liked_dishes = Vec::new();
    for m in members::list_members(pool, &hh.id).await? {
        for p in members::list_preferences(pool, &m.id).await? {
            match (p.kind.as_str(), p.ref_id.as_deref()) {
                ("dislike", Some(pid)) => disliked.push(pid.to_string()),
                ("like", Some(pid)) => liked.push(pid.to_string()),
                ("preferred_ingredient", Some(pid)) => liked.push(pid.to_string()),
                ("preferred_dish", Some(rid)) => liked_dishes.push(rid.to_string()),
                _ => {}
            }
        }
    }
    let mut min_legume: Option<i64> = None;
    for r in all_rules.iter().filter(|r| r.enabled) {
        if r.rule_type == "min_legume_meals" && min_legume.is_none() {
            if let Ok(p) = serde_json::from_str::<serde_json::Value>(r.params.as_deref().unwrap_or("{}")) {
                min_legume = p["minimum_per_week"].as_i64();
            }
        }
    }
    let use_freezer_first = all_rules.iter().any(|r| r.enabled && r.rule_type == "use_freezer_first");
    let staples = rules::meal_staples(pool, &hh.id).await?;
    let equipment: Vec<String> = hh
        .kitchen_equipment
        .as_ref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();

    Ok(EngineContext {
        hh: hh.clone(),
        excluded_products: dedup(excluded),
        disliked_products: dedup(disliked),
        liked_products: dedup(liked),
        liked_dishes,
        hard_rules: hard,
        soft_rules: soft,
        min_legume_per_week: min_legume,
        use_freezer_first,
        staples,
        equipment,
        policy: hh.price_policy.clone(),
    })
}

fn dedup(mut v: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    v.retain(|x| seen.insert(x.clone()));
    v
}

fn equipment_ok(recipe: &recipes::Recipe, have: &[String]) -> bool {
    if have.is_empty() {
        return true; // household hasn't specified equipment → don't block
    }
    recipe.equipment.iter().all(|e| have.contains(e))
}

/// Loads one recipe as a candidate regardless of hard filters (for user-locked meals).
async fn load_candidate(
    pool: &SqlitePool,
    recipe_id: &str,
    servings: i64,
    policy: &str,
) -> Result<Option<Candidate>, CommonError> {
    let all = recipes::get_recipe(pool, recipe_id).await;
    let Ok((recipe, ingredients, _)) = all else {
        return Ok(None);
    };
    let cost = costing::compute_recipe_cost(pool, recipe_id, servings, policy).await?;
    Ok(Some(Candidate {
        cost_minor: cost.total_minor.unwrap_or(cost.known_total_minor),
        cost_known: cost.unknown_count == 0,
        legume: recipe.tags.iter().any(|t| t == "legume"),
        pref_score: 0.0,
        liked_reasons: vec![],
        disliked_reasons: vec![],
        recipe,
        ingredients,
    }))
}

#[allow(clippy::too_many_arguments)]
pub async fn generate_plan_in_memory(
    pool: &SqlitePool,
    hh: &Household,
    days: i64,
    periods_enabled: Vec<String>,
    servings_per_meal: i64,
    start_date: chrono::NaiveDate,
    locked: &[LockedSlot],
) -> Result<PlanOutput, CommonError> {
    let ctx = build_ctx(pool, hh).await?;
    let all_recipes = recipes::list_recipes(pool, &hh.id, None, None, 500, 0).await?;
    let mut conflicts = Vec::new();
    let mut unknown_products: Vec<String> = Vec::new();
    let mut used_count: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    let mut previous_dates: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    // Recent plan history for repetition windows.
    let window_days = max_repeat_window(&ctx.soft_rules, &ctx.hard_rules).max(1);
    let history_start = (start_date - chrono::Duration::days(window_days))
        .format("%Y-%m-%d")
        .to_string();
    let hist = sqlx::query(
        "SELECT pm.recipe_id, pm.date FROM planned_meals pm
         JOIN meal_plans p ON p.id = pm.plan_id
         WHERE p.household_id = ? AND p.status IN ('accepted','draft') AND pm.date >= ?",
    )
    .bind(&hh.id)
    .bind(&history_start)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    for r in hist {
        previous_dates
            .entry(r.get::<String, _>("recipe_id"))
            .or_default()
            .push(r.get::<String, _>("date"));
    }

    // Candidates per period with hard filtering.
    let mut candidates_by_period: std::collections::HashMap<String, Vec<Candidate>> = Default::default();
    for period in &periods_enabled {
        let mut cands = Vec::new();
        for r in &all_recipes {
            if !r.meal_types.iter().any(|t| t == period) {
                continue;
            }
            let (_, ingredients, _) = recipes::get_recipe(pool, &r.id).await?;
            // Hard filter: excluded/allergen products.
            let excluded_hit = ingredients.iter().any(|i| {
                i.product_id
                    .as_ref()
                    .map(|pid| ctx.excluded_products.contains(pid))
                    .unwrap_or(false)
            });
            if excluded_hit {
                continue;
            }
            // Hard filter: equipment.
            if !equipment_ok(r, &ctx.equipment) {
                continue;
            }
            let cost = costing::compute_recipe_cost(pool, &r.id, servings_per_meal, &ctx.policy).await?;
            for line in &cost.lines {
                if !line.known && !line.product_id.is_empty() {
                    unknown_products.push(line.product_id.clone());
                }
            }
            // Preference scoring.
            let mut pref = 0.0;
            let mut likes = Vec::new();
            let mut dislikes = Vec::new();
            for ing in &ingredients {
                if let Some(pid) = &ing.product_id {
                    if ctx.liked_products.contains(pid) {
                        pref += 3.0;
                        likes.push(ing.raw_name_ar.clone().unwrap_or_else(|| pid.clone()));
                    }
                    if ctx.disliked_products.contains(pid) {
                        pref -= 2.0;
                        dislikes.push(ing.raw_name_ar.clone().unwrap_or_else(|| pid.clone()));
                    }
                }
            }
            if ctx.liked_dishes.iter().any(|d| d == &r.id) {
                pref += 4.0;
            }
            cands.push(Candidate {
                recipe: r.clone(),
                ingredients,
                cost_minor: cost.total_minor.unwrap_or(cost.known_total_minor),
                cost_known: cost.unknown_count == 0,
                legume: r.tags.iter().any(|t| t == "legume"),
                pref_score: pref,
                liked_reasons: likes,
                disliked_reasons: dislikes,
            });
        }
        if cands.is_empty() {
            conflicts.push(format!(
                "لا توجد وصفات متاحة لفترة {period} بعد تطبيق القيود الصارمة (حساسية/استثناءات/معدات)"
            ));
        }
        candidates_by_period.insert(period.clone(), cands);
    }

    let stock_lots = inventory::list_lots(pool, &hh.id, None, None, Some("active")).await?;
    let mut slots: Vec<PlannedMealOut> = Vec::new();
    let mut days_cost: Vec<(String, i64)> = Vec::new();
    let legume_target = ctx.min_legume_per_week.unwrap_or(0);
    let mut legume_count = 0i64;
    let daily_share = ctx.hh.weekly_budget_minor.map(|b| b as f64 / 7.0);

    for day_i in 0..days.max(1) {
        let date = start_date + chrono::Duration::days(day_i);
        let date_s = date.format("%Y-%m-%d").to_string();
        let mut day_total = 0i64;
        for period in &periods_enabled {
            let Some(cands) = candidates_by_period.get(period) else {
                continue;
            };
            let locked_for_slot = locked
                .iter()
                .find(|l| l.day_index == day_i && l.period == *period);

            let chosen: Option<Candidate> = match locked_for_slot {
                Some(l) => {
                    if let Some(c) = cands.iter().find(|c| c.recipe.id == l.recipe_id) {
                        Some(c.clone())
                    } else {
                        load_candidate(pool, &l.recipe_id, servings_per_meal, &ctx.policy).await?
                    }
                }
                None => pick_best(
                    cands,
                    &ctx,
                    period,
                    &date_s,
                    &mut used_count,
                    &previous_dates,
                    &stock_lots,
                    legume_count < legume_target,
                    daily_share,
                    day_total,
                ),
            };
            let Some(cand) = chosen else {
                conflicts.push(format!("لا توجد خيارات مجدية لوجبة {period} بتاريخ {date_s}"));
                continue;
            };
            if cand.legume {
                legume_count += 1;
            }
            *used_count.entry(cand.recipe.id.clone()).or_insert(0) += 1;
            day_total += cand.cost_minor;

            // Quantities, inventory coverage, missing items.
            let scale = if cand.recipe.default_servings > 0 {
                servings_per_meal as f64 / cand.recipe.default_servings as f64
            } else {
                1.0
            };
            let mut inv_used = Vec::new();
            let mut missing = Vec::new();
            let mut shopping_cost = 0i64;
            let mut unknown = false;
            for ing in &cand.ingredients {
                let Some(pid) = &ing.product_id else { continue };
                let qty = ing.quantity * scale;
                let product = crate::catalog::get_product(pool, pid).await?;
                let stock = inventory::product_stock(pool, &hh.id, pid, &ing.unit).await?;
                let covered = stock.min(qty);
                if covered > 0.0 {
                    inv_used.push(InventoryUse {
                        product_id: pid.clone(),
                        name_ar: product.name_ar.clone(),
                        quantity: covered,
                        unit: ing.unit.clone(),
                    });
                }
                let miss_qty = (qty - stock).max(0.0);
                if miss_qty > 0.0 {
                    let obs = pricing::best_observation_for_product(pool, pid, &ctx.policy).await?;
                    let priced = obs
                        .as_ref()
                        .and_then(|o| costing::price_per_unit(o, &ing.unit))
                        .map(|per| (miss_qty * per).round() as i64);
                    if priced.is_none() {
                        unknown = true;
                    }
                    shopping_cost += priced.unwrap_or(0);
                    missing.push(MissingItem {
                        product_id: pid.clone(),
                        name_ar: product.name_ar.clone(),
                        quantity: miss_qty,
                        unit: ing.unit.clone(),
                        price_known: priced.is_some(),
                    });
                }
            }
            let mut explanation = format!("اختيار {period} بتاريخ {date_s}: ");
            if !cand.liked_reasons.is_empty() {
                explanation.push_str(&format!("يحتوي مكونات مفضلة ({})؛ ", cand.liked_reasons.join("، ")));
            }
            if !cand.disliked_reasons.is_empty() {
                explanation
                    .push_str(&format!("تنبيه: يحتوي مكونات غير مفضلة ({})؛ ", cand.disliked_reasons.join("، ")));
            }
            if used_count.get(&cand.recipe.id).copied().unwrap_or(1) > 1 {
                explanation.push_str("تم تكرار الوصفة ضمن الخطة؛ ");
            }
            if cand.legume {
                explanation.push_str("وجبة بقوليات؛ ");
            }
            if !cand.cost_known {
                explanation.push_str("التكلفة تقديرية بسبب مكونات غير مسعرة؛ ");
            }
            if unknown {
                explanation.push_str("تنبيه: مكونات مفقودة بدون سعر معروف؛ ");
            }
            if ctx.staples.iter().any(|(p, pid)| {
                p == period
                    && cand
                        .ingredients
                        .iter()
                        .any(|i| i.product_id.as_deref() == Some(pid.as_str()))
            }) {
                explanation.push_str("يناسب الغذاء الرئيسي المفضل لهذه الوجبة؛ ");
            }
            if !inv_used.is_empty() {
                explanation.push_str(&format!(
                    "يستخدم بقايا متوفرة ({}) تُحتسب من المخزون ولا تُشترى مجددًا؛ ",
                    inv_used
                        .iter()
                        .map(|u| format!("{} {} {}", u.name_ar, u.quantity, u.unit))
                        .collect::<Vec<_>>()
                        .join("، ")
                ));
            }
            slots.push(PlannedMealOut {
                id: String::new(),
                date: date_s.clone(),
                day_index: day_i,
                period: period.clone(),
                recipe_id: cand.recipe.id.clone(),
                recipe_name_ar: cand.recipe.name_ar.clone(),
                servings: servings_per_meal,
                cost_minor: cand.cost_minor,
                shopping_cost_minor: shopping_cost,
                inventory_used: inv_used,
                missing,
                explanation,
                locked: locked_for_slot.is_some(),
                kcal_per_serving: cand.recipe.kcal_per_serving,
            });
        }
        days_cost.push((date_s, day_total));
    }

    let total: i64 = slots.iter().map(|s| s.cost_minor).sum();
    let shopping_total: i64 = slots.iter().map(|s| s.shopping_cost_minor).sum();
    if let Some(b) = ctx.hh.weekly_budget_minor {
        if days >= 7 && total > b {
            conflicts.push(format!(
                "إجمالي الخطة ({total}) يتجاوز الميزانية الأسبوعية ({b}) — يمكنك رفع الميزانية أو تخفيف الوجبات"
            ));
        }
    }
    if legume_target > 0 && legume_count < legume_target && days >= 7 {
        conflicts.push(format!(
            "لم يتحقق هدف البقوليات ({legume_count}/{legume_target} وجبة) — أضف وصفات بقوليات أو عدّل القاعدة"
        ));
    }
    Ok(PlanOutput {
        slots,
        total_cost_minor: total,
        total_shopping_cost_minor: shopping_total,
        budget_minor: ctx.hh.weekly_budget_minor,
        days_cost,
        conflicts,
        unknown_price_products: dedup(unknown_products),
    })
}

/// Deterministic greedy selection with explainable scoring.
#[allow(clippy::too_many_arguments)]
fn pick_best(
    cands: &[Candidate],
    ctx: &EngineContext,
    period: &str,
    date_s: &str,
    used_count: &std::collections::HashMap<String, i64>,
    previous_dates: &std::collections::HashMap<String, Vec<String>>,
    stock_lots: &[inventory::StockLot],
    legume_needed: bool,
    daily_share: Option<f64>,
    day_total: i64,
) -> Option<Candidate> {
    let mut best: Option<(f64, String, &Candidate)> = None;
    for c in cands {
        let mut score = 0.0f64;
        score += c.pref_score;
        // repetition within this plan
        let used = used_count.get(&c.recipe.id).copied().unwrap_or(0);
        if used > 0 {
            score -= 5.0 * used as f64;
        }
        // repetition vs recent accepted plans
        if let Some(dates) = previous_dates.get(&c.recipe.id) {
            let window = repeat_limit_days(ctx).unwrap_or(7);
            let recent = dates.iter().any(|d| {
                if let (Ok(pd), Ok(td)) = (
                    chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d"),
                    chrono::NaiveDate::parse_from_str(date_s, "%Y-%m-%d"),
                ) {
                    let delta = (td - pd).num_days();
                    delta >= 0 && delta <= window
                } else {
                    false
                }
            });
            if recent {
                score -= 10.0;
            }
        }
        // consecutive-day repetition of the same recipe
        if same_recipe_yesterday(c, date_s, previous_dates) {
            if is_hard_consecutive(ctx) {
                score -= 1000.0;
            } else {
                score -= 8.0;
            }
        }
        // budget fit
        if let Some(share) = daily_share {
            if day_total as f64 + c.cost_minor as f64 > share {
                score -= 4.0;
            }
        }
        // stock usage
        score += 5.0 * coverage_ratio(c, stock_lots);
        // freezer-first rule
        if ctx.use_freezer_first && uses_freezer_stock(c, stock_lots) {
            score += 2.0;
        }
        // staple preference for this meal period (rice at lunch, bread at
        // breakfast/dinner). Soft: it nudges but never filters candidates.
        if ctx.staples.iter().any(|(p, pid)| {
            p == period
                && c.ingredients
                    .iter()
                    .any(|i| i.product_id.as_deref() == Some(pid.as_str()))
        }) {
            score += 8.0;
        }
        // legume weekly target
        if legume_needed && c.legume {
            score += 3.0;
        }
        // prefer fully-costed recipes
        if !c.cost_known {
            score -= 1.0;
        }
        // deterministic tie-break: score, then recipe id
        let key = (score, c.recipe.id.clone());
        let better = match &best {
            Some((bs, bid, _)) => key > (*bs, bid.clone()),
            None => true,
        };
        if better {
            best = Some((score, c.recipe.id.clone(), c));
        }
    }
    best.map(|(_, _, c)| c.clone())
}

fn repeat_limit_days(ctx: &EngineContext) -> Option<i64> {
    ctx.hard_rules
        .iter()
        .chain(ctx.soft_rules.iter())
        .filter(|r| r.rule_type == "meal_repeat_limit" || r.rule_type == "dish_frequency_limit")
        .filter_map(|r| serde_json::from_str::<serde_json::Value>(r.params.as_deref().unwrap_or("{}")).ok())
        .filter_map(|p| p["period_days"].as_i64())
        .max()
}

fn max_repeat_window(soft: &[rules::HouseholdRule], hard: &[rules::HouseholdRule]) -> i64 {
    soft.iter()
        .chain(hard.iter())
        .filter(|r| r.rule_type == "meal_repeat_limit" || r.rule_type == "dish_frequency_limit")
        .filter_map(|r| serde_json::from_str::<serde_json::Value>(r.params.as_deref().unwrap_or("{}")).ok())
        .filter_map(|p| p["period_days"].as_i64())
        .max()
        .unwrap_or(7)
}

fn uses_freezer_stock(c: &Candidate, lots: &[inventory::StockLot]) -> bool {
    let product_ids: std::collections::HashSet<&String> =
        c.ingredients.iter().filter_map(|i| i.product_id.as_ref()).collect();
    lots.iter().any(|l| product_ids.contains(&l.product_id) && l.quantity > 0.0)
}

fn coverage_ratio(c: &Candidate, lots: &[inventory::StockLot]) -> f64 {
    let relevant: Vec<&recipes::RecipeIngredient> =
        c.ingredients.iter().filter(|i| i.product_id.is_some()).collect();
    if relevant.is_empty() {
        return 0.0;
    }
    let mut covered = 0;
    for i in &relevant {
        let pid = i.product_id.as_ref().unwrap();
        if lots
            .iter()
            .any(|l| &l.product_id == pid && l.unit == i.unit && l.quantity > 0.0)
        {
            covered += 1;
        }
    }
    covered as f64 / relevant.len() as f64
}

fn same_recipe_yesterday(
    c: &Candidate,
    date_s: &str,
    previous_dates: &std::collections::HashMap<String, Vec<String>>,
) -> bool {
    if let Ok(t) = chrono::NaiveDate::parse_from_str(date_s, "%Y-%m-%d") {
        let y = (t - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        return previous_dates
            .get(&c.recipe.id)
            .map(|d| d.iter().any(|x| *x == y))
            .unwrap_or(false);
    }
    false
}

fn is_hard_consecutive(ctx: &EngineContext) -> bool {
    ctx.hard_rules
        .iter()
        .any(|r| r.rule_type == "dish_consecutive_limit")
}

pub fn add_days(d: chrono::NaiveDate, n: i64) -> chrono::NaiveDate {
    d + chrono::Duration::days(n)
}

/// Persists an in-memory plan as a draft and returns its id.
#[allow(clippy::too_many_arguments)]
pub async fn save_plan(
    pool: &SqlitePool,
    hh: &Household,
    out: &PlanOutput,
    days: i64,
    periods_enabled: &[String],
    servings: i64,
    start_date: chrono::NaiveDate,
    actor: &str,
    locked: &[LockedSlot],
) -> Result<String, CommonError> {
    let plan_id = crate::new_id();
    let now = crate::now_rfc3339();
    let end = start_date + chrono::Duration::days(days.max(1) - 1);
    sqlx::query(
        "INSERT INTO meal_plans (id, household_id, start_date, end_date, days, status, budget_amount_minor, servings_per_meal, periods_enabled, shopping_pattern, estimated_total_cost_minor, created_by, created_at)
         VALUES (?, ?, ?, ?, ?, 'draft', ?, ?, ?, 'weekly', ?, ?, ?)",
    )
    .bind(&plan_id)
    .bind(&hh.id)
    .bind(start_date.format("%Y-%m-%d").to_string())
    .bind(end.format("%Y-%m-%d").to_string())
    .bind(days)
    .bind(hh.weekly_budget_minor)
    .bind(servings)
    .bind(serde_json::to_string(periods_enabled).unwrap())
    .bind(out.total_cost_minor)
    .bind(actor)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_db)?;
    sqlx::query("INSERT INTO planning_constraints (id, plan_id, snapshot) VALUES (?, ?, ?)")
        .bind(crate::new_id())
        .bind(&plan_id)
        .bind(json!({"days": days, "periods": periods_enabled, "servings": servings, "locked": locked}).to_string())
        .execute(pool)
        .await
        .map_err(map_db)?;
    sqlx::query(
        "INSERT INTO planning_run_results (id, plan_id, run_at, success, message, conflicts, stats) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(crate::new_id())
    .bind(&plan_id)
    .bind(&now)
    .bind(if out.conflicts.is_empty() { 1 } else { 0 })
    .bind(if out.conflicts.is_empty() { "generated".to_string() } else { "generated with conflicts".to_string() })
    .bind(json!(&out.conflicts).to_string())
    .bind(json!({"total_cost_minor": out.total_cost_minor, "slots": out.slots.len()}).to_string())
    .execute(pool)
    .await
    .map_err(map_db)?;
    for s in &out.slots {
        let day_id: String = match sqlx::query_scalar::<_, String>(
            "SELECT id FROM meal_plan_days WHERE plan_id = ? AND day_index = ?",
        )
        .bind(&plan_id)
        .bind(s.day_index)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        {
            Some(id) => id,
            None => {
                let id = crate::new_id();
                sqlx::query("INSERT INTO meal_plan_days (id, plan_id, date, day_index) VALUES (?, ?, ?, ?)")
                    .bind(&id)
                    .bind(&plan_id)
                    .bind(&s.date)
                    .bind(s.day_index)
                    .execute(pool)
                    .await
                    .map_err(map_db)?;
                id
            }
        };
        sqlx::query(
            "INSERT INTO planned_meals (id, plan_id, day_id, date, period, recipe_id, servings, locked, status, cost_estimate_minor, inventory_used, missing_ingredients, explanation, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'proposed', ?, ?, ?, ?, ?)",
        )
        .bind(crate::new_id())
        .bind(&plan_id)
        .bind(&day_id)
        .bind(&s.date)
        .bind(&s.period)
        .bind(&s.recipe_id)
        .bind(s.servings)
        .bind(if s.locked { 1 } else { 0 })
        .bind(s.cost_minor)
        .bind(json!(&s.inventory_used).to_string())
        .bind(json!(&s.missing).to_string())
        .bind(&s.explanation)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(map_db)?;
    }
    Ok(plan_id)
}

/// Reads a stored plan back into PlanOutput form.
pub async fn read_plan(pool: &SqlitePool, plan_id: &str) -> Result<PlanOutput, CommonError> {
    let rows = sqlx::query(
        "SELECT pm.*, r.name_ar AS recipe_name FROM planned_meals pm
         JOIN recipes r ON r.id = pm.recipe_id
         WHERE pm.plan_id = ? ORDER BY pm.date, pm.created_at",
    )
    .bind(plan_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let kcal_map = crate::nutrition::recipe_kcal_map(pool).await?;
    let mut slots = Vec::new();
    for (i, r) in rows.into_iter().enumerate() {
        slots.push(PlannedMealOut {
            id: r.get("id"),
            date: r.get("date"),
            day_index: i as i64,
            period: r.get("period"),
            recipe_id: r.get("recipe_id"),
            recipe_name_ar: r.get("recipe_name"),
            servings: r.get("servings"),
            cost_minor: r.get::<Option<i64>, _>("cost_estimate_minor").unwrap_or(0),
            shopping_cost_minor: 0,
            inventory_used: r
                .get::<Option<String>, _>("inventory_used")
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            missing: r
                .get::<Option<String>, _>("missing_ingredients")
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
            explanation: r.get::<Option<String>, _>("explanation").unwrap_or_default(),
            locked: r.get::<i64, _>("locked") != 0,
            kcal_per_serving: kcal_map.get(&r.get::<String, _>("recipe_id")).copied(),
        });
    }
    let total: i64 = slots.iter().map(|s| s.cost_minor).sum();
    Ok(PlanOutput {
        slots,
        total_cost_minor: total,
        total_shopping_cost_minor: 0,
        budget_minor: None,
        days_cost: vec![],
        conflicts: vec![],
        unknown_price_products: vec![],
    })
}

/// Marks a planned meal prepared → deducts inventory per household workflow.
pub async fn mark_prepared(
    pool: &SqlitePool,
    household_id: &str,
    planned_meal_id: &str,
) -> Result<(), CommonError> {
    let row = sqlx::query(
        "SELECT pm.inventory_used, h.consumption_workflow FROM planned_meals pm
         JOIN meal_plans mp ON mp.id = pm.plan_id
         JOIN households h ON h.id = mp.household_id
         WHERE pm.id = ? AND mp.household_id = ?",
    )
    .bind(planned_meal_id)
    .bind(household_id)
    .fetch_optional(pool)
    .await
    .map_err(map_db)?
    .ok_or_else(|| CommonError::NotFound("planned meal".into()))?;
    let workflow: String = row.get("consumption_workflow");
    let inventory_used: Option<String> = row.get("inventory_used");
    if workflow != "confirm_on_prepare" {
        sqlx::query("UPDATE planned_meals SET status = 'prepared' WHERE id = ?")
            .bind(planned_meal_id)
            .execute(pool)
            .await
            .map_err(map_db)?;
        return Ok(());
    }
    let uses: Vec<InventoryUse> = inventory_used
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    for u in &uses {
        inventory::consume_fefo(
            pool,
            household_id,
            &u.product_id,
            u.quantity,
            &u.unit,
            "planned meal prepared",
            Some(planned_meal_id),
        )
        .await?;
    }
    sqlx::query("UPDATE planned_meals SET status = 'prepared' WHERE id = ?")
        .bind(planned_meal_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(())
}
