use crate::domain::user::vo::{Email, UserName};

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("invalid user name: {0}")]
    InvalidUserName(UserName),

    #[error("invalid email: {0}")]
    InvalidEmail(String),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("not a password auth user")]
    NotPasswordAuthUser,

    #[error("current password is incorrect")]
    IncorrectPassword,

    #[error("unauthorized")]
    Unauthorized,

    #[error("user not found")]
    UserNotFound,

    #[error("unexpected error: {0}")]
    Unexpected(String),

    #[error("email already exists: {0}")]
    EmailAlreadyExists(Email),
}
