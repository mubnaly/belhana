//! Recipes: list/create/get/delete, cost computation, and batch cooking guides.

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::{costing, recipes};

#[derive(Deserialize)]
pub struct RecipeQuery {
    pub meal_type: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CostQuery {
    pub servings: Option<i64>,
}

#[derive(Deserialize)]
pub struct CostBody {
    pub servings: Option<i64>,
}

async fn list_recipes(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<RecipeQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let list = recipes::list_recipes(
        &state.pool,
        &user.household_id,
        q.meal_type.as_deref(),
        q.search.as_deref(),
        q.limit.unwrap_or(50).clamp(1, 200),
        q.offset.unwrap_or(0).max(0),
    )
    .await?;
    Ok(Json(json!(list)))
}

async fn get_recipe(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let (r, ingredients, steps) = recipes::get_recipe(&state.pool, &id).await?;
    Ok(Json(json!({"recipe": r, "ingredients": ingredients, "steps": steps})))
}

async fn create_recipe(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<recipes::CreateRecipe>,
) -> ApiResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    if input.name_ar.trim().is_empty() {
        return Err(ApiError::from(CommonError::Validation("اسم الوصفة مطلوب".into())));
    }
    if input.ingredients.is_empty() {
        return Err(ApiError::from(CommonError::Validation("الوصفة تحتاج مكونًا واحدًا على الأقل".into())));
    }
    let id = recipes::create_recipe(&state.pool, Some(&user.household_id), Some(&user.user_id), &input).await?;
    let (r, ingredients, steps) = recipes::get_recipe(&state.pool, &id).await?;
    Ok((axum::http::StatusCode::CREATED, Json(json!({"recipe": r, "ingredients": ingredients, "steps": steps}))))
}

async fn delete_recipe(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    // only household-owned (non-system) recipes can be deleted
    let (r, _ing, _st) = recipes::get_recipe(&state.pool, &id).await?;
    if r.is_system {
        return Err(ApiError::from(CommonError::Forbidden("لا يمكن حذف الوصفات النظامية".into())));
    }
    if r.household_id.as_deref() != Some(user.household_id.as_str()) {
        return Err(ApiError::from(CommonError::Forbidden("هذه الوصفة لا تنتمي لأسرك".into())));
    }
    recipes::delete_recipe(&state.pool, &id).await?;
    Ok(Json(json!({"ok": true})))
}

/// GET computes the cost with the current default servings; POST accepts a body.
async fn compute_cost(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Query(q): Query<CostQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let hh = belhana_core::shopping::load_household(&state.pool, &user.household_id).await?;
    let (r, _ing, _st) = recipes::get_recipe(&state.pool, &id).await?;
    let servings = q.servings.unwrap_or(r.default_servings).max(1);
    let cost = costing::compute_recipe_cost(&state.pool, &id, servings, &hh.price_policy).await?;
    Ok(Json(json!(cost)))
}

async fn compute_cost_body(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(body): Json<CostBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let hh = belhana_core::shopping::load_household(&state.pool, &user.household_id).await?;
    let (r, _ing, _st) = recipes::get_recipe(&state.pool, &id).await?;
    let servings = body.servings.unwrap_or(r.default_servings).max(1);
    let cost = costing::compute_recipe_cost(&state.pool, &id, servings, &hh.price_policy).await?;
    Ok(Json(json!(cost)))
}

async fn batch_guides(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = recipes::list_batch_guides(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

async fn convert_guide(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<recipes::ConvertGuideRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let lot = recipes::convert_guide_to_inventory(&state.pool, &user.household_id, &id, &req).await?;
    Ok(Json(json!(lot)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/recipes", get(list_recipes).post(create_recipe))
        .route("/api/recipes/{id}", get(get_recipe).delete(delete_recipe))
        .route("/api/recipes/{id}/cost", get(compute_cost).post(compute_cost_body))
        .route("/api/batch-guides", get(batch_guides))
        .route("/api/batch-guides/{id}/convert", post(convert_guide))
}
