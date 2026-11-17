use belhana_common::CommonError;
use sqlx::SqlitePool;

use crate::new_id;
use crate::now_rfc3339;

/// Appends an audit log entry. Never fails the caller on logging error.
pub async fn log(
    pool: &SqlitePool,
    household_id: Option<&str>,
    actor_user_id: Option<&str>,
    action: &str,
    entity_type: &str,
    entity_id: &str,
    before: Option<serde_json::Value>,
    after: Option<serde_json::Value>,
) -> Result<(), CommonError> {
    let _ = sqlx::query(
        "INSERT INTO audit_logs (id, household_id, actor_user_id, action, entity_type, entity_id, before, after, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(household_id)
    .bind(actor_user_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(before.map(|v| v.to_string()))
    .bind(after.map(|v| v.to_string()))
    .bind(now_rfc3339())
    .execute(pool)
    .await;
    Ok(())
}
