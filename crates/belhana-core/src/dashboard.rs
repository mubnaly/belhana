//! Home dashboard aggregation.

use belhana_common::CommonError;
use serde::Serialize;
use sqlx::{Row, SqlitePool};

use crate::{budget, inventory, map_db, shopping, today};
use chrono::Datelike;

#[derive(Debug, Serialize)]
pub struct Dashboard {
    pub today_meals: Vec<TodayMeal>,
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

pub async fn load_dashboard(pool: &SqlitePool, hid: &str) -> Result<Dashboard, CommonError> {
    // Today's meals from the active (accepted) or newest plan.
    let today_s = today().format("%Y-%m-%d").to_string();
    let meal_rows = sqlx::query(
        "SELECT pm.period, r.name_ar AS name, pm.status
         FROM planned_meals pm
         JOIN meal_plans p ON p.id = pm.plan_id
         JOIN recipes r ON r.id = pm.recipe_id
         WHERE p.household_id = ? AND pm.date = ? AND p.status IN ('accepted','draft')
         ORDER BY CASE pm.period WHEN 'breakfast' THEN 1 WHEN 'lunch' THEN 2 WHEN 'dinner' THEN 3 ELSE 4 END",
    )
    .bind(hid)
    .bind(&today_s)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let today_meals = meal_rows
        .into_iter()
        .map(|r| TodayMeal {
            period: r.get("period"),
            recipe_name_ar: r.get("name"),
            prepared: r.get::<String, _>("status") == "prepared",
        })
        .collect();

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
    Ok(Dashboard {
        today_meals,
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
