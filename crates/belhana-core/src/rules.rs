use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Clone, Serialize)]
pub struct HouseholdRule {
    pub id: String,
    pub household_id: String,
    pub rule_type: String,
    pub priority: String,
    pub params: Option<String>,
    pub enabled: bool,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<sqlx::sqlite::SqliteRow> for HouseholdRule {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        HouseholdRule {
            id: r.get("id"),
            household_id: r.get("household_id"),
            rule_type: r.get("rule_type"),
            priority: r.get("priority"),
            params: r.get("params"),
            enabled: r.get::<i64, _>("enabled") != 0,
            notes: r.get("notes"),
            created_at: r.get("created_at"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRule {
    pub rule_type: String,
    pub priority: String,
    pub params: serde_json::Value,
    pub notes: Option<String>,
}

const RULE_TYPES: &[&str] = &[
    "meal_repeat_limit",
    "dish_consecutive_limit",
    "min_legume_meals",
    "use_freezer_first",
    "member_dislike_avoid",
    "allergen_ban",
    "dish_frequency_limit",
    // Free-text sentence injected into the AI assistant's context.
    "ai_instruction",
    // Staple ingredient preferred for a meal period (rice at lunch, bread at
    // breakfast/dinner). Scored by the deterministic planner.
    "meal_staple",
    // Buy an amount of money's worth of a product (spices, e.g. 20 EGP salt).
    "fixed_spend",
    // Explicitly mark whether unused stock rolls into the following week.
    "leftover_carryover",
];

/// Structural validation of rule parameters, per rule type.
pub fn validate_rule(rule_type: &str, params: &serde_json::Value) -> Result<(), CommonError> {
    if !RULE_TYPES.contains(&rule_type) {
        return Err(CommonError::Validation(format!("unknown rule type: {rule_type}")));
    }
    match rule_type {
        "meal_repeat_limit" | "dish_frequency_limit" => {
            let obj = params.as_object().ok_or_else(|| CommonError::Validation("params must be an object".into()))?;
            if !obj.contains_key("maximum_occurrences") || !obj.contains_key("period_days") {
                return Err(CommonError::Validation(
                    "this rule needs maximum_occurrences and period_days".into(),
                ));
            }
        }
        "dish_consecutive_limit" => {
            let obj = params.as_object().ok_or_else(|| CommonError::Validation("params must be an object".into()))?;
            if !obj.contains_key("product_id") && !obj.contains_key("category_id") {
                return Err(CommonError::Validation(
                    "consecutive-days rule needs a product_id or category_id (e.g. fish)".into(),
                ));
            }
        }
        "min_legume_meals" => {
            let obj = params.as_object().ok_or_else(|| CommonError::Validation("params must be an object".into()))?;
            if !obj.contains_key("minimum_per_week") {
                return Err(CommonError::Validation("legume rule needs minimum_per_week".into()));
            }
        }
        "ai_instruction" => {
            let sentence = params["sentence"].as_str().unwrap_or("").trim();
            if sentence.is_empty() {
                return Err(CommonError::Validation(
                    "ai_instruction needs a non-empty sentence".into(),
                ));
            }
        }
        "meal_staple" => {
            let obj = params.as_object().ok_or_else(|| CommonError::Validation("params must be an object".into()))?;
            let period = params["period"].as_str().unwrap_or("");
            if !["breakfast", "lunch", "dinner"].contains(&period) {
                return Err(CommonError::Validation(
                    "meal_staple needs period = breakfast | lunch | dinner".into(),
                ));
            }
            if !obj.contains_key("product_id") {
                return Err(CommonError::Validation("meal_staple needs product_id".into()));
            }
        }
        "fixed_spend" => {
            let obj = params.as_object().ok_or_else(|| CommonError::Validation("params must be an object".into()))?;
            if !obj.contains_key("product_id") {
                return Err(CommonError::Validation("fixed_spend needs product_id".into()));
            }
            match params["amount_minor"].as_i64() {
                Some(amount) if amount > 0 => {}
                _ => {
                    return Err(CommonError::Validation(
                        "fixed_spend needs a positive amount_minor".into(),
                    ))
                }
            }
        }
        "leftover_carryover" => {}
        _ => {}
    }
    Ok(())
}

fn params_value(rule: &HouseholdRule) -> serde_json::Value {
    serde_json::from_str(rule.params.as_deref().unwrap_or("{}")).unwrap_or(serde_json::Value::Null)
}

/// Reads a string parameter from a stored rule (safe on malformed JSON).
pub fn rule_string(rule: &HouseholdRule, key: &str) -> Option<String> {
    params_value(rule)[key].as_str().map(|s| s.to_string())
}

/// Reads an integer parameter from a stored rule (safe on malformed JSON).
pub fn rule_i64(rule: &HouseholdRule, key: &str) -> Option<i64> {
    params_value(rule)[key].as_i64()
}

/// Enabled free-text sentences that must be included in the AI's context.
/// These are household instructions only — they never write data themselves.
pub async fn ai_instruction_sentences(
    pool: &SqlitePool,
    household_id: &str,
) -> Result<Vec<String>, CommonError> {
    let rules = list_rules(pool, household_id).await?;
    Ok(rules
        .into_iter()
        .filter(|r| r.enabled && r.rule_type == "ai_instruction")
        .filter_map(|r| rule_string(&r, "sentence"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

/// Enabled (period, product_id) staple preferences for the planner.
pub async fn meal_staples(
    pool: &SqlitePool,
    household_id: &str,
) -> Result<Vec<(String, String)>, CommonError> {
    let rules = list_rules(pool, household_id).await?;
    Ok(rules
        .into_iter()
        .filter(|r| r.enabled && r.rule_type == "meal_staple")
        .filter_map(|r| {
            let period = rule_string(&r, "period")?;
            let product_id = rule_string(&r, "product_id")?;
            Some((period, product_id))
        })
        .collect())
}

/// Enabled fixed-spend rules: buy a set amount of money of a product.
pub async fn fixed_spends(
    pool: &SqlitePool,
    household_id: &str,
) -> Result<Vec<(String, i64)>, CommonError> {
    let rules = list_rules(pool, household_id).await?;
    Ok(rules
        .into_iter()
        .filter(|r| r.enabled && r.rule_type == "fixed_spend")
        .filter_map(|r| {
            let product_id = rule_string(&r, "product_id")?;
            let amount = rule_i64(&r, "amount_minor")?;
            if amount > 0 {
                Some((product_id, amount))
            } else {
                None
            }
        })
        .collect())
}

pub async fn list_rules(pool: &SqlitePool, household_id: &str) -> Result<Vec<HouseholdRule>, CommonError> {
    let rows = sqlx::query("SELECT * FROM household_rules WHERE household_id = ? ORDER BY created_at")
        .bind(household_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    Ok(rows.into_iter().map(HouseholdRule::from).collect())
}

pub async fn create_rule(
    pool: &SqlitePool,
    household_id: &str,
    input: &CreateRule,
) -> Result<HouseholdRule, CommonError> {
    if input.priority != "hard" && input.priority != "soft" {
        return Err(CommonError::Validation("priority must be hard or soft".into()));
    }
    validate_rule(&input.rule_type, &input.params)?;
    let id = new_id();
    sqlx::query(
        "INSERT INTO household_rules (id, household_id, rule_type, priority, params, enabled, notes, created_at)
         VALUES (?, ?, ?, ?, ?, 1, ?, ?)",
    )
    .bind(&id)
    .bind(household_id)
    .bind(&input.rule_type)
    .bind(&input.priority)
    .bind(input.params.to_string())
    .bind(&input.notes)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    let row = sqlx::query("SELECT * FROM household_rules WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(map_db)?;
    Ok(HouseholdRule::from(row))
}

pub async fn update_rule(
    pool: &SqlitePool,
    household_id: &str,
    rule_id: &str,
    input: &CreateRule,
) -> Result<HouseholdRule, CommonError> {
    get_rule(pool, household_id, rule_id).await?;
    if input.priority != "hard" && input.priority != "soft" {
        return Err(CommonError::Validation("priority must be hard or soft".into()));
    }
    validate_rule(&input.rule_type, &input.params)?;
    sqlx::query(
        "UPDATE household_rules SET rule_type = ?, priority = ?, params = ?, notes = ? WHERE id = ?",
    )
    .bind(&input.rule_type)
    .bind(&input.priority)
    .bind(input.params.to_string())
    .bind(&input.notes)
    .bind(rule_id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_rule(pool, household_id, rule_id).await
}

pub async fn get_rule(pool: &SqlitePool, household_id: &str, id: &str) -> Result<HouseholdRule, CommonError> {
    let row = sqlx::query("SELECT * FROM household_rules WHERE id = ? AND household_id = ?")
        .bind(id)
        .bind(household_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("rule".into()))?;
    Ok(HouseholdRule::from(row))
}

pub async fn set_rule_enabled(
    pool: &SqlitePool,
    household_id: &str,
    id: &str,
    enabled: bool,
) -> Result<HouseholdRule, CommonError> {
    get_rule(pool, household_id, id).await?;
    sqlx::query("UPDATE household_rules SET enabled = ? WHERE id = ?")
        .bind(enabled as i64)
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    get_rule(pool, household_id, id).await
}

pub async fn delete_rule(pool: &SqlitePool, household_id: &str, id: &str) -> Result<(), CommonError> {
    get_rule(pool, household_id, id).await?;
    sqlx::query("DELETE FROM household_rules WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(())
}
