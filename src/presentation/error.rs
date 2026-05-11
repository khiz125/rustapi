use crate::domain::error::DomainError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub struct AppError(DomainError);

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        Self(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            DomainError::UserNotFound => (StatusCode::NOT_FOUND, "user not found"),
            DomainError::EmailAlreadyExists(_) => (StatusCode::CONFLICT, "email already exists"),
            DomainError::IncorrectPassword => (StatusCode::UNAUTHORIZED, "incorrect password"),
            DomainError::NotPasswordAuthUser => {
                (StatusCode::BAD_REQUEST, "not a password auth user")
            }
            DomainError::InvalidEmail(_) => (StatusCode::BAD_REQUEST, "invalid email"),
            DomainError::InvalidUserName(_) => (StatusCode::BAD_REQUEST, "invalid user name"),
            DomainError::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, "unexpected error"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
