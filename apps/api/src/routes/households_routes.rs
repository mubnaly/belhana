//! Household profile, budgets, and preferences.

use axum::extract::State;
use axum::routing::{get, put};
use axum::{Json, Router};
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::households;

async fn get_household(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let hh = households::get_household(&state.pool, &user.household_id).await?;
    Ok(Json(json!(hh)))
}

async fn update_household(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<households::UpdateHousehold>,
) -> ApiResult<Json<serde_json::Value>> {
    let hh = households::update_household(&state.pool, &user.household_id, &user.user_id, &input).await?;
    Ok(Json(json!(hh)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/household", get(get_household))
        .route("/api/household", put(update_household))
}

/// Shared nested routes that need the household id path segment (used by admin
/// endpoints that operate on any household the user belongs to).
/// Shared nested routes kept for the multi-household admin flow (unused in
/// the MVP router but part of the public route surface).
#[allow(dead_code)]
pub async fn households_of_user(
    State(state): State<SharedState>,
    user: AuthUser,
) -> ApiResult<Json<serde_json::Value>> {
    let list = households::list_households_for_user(&state.pool, &user.user_id).await?;
    Ok(Json(json!(list)))
}

#[allow(dead_code)]
pub fn require_role(role: &str, needed: &str) -> Result<(), CommonError> {
    if role == "owner" || role == "admin" || role == needed {
        Ok(())
    } else {
        Err(CommonError::Forbidden("هذه العملية تتطلب صلاحيات أعلى".into()))
    }
}
