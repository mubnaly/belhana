//! Budget status, spending history, and what-if simulation.

use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_core::budget;

#[derive(Deserialize)]
pub struct PeriodQuery {
    pub period: Option<String>,
}

#[derive(Deserialize)]
pub struct DaysQuery {
    pub days: Option<i64>,
}

async fn status(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<PeriodQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let period = q.period.unwrap_or_else(|| "weekly".into());
    let s = budget::budget_status(&state.pool, &user.household_id, &period).await?;
    Ok(Json(json!(s)))
}

async fn history(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<DaysQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let days = q.days.unwrap_or(30).clamp(1, 365);
    let rows = budget::spending_history(&state.pool, &user.household_id, days).await?;
    Ok(Json(json!(
        rows.into_iter()
            .map(|(date, minor)| json!({"date": date, "amount_minor": minor}))
            .collect::<Vec<_>>()
    )))
}

async fn what_if(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<budget::WhatIfRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let v = budget::what_if(&state.pool, &user.household_id, &req).await?;
    Ok(Json(v))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/budget/status", get(status))
        .route("/api/budget/history", get(history))
        .route("/api/budget/what-if", post(what_if))
}
