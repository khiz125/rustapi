use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::usecase::auth::token::hash_refresh_token;
use crate::usecase::error::UsecaseError;
use std::sync::Arc;

pub struct LogoutInput {
    pub refresh_token: String,
}

#[async_trait::async_trait]
pub trait LogoutUsecase: Send + Sync {
    async fn execute(&self, input: LogoutInput) -> Result<(), UsecaseError>;
}

pub struct LogoutUsecaseImpl<RT: RefreshTokenRepository> {
    refresh_token_repository: Arc<RT>,
}

impl<RT: RefreshTokenRepository> LogoutUsecaseImpl<RT> {
    pub fn new(refresh_token_repository: Arc<RT>) -> Self {
        Self {
            refresh_token_repository,
        }
    }
}

#[async_trait::async_trait]
impl<RT> LogoutUsecase for LogoutUsecaseImpl<RT>
where
    RT: RefreshTokenRepository + Send + Sync,
{
    async fn execute(&self, input: LogoutInput) -> Result<(), UsecaseError> {
        let token_hash = hash_refresh_token(&input.refresh_token);

        let stored_token = self
            .refresh_token_repository
            .find_by_hash(&token_hash)
            .await
            .map_err(UsecaseError::Repository)?
            .ok_or(UsecaseError::Domain(DomainError::Unauthorized))?;

        if !stored_token.is_valid() {
            return Err(UsecaseError::Domain(DomainError::Unauthorized))?;
        }

        self.refresh_token_repository
            .revoke(stored_token.id)
            .await
            .map_err(UsecaseError::Repository)?;

        Ok(())
    }
}
