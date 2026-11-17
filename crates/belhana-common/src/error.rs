use thiserror::Error;

/// Error type shared across domain crates. The API layer maps these to HTTP
/// responses; storage layers convert `sqlx::Error` into `Db` variants locally.
#[derive(Debug, Error)]
pub enum CommonError {
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type CommonResult<T> = Result<T, CommonError>;
