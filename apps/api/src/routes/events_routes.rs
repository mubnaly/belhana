//! Occasions (events): create, generate 3-tier menus, approve, event shopping,
//! actual costs, planned-vs-actual comparison.

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::occasions;

#[derive(Debug, Deserialize)]
pub struct ActualCostRequest {
    pub category: String,
    pub amount_minor: i64,
    pub note: Option<String>,
}

async fn list_events(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = occasions::list_events(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

async fn get_event(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let e = occasions::get_event(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!(e)))
}

async fn create_event(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<occasions::CreateEvent>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    if input.name.trim().is_empty() {
        return Err(ApiError::from(CommonError::Validation("اسم المناسبة مطلوب".into())));
    }
    let e = occasions::create_event(&state.pool, &user.household_id, &input).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(e))))
}

async fn update_event(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<occasions::UpdateEvent>,
) -> ApiResult<Json<serde_json::Value>> {
    let e = occasions::update_event(&state.pool, &user.household_id, &id, &input).await?;
    Ok(Json(json!(e)))
}

async fn generate_menus(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let menus = occasions::generate_menus(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!(menus)))
}

async fn list_menus(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    occasions::get_event(&state.pool, &user.household_id, &id).await?;
    let menus = occasions::list_menus(&state.pool, &id).await?;
    Ok(Json(json!(menus)))
}

async fn get_menu(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(menu_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let m = occasions::get_menu(&state.pool, &menu_id).await?;
    Ok(Json(json!(m)))
}

async fn approve_menu(
    State(state): State<SharedState>,
    user: AuthUser,
    Path((event_id, menu_id)): Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    let m = occasions::approve_menu(&state.pool, &user.household_id, &event_id, &menu_id).await?;
    Ok(Json(json!(m)))
}

async fn event_shopping(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    let v = occasions::generate_event_shopping(&state.pool, &user.household_id, &id).await?;
    Ok((axum::http::StatusCode::CREATED, Json(v)))
}

async fn actual_cost(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<ActualCostRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    if req.amount_minor <= 0 {
        return Err(ApiError::from(CommonError::Validation("المبلغ يجب أن يكون أكبر من صفر".into())));
    }
    let e = occasions::record_actual_cost(
        &state.pool,
        &user.household_id,
        &id,
        &req.category,
        req.amount_minor,
        req.note.as_deref(),
    )
    .await?;
    Ok(Json(json!(e)))
}

async fn planned_vs_actual(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let v = occasions::planned_vs_actual(&state.pool, &user.household_id, &id).await?;
    Ok(Json(v))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/events", get(list_events).post(create_event))
        .route("/api/events/{id}", get(get_event).put(update_event))
        .route("/api/events/{id}/menus", get(list_menus).post(generate_menus))
        .route("/api/events/{id}/menus/approve/{menu_id}", post(approve_menu))
        .route("/api/events/{id}/shopping", post(event_shopping))
        .route("/api/events/{id}/actual-costs", post(actual_cost))
        .route("/api/events/{id}/planned-vs-actual", get(planned_vs_actual))
        .route("/api/event-menus/{menu_id}", get(get_menu))
}
