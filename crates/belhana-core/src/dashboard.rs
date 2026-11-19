//! Home dashboard aggregation.

use belhana_common::CommonError;
use serde::Serialize;
use sqlx::{Row, SqlitePool};

use crate::{budget, inventory, map_db, recipes, shopping, today};
use chrono::Datelike;

#[derive(Debug, Serialize)]
pub struct Dashboard {
    pub today_meals: Vec<TodayMeal>,
    /// Structured day: main dishes plus the bread/adama/salad/snack components
    /// that make each meal complete. Suggestions are deterministic.
    pub today_table: Vec<TodayCourse>,
    /// Quick homemade basics (bread, rice, salad, snacks) worth surfacing.
    pub quick_basics: Vec<TodayDish>,
    pub budget: Option<budget::BudgetStatus>,
    pub next_shopping_date: Option<String>,
    pub shopping_list_status: Option<ShoppingSummary>,
    pub expiring_stock: Vec<inventory::StockLot>,
    pub freezer_count: i64,
    pub pantry_count: i64,
    pub recent_prices: Vec<RecentPrice>,
    pub suggestion: String,
}

#[derive(Debug, Serialize)]
pub struct TodayMeal {
    pub period: String,
    pub recipe_name_ar: String,
    pub prepared: bool,
}

#[derive(Debug, Serialize)]
pub struct TodayCourse {
    /// breakfast | morning_snack | lunch | afternoon_snack | dinner
    pub key: String,
    /// Planner period when this course comes from a planned meal.
    pub period: Option<String>,
    pub main: Option<TodayDish>,
    pub sides: Vec<TodayDish>,
    /// True when the main dish is a deterministic suggestion, not today's plan.
    pub suggested: bool,
    pub prepared: bool,
}

#[derive(Debug, Serialize)]
pub struct TodayDish {
    pub recipe_id: Option<String>,
    pub name_ar: String,
    /// dish | bread | adama | staple | salad | snack
    pub role: String,
    pub suggested: bool,
}

struct PlannedToday {
    recipe_id: String,
    name_ar: String,
    prepared: bool,
}

#[derive(Debug, Serialize)]
pub struct ShoppingSummary {
    pub list_id: String,
    pub total_items: i64,
    pub purchased_items: i64,
    pub estimated_total_minor: i64,
}

#[derive(Debug, Serialize)]
pub struct RecentPrice {
    pub product_name_ar: String,
    pub amount_minor: Option<i64>,
    pub unit: Option<String>,
    pub observation_date: Option<String>,
    pub review_status: String,
    pub merchant_name: Option<String>,
}

fn pick_by_tag<'a>(
    all: &'a [recipes::Recipe],
    tag: &str,
    rotate: usize,
    exclude: &[String],
) -> Option<&'a recipes::Recipe> {
    let mut matches: Vec<&recipes::Recipe> = all
        .iter()
        .filter(|r| r.tags.iter().any(|t| t == tag) && !exclude.iter().any(|e| e == &r.id))
        .collect();
    matches.sort_by(|a, b| a.name_ar.cmp(&b.name_ar));
    if matches.is_empty() {
        None
    } else {
        Some(matches[rotate % matches.len()])
    }
}

fn pick_many_by_tag<'a>(
    all: &'a [recipes::Recipe],
    tag: &str,
    rotate: usize,
    exclude: &[String],
    count: usize,
) -> Vec<&'a recipes::Recipe> {
    let mut matches: Vec<&recipes::Recipe> = all
        .iter()
        .filter(|r| r.tags.iter().any(|t| t == tag) && !exclude.iter().any(|e| e == &r.id))
        .collect();
    matches.sort_by(|a, b| a.name_ar.cmp(&b.name_ar));
    if matches.is_empty() {
        return Vec::new();
    }
    (0..count.min(matches.len()))
        .map(|i| matches[(rotate + i) % matches.len()])
        .collect()
}

