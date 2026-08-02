use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::usecase::auth::token::{generate_refresh_token, hash_refresh_token, issue_token};
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

pub struct RefreshTokenUsecase<R: UserRepository, RT: RefreshTokenRepository> {
    user_repository: Arc<R>,
    refresh_token_repository: Arc<RT>,
    jwt_secret: String,
}

impl<R: UserRepository, RT: RefreshTokenRepository> RefreshTokenUsecase<R, RT> {
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

    pub async fn execute(
        &self,
        input: RefreshTokenInput,
    ) -> Result<RefreshTokenOutput, DomainError> {
        let token_hash = hash_refresh_token(&input.refresh_token);
        let stored_token = self
            .refresh_token_repository
            .find_by_hash(&token_hash)
            .await?
            .ok_or(DomainError::Unauthorized)?;

        if !stored_token.is_valid() {
            return Err(DomainError::Unauthorized);
        }

        let user = self
            .user_repository
            .find_by_id(stored_token.user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        self.refresh_token_repository
            .revoke(stored_token.id)
            .await?;

        let access_token = issue_token(user.id.value(), &self.jwt_secret)?;
        let new_refresh_token = generate_refresh_token();
        let new_token_hash = hash_refresh_token(&new_refresh_token);
        let expires_at = Utc::now() + chrono::Duration::days(30);

        self.refresh_token_repository
            .create(user.id, new_token_hash, expires_at)
            .await?;

        Ok(RefreshTokenOutput {
            user_id: user.id.value(),
            access_token,
            refresh_token: new_refresh_token,
        })
    }
}
