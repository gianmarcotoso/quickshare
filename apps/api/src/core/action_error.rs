use std::fmt;

use axum::{
    Json,
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::core::error_code::{ErrorCode, ErrorDto};

#[derive(Debug)]
pub enum ActionError {
    LogicViolation(ErrorCode),
    NotFound(ErrorCode),
    PayloadTooLarge,
    Internal(ErrorDto),
}

impl ActionError {
    pub fn internal(cause: impl fmt::Display) -> Self {
        tracing::error!(%cause, "internal error");
        ActionError::Internal(ErrorDto::new(ErrorCode::Internal))
    }
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::LogicViolation(code) | ActionError::NotFound(code) => {
                write!(f, "{}", code.as_str())
            }
            ActionError::PayloadTooLarge => write!(f, "{}", ErrorCode::FileTooLarge.as_str()),
            ActionError::Internal(error) => write!(f, "{}", error.code.as_str()),
        }
    }
}

impl From<MultipartError> for ActionError {
    fn from(error: MultipartError) -> Self {
        if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
            ActionError::PayloadTooLarge
        } else {
            ActionError::LogicViolation(ErrorCode::UploadInterrupted)
        }
    }
}

impl IntoResponse for ActionError {
    fn into_response(self) -> Response {
        match self {
            ActionError::LogicViolation(code) => {
                (StatusCode::BAD_REQUEST, Json(ErrorDto::new(code))).into_response()
            }
            ActionError::NotFound(code) => {
                (StatusCode::NOT_FOUND, Json(ErrorDto::new(code))).into_response()
            }
            ActionError::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(ErrorDto::new(ErrorCode::FileTooLarge)),
            )
                .into_response(),
            ActionError::Internal(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
            }
        }
    }
}
