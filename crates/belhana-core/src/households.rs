use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Household {
    pub id: String,
    pub name: String,
    pub country: String,
    pub city: Option<String>,
    pub area: Option<String>,
    pub currency: String,
    pub household_size: i64,
    pub shopping_frequency: String,
    pub daily_budget_minor: Option<i64>,
    pub weekly_budget_minor: Option<i64>,
    pub monthly_budget_minor: Option<i64>,
    pub occasion_budget_minor: Option<i64>,
    pub snacks_budget_minor: Option<i64>,
    pub contingency_minor: Option<i64>,
    pub preferred_merchants: Option<String>,
    pub kitchen_equipment: Option<String>,
    pub notes: Option<String>,
    pub price_policy: String,
    pub consumption_workflow: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<sqlx::sqlite::SqliteRow> for Household {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        Household {
            id: r.get("id"),
            name: r.get("name"),
            country: r.get("country"),
            city: r.get("city"),
            area: r.get("area"),
            currency: r.get("currency"),
            household_size: r.get::<i64, _>("household_size"),
            shopping_frequency: r.get("shopping_frequency"),
            daily_budget_minor: r.get("daily_budget_minor"),
            weekly_budget_minor: r.get("weekly_budget_minor"),
            monthly_budget_minor: r.get("monthly_budget_minor"),
            occasion_budget_minor: r.get("occasion_budget_minor"),
            snacks_budget_minor: r.get("snacks_budget_minor"),
            contingency_minor: r.get("contingency_minor"),
            preferred_merchants: r.get("preferred_merchants"),
            kitchen_equipment: r.get("kitchen_equipment"),
            notes: r.get("notes"),
            price_policy: r.get("price_policy"),
            consumption_workflow: r.get("consumption_workflow"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateHousehold {
    pub name: String,
    pub country: Option<String>,
    pub city: Option<String>,
    pub area: Option<String>,
    pub currency: Option<String>,
    pub household_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHousehold {
    pub name: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub area: Option<String>,
    pub currency: Option<String>,
    pub household_size: Option<i64>,
    pub shopping_frequency: Option<String>,
    pub daily_budget_minor: Option<i64>,
    pub weekly_budget_minor: Option<i64>,
    pub monthly_budget_minor: Option<i64>,
    pub occasion_budget_minor: Option<i64>,
    pub snacks_budget_minor: Option<i64>,
    pub contingency_minor: Option<i64>,
    pub preferred_merchants: Option<String>,
    pub kitchen_equipment: Option<String>,
    pub notes: Option<String>,
    pub price_policy: Option<String>,
    pub consumption_workflow: Option<String>,
}

/// Creates a household and adds `owner_user_id` as its owner.
pub async fn create_household(
    pool: &SqlitePool,
    owner_user_id: &str,
    input: &CreateHousehold,
) -> Result<Household, CommonError> {
    if input.name.trim().is_empty() {
        return Err(CommonError::Validation("household name is required".into()));
    }
    let id = new_id();
    let now = now_rfc3339();
    sqlx::query(
        "INSERT INTO households (id, name, country, city, area, currency, household_size, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(input.name.trim())
    .bind(input.country.clone().unwrap_or_else(|| "EG".into()))
    .bind(&input.city)
    .bind(&input.area)
    .bind(input.currency.clone().unwrap_or_else(|| "EGP".into()))
    .bind(input.household_size.unwrap_or(1).max(1))
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_db)?;
    sqlx::query(
        "INSERT INTO household_memberships (id, household_id, user_id, role, created_at) VALUES (?, ?, ?, 'owner', ?)",
    )
    .bind(new_id())
    .bind(&id)
    .bind(owner_user_id)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_db)?;
    crate::audit::log(pool, Some(&id), Some(owner_user_id), "create", "household", &id, None, None).await?;
    get_household(pool, &id).await
}

pub async fn get_household(pool: &SqlitePool, id: &str) -> Result<Household, CommonError> {
    sqlx::query("SELECT * FROM households WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .map(Household::from)
        .ok_or_else(|| CommonError::NotFound("household".into()))
}

/// Households a user belongs to.
pub async fn list_households_for_user(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<Household>, CommonError> {
    let rows = sqlx::query(
        "SELECT h.* FROM households h
         JOIN household_memberships m ON m.household_id = h.id
         WHERE m.user_id = ? ORDER BY h.created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.into_iter().map(Household::from).collect())
}

pub async fn update_household(
    pool: &SqlitePool,
    id: &str,
    actor: &str,
    input: &UpdateHousehold,
) -> Result<Household, CommonError> {
    let current = get_household(pool, id).await?;
    let now = now_rfc3339();
    let name = input.name.clone().unwrap_or_else(|| current.name.clone());
    if name.trim().is_empty() {
        return Err(CommonError::Validation("household name cannot be empty".into()));
    }
    let size = input.household_size.unwrap_or(current.household_size).max(1);
    sqlx::query(
        "UPDATE households SET name=?, country=?, city=?, area=?, currency=?, household_size=?,
            shopping_frequency=?, daily_budget_minor=?, weekly_budget_minor=?, monthly_budget_minor=?,
            occasion_budget_minor=?, snacks_budget_minor=?, contingency_minor=?, preferred_merchants=?,
            kitchen_equipment=?, notes=?, price_policy=?, consumption_workflow=?, updated_at=? WHERE id=?",
    )
    .bind(&name)
    .bind(input.country.clone().unwrap_or(current.country.clone()))
    .bind(input.city.clone().or(current.city.clone()))
    .bind(input.area.clone().or(current.area.clone()))
    .bind(input.currency.clone().unwrap_or(current.currency.clone()))
    .bind(size)
    .bind(input.shopping_frequency.clone().unwrap_or(current.shopping_frequency.clone()))
    .bind(input.daily_budget_minor.or(current.daily_budget_minor))
    .bind(input.weekly_budget_minor.or(current.weekly_budget_minor))
    .bind(input.monthly_budget_minor.or(current.monthly_budget_minor))
    .bind(input.occasion_budget_minor.or(current.occasion_budget_minor))
    .bind(input.snacks_budget_minor.or(current.snacks_budget_minor))
    .bind(input.contingency_minor.or(current.contingency_minor))
    .bind(input.preferred_merchants.clone().or(current.preferred_merchants.clone()))
    .bind(input.kitchen_equipment.clone().or(current.kitchen_equipment.clone()))
    .bind(input.notes.clone().or(current.notes.clone()))
    .bind(input.price_policy.clone().unwrap_or(current.price_policy.clone()))
    .bind(input.consumption_workflow.clone().unwrap_or(current.consumption_workflow.clone()))
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    crate::audit::log(pool, Some(id), Some(actor), "update", "household", id, None, None).await?;
    get_household(pool, id).await
}

/// Authorization check: does this user belong to this household?
pub async fn ensure_member(
    pool: &SqlitePool,
    user_id: &str,
    household_id: &str,
) -> Result<String, CommonError> {
    let role: Option<String> =
        sqlx::query_scalar("SELECT role FROM household_memberships WHERE user_id = ? AND household_id = ?")
            .bind(user_id)
            .bind(household_id)
            .fetch_optional(pool)
            .await
            .map_err(map_db)?;
    role.ok_or_else(|| CommonError::Forbidden("user is not a member of this household".into()))
}
