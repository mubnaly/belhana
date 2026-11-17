//! Controlled domain tools exposed to the AI provider.
//!
//! The Rust backend validates every tool input; financial and quantity math
//! stays in the deterministic engine. Mutating tools only create *proposals*.

use belhana_common::CommonError;
use serde_json::{json, Value};
use sqlx::SqlitePool;

pub fn tool_schemas() -> Vec<Value> {
    vec![
    json!({
        "name": "get_household_context",
        "description": "حجم الأسرة، المدينة، الميزانية، المعدات، عدد الأفراد",
        "parameters": {"type": "object", "properties": {}}
    }),
    json!({
        "name": "get_budget_status",
        "description": "حالة الميزانية الأسبوعية أو الشهرية (المتاح، المصروف، المخطط)",
        "parameters": {"type": "object", "properties": {"period": {"type": "string", "enum": ["weekly", "monthly", "daily"]}}}
    }),
    json!({
        "name": "search_products",
        "description": "البحث في كتالوج المنتجات بالاسم العربي",
        "parameters": {"type": "object", "properties": {"query": {"type": "string"}}}
    }),
    json!({
        "name": "get_current_price_observations",
        "description": "أسعار منتج محدد مع المصدر والتاريخ والحالة",
        "parameters": {"type": "object", "properties": {"product_name": {"type": "string"}}}
    }),
    json!({
        "name": "get_inventory",
        "description": "المخزون الحالي (مخزن/ثلاجة/فريزر)، اختيارياً لفئة",
        "parameters": {"type": "object", "properties": {"kind": {"type": "string", "enum": ["pantry", "fridge", "freezer"]}}}
    }),
    json!({
        "name": "search_recipes",
        "description": "البحث في الوصفات المتاحة لنوع وجبة أو بكلمة مفتاحية",
        "parameters": {"type": "object", "properties": {"meal_type": {"type": "string"}, "query": {"type": "string"}}}
    }),
    json!({
        "name": "calculate_recipe_cost",
        "description": "حساب تكلفة وصفة من الأسعار المختارة",
        "parameters": {"type": "object", "properties": {"recipe_name": {"type": "string"}, "servings": {"type": "integer"}}}
    }),
    json!({
        "name": "propose_meal_plan",
        "description": "توليد مقترح خطة وجبات (لن تُحفظ قبل موافقة المستخدم)",
        "parameters": {"type": "object", "properties": {
            "days": {"type": "integer"},
            "periods": {"type": "array", "items": {"type": "string"}},
            "servings": {"type": "integer"}
        }}
    }),
    json!({
        "name": "calculate_shopping_list",
        "description": "حساب قائمة مشتريات من الخطة الحالية (معاينة)",
        "parameters": {"type": "object", "properties": {}}
    }),
    json!({
        "name": "propose_food_rule",
        "description": "اقتراح قاعدة أسرة جديدة (تحتاج موافقة المستخدم)",
        "parameters": {"type": "object", "properties": {
            "rule_type": {"type": "string", "enum": ["meal_repeat_limit", "dish_consecutive_limit", "min_legume_meals", "use_freezer_first", "member_dislike_avoid", "allergen_ban", "dish_frequency_limit", "ai_instruction", "meal_staple", "fixed_spend", "leftover_carryover"]},
            "priority": {"type": "string", "enum": ["hard", "soft"]},
            "params": {"type": "object"}
        }}
    }),
    json!({
        "name": "propose_event_menu",
        "description": "اقتراح قوائم عزومة حسب عدد الضيوف والميزانية (معاينة)",
        "parameters": {"type": "object", "properties": {
            "name": {"type": "string"},
            "adults": {"type": "integer"},
            "children": {"type": "integer"},
            "budget_total_minor": {"type": "integer"},
            "meal_type": {"type": "string"}
        }}
    }),
    json!({
        "name": "explain_plan",
        "description": "شرح الخطة الحالية وتكاليفها وسبب اختيار كل وجبة",
        "parameters": {"type": "object", "properties": {}}
    }),
    ]
}

