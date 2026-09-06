use crate::domain::error::DomainError;
use crate::domain::subscription::Subscription;
use crate::domain::subscription::vo::{
    ProviderSubscriptionId, SubscriptionProvider, SubscriptionStatus,
};
use crate::domain::user::vo::{UserId, UserPlan};
use chrono::{DateTime, Utc};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait SubscriptionRepository: Send + Sync {
    async fn find_by_provider_subscription_id(
        &self,
        provder: &SubscriptionProvider,
        provider_subscription_id: &ProviderSubscriptionId,
    ) -> Result<Option<Subscription>, DomainError>;

    async fn find_active_by_user_id(
        &self,
        user_id: UserId,
    ) -> Result<Option<Subscription>, DomainError>;

    async fn create(
        &self,
        user_id: UserId,
        provder: &SubscriptionProvider,
        provider_subscription_id: ProviderSubscriptionId,
        plan: UserPlan,
        expires_at: DateTime<Utc>,
    ) -> Result<Subscription, DomainError>;

    async fn update_status(
        &self,
        provder: &SubscriptionProvider,
        provider_subscription_id: ProviderSubscriptionId,
        status: SubscriptionStatus,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), DomainError>;
}
