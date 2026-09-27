use crate::domain::error::DomainError;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::vo::{
    ProviderSubscriptionId, SubscriptionProvider, SubscriptionStatus,
};
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::UserPlan;
use chrono::{DateTime, Utc};

pub async fn handle_subscription_renewed<R: UserRepository, S: SubscriptionRepository>(
    user_repository: &R,
    subscription_repository: &S,
    provider: &SubscriptionProvider,
    provider_subscription_id: &ProviderSubscriptionId,
    new_expires_at: DateTime<Utc>,
) -> Result<(), DomainError> {
    let subscription = subscription_repository
        .find_by_provider_subscription_id(provider, provider_subscription_id)
        .await?
        .ok_or(DomainError::SubscriptionNotFound)?;

    subscription_repository
        .update_status(
            provider,
            provider_subscription_id,
            SubscriptionStatus::Active,
            Some(new_expires_at),
        )
        .await?;

    user_repository
        .update_plan(
            subscription.user_id,
            UserPlan::Premium,
            Some(new_expires_at),
        )
        .await?;

    Ok(())
}

pub async fn handle_subscription_expired<R: UserRepository, S: SubscriptionRepository>(
    user_repository: &R,
    subscription_repository: &S,
    provider: &SubscriptionProvider,
    provider_subscription_id: &ProviderSubscriptionId,
    status: SubscriptionStatus,
) -> Result<(), DomainError> {
    let subscription = subscription_repository
        .find_by_provider_subscription_id(provider, provider_subscription_id)
        .await?
        .ok_or(DomainError::SubscriptionNotFound)?;

    subscription_repository
        .update_status(provider, provider_subscription_id, status, None)
        .await?;

    user_repository
        .update_plan(subscription.user_id, UserPlan::Free, None)
        .await?;

    Ok(())
}