/// Executes a validated tool call against the domain core.
/// Mutating tools create ai_action_proposals rows instead of writing data.
pub async fn execute_tool(
    pool: &SqlitePool,
    household_id: &str,
    user_id: &str,
    name: &str,
    args: &Value,
) -> Result<Value, CommonError> {
    match name {
        "get_household_context" => {
            let hh = belhana_core::households::get_household(pool, household_id).await?;
            let members = belhana_core::members::list_members(pool, household_id).await?;
            Ok(json!({
                "name": hh.name, "city": hh.city, "area": hh.area, "currency": hh.currency,
                "household_size": hh.household_size,
                "weekly_budget_minor": hh.weekly_budget_minor,
                "monthly_budget_minor": hh.monthly_budget_minor,
                "shopping_frequency": hh.shopping_frequency,
                "price_policy": hh.price_policy,
                "members": members.iter().map(|m| json!({"name": m.display_name, "age_band": m.age_band})).collect::<Vec<_>>(),
            }))
        }
        "get_budget_status" => {
            let period = args["period"].as_str().unwrap_or("weekly");
            let s = belhana_core::budget::budget_status(pool, household_id, period).await?;
            Ok(json!({
                "budget_minor": s.budget_minor, "actual_spent_minor": s.actual_spent_minor,
                "planned_minor": s.planned_cost_minor, "remaining_minor": s.remaining_minor,
                "period": period,
            }))
        }
        "search_products" => {
            let q = args["query"].as_str().unwrap_or("");
            let products = belhana_core::catalog::list_products(pool, None, Some(q), 10, 0).await?;
            Ok(json!(products
                .iter()
                .map(|p| json!({"id": p.id, "name_ar": p.name_ar, "unit": p.default_unit}))
                .collect::<Vec<_>>()))
        }
        "get_current_price_observations" => {
            let name = args["product_name"].as_str().unwrap_or("");
            let products = belhana_core::catalog::list_products(pool, None, Some(name), 1, 0).await?;
            let Some(p) = products.first() else {
                return Ok(json!({"found": false, "message": "المنتج غير موجود في الكتالوج"}));
            };
            let obs = belhana_core::pricing::list_observations(pool, Some(&p.id), None, None, None, 5, 0).await?;
            Ok(json!({"found": true, "product": p.name_ar, "observations": obs
                .iter()
                .map(|o| json!({
                    "amount_minor": o.amount_minor, "unit": o.unit, "unit_status": o.unit_status,
                    "observation_date": o.observation_date, "review_status": o.review_status,
                }))
                .collect::<Vec<_>>()}))
        }
        "get_inventory" => {
            let kind = args["kind"].as_str();
            let lots = belhana_core::inventory::list_lots(pool, household_id, None, kind, Some("active")).await?;
            Ok(json!(lots
                .iter()
                .map(|l| json!({"product": l.product_name_ar, "quantity": l.quantity, "unit": l.unit, "location": l.location_name}))
                .collect::<Vec<_>>()))
        }
        "search_recipes" => {
            let mt = args["meal_type"].as_str();
            let q = args["query"].as_str();
            let recipes = belhana_core::recipes::list_recipes(pool, household_id, mt, q, 10, 0).await?;
            Ok(json!(recipes
                .iter()
                .map(|r| json!({"id": r.id, "name_ar": r.name_ar, "meal_types": r.meal_types, "servings": r.default_servings}))
                .collect::<Vec<_>>()))
        }
        "calculate_recipe_cost" => {
            let name = args["recipe_name"].as_str().unwrap_or("");
            let recipes = belhana_core::recipes::list_recipes(pool, household_id, None, Some(name), 1, 0).await?;
            let Some(r) = recipes.first() else {
                return Ok(json!({"found": false}));
            };
            let hh = belhana_core::households::get_household(pool, household_id).await?;
            let servings = args["servings"].as_i64().unwrap_or(r.default_servings);
            let cost = belhana_core::costing::compute_recipe_cost(pool, &r.id, servings, &hh.price_policy).await?;
            Ok(json!({
                "recipe": r.name_ar, "servings": servings,
                "total_minor": cost.total_minor, "per_serving_minor": cost.per_serving_minor,
                "unknown_components": cost.unknown_count,
                "lines": cost.lines.iter().map(|l| json!({"name": l.name_ar, "qty": l.quantity, "unit": l.unit, "known": l.known, "price_minor": l.price_minor})).collect::<Vec<_>>(),
            }))
        }
        "propose_meal_plan" => {
            let hh = belhana_core::households::get_household(pool, household_id).await?;
            let days = args["days"].as_i64().unwrap_or(7).clamp(1, 31);
            let periods: Vec<String> = args["periods"]
                .as_array()
                .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_else(|| vec!["breakfast".into(), "lunch".into(), "dinner".into()]);
            let servings = args["servings"].as_i64().unwrap_or(hh.household_size);
            let out = belhana_core::planning::generate_plan_in_memory(
                pool,
                &hh,
                days,
                periods.clone(),
                servings,
                belhana_core::today(),
                &[],
            )
            .await?;
            Ok(json!({
                "preview": true, "needs_user_approval": true,
                "days": days, "periods": periods, "servings": servings,
                "total_cost_minor": out.total_cost_minor,
                "conflicts": out.conflicts,
                "slots": out.slots.iter().map(|s| json!({"date": s.date, "period": s.period, "recipe": s.recipe_name_ar, "cost_minor": s.cost_minor, "explanation": s.explanation})).collect::<Vec<_>>(),
            }))
        }
        "calculate_shopping_list" => {
            let active: Option<String> = sqlx::query_scalar(
                "SELECT id FROM meal_plans WHERE household_id = ? AND status = 'accepted' ORDER BY created_at DESC LIMIT 1",
            )
            .bind(household_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| CommonError::Internal(e.to_string()))?;
            let Some(plan_id) = active else {
                return Ok(json!({"found": false, "message": "لا توجد خطة مقبولة حالياً"}));
            };
            let list = belhana_core::shopping::generate_from_plan(pool, household_id, &plan_id).await?;
            Ok(json!({
                "list_id": list.id,
                "estimated_total_minor": list.estimated_total_minor,
                "items": list.items.iter().map(|i| json!({"product": i.product_name_ar, "shopping_qty": i.shopping_qty, "unit": i.shopping_unit, "estimated_total_minor": i.estimated_total_minor})).collect::<Vec<_>>(),
            }))
        }
        "propose_food_rule" => {
            let rule_type = args["rule_type"].as_str().unwrap_or("");
            let priority = args["priority"].as_str().unwrap_or("soft");
            let params = args["params"].clone();
            // validate structurally before proposing
            belhana_core::rules::validate_rule(
                rule_type,
                &params,
            )?;
            let payload = json!({"rule_type": rule_type, "priority": priority, "params": params});
            let summary = format!("اقتراح قاعدة: {rule_type} ({priority})");
            sqlx::query(
                "INSERT INTO ai_action_proposals (id, household_id, user_id, action_type, payload, summary, status, created_at)
                 VALUES (?, ?, ?, 'add_rule', ?, ?, 'pending', datetime('now'))",
            )
            .bind(belhana_core::new_id())
            .bind(household_id)
            .bind(user_id)
            .bind(payload.to_string())
            .bind(&summary)
            .execute(pool)
            .await
            .map_err(|e| CommonError::Internal(e.to_string()))?;
            Ok(json!({"proposed": true, "needs_user_approval": true, "summary": summary}))
        }
        "propose_event_menu" => {
            let name = args["name"].as_str().unwrap_or("عزومة");
            let adults = args["adults"].as_i64().unwrap_or(4);
            let children = args["children"].as_i64().unwrap_or(0);
            let budget = args["budget_total_minor"].as_i64();
            let meal_type = args["meal_type"].as_str().unwrap_or("lunch");
            let event = belhana_core::occasions::create_event(
                pool,
                household_id,
                &belhana_core::occasions::CreateEvent {
                    name: format!("(اقتراح المساعد) {name}"),
                    occasion_type: None,
                    date: None,
                    time: None,
                    adults: Some(adults),
                    children: Some(children),
                    budget_total_minor: budget,
                    budget_per_guest_minor: None,
                    meal_type: Some(meal_type.into()),
                    style: None,
                    include_drinks: None,
                    include_sweets: None,
                    include_fruit: None,
                    include_appetizers: None,
                    use_inventory: None,
                },
            )
            .await?;
            let menus = belhana_core::occasions::generate_menus(pool, household_id, &event.id).await?;
            Ok(json!({
                "event_id": event.id,
                "menus": menus.iter().map(|m| json!({
                    "tier": m.tier, "title": m.title,
                    "total_cost_minor": m.total_cost_minor, "cost_per_guest_minor": m.cost_per_guest_minor,
                    "dishes": m.dishes.iter().map(|d| json!({"course": d.course, "recipe": d.recipe_name_ar, "servings": d.servings})).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
                "note": "تم إنشاء حدث كمسودة لمعاينته — لن يظهر في خطتك إلا بعد الموافقة",
            }))
        }
        "explain_plan" => {
            let plan: Option<String> = sqlx::query_scalar(
                "SELECT id FROM meal_plans WHERE household_id = ? AND status IN ('accepted','draft') ORDER BY created_at DESC LIMIT 1",
            )
            .bind(household_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| CommonError::Internal(e.to_string()))?;
            let Some(plan_id) = plan else {
                return Ok(json!({"found": false, "message": "لا توجد خطة بعد"}));
            };
            let out = belhana_core::planning::read_plan(pool, &plan_id).await?;
            Ok(json!({
                "plan_id": plan_id,
                "total_cost_minor": out.total_cost_minor,
                "slots": out.slots.iter().map(|s| json!({"date": s.date, "period": s.period, "recipe": s.recipe_name_ar, "cost_minor": s.cost_minor, "explanation": s.explanation, "inventory_used": s.inventory_used, "missing": s.missing})).collect::<Vec<_>>(),
            }))
        }
        other => Err(CommonError::Validation(format!("unknown AI tool: {other}"))),
    }
}
