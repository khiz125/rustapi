#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("invalid password")]
    InvalidPassword,

    #[error("invalid user name: {0}")]
    InvalidUserName(String),

    #[error("invalid email: {0}")]
    InvalidEmail(String),

    #[error("not a password auth user")]
    NotPasswordAuthUser,

    #[error("current password is incorrect")]
    IncorrectPassword,

    #[error("user not found")]
    UserNotFound,

    #[error("unexpected error: {0}")]
    Unexpected(String),

    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),
}
