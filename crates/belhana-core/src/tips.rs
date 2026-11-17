//! Practical meal-making tips and tricks.
//!
//! Tips are editorial guidance only: they never invent prices, quantities or
//! nutrition claims. System tips are global; households may add their own.

use belhana_common::CommonError;
use serde::Serialize;
use sqlx::{Row, SqlitePool};

use crate::map_db;

#[derive(Debug, Clone, Serialize)]
pub struct MealTip {
    pub id: String,
    pub household_id: Option<String>,
    pub title_ar: String,
    pub title_en: String,
    pub category: String,
    pub body_ar: String,
    pub body_en: String,
    pub sort: i64,
    pub is_system: bool,
}

impl From<sqlx::sqlite::SqliteRow> for MealTip {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        MealTip {
            id: r.get("id"),
            household_id: r.get("household_id"),
            title_ar: r.get("title_ar"),
            title_en: r.get("title_en"),
            category: r.get("category"),
            body_ar: r.get("body_ar"),
            body_en: r.get("body_en"),
            sort: r.get::<Option<i64>, _>("sort").unwrap_or(0),
            is_system: r.get::<i64, _>("is_system") != 0,
        }
    }
}

/// Lists tips visible to a household: system tips plus the household's own.
pub async fn list_tips(
    pool: &SqlitePool,
    household_id: &str,
    category: Option<&str>,
) -> Result<Vec<MealTip>, CommonError> {
    let mut sql = String::from(
        "SELECT * FROM meal_tips
         WHERE (is_system = 1 OR household_id = ?)",
    );
    if category.is_some() {
        sql.push_str(" AND category = ?");
    }
    sql.push_str(" ORDER BY sort, title_ar");
    let mut q = sqlx::query(&sql).bind(household_id);
    if let Some(c) = category {
        q = q.bind(c);
    }
    let rows = q.fetch_all(pool).await.map_err(map_db)?;
    Ok(rows.into_iter().map(MealTip::from).collect())
}

pub async fn categories(pool: &SqlitePool, household_id: &str) -> Result<Vec<String>, CommonError> {
    let rows = sqlx::query(
        "SELECT DISTINCT category FROM meal_tips
         WHERE (is_system = 1 OR household_id = ?) ORDER BY category",
    )
    .bind(household_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.into_iter().map(|r| r.get("category")).collect())
}
