//! Import batches from community price posts (Olad Zain style) and their
//! review workflow: confirm date, map products, accept/close batches.

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::imports;

#[derive(Debug, Deserialize)]
pub struct CreateBatchRequest {
    pub raw_text: String,
    pub source_type: Option<String>,
    pub source_url: Option<String>,
    pub merchant_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatusRequest {
    pub status: String,
}

/// POST /api/imports/parse — dry-run parse of a post without persisting.
async fn parse_preview(
    State(_state): State<SharedState>,
    _user: AuthUser,
    Json(req): Json<CreateBatchRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    if req.raw_text.trim().is_empty() {
        return Err(ApiError::from(CommonError::Validation("النص المُدخل فارغ".into())));
    }
    let parsed = belhana_core::import_parse::parse_post(&req.raw_text);
    Ok(Json(json!(parsed)))
}

async fn create_batch(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<CreateBatchRequest>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    if req.raw_text.trim().is_empty() {
        return Err(ApiError::from(CommonError::Validation("النص المُدخل فارغ".into())));
    }
    let (batch, obs) = imports::create_batch_from_text(
        &state.pool,
        &user.user_id,
        &req.raw_text,
        req.source_type.as_deref().unwrap_or("community_post"),
        req.source_url.as_deref(),
        req.merchant_id.as_deref(),
    )
    .await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({"batch": batch, "observations": obs}))))
}

async fn list_batches(State(state): State<SharedState>, _user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = imports::list_batches(&state.pool).await?;
    Ok(Json(json!(list)))
}

async fn get_batch(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let b = imports::get_batch(&state.pool, &id).await?;
    Ok(Json(json!(b)))
}

/// GET /api/imports/batches/{id}/observations - the price rows of this batch for review.
async fn batch_observations(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    imports::get_batch(&state.pool, &id).await?;
    let list = belhana_core::pricing::list_observations(&state.pool, None, None, None, Some(&id), 500, 0).await?;
    Ok(Json(json!(list)))
}

async fn confirm_date(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<imports::ConfirmDateRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    imports::confirm_batch_date(&state.pool, &user.user_id, &id, &req).await?;
    let b = imports::get_batch(&state.pool, &id).await?;
    let post = imports::get_source_post(&state.pool, &b.source_post_id).await?;
    Ok(Json(json!({"batch": b, "source_post": post})))
}

async fn source_post(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let b = imports::get_batch(&state.pool, &id).await?;
    let post = imports::get_source_post(&state.pool, &b.source_post_id).await?;
    Ok(Json(json!(post)))
}

async fn set_status(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<StatusRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let b = imports::set_batch_status(&state.pool, &user.user_id, &id, &req.status).await?;
    Ok(Json(json!(b)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/imports/parse", post(parse_preview))
        .route("/api/imports/batches", get(list_batches).post(create_batch))
        .route("/api/imports/batches/{id}", get(get_batch))
        .route("/api/imports/batches/{id}/observations", get(batch_observations))
        .route("/api/imports/batches/{id}/source-post", get(source_post))
        .route("/api/imports/batches/{id}/confirm-date", post(confirm_date))
        .route("/api/imports/batches/{id}/status", post(set_status))
}
