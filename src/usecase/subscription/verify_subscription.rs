use crate::domain::error::DomainError;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::vo::{ProviderSubscriptionId, SubscriptionProvider};
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{UserId, UserPlan};

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

pub struct VerifySubscriptionUsecase<R: UserRepository, S: SubscriptionRepository> {
    user_repository: Arc<R>,
    subscription_repository: Arc<S>,
}

impl<R: UserRepository, S: SubscriptionRepository> VerifySubscriptionUsecase<R, S> {
    pub fn new(user_repository: Arc<R>, subscription_repository: Arc<S>) -> Self {
        Self {
            user_repository,
            subscription_repository,
        }
    }

    pub async fn execute(
        &self,
        input: VerifySubscriptionInput,
    ) -> Result<VerifySubscriptionOutput, DomainError> {
        let user_id = UserId::new(input.user_id);

        self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let provider_subscription_id = ProviderSubscriptionId::new(input.provider_subscription_id);

        if self
            .subscription_repository
            .find_by_provider_subscription_id(&input.provider, &provider_subscription_id)
            .await?
            .is_some()
        {
            return Err(DomainError::SubscriptionAlreadyExists);
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
            .await?;

        self.user_repository
            .update_plan(user_id, UserPlan::Premium, Some(input.expires_at))
            .await?;

        Ok(VerifySubscriptionOutput {
            subscription_id: subscription.id.value(),
            expires_at: subscription.expires_at,
        })
    }
}
