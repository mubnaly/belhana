//! Household rules (hard constraints, dietary rules, budget rules...).

use axum::extract::{Path, State};
use axum::routing::{get, put};
use axum::{Json, Router};
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_core::rules;

#[derive(serde::Deserialize)]
pub struct EnableBody {
    pub enabled: bool,
}

async fn list_rules(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = rules::list_rules(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

async fn create_rule(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<rules::CreateRule>,
) -> ApiResult<Json<serde_json::Value>> {
    let r = rules::create_rule(&state.pool, &user.household_id, &input).await?;
    Ok(Json(json!(r)))
}

async fn get_rule(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let r = rules::get_rule(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!(r)))
}

async fn update_rule(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<rules::CreateRule>,
) -> ApiResult<Json<serde_json::Value>> {
    let r = rules::update_rule(&state.pool, &user.household_id, &id, &input).await?;
    Ok(Json(json!(r)))
}

async fn set_enabled(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(body): Json<EnableBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let r = rules::set_rule_enabled(&state.pool, &user.household_id, &id, body.enabled).await?;
    Ok(Json(json!(r)))
}

async fn delete_rule(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    rules::delete_rule(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!({"ok": true})))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/rules", get(list_rules).post(create_rule))
        .route("/api/rules/{id}", get(get_rule).put(update_rule).delete(delete_rule))
        .route("/api/rules/{id}/enabled", put(set_enabled))
}
