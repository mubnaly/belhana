use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Maps domain errors to HTTP responses with Arabic-friendly messages.
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl From<belhana_common::CommonError> for ApiError {
    fn from(e: belhana_common::CommonError) -> Self {
        use belhana_common::CommonError::*;
        match e {
            NotFound(m) => ApiError { status: StatusCode::NOT_FOUND, message: m },
            Unauthorized(m) => ApiError { status: StatusCode::UNAUTHORIZED, message: m },
            Forbidden(m) => ApiError { status: StatusCode::FORBIDDEN, message: m },
            Validation(m) => ApiError { status: StatusCode::BAD_REQUEST, message: m },
            Conflict(m) => ApiError { status: StatusCode::CONFLICT, message: m },
            Internal(m) => {
                tracing::error!("internal error: {m}");
                ApiError { status: StatusCode::INTERNAL_SERVER_ERROR, message: "خطأ داخلي في الخادم".into() }
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({ "error": { "code": self.status.as_u16(), "message": self.message } });
        (self.status, axum::Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
