//! Belhana core domain logic: households, family, rules, budget, catalog,
//! pricing, imports, recipes, inventory, meal planning, shopping, occasions.

pub mod audit;
pub mod budget;
pub mod catalog;
pub mod costing;
pub mod dashboard;
pub mod households;
pub mod import_parse;
pub mod imports;
pub mod inventory;
pub mod members;
pub mod merchants;
pub mod nutrition;
pub mod occasions;
pub mod planning;
pub mod pricing;
pub mod recipes;
pub mod rules;
pub mod shopping;
pub mod tips;

use belhana_common::CommonError;
use chrono::Utc;

/// Common SQLx error mapping used by all core modules.
pub fn map_db(e: sqlx::Error) -> CommonError {
    match &e {
        sqlx::Error::RowNotFound => CommonError::NotFound("row not found".into()),
        sqlx::Error::Database(db) if db.code().as_deref() == Some("2067") => {
            CommonError::Conflict("unique constraint violated".into())
        }
        _ => CommonError::Internal(format!("database error: {e}")),
    }
}

pub fn now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn today() -> chrono::NaiveDate {
    Utc::now().date_naive()
}

pub fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