/// A full plan-worthy dish for a period, never a component/side recipe.
fn pick_main<'a>(
    all: &'a [recipes::Recipe],
    period: &str,
    rotate: usize,
    exclude: &[String],
) -> Option<&'a recipes::Recipe> {
    let mut matches: Vec<&recipes::Recipe> = all
        .iter()
        .filter(|r| r.meal_types.iter().any(|m| m == period))
        .filter(|r| !r.tags.iter().any(|t| t == "component"))
        .filter(|r| !exclude.iter().any(|e| e == &r.id))
        .collect();
    matches.sort_by(|a, b| a.name_ar.cmp(&b.name_ar));
    if matches.is_empty() {
        None
    } else {
        Some(matches[rotate % matches.len()])
    }
}

fn dish(r: &recipes::Recipe, role: &str, suggested: bool) -> TodayDish {
    TodayDish {
        recipe_id: Some(r.id.clone()),
        name_ar: r.name_ar.clone(),
        role: role.to_string(),
        suggested,
    }
}

/// Common Egyptian rice dishes already contain rice; no need to add a plain
/// rice side on top of them.
fn main_is_rice(name: &str) -> bool {
    ["أرز", "كشري", "محمرة", "مجدرة", "فتة"]
        .iter()
        .any(|k| name.contains(k))
}

