use belhana_common::{CommonError, Money};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{households::Household, map_db, new_id, now_rfc3339, shopping, today};

#[derive(Debug, Serialize)]
pub struct BudgetStatus {
    pub budget_minor: i64,
    pub actual_spent_minor: i64,
    pub planned_cost_minor: i64,
    pub remaining_minor: i64,
    pub period: String,
    pub period_start: String,
    pub period_end: String,
    pub actual_detail: Vec<CategorySpend>,
}

#[derive(Debug, Serialize)]
pub struct CategorySpend {
    pub category: String,
    pub amount_minor: i64,
}

/// Resolves the effective budget for a period from the household profile.
pub fn effective_budget(hh: &Household, period: &str) -> Option<i64> {
    match period {
        "daily" => hh.daily_budget_minor,
        "weekly" => hh.weekly_budget_minor,
        "monthly" => hh.monthly_budget_minor,
        "occasion" => hh.occasion_budget_minor,
        "snacks" => hh.snacks_budget_minor,
        _ => None,
    }
}

fn period_bounds(period: &str, today: chrono::NaiveDate) -> (chrono::NaiveDate, chrono::NaiveDate) {
    match period {
        "daily" => (today, today),
        "monthly" => {
            let start = today.with_day(1).unwrap();
            let end = crate::planning::add_days(start, 30);
            let end = chrono::NaiveDate::from_ymd_opt(
                start.year(),
                match start.month() {
                    12 => 12,
                    m => m,
                },
                1,
            )
            .and_then(|d| d.checked_add_months(chrono::Months::new(1)))
            .and_then(|d| d.checked_add_days(chrono::Days::new(0)))
            .unwrap_or(end)
            .checked_add_days(chrono::Days::new(0))
            .unwrap_or(end);
            // last day of month
            let end = start
                .checked_add_months(chrono::Months::new(1))
                .map(|n| n.checked_sub_days(chrono::Days::new(1)).unwrap())
                .unwrap_or(end);
            (start, end)
        }
        // weekly (default) & trip: current ISO week
        _ => {
            let days_from_monday = today.weekday().num_days_from_monday() as i64;
            let start = today - chrono::Duration::days(days_from_monday);
            let end = start + chrono::Duration::days(6);
            (start, end)
        }
    }
}

use chrono::Datelike;

/// Computes budget status: budget − actual spending − planned-but-unpurchased cost.
pub async fn budget_status(
    pool: &SqlitePool,
    household_id: &str,
    period: &str,
) -> Result<BudgetStatus, CommonError> {
    let hh = crate::households::get_household(pool, household_id).await?;
    let budget_minor = effective_budget(&hh, period)
        .ok_or_else(|| CommonError::NotFound(format!("no {period} budget configured for household")))?;
    let (start, end) = period_bounds(period, today());
    let start_s = start.format("%Y-%m-%d").to_string();
    let end_s = end.format("%Y-%m-%d").to_string();

    let rows = sqlx::query(
        "SELECT category, COALESCE(SUM(amount_minor),0) AS total FROM actual_expenses
         WHERE household_id = ? AND date >= ? AND date <= ? GROUP BY category",
    )
    .bind(household_id)
    .bind(&start_s)
    .bind(&end_s)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let mut actual = 0i64;
    let mut detail = Vec::new();
    for r in rows {
        let t: i64 = r.get("total");
        actual += t;
        detail.push(CategorySpend { category: r.get("category"), amount_minor: t });
    }

    // Planned (estimated, not yet purchased) from active shopping lists in this window.
    let planned: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(CASE WHEN i.purchased = 0 THEN i.estimated_total_minor ELSE 0 END), 0)
         FROM shopping_list_items i JOIN shopping_lists l ON l.id = i.list_id
         WHERE l.household_id = ? AND l.status IN ('draft','active') AND i.status != 'removed'
           AND l.created_at >= ?",
    )
    .bind(household_id)
    .bind(&start_s)
    .fetch_one(pool)
    .await
    .map_err(map_db)?;

    Ok(BudgetStatus {
        budget_minor,
        actual_spent_minor: actual,
        planned_cost_minor: planned,
        remaining_minor: budget_minor - actual - planned,
        period: period.to_string(),
        period_start: start_s,
        period_end: end_s,
        actual_detail: detail,
    })
}

/// Daily/weekly/monthly history of actual spending.
pub async fn spending_history(
    pool: &SqlitePool,
    household_id: &str,
    days_back: i64,
) -> Result<Vec<(String, i64)>, CommonError> {
    let start = (today() - chrono::Duration::days(days_back)).format("%Y-%m-%d").to_string();
    let rows = sqlx::query(
        "SELECT date, SUM(amount_minor) AS total FROM actual_expenses
         WHERE household_id = ? AND date >= ? GROUP BY date ORDER BY date",
    )
    .bind(household_id)
    .bind(&start)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows
        .into_iter()
        .map(|r| (r.get::<String, _>("date"), r.get::<i64, _>("total")))
        .collect())
}

#[derive(Debug, Deserialize)]
pub struct WhatIfRequest {
    pub new_budget_minor: i64,
    pub days: Option<i64>,
    pub periods_enabled: Option<Vec<String>>,
    pub servings_per_meal: Option<i64>,
}

/// Budget what-if: re-runs the planner in memory against a new budget and
/// compares estimated totals. Nothing is persisted.
pub async fn what_if(
    pool: &SqlitePool,
    household_id: &str,
    req: &WhatIfRequest,
) -> Result<serde_json::Value, CommonError> {
    let hh = crate::households::get_household(pool, household_id).await?;
    let days = req.days.unwrap_or(7);
    let mut hh2 = hh.clone();
    hh2.weekly_budget_minor = Some(req.new_budget_minor);
    let plan = crate::planning::generate_plan_in_memory(
        pool,
        &hh2,
        days,
        req.periods_enabled.clone().unwrap_or_else(|| {
            vec!["breakfast".into(), "lunch".into(), "dinner".into()]
        }),
        req.servings_per_meal.unwrap_or(hh.household_size),
        today(),
        &[],
    )
    .await?;
    Ok(serde_json::json!({
        "new_budget_minor": req.new_budget_minor,
        "estimated_total_minor": plan.total_cost_minor,
        "fits_budget": plan.total_cost_minor <= req.new_budget_minor,
        "days": days,
        "slots": plan.slots.iter().map(|s| serde_json::json!({
            "date": s.date, "period": s.period, "recipe_name_ar": s.recipe_name_ar,
            "cost_minor": s.cost_minor, "explanation": s.explanation,
        })).collect::<Vec<_>>(),
        "conflicts": plan.conflicts,
    }))
}

/// Records a budget change in history for auditability.
pub async fn record_budget_change(
    pool: &SqlitePool,
    household_id: &str,
    period: &str,
    amount_minor: i64,
) -> Result<(), CommonError> {
    sqlx::query(
        "INSERT INTO budget_history (id, household_id, period, amount_minor, period_start, created_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(household_id)
    .bind(period)
    .bind(amount_minor)
    .bind(today().format("%Y-%m-%d").to_string())
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    Ok(())
}

/// A short display helper used by the dashboard.
pub fn money_str(minor: i64) -> String {
    Money::egp(minor).display(true)
}

#[allow(dead_code)]
async fn _unused_touch(pool: &SqlitePool) -> Result<(), CommonError> {
    shopping::touch_pool(pool).await
}
