//! Dashboard aggregation and audit log (for the settings screen).

use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::dashboard;

async fn get_dashboard(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let d = dashboard::load_dashboard(&state.pool, &user.household_id).await?;
    Ok(Json(json!(d)))
}

#[derive(Deserialize)]
pub struct LimitQuery {
    pub limit: Option<i64>,
}

async fn audit_log(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<LimitQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let limit = q.limit.unwrap_or(100).clamp(1, 500);
    let rows = sqlx::query(
        "SELECT action, entity_type, entity_id, actor_user_id, created_at
         FROM audit_logs WHERE household_id = ? ORDER BY created_at DESC LIMIT ?",
    )
    .bind(&user.household_id)
    .bind(limit)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    Ok(Json(json!(rows
        .iter()
        .map(|r| json!({
            "action": r.get::<String, _>("action"),
            "entity_type": r.get::<String, _>("entity_type"),
            "entity_id": r.get::<Option<String>, _>("entity_id"),
            "actor_user_id": r.get::<Option<String>, _>("actor_user_id"),
            "created_at": r.get::<String, _>("created_at"),
        }))
        .collect::<Vec<_>>())))
}

async fn notifications(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<LimitQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let limit = q.limit.unwrap_or(50).clamp(1, 200);
    let rows = sqlx::query(
        "SELECT id, kind, title, body, read, created_at
         FROM notifications WHERE household_id = ? ORDER BY created_at DESC LIMIT ?",
    )
    .bind(&user.household_id)
    .bind(limit)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    Ok(Json(json!(rows
        .iter()
        .map(|r| json!({
            "id": r.get::<String, _>("id"),
            "kind": r.get::<String, _>("kind"),
            "title": r.get::<String, _>("title"),
            "body": r.get::<Option<String>, _>("body"),
            "read": r.get::<i64, _>("read") != 0,
            "created_at": r.get::<String, _>("created_at"),
        }))
        .collect::<Vec<_>>())))
}

async fn healthz() -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(json!({"ok": true, "app": "belhana-api"})))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/dashboard", get(get_dashboard))
        .route("/api/audit", get(audit_log))
        .route("/api/notifications", get(notifications))
        .route("/healthz", get(healthz))
}