pub async fn load_dashboard(pool: &SqlitePool, hid: &str) -> Result<Dashboard, CommonError> {
    // Today's meals from the active (accepted) or newest plan.
    let today_s = today().format("%Y-%m-%d").to_string();
    let meal_rows = sqlx::query(
        "SELECT pm.period, pm.recipe_id, r.name_ar AS name, pm.status
         FROM planned_meals pm
         JOIN meal_plans p ON p.id = pm.plan_id
         JOIN recipes r ON r.id = pm.recipe_id
         WHERE p.household_id = ? AND pm.date = ? AND p.status IN ('accepted','draft')
         ORDER BY CASE pm.period WHEN 'breakfast' THEN 1 WHEN 'lunch' THEN 2 WHEN 'dinner' THEN 3 ELSE 4 END, pm.created_at",
    )
    .bind(hid)
    .bind(&today_s)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let mut today_meals = Vec::new();
    let mut planned: std::collections::HashMap<String, PlannedToday> = std::collections::HashMap::new();
    for r in meal_rows {
        let period: String = r.get("period");
        let recipe_id: String = r.get("recipe_id");
        let name_ar: String = r.get("name");
        let prepared = r.get::<String, _>("status") == "prepared";
        today_meals.push(TodayMeal {
            period: period.clone(),
            recipe_name_ar: name_ar.clone(),
            prepared,
        });
        planned.entry(period).or_insert(PlannedToday {
            recipe_id,
            name_ar,
            prepared,
        });
    }

    let budget = budget::budget_status(pool, hid, "weekly").await.ok();

    // Shopping list status.
    let list = shopping::active_list(pool, hid).await?;
    let shopping_list_status = list.as_ref().map(|l| ShoppingSummary {
        total_items: l.items.len() as i64,
        purchased_items: l.items.iter().filter(|i| i.purchased).count() as i64,
        estimated_total_minor: l.estimated_total_minor,
        list_id: l.id.clone(),
    });

    // Next shopping date heuristic: weekly cadence from household setting.
    let next_shopping_date = match shopping::load_household(pool, hid).await?.shopping_frequency.as_str() {
        "twice_week" => Some((today() + chrono::Duration::days(3)).format("%Y-%m-%d").to_string()),
        "monthly" => Some((today() + chrono::Duration::days(30)).format("%Y-%m-%d").to_string()),
        _ => Some((today() + chrono::Duration::days(7 - today().weekday().num_days_from_monday() as i64))
            .format("%Y-%m-%d")
            .to_string()),
    };

    let expiring_stock = inventory::expiring_soon(pool, hid, 3).await?;
    let freezer_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stock_lots l JOIN storage_locations s ON s.id = l.location_id
         WHERE l.household_id = ? AND l.status = 'active' AND s.kind = 'freezer'",
    )
    .bind(hid)
    .fetch_one(pool)
    .await
    .map_err(map_db)?;
    let pantry_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stock_lots l JOIN storage_locations s ON s.id = l.location_id
         WHERE l.household_id = ? AND l.status = 'active' AND s.kind = 'pantry'",
    )
    .bind(hid)
    .fetch_one(pool)
    .await
    .map_err(map_db)?;

    // Recent relevant price observations — preference to confirmed locations in
    // the household city, otherwise latest observations with a caveat.
    let hh_city: Option<String> = sqlx::query_scalar("SELECT city FROM households WHERE id = ?")
        .bind(hid)
        .fetch_one(pool)
        .await
        .map_err(map_db)?;
    let rows = sqlx::query(
        "SELECT o.amount_minor, o.unit, o.observation_date, o.review_status, p.name_ar AS product, m.name AS merchant
         FROM price_observations o
         LEFT JOIN products p ON p.id = o.product_id
         LEFT JOIN merchants m ON m.id = o.merchant_id
         WHERE o.amount_minor IS NOT NULL
         ORDER BY o.submitted_at DESC LIMIT 5",
    )
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let recent_prices = rows
        .into_iter()
        .map(|r| RecentPrice {
            product_name_ar: r.get("product"),
            amount_minor: r.get("amount_minor"),
            unit: r.get("unit"),
            observation_date: r.get("observation_date"),
            review_status: r.get("review_status"),
            merchant_name: r.get("merchant"),
        })
        .collect();

    let suggestion = if !expiring_stock.is_empty() {
        format!(
            "لديك {} صنف قارب على الانتهاء — استخدمه قبل الشراء",
            expiring_stock.len()
        )
    } else if list.as_ref().map(|l| l.items.iter().any(|i| !i.purchased)).unwrap_or(false) {
        "قائمة المشتريات جاهزة — أكمل الشراء وسجل الأسعار الفعلية".into()
    } else {
        "أنت جاهز! راجع خطة الأسبوع أو استكشف وصفات اقتصادية".into()
    };
    let _ = hh_city;

    // Structured day table. Today's planned dishes are the mains when present;
    // bread, adama, salad and snack components come from the tagged system
    // library. Rotation by day-of-year keeps suggestions varied but deterministic.
    let all_recipes = recipes::list_recipes(pool, hid, None, None, 500, 0).await?;
    let rotate = today().ordinal() as usize;
    let mut today_table: Vec<TodayCourse> = Vec::new();

    // Breakfast: planned dish (or a suggested one) + bread + at least two adamas.
    let b_planned = planned.get("breakfast");
    let mut b_main = b_planned.map(|p| TodayDish {
        recipe_id: Some(p.recipe_id.clone()),
        name_ar: p.name_ar.clone(),
        role: "dish".into(),
        suggested: false,
    });
    if b_main.is_none() {
        b_main = pick_main(&all_recipes, "breakfast", rotate, &[]).map(|r| dish(r, "dish", true));
    }
    let mut b_exclude: Vec<String> = Vec::new();
    if let Some(m) = &b_main {
        if let Some(id) = &m.recipe_id {
            b_exclude.push(id.clone());
        }
    }
    let b_bread = pick_by_tag(&all_recipes, "bread", rotate, &b_exclude);
    if let Some(r) = b_bread {
        b_exclude.push(r.id.clone());
    }
    let b_adamas = pick_many_by_tag(&all_recipes, "adama", rotate, &b_exclude, 2);
    let mut b_sides = Vec::new();
    if let Some(r) = b_bread {
        b_sides.push(dish(r, "bread", false));
    }
    for r in b_adamas {
        b_sides.push(dish(r, "adama", false));
    }
    today_table.push(TodayCourse {
        key: "breakfast".into(),
        period: Some("breakfast".into()),
        main: b_main,
        sides: b_sides,
        suggested: b_planned.is_none(),
        prepared: b_planned.map(|p| p.prepared).unwrap_or(false),
    });

    // Lunch: planned dish (or suggested) + rice when it is not a rice dish + salad.
    let l_planned = planned.get("lunch");
    let mut l_main = l_planned.map(|p| TodayDish {
        recipe_id: Some(p.recipe_id.clone()),
        name_ar: p.name_ar.clone(),
        role: "dish".into(),
        suggested: false,
    });
    if l_main.is_none() {
        l_main = pick_main(&all_recipes, "lunch", rotate, &[]).map(|r| dish(r, "dish", true));
    }
    let mut l_exclude: Vec<String> = Vec::new();
    if let Some(m) = &l_main {
        if let Some(id) = &m.recipe_id {
            l_exclude.push(id.clone());
        }
    }
    let mut l_sides = Vec::new();
    let already_rice = l_main
        .as_ref()
        .map(|m| main_is_rice(&m.name_ar))
        .unwrap_or(false);
    if !already_rice {
        if let Some(r) = pick_by_tag(&all_recipes, "rice", rotate, &l_exclude) {
            l_sides.push(dish(r, "staple", false));
            l_exclude.push(r.id.clone());
        }
    }
    if let Some(r) = pick_by_tag(&all_recipes, "salad", rotate, &l_exclude) {
        l_sides.push(dish(r, "salad", false));
    }
    today_table.push(TodayCourse {
        key: "lunch".into(),
        period: Some("lunch".into()),
        main: l_main,
        sides: l_sides,
        suggested: l_planned.is_none(),
        prepared: l_planned.map(|p| p.prepared).unwrap_or(false),
    });

    // Dinner: light — planned/suggested dish + bread + one adama + salad.
    let d_planned = planned.get("dinner");
    let mut d_main = d_planned.map(|p| TodayDish {
        recipe_id: Some(p.recipe_id.clone()),
        name_ar: p.name_ar.clone(),
        role: "dish".into(),
        suggested: false,
    });
    if d_main.is_none() {
        d_main = pick_main(&all_recipes, "dinner", rotate + 1, &[])
            .map(|r| dish(r, "dish", true));
    }
    let mut d_exclude: Vec<String> = Vec::new();
    if let Some(m) = &d_main {
        if let Some(id) = &m.recipe_id {
            d_exclude.push(id.clone());
        }
    }
    let d_bread = pick_by_tag(&all_recipes, "bread", rotate + 1, &d_exclude);
    if let Some(r) = d_bread {
        d_exclude.push(r.id.clone());
    }
    let d_adama = pick_by_tag(&all_recipes, "adama", rotate + 1, &d_exclude);
    let mut d_sides = Vec::new();
    if let Some(r) = d_bread {
        d_sides.push(dish(r, "bread", false));
    }
    if let Some(r) = d_adama {
        d_sides.push(dish(r, "adama", false));
    }
    today_table.push(TodayCourse {
        key: "dinner".into(),
        period: Some("dinner".into()),
        main: d_main,
        sides: d_sides,
        suggested: d_planned.is_none(),
        prepared: d_planned.map(|p| p.prepared).unwrap_or(false),
    });

    // Snacks between breakfast/lunch and lunch/dinner: fruit then vegetables.
    let fruit_snack = pick_by_tag(&all_recipes, "fruit_snack", rotate, &[]);
    let veg_snack = pick_by_tag(&all_recipes, "veg_snack", rotate, &[]);
    today_table.insert(
        1,
        TodayCourse {
            key: "morning_snack".into(),
            period: None,
            main: fruit_snack.map(|r| dish(r, "snack", true)),
            sides: Vec::new(),
            suggested: true,
            prepared: false,
        },
    );
    today_table.insert(
        3,
        TodayCourse {
            key: "afternoon_snack".into(),
            period: None,
            main: veg_snack.map(|r| dish(r, "snack", true)),
            sides: Vec::new(),
            suggested: true,
            prepared: false,
        },
    );

    // Easy homemade basics worth surfacing on the home page.
    let mut quick_basics: Vec<TodayDish> = Vec::new();
    let mut seen_basics = std::collections::HashSet::new();
    for (tag, role) in [
        ("bread", "bread"),
        ("rice", "staple"),
        ("salad", "salad"),
        ("adama", "adama"),
        ("fruit_snack", "snack"),
        ("veg_snack", "snack"),
    ] {
        if let Some(r) = pick_by_tag(&all_recipes, tag, rotate, &[]) {
            if seen_basics.insert(r.id.clone()) {
                quick_basics.push(dish(r, role, false));
            }
        }
    }

    Ok(Dashboard {
        today_meals,
        today_table,
        quick_basics,
        budget,
        next_shopping_date,
        shopping_list_status,
        expiring_stock,
        freezer_count,
        pantry_count,
        recent_prices,
        suggestion,
    })
}
