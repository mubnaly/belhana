//! Registration, login, token refresh, logout, and current-user lookup.

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::error::{ApiError, ApiResult};
use crate::state::SharedState;
use belhana_common::CommonError;
use belhana_core::households;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub household_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct SwitchHouseholdRequest {
    pub household_id: String,
}

fn valid_email(email: &str) -> bool {
    let e = email.trim().to_lowercase();
    e.contains('@') && e.contains('.') && !e.contains(' ') && e.len() >= 6
}

fn issue_tokens(state: &SharedState, user_id: &str, household_id: &str, role: &str) -> ApiResult<serde_json::Value> {
    use chrono::{Duration, Utc};
    let now = Utc::now();
    let claims = belhana_auth::AccessClaims {
        sub: user_id.to_string(),
        household: household_id.to_string(),
        role: role.to_string(),
        exp: (now + Duration::minutes(belhana_auth::ACCESS_TOKEN_TTL_MINUTES)).timestamp(),
        iat: now.timestamp(),
    };
    let access = belhana_auth::encode_access_token(&claims, &state.config.jwt_secret)?;
    Ok(json!({
        "access_token": access,
        "token_type": "bearer",
        "expires_in": belhana_auth::ACCESS_TOKEN_TTL_MINUTES * 60,
    }))
}

async fn register(
    State(state): State<SharedState>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let email = req.email.trim().to_lowercase();
    if !valid_email(&email) {
        return Err(ApiError::from(CommonError::Validation("البريد الإلكتروني غير صالح".into())));
    }
    if req.password.len() < 8 {
        return Err(ApiError::from(CommonError::Validation(
            "كلمة المرور يجب ألا تقل عن 8 أحرف".into(),
        )));
    }
    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    if exists.is_some() {
        return Err(ApiError::from(CommonError::Conflict("هذا البريد مسجل بالفعل".into())));
    }
    let user_id = belhana_core::new_id();
    let display_name = req
        .display_name
        .clone()
        .unwrap_or_else(|| email.split('@').next().unwrap_or("user").to_string());
    let hash = belhana_auth::hash_password(&req.password)?;
    let now = belhana_core::now_rfc3339();
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, display_name, locale, role, created_at, updated_at)
         VALUES (?, ?, ?, ?, 'ar', 'user', ?, ?)",
    )
    .bind(&user_id)
    .bind(&email)
    .bind(&hash)
    .bind(&display_name)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;

    let hh_name = req
        .household_name
        .clone()
        .unwrap_or_else(|| format!("أسرة {}", display_name));
    let hh = households::create_household(
        &state.pool,
        &user_id,
        &households::CreateHousehold {
            name: hh_name,
            country: None,
            city: None,
            area: None,
            currency: None,
            household_size: None,
        },
    )
    .await?;
    let refresh = belhana_auth::create_session(&state.pool, &user_id, None).await?;
    let mut tokens = issue_tokens(&state, &user_id, &hh.id, "owner")?;
    tokens["refresh_token"] = json!(refresh);
    tokens["user"] = json!({"id": user_id, "email": email, "display_name": display_name});
    tokens["household"] = json!(hh);
    Ok((StatusCode::CREATED, Json(tokens)))
}

async fn login(State(state): State<SharedState>, Json(req): Json<LoginRequest>) -> ApiResult<Json<serde_json::Value>> {
    let email = req.email.trim().to_lowercase();
    let row: Option<(String, String)> =
        sqlx::query_as("SELECT id, password_hash FROM users WHERE email = ?")
            .bind(&email)
            .fetch_optional(&state.pool)
            .await
            .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    let Some((user_id, password_hash)) = row else {
        return Err(ApiError::from(CommonError::Unauthorized("البريد أو كلمة المرور غير صحيحة".into())));
    };
    if !belhana_auth::verify_password(&req.password, &password_hash) {
        return Err(ApiError::from(CommonError::Unauthorized("البريد أو كلمة المرور غير صحيحة".into())));
    }
    // pick the first household membership as the active one
    let hh = households::list_households_for_user(&state.pool, &user_id)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| ApiError::from(CommonError::Internal("user has no household".into())))?;
    let role: String = sqlx::query_scalar(
        "SELECT role FROM household_memberships WHERE user_id = ? AND household_id = ?",
    )
    .bind(&user_id)
    .bind(&hh.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    let refresh = belhana_auth::create_session(&state.pool, &user_id, None).await?;
    let mut tokens = issue_tokens(&state, &user_id, &hh.id, &role)?;
    tokens["refresh_token"] = json!(refresh);
    tokens["user"] = json!({"id": user_id, "email": email});
    tokens["household"] = json!(hh);
    Ok(Json(tokens))
}

async fn refresh(State(state): State<SharedState>, Json(req): Json<RefreshRequest>) -> ApiResult<Json<serde_json::Value>> {
    let (new_refresh, user_id) = belhana_auth::rotate_refresh_token(&state.pool, &req.refresh_token).await?;
    let hh = households::list_households_for_user(&state.pool, &user_id)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| ApiError::from(CommonError::Internal("user has no household".into())))?;
    let role: String = sqlx::query_scalar(
        "SELECT role FROM household_memberships WHERE user_id = ? AND household_id = ?",
    )
    .bind(&user_id)
    .bind(&hh.id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    let mut tokens = issue_tokens(&state, &user_id, &hh.id, &role)?;
    tokens["refresh_token"] = json!(new_refresh);
    Ok(Json(tokens))
}

async fn logout(State(state): State<SharedState>, Json(req): Json<RefreshRequest>) -> ApiResult<Json<serde_json::Value>> {
    belhana_auth::revoke_session(&state.pool, &req.refresh_token).await?;
    Ok(Json(json!({"ok": true})))
}

async fn me(State(state): State<SharedState>, user: crate::auth::AuthUser) -> ApiResult<Json<serde_json::Value>> {
    let display_name: String =
        sqlx::query_scalar("SELECT display_name FROM users WHERE id = ?")
            .bind(&user.user_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| ApiError::from(CommonError::Internal(e.to_string())))?;
    let hh = households::get_household(&state.pool, &user.household_id).await?;
    Ok(Json(json!({
        "user": {"id": user.user_id, "email": user.email, "display_name": display_name},
        "role": user.role,
        "household": hh,
    })))
}

async fn switch_household(
    State(state): State<SharedState>,
    user: crate::auth::AuthUser,
    Json(req): Json<SwitchHouseholdRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let role = households::ensure_member(&state.pool, &user.user_id, &req.household_id).await?;
    let mut tokens = issue_tokens(&state, &user.user_id, &req.household_id, &role)?;
    tokens["household_id"] = json!(req.household_id);
    Ok(Json(tokens))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(me))
        .route("/api/auth/switch-household", post(switch_household))
}
