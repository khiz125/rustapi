use crate::domain::error::DomainError;
use crate::domain::user::User;
use crate::domain::user::vo::email::Email;
use crate::domain::user::vo::oauth_provider::OAuthProvider;
use crate::domain::user::vo::provider_user_id::ProviderUserId;
use crate::domain::user::vo::{password_hash::PasswordHash, user_id::UserId, user_name::UserName};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_provider(
        &self,
        provider: &OAuthProvider,
        provider_user_id: &ProviderUserId,
    ) -> Result<Option<User>, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn update_password(
        &self,
        user_id: UserId,
        new_password_hash: PasswordHash,
    ) -> Result<(), DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
}
