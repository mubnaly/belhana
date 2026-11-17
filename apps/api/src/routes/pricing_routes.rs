//! Price observations: list, submit, review workflow, and best-price lookup.

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_core::pricing;

#[derive(Deserialize)]
pub struct ObservationQuery {
    pub product_id: Option<String>,
    pub merchant_id: Option<String>,
    pub review_status: Option<String>,
    pub batch_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct BaselineRequest {
    pub product_id: String,
    pub amount_minor: i64,
    pub unit: String,
    pub observation_date: Option<String>,
}

async fn list_observations(
    State(state): State<SharedState>,
    _user: AuthUser,
    Query(q): Query<ObservationQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = pricing::list_observations(
        &state.pool,
        q.product_id.as_deref(),
        q.merchant_id.as_deref(),
        q.review_status.as_deref(),
        q.batch_id.as_deref(),
        q.limit.unwrap_or(50).clamp(1, 200),
        q.offset.unwrap_or(0).max(0),
    )
    .await?;
    Ok(Json(json!(list)))
}

async fn get_observation(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let o = pricing::get_observation(&state.pool, &id).await?;
    Ok(Json(json!(o)))
}

async fn create_observation(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<pricing::CreatePriceObservation>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    let o = pricing::create_observation(&state.pool, Some(&user.user_id), &input).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(o))))
}

async fn update_observation(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<pricing::UpdateObservation>,
) -> ApiResult<Json<serde_json::Value>> {
    let o = pricing::update_observation(&state.pool, &user.user_id, &id, &input).await?;
    Ok(Json(json!(o)))
}

async fn review_observation(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(action): Json<pricing::ReviewAction>,
) -> ApiResult<Json<serde_json::Value>> {
    let o = pricing::review_observation(&state.pool, &user.user_id, &id, &action).await?;
    Ok(Json(json!(o)))
}

async fn create_baseline(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<BaselineRequest>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    let o = pricing::create_baseline_estimate(
        &state.pool,
        &user.user_id,
        &req.product_id,
        req.amount_minor,
        &req.unit,
        req.observation_date.clone(),
    )
    .await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(o))))
}

async fn best_price(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(product_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let hh = belhana_core::shopping::load_household(&state.pool, &user.household_id).await?;
    let o = pricing::best_observation_for_product(&state.pool, &product_id, &hh.price_policy).await?;
    Ok(Json(json!(o)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/prices", get(list_observations).post(create_observation))
        .route("/api/prices/{id}", get(get_observation).put(update_observation))
        .route("/api/prices/{id}/review", post(review_observation))
        .route("/api/prices/baseline", post(create_baseline))
        .route("/api/prices/best/{product_id}", get(best_price))
}
