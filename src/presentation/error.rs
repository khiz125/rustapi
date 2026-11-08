use crate::domain::error::{DomainError, OAuthError};
use crate::domain::repo_error::RepositoryError;
use crate::presentation::error_code as ec;
use crate::usecase::error::UsecaseError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

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
pub struct AppError(UsecaseError);

impl From<UsecaseError> for AppError {
    fn from(e: UsecaseError) -> Self {
        Self(e)
    }
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        Self(UsecaseError::Domain(e))
    }
}

impl From<OAuthError> for AppError {
    fn from(e: OAuthError) -> Self {
        Self(UsecaseError::OAuth(e))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, detail) = match self.0 {
            UsecaseError::Domain(e) => match e {
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
                DomainError::SubscriptionNotFound => {
                    (StatusCode::NOT_FOUND, &ec::SUBSCRIPTION_NOT_FOUND, None)
                }
                DomainError::SubscriptionAlreadyExists => {
                    (StatusCode::CONFLICT, &ec::SUBSCRIPTION_ALREADY_EXISTS, None)
                }
                DomainError::InvalidSubscription(detail) => (
                    StatusCode::BAD_REQUEST,
                    &ec::INVALID_SUBSCRIPTION,
                    Some(detail),
                ),
                DomainError::PlanUpgradeRequired => {
                    (StatusCode::FORBIDDEN, &ec::PLAN_UPGRADE_REQUIRED, None)
                }
                DomainError::WebhookVerificationFailed => (
                    StatusCode::UNAUTHORIZED,
                    &ec::WEBHOOK_VERIFICATION_FAILED,
                    None,
                ),
                DomainError::InvalidCredentials => {
                    (StatusCode::UNAUTHORIZED, &ec::INVALID_CREDENTIALS, None)
                }
            },
            UsecaseError::Repository(e) => {
                match &e {
                    RepositoryError::DatabaseError(m) => tracing::error!("database error: {}", m),
                    RepositoryError::DataIntegrityError(m) => {
                        tracing::error!("data integrity error: {}", m)
                    }
                }
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &ec::INTERNAL_SERVER_ERROR,
                    None,
                )
            }
            UsecaseError::OAuth(e) => {
                tracing::error!("oauth error: {}", e);
                (StatusCode::BAD_GATEWAY, &ec::OAUTH_PROVIDER_ERROR, None)
            }
            UsecaseError::Token(m) | UsecaseError::Crypto(m) => {
                tracing::error!("internal error: {}", m);
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
