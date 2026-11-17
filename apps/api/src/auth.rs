//! Authentication middleware: Bearer access-token extraction and household
//! membership resolution for every protected route.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use serde::Serialize;

use crate::error::ApiError;
use crate::state::SharedState;

#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub user_id: String,
    pub email: String,
    pub household_id: String,
    pub role: String,
}

impl FromRequestParts<SharedState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &SharedState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiError { status: StatusCode::UNAUTHORIZED, message: "missing Authorization header".into() })?;
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(ApiError { status: StatusCode::UNAUTHORIZED, message: "invalid Authorization header".into() })?;
        let claims = belhana_auth::decode_access_token(token, &state.config.jwt_secret)
            .map_err(|e| ApiError::from(e))?;
        // verify the user still exists and membership still holds
        let row: Option<(String, String)> = sqlx::query_as(
            "SELECT u.email, m.role FROM users u
             JOIN household_memberships m ON m.user_id = u.id AND m.household_id = ?
             WHERE u.id = ?",
        )
        .bind(&claims.household)
        .bind(&claims.sub)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| ApiError::from(belhana_common::CommonError::Internal(e.to_string())))?;
        let (email, role) =
            row.ok_or(ApiError { status: StatusCode::UNAUTHORIZED, message: "session no longer valid".into() })?;
        Ok(AuthUser {
            user_id: claims.sub,
            email,
            household_id: claims.household,
            role,
        })
    }
}
