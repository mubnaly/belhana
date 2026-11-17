//! Product catalog and categories.

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::catalog;

#[derive(Deserialize)]
pub struct ProductQuery {
    pub category_id: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

async fn categories(State(state): State<SharedState>) -> ApiResult<Json<serde_json::Value>> {
    let list = catalog::list_categories(&state.pool).await?;
    Ok(Json(json!(list)))
}

async fn get_category(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let c = catalog::get_category(&state.pool, &id).await?;
    Ok(Json(json!(c)))
}

async fn products(
    State(state): State<SharedState>,
    Query(q): Query<ProductQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = catalog::list_products(
        &state.pool,
        q.category_id.as_deref(),
        q.search.as_deref(),
        q.limit.unwrap_or(50).clamp(1, 200),
        q.offset.unwrap_or(0).max(0),
    )
    .await?;
    Ok(Json(json!(list)))
}

async fn get_product(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let p = catalog::get_product(&state.pool, &id).await?;
    Ok(Json(json!(p)))
}

async fn create_product(
    State(state): State<SharedState>,
    _user: AuthUser,
    Json(input): Json<catalog::CreateProduct>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    if input.name_ar.trim().is_empty() {
        return Err(ApiError::from(CommonError::Validation("اسم المنتج بالعربية مطلوب".into())));
    }
    let p = catalog::create_product(&state.pool, &input).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!(p))))
}

async fn update_product(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<catalog::UpdateProduct>,
) -> ApiResult<Json<serde_json::Value>> {
    let p = catalog::update_product(&state.pool, &id, &input).await?;
    Ok(Json(json!(p)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/catalog/categories", get(categories))
        .route("/api/catalog/categories/{id}", get(get_category))
        .route("/api/catalog/products", get(products).post(create_product))
        .route("/api/catalog/products/{id}", get(get_product).put(update_product))
}
