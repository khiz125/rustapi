use crate::domain::error::DomainError;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::vo::{ProviderSubscriptionId, SubscriptionProvider};
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{UserId, UserPlan};
use crate::usecase::error::UsecaseError;

use chrono::{DateTime, Utc};
use std::sync::Arc;

pub struct VerifySubscriptionInput {
    pub user_id: i64,
    pub provider: SubscriptionProvider,
    pub provider_subscription_id: String,
    pub expires_at: DateTime<Utc>,
}

pub struct VerifySubscriptionOutput {
    pub subscription_id: i64,
    pub expires_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait VerifySubscriptionUsecase: Send + Sync {
    async fn execute(
        &self,
        input: VerifySubscriptionInput,
    ) -> Result<VerifySubscriptionOutput, UsecaseError>;
}

pub struct VerifySubscriptionUsecaseImpl<R: UserRepository, S: SubscriptionRepository> {
    user_repository: Arc<R>,
    subscription_repository: Arc<S>,
}

impl<R: UserRepository, S: SubscriptionRepository> VerifySubscriptionUsecaseImpl<R, S> {
    pub fn new(user_repository: Arc<R>, subscription_repository: Arc<S>) -> Self {
        Self {
            user_repository,
            subscription_repository,
        }
    }
}

#[async_trait::async_trait]
impl<R, S> VerifySubscriptionUsecase for VerifySubscriptionUsecaseImpl<R, S>
where
    R: UserRepository + Send + Sync,
    S: SubscriptionRepository + Send + Sync,
{
    async fn execute(
        &self,
        input: VerifySubscriptionInput,
    ) -> Result<VerifySubscriptionOutput, UsecaseError> {
        let user_id = UserId::new(input.user_id);

        self.user_repository
            .find_by_id(user_id)
            .await
            .map_err(UsecaseError::Repository)?
            .ok_or(UsecaseError::Domain(DomainError::UserNotFound))?;

        let provider_subscription_id = ProviderSubscriptionId::new(input.provider_subscription_id);

        if self
            .subscription_repository
            .find_by_provider_subscription_id(&input.provider, &provider_subscription_id)
            .await
            .map_err(UsecaseError::Repository)?
            .is_some()
        {
            return Err(UsecaseError::Domain(DomainError::SubscriptionAlreadyExists));
        }

        let subscription = self
            .subscription_repository
            .create(
                user_id,
                input.provider,
                provider_subscription_id,
                UserPlan::Premium,
                input.expires_at,
            )
            .await
            .map_err(UsecaseError::Repository)?;

        self.user_repository
            .update_plan(user_id, UserPlan::Premium, Some(input.expires_at))
            .await
            .map_err(UsecaseError::Repository)?;

        Ok(VerifySubscriptionOutput {
            subscription_id: subscription.id.value(),
            expires_at: subscription.expires_at,
        })
    }
}
