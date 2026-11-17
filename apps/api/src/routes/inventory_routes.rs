//! Inventory: locations, stock lots (FEFO), consumption, waste, valuation.

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::inventory;

#[derive(Deserialize)]
pub struct LotQuery {
    pub location_id: Option<String>,
    pub kind: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct ConsumeRequest {
    pub product_id: String,
    pub qty: f64,
    pub unit: String,
    pub reason: Option<String>,
    pub related_meal: Option<String>,
}

#[derive(Deserialize)]
pub struct AdjustRequest {
    pub new_qty: f64,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct MoveRequest {
    pub to_location: String,
}

#[derive(Deserialize)]
pub struct WasteRequest {
    pub product_id: String,
    pub qty: f64,
    pub unit: String,
    pub reason: Option<String>,
    pub estimated_cost_minor: Option<i64>,
}

#[derive(Deserialize)]
pub struct ExpiringQuery {
    pub days: Option<i64>,
}

#[derive(Deserialize)]
pub struct StockQuery {
    pub unit: Option<String>,
}

async fn list_locations(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = inventory::list_locations(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

async fn list_lots(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<LotQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = inventory::list_lots(
        &state.pool,
        &user.household_id,
        q.location_id.as_deref(),
        q.kind.as_deref(),
        q.status.as_deref(),
    )
    .await?;
    Ok(Json(json!(list)))
}

async fn get_lot(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let l = inventory::get_lot(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!(l)))
}

async fn add_lot(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<inventory::CreateLot>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    if input.quantity <= 0.0 {
        return Err(ApiError::from(CommonError::Validation("الكمية يجب أن تكون أكبر من صفر".into())));
    }
    inventory::ensure_default_locations(&state.pool, &user.household_id).await?;
    let l = inventory::add_lot(&state.pool, &user.household_id, &input, "manual").await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(l))))
}

async fn consume(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<ConsumeRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    if req.qty <= 0.0 {
        return Err(ApiError::from(CommonError::Validation("الكمية يجب أن تكون أكبر من صفر".into())));
    }
    let parts = inventory::consume_fefo(
        &state.pool,
        &user.household_id,
        &req.product_id,
        req.qty,
        &req.unit,
        req.reason.as_deref().unwrap_or("meal"),
        req.related_meal.as_deref(),
    )
    .await?;
    Ok(Json(json!(parts
        .into_iter()
        .map(|(lot_id, qty)| json!({"lot_id": lot_id, "qty": qty}))
        .collect::<Vec<_>>())))
}

async fn adjust_lot(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<AdjustRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let l = inventory::adjust_lot(
        &state.pool,
        &user.household_id,
        &id,
        req.new_qty,
        req.reason.as_deref().unwrap_or("adjust"),
    )
    .await?;
    Ok(Json(json!(l)))
}

async fn move_lot(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<MoveRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let l = inventory::move_lot(&state.pool, &user.household_id, &id, &req.to_location).await?;
    Ok(Json(json!(l)))
}

async fn update_lot(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<inventory::UpdateLot>,
) -> ApiResult<Json<serde_json::Value>> {
    let l = inventory::update_lot(&state.pool, &user.household_id, &id, &input).await?;
    Ok(Json(json!(l)))
}

async fn delete_lot(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    // archive rather than destroy: keep the movement history intact
    let lot = inventory::get_lot(&state.pool, &user.household_id, &id).await?;
    inventory::adjust_lot(&state.pool, &user.household_id, &id, 0.0, "removed").await?;
    let _ = lot;
    Ok(Json(json!({"ok": true})))
}

async fn waste(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<WasteRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    if req.qty <= 0.0 {
        return Err(ApiError::from(CommonError::Validation("الكمية يجب أن تكون أكبر من صفر".into())));
    }
    inventory::record_waste(
        &state.pool,
        &user.household_id,
        &req.product_id,
        req.qty,
        &req.unit,
        req.reason.as_deref(),
        req.estimated_cost_minor,
    )
    .await?;
    Ok(Json(json!({"ok": true})))
}

async fn expiring(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<ExpiringQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = inventory::expiring_soon(&state.pool, &user.household_id, q.days.unwrap_or(3).clamp(1, 30)).await?;
    Ok(Json(json!(list)))
}

async fn valuation(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let v = inventory::valuation(&state.pool, &user.household_id).await?;
    Ok(Json(json!({"valuation_minor": v})))
}

async fn product_stock(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(product_id): Path<String>,
    Query(q): Query<StockQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let unit = q.unit.unwrap_or_else(|| "g".into());
    let qty = inventory::product_stock(&state.pool, &user.household_id, &product_id, &unit).await?;
    Ok(Json(json!({"product_id": product_id, "qty": qty, "unit": unit})))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/inventory/locations", get(list_locations))
        .route("/api/inventory/lots", get(list_lots).post(add_lot))
        .route("/api/inventory/lots/{id}", get(get_lot).put(update_lot).delete(delete_lot))
        .route("/api/inventory/lots/{id}/adjust", post(adjust_lot))
        .route("/api/inventory/lots/{id}/move", post(move_lot))
        .route("/api/inventory/consume", post(consume))
        .route("/api/inventory/waste", post(waste))
        .route("/api/inventory/expiring", get(expiring))
        .route("/api/inventory/valuation", get(valuation))
        .route("/api/inventory/stock/{product_id}", get(product_stock))
}
