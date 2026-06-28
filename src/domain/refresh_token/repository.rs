use crate::domain::error::DomainError;
use crate::domain::refresh_token::refresh_token::RefreshToken;
use crate::domain::refresh_token::vo::{RefreshTokenId, TokenHash};
use crate::domain::user::vo::UserId;

use crate::domain::types::UtcDateTime;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn find_by_hash(
        &self,
        token_hash: &TokenHash,
    ) -> Result<Option<RefreshToken>, DomainError>;
    async fn create(
        &self,
        user_id: UserId,
        token_hash: TokenHash,
        expires_at: UtcDateTime,
    ) -> Result<RefreshToken, DomainError>;
    async fn revoke(&self, id: RefreshTokenId) -> Result<(), DomainError>;
    async fn revoke_all_for_user(&self, user_id: UserId) -> Result<(), DomainError>;
}
