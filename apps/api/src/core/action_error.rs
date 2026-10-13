use std::{borrow::Cow, fmt};

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
    Unauthorized,
    PayloadTooLarge,
    Internal(Cow<'static, str>),
}

impl ActionError {
    pub fn internal(error: impl ToString) -> Self {
        ActionError::Internal(error.to_string().into())
    }
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::LogicViolation(code) | ActionError::NotFound(code) => {
                write!(f, "{}", code.as_str())
            }
            ActionError::Unauthorized => write!(f, "{}", ErrorCode::InvalidToken.as_str()),
            ActionError::PayloadTooLarge => write!(f, "{}", ErrorCode::FileTooLarge.as_str()),
            ActionError::Internal(error) => write!(f, "{}: {error}", ErrorCode::Internal.as_str()),
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
            ActionError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorDto::new(ErrorCode::InvalidToken)),
            )
                .into_response(),
            ActionError::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(ErrorDto::new(ErrorCode::FileTooLarge)),
            )
                .into_response(),
            ActionError::Internal(error) => {
                tracing::error!(%error, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorDto::new(ErrorCode::Internal)),
                )
                    .into_response()
            }
        }
    }
}
