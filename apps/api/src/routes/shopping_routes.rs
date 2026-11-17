//! Shopping lists: generate from plan, update items during the trip, complete.

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_core::shopping;

#[derive(Debug, Deserialize)]
pub struct GenerateListRequest {
    pub plan_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CompleteTripRequest {
    pub merchant_id: Option<String>,
}

async fn active_list(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = shopping::active_list(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

async fn generate(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<GenerateListRequest>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    let list = shopping::generate_from_plan(&state.pool, &user.household_id, &req.plan_id).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(list))))
}

async fn get_list(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = shopping::get_list(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!(list)))
}

async fn update_item(
    State(state): State<SharedState>,
    user: AuthUser,
    Path((list_id, item_id)): Path<(String, String)>,
    Json(req): Json<shopping::UpdateItemRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = shopping::update_item(&state.pool, &user.household_id, &list_id, &item_id, &req).await?;
    Ok(Json(json!(list)))
}

async fn complete_trip(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<CompleteTripRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = shopping::complete_trip(&state.pool, &user.household_id, &id, req.merchant_id.as_deref(), &user.user_id).await?;
    Ok(Json(json!(list)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/shopping-lists/active", get(active_list))
        .route("/api/shopping-lists", post(generate))
        .route("/api/shopping-lists/{id}", get(get_list))
        .route("/api/shopping-lists/{id}/items/{item_id}", post(update_item))
        .route("/api/shopping-lists/{id}/complete", post(complete_trip))
}
