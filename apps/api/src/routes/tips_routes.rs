//! Tips & tricks endpoints.

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_core::tips;

#[derive(Deserialize)]
pub struct TipQuery {
    pub category: Option<String>,
}

async fn list(State(state): State<SharedState>, user: AuthUser, Query(q): Query<TipQuery>) -> ApiResult<Json<serde_json::Value>> {
    let list = tips::list_tips(&state.pool, &user.household_id, q.category.as_deref()).await?;
    Ok(Json(json!(list)))
}

async fn list_categories(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = tips::categories(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/meal-tips", get(list))
        .route("/api/meal-tips/categories", get(list_categories))
}
