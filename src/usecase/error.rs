use crate::domain::error::{self, DomainError, OAuthError};
use crate::domain::repo_error::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum UsecaseError {
    #[error(transparent)]
    Domain(DomainError),

    #[error(transparent)]
    Repository(RepositoryError),

    #[error(transparent)]
    OAuth(OAuthError),

    #[error("token error: {0}")]
    Token(String),

    #[error("crypto error: {0}")]
    Crypto(String),
}
