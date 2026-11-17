//! Weekly meal planning: generate (preview), save (accept), read, and
//! mark-as-prepared (which consumes inventory FEFO).

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::{households, inventory, planning};

#[derive(Debug, Deserialize)]
pub struct GeneratePlanRequest {
    pub days: Option<i64>,
    pub periods_enabled: Option<Vec<String>>,
    pub servings_per_meal: Option<i64>,
    pub start_date: Option<String>,
    pub locked: Option<Vec<planning::LockedSlot>>,
    /// When true (default) the plan is saved immediately as an accepted plan.
    pub save: Option<bool>,
}

#[derive(Deserialize)]
pub struct ListPlansQuery {
    pub status: Option<String>,
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, ApiError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| ApiError::from(CommonError::Validation("تنسيق التاريخ يجب أن يكون YYYY-MM-DD".into())))
}

async fn list_plans(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(q): Query<ListPlansQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "SELECT id, start_date, end_date, days, status, servings_per_meal, estimated_total_cost_minor, created_at
         FROM meal_plans WHERE household_id = ? AND (? IS NULL OR status = ?)
         ORDER BY created_at DESC LIMIT 50",
    )
    .bind(&user.household_id)
    .bind(q.status.as_deref())
    .bind(q.status.as_deref())
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    Ok(Json(json!(rows
        .iter()
        .map(|r| json!({
            "id": r.get::<String, _>("id"),
            "start_date": r.get::<String, _>("start_date"),
            "end_date": r.get::<String, _>("end_date"),
            "days": r.get::<i64, _>("days"),
            "status": r.get::<String, _>("status"),
            "servings_per_meal": r.get::<i64, _>("servings_per_meal"),
            "estimated_total_cost_minor": r.get::<Option<i64>, _>("estimated_total_cost_minor"),
            "created_at": r.get::<String, _>("created_at"),
        }))
        .collect::<Vec<_>>())))
}

async fn generate(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<GeneratePlanRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    if req.days.is_some() && !(1..=7).contains(&req.days.unwrap()) {
        return Err(ApiError::from(CommonError::Validation("عدد الأيام يجب أن يكون من 1 إلى 7".into())));
    }
    let hh = households::get_household(&state.pool, &user.household_id).await?;
    inventory::ensure_default_locations(&state.pool, &user.household_id).await?;
    let days = req.days.unwrap_or(7).clamp(1, 7);
    let periods = req
        .periods_enabled
        .clone()
        .unwrap_or_else(|| vec!["breakfast".into(), "lunch".into(), "dinner".into()]);
    let servings = req.servings_per_meal.unwrap_or(hh.household_size.max(1));
    let start = match &req.start_date {
        Some(s) => parse_date(s)?,
        None => belhana_core::today(),
    };
    let locked = req.locked.clone().unwrap_or_default();
    let out = planning::generate_plan_in_memory(
        &state.pool, &hh, days, periods.clone(), servings, start, &locked,
    )
    .await?;
    if req.save.unwrap_or(true) {
        let plan_id = planning::save_plan(
            &state.pool, &hh, &out, days, &periods, servings, start, &user.user_id, &locked,
        )
        .await?;
        let saved = planning::read_plan(&state.pool, &plan_id).await?;
        return Ok(Json(json!({"plan_id": plan_id, "plan": saved})));
    }
    Ok(Json(json!({"plan": out})))
}

async fn get_plan(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let out = planning::read_plan(&state.pool, &id).await?;
    Ok(Json(json!(out)))
}

async fn mark_prepared(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(meal_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    planning::mark_prepared(&state.pool, &user.household_id, &meal_id).await?;
    Ok(Json(json!({"ok": true})))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/meal-plans", get(list_plans).post(generate))
        .route("/api/meal-plans/{id}", get(get_plan))
        .route("/api/planned-meals/{id}/prepared", post(mark_prepared))
}
