use crate::domain::error::DomainError;
use crate::domain::user::user_auth::UserAuth;
use crate::domain::user::vo::device_id::DeviceId;
use crate::domain::user::vo::email::Email;
use crate::domain::user::vo::oauth_provider::OAuthProvider;
use crate::domain::user::vo::provider_user_id::ProviderUserId;
use crate::domain::user::vo::user_id::UserId;
use crate::domain::user::vo::{UserName, UserPlan};
use crate::domain::user::{NewUser, User};

use chrono::{DateTime, Utc};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_device_id(&self, device_id: &DeviceId) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn find_by_provider(
        &self,
        provider: &OAuthProvider,
        provider_user_id: &ProviderUserId,
    ) -> Result<Option<User>, DomainError>;
    async fn create(&self, new_user: NewUser) -> Result<User, DomainError>;
    async fn update_name(&self, user_id: UserId, new_name: UserName) -> Result<(), DomainError>;
    async fn update_plan(
        &self,
        user_id: UserId,
        plan: UserPlan,
        plan_expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), DomainError>;
    async fn save_auth(&self, user_auth: &UserAuth) -> Result<(), DomainError>;
}
