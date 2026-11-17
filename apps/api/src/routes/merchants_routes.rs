//! Merchants (markets/grocers) and their locations.

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::merchants;

#[derive(Deserialize)]
pub struct MerchantQuery {
    pub search: Option<String>,
}

async fn list_merchants(
    State(state): State<SharedState>,
    Query(q): Query<MerchantQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = merchants::list_merchants(&state.pool, q.search.as_deref()).await?;
    Ok(Json(json!(list)))
}

async fn get_merchant(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let m = merchants::get_merchant(&state.pool, &id).await?;
    Ok(Json(json!(m)))
}

async fn create_merchant(
    State(state): State<SharedState>,
    _user: AuthUser,
    Json(input): Json<merchants::CreateMerchant>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    if input.name.trim().is_empty() {
        return Err(ApiError::from(CommonError::Validation("اسم التاجر مطلوب".into())));
    }
    let m = merchants::create_merchant(&state.pool, &input).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(m))))
}

async fn add_location(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<merchants::CreateLocation>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    let l = merchants::add_location(&state.pool, &id, &input).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(l))))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/merchants", get(list_merchants).post(create_merchant))
        .route("/api/merchants/{id}", get(get_merchant))
        .route("/api/merchants/{id}/locations", post(add_location))
}
