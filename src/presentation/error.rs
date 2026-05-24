use crate::domain::error::DomainError;
use crate::presentation::error_code as ec;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub struct AppError(DomainError);

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        Self(e)
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: &'static str,
    detail: Option<String>,
}

impl ErrorBody {
    fn new(code: &'static str, message: &'static str, detail: Option<String>) -> Self {
        Self {
            error: ErrorDetail {
                code,
                message,
                detail,
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, detail) = match self.0 {
            DomainError::UserNotFound => (StatusCode::NOT_FOUND, &ec::USER_NOT_FOUND, None),
            DomainError::EmailAlreadyExists => {
                (StatusCode::CONFLICT, &ec::EMAIL_ALREADY_EXISTS, None)
            }
            DomainError::IncorrectPassword => {
                (StatusCode::UNAUTHORIZED, &ec::INCORRECT_PASSWORD, None)
            }
            DomainError::NotPasswordAuthUser => {
                (StatusCode::BAD_REQUEST, &ec::NOT_PASSWORD_AUTH_USER, None)
            }
            DomainError::InvalidEmail(detail) => (
                StatusCode::BAD_REQUEST,
                &ec::INVALID_EMAIL,
                Some(detail.to_string()),
            ),
            DomainError::InvalidUserName(detail) => (
                StatusCode::BAD_REQUEST,
                &ec::INVALID_USER_NAME,
                Some(detail.to_string()),
            ),
            DomainError::Unauthorized => (StatusCode::UNAUTHORIZED, &ec::UNAUTHORIZED, None),
            DomainError::InvalidRequest(detail) => {
                (StatusCode::BAD_REQUEST, &ec::INVALID_REQUEST, Some(detail))
            }
            DomainError::Unexpected(e) => {
                tracing::error!("unexpected error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &ec::INTERNAL_SERVER_ERROR,
                    None,
                )
            }
        };

        (
            status,
            Json(ErrorBody::new(error_code.code, error_code.message, detail)),
        )
            .into_response()
    }
}
