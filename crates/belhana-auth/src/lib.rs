//! Authentication for Belhana: Argon2id password hashing, opaque session
//! refresh tokens (stored hashed), short-lived JWT access tokens, and session
//! lifecycle management.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use belhana_common::CommonError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,       // user id
    pub household: String, // active household id
    pub role: String,      // membership role
    pub exp: i64,
    pub iat: i64,
}

pub const ACCESS_TOKEN_TTL_MINUTES: i64 = 30;
pub const REFRESH_TOKEN_TTL_DAYS: i64 = 30;

pub fn hash_password(password: &str) -> Result<String, CommonError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| CommonError::Internal(format!("password hashing failed: {e}")))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .ok()
        .and_then(|parsed| {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .ok()
        })
        .is_ok()
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let out = hasher.finalize();
    out.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn generate_refresh_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn encode_access_token(
    claims: &AccessClaims,
    secret: &str,
) -> Result<String, CommonError> {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| CommonError::Internal(format!("token encoding failed: {e}")))
}

pub fn decode_access_token(token: &str, secret: &str) -> Result<AccessClaims, CommonError> {
    decode::<AccessClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|_| CommonError::Unauthorized("invalid or expired access token".into()))
}

/// Creates a new session row and returns the plaintext refresh token.
pub async fn create_session(
    pool: &SqlitePool,
    user_id: &str,
    device_id: Option<&str>,
) -> Result<String, CommonError> {
    let refresh = generate_refresh_token();
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires = now + Duration::days(REFRESH_TOKEN_TTL_DAYS);
    sqlx::query(
        "INSERT INTO sessions (id, user_id, refresh_token_hash, device_id, created_at, expires_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(hash_token(&refresh))
    .bind(device_id)
    .bind(now.to_rfc3339())
    .bind(expires.to_rfc3339())
    .execute(pool)
    .await
    .map_err(|e| CommonError::Internal(format!("session creation failed: {e}")))?;
    Ok(refresh)
}

/// Rotates a refresh token: revokes the old one and issues a new one.
pub async fn rotate_refresh_token(
    pool: &SqlitePool,
    presented: &str,
) -> Result<(String, String), CommonError> {
    let hash = hash_token(presented);
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT id, user_id FROM sessions WHERE refresh_token_hash = ? AND revoked_at IS NULL AND expires_at > ?",
    )
    .bind(&hash)
    .bind(Utc::now().to_rfc3339())
    .fetch_optional(pool)
    .await
    .map_err(|e| CommonError::Internal(e.to_string()))?;
    let (session_id, user_id) = row
        .ok_or_else(|| CommonError::Unauthorized("invalid refresh token".into()))?;
    sqlx::query("UPDATE sessions SET revoked_at = ? WHERE id = ?")
        .bind(Utc::now().to_rfc3339())
        .bind(&session_id)
        .execute(pool)
        .await
        .map_err(|e| CommonError::Internal(e.to_string()))?;
    let new_refresh = create_session(pool, &user_id, None).await?;
    Ok((new_refresh, user_id))
}

pub async fn revoke_session(pool: &SqlitePool, refresh_token: &str) -> Result<(), CommonError> {
    sqlx::query("UPDATE sessions SET revoked_at = ? WHERE refresh_token_hash = ?")
        .bind(Utc::now().to_rfc3339())
        .bind(hash_token(refresh_token))
        .execute(pool)
        .await
        .map_err(|e| CommonError::Internal(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_roundtrip() {
        let h = hash_password("secret123").unwrap();
        assert!(verify_password("secret123", &h));
        assert!(!verify_password("wrong", &h));
        // Argon2 hashes are salted: two hashes of the same password differ.
        let h2 = hash_password("secret123").unwrap();
        assert_ne!(h, h2);
    }

    #[test]
    fn token_hash_stable() {
        assert_eq!(hash_token("abc"), hash_token("abc"));
        assert_ne!(hash_token("abc"), hash_token("abd"));
    }
}
