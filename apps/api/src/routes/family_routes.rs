//! Family members with their preferences, allergies, and exclusions.

use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_core::members;

async fn list_members(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = members::list_members(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

async fn create_member(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<members::CreateMember>,
) -> ApiResult<Json<serde_json::Value>> {
    let m = members::create_member(&state.pool, &user.household_id, &input).await?;
    Ok(Json(json!(m)))
}

async fn get_member(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let m = members::get_member(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!(m)))
}

async fn update_member(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<members::CreateMember>,
) -> ApiResult<Json<serde_json::Value>> {
    let m = members::update_member(&state.pool, &user.household_id, &id, &input).await?;
    Ok(Json(json!(m)))
}

async fn delete_member(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    members::delete_member(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!({"ok": true})))
}

async fn list_preferences(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    // ensure the member belongs to this household before reading details
    members::get_member(&state.pool, &user.household_id, &id).await?;
    let list = members::list_preferences(&state.pool, &id).await?;
    Ok(Json(json!(list)))
}

async fn create_preference(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<members::CreatePreference>,
) -> ApiResult<Json<serde_json::Value>> {
    members::get_member(&state.pool, &user.household_id, &id).await?;
    let p = members::create_preference(&state.pool, &id, &input).await?;
    Ok(Json(json!(p)))
}

async fn delete_preference(
    State(state): State<SharedState>,
    user: AuthUser,
    Path((id, pref_id)): Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    members::get_member(&state.pool, &user.household_id, &id).await?;
    members::delete_preference(&state.pool, &id, &pref_id).await?;
    Ok(Json(json!({"ok": true})))
}

async fn list_allergies(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    members::get_member(&state.pool, &user.household_id, &id).await?;
    let list = members::list_allergies(&state.pool, &id).await?;
    Ok(Json(json!(list)))
}

async fn create_allergy(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(input): Json<members::CreateAllergy>,
) -> ApiResult<Json<serde_json::Value>> {
    members::get_member(&state.pool, &user.household_id, &id).await?;
    let a = members::create_allergy(&state.pool, &id, &input).await?;
    Ok(Json(json!(a)))
}

async fn delete_allergy(
    State(state): State<SharedState>,
    user: AuthUser,
    Path((id, allergy_id)): Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    members::get_member(&state.pool, &user.household_id, &id).await?;
    members::delete_allergy(&state.pool, &id, &allergy_id).await?;
    Ok(Json(json!({"ok": true})))
}

async fn list_exclusions(State(state): State<SharedState>, user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let list = members::list_exclusions(&state.pool, &user.household_id).await?;
    Ok(Json(json!(list)))
}

async fn create_exclusion(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(input): Json<members::CreateExclusion>,
) -> ApiResult<Json<serde_json::Value>> {
    let e = members::create_exclusion(&state.pool, &user.household_id, &input).await?;
    Ok(Json(json!(e)))
}

async fn delete_exclusion(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    members::delete_exclusion(&state.pool, &user.household_id, &id).await?;
    Ok(Json(json!({"ok": true})))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/family/members", get(list_members).post(create_member))
        .route("/api/family/members/{id}", get(get_member).put(update_member).delete(delete_member))
        .route("/api/family/members/{id}/preferences", get(list_preferences).post(create_preference))
        .route("/api/family/members/{id}/preferences/{pref_id}", delete(delete_preference))
        .route("/api/family/members/{id}/allergies", get(list_allergies).post(create_allergy))
        .route("/api/family/members/{id}/allergies/{allergy_id}", delete(delete_allergy))
        .route("/api/family/exclusions", get(list_exclusions).post(create_exclusion))
        .route("/api/family/exclusions/{id}", delete(delete_exclusion))
}
