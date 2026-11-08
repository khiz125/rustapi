use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::UserPlan;
use crate::usecase::auth::token::{generate_refresh_token, hash_refresh_token, issue_token};
use crate::usecase::error::UsecaseError;
use chrono::Utc;
use std::sync::Arc;

pub struct RefreshTokenInput {
    pub refresh_token: String,
}

pub struct RefreshTokenOutput {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
}

#[async_trait::async_trait]
pub trait RefreshTokenUsecase: Send + Sync {
    async fn execute(&self, input: RefreshTokenInput) -> Result<RefreshTokenOutput, UsecaseError>;
}

pub struct RefreshTokenUsecaseImpl<R: UserRepository, RT: RefreshTokenRepository> {
    user_repository: Arc<R>,
    refresh_token_repository: Arc<RT>,
    jwt_secret: String,
}

impl<R: UserRepository, RT: RefreshTokenRepository> RefreshTokenUsecaseImpl<R, RT> {
    pub fn new(
        user_repository: Arc<R>,
        refresh_token_repository: Arc<RT>,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            jwt_secret,
        }
    }
}

#[async_trait::async_trait]
impl<R, RT> RefreshTokenUsecase for RefreshTokenUsecaseImpl<R, RT>
where
    R: UserRepository + Send + Sync,
    RT: RefreshTokenRepository + Send + Sync,
{
    async fn execute(&self, input: RefreshTokenInput) -> Result<RefreshTokenOutput, UsecaseError> {
        let token_hash = hash_refresh_token(&input.refresh_token);
        let stored_token = self
            .refresh_token_repository
            .find_by_hash(&token_hash)
            .await
            .map_err(UsecaseError::Repository)?
            .ok_or(UsecaseError::Domain(DomainError::Unauthorized))?;

        if !stored_token.is_valid() {
            return Err(UsecaseError::Domain(DomainError::Unauthorized));
        }

        let mut user = self
            .user_repository
            .find_by_id(stored_token.user_id)
            .await
            .map_err(UsecaseError::Repository)?
            .ok_or(UsecaseError::Domain(DomainError::UserNotFound))?;

        if user.plan.is_premium() && !user.is_premium() {
            self.user_repository
                .update_plan(user.id, UserPlan::Free, None)
                .await
                .map_err(UsecaseError::Repository)?;
            user.plan = UserPlan::Free;
            user.plan_expires_at = None
        }

        self.refresh_token_repository
            .revoke(stored_token.id)
            .await
            .map_err(UsecaseError::Repository)?;

        let plan = user.plan.as_str();
        let access_token = issue_token(user.id.value(), &self.jwt_secret, plan)?;
        let new_refresh_token = generate_refresh_token();
        let new_token_hash = hash_refresh_token(&new_refresh_token);
        let expires_at = Utc::now() + chrono::Duration::days(30);

        self.refresh_token_repository
            .create(user.id, new_token_hash, expires_at)
            .await
            .map_err(UsecaseError::Repository)?;

        Ok(RefreshTokenOutput {
            user_id: user.id.value(),
            access_token,
            refresh_token: new_refresh_token,
        })
    }
}
