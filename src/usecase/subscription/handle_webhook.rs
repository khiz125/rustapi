use crate::domain::error::DomainError;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::vo::{
    ProviderSubscriptionId, SubscriptionProvider, SubscriptionStatus,
};
use crate::domain::user::repository::UserRepository;
use crate::usecase::subscription::webhook_subscription::{
    handle_subscription_expired, handle_subscription_renewed,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub enum WebhookEvent {
    Renewed { new_expires_at: DateTime<Utc> },
    Canceled,
    Expired,
    PastDue,
    Purchased,
}

pub struct HandleWebhookInput {
    pub provider: SubscriptionProvider,
    pub provider_subscription_id: String,
    pub event: WebhookEvent,
}

pub struct HandleWebhookUsecase<R: UserRepository, S: SubscriptionRepository> {
    user_repository: Arc<R>,
    subscription_repository: Arc<S>,
}

impl<R: UserRepository, S: SubscriptionRepository> HandleWebhookUsecase<R, S> {
    pub fn new(user_repository: Arc<R>, subscription_repository: Arc<S>) -> Self {
        Self {
            user_repository,
            subscription_repository,
        }
    }

    pub async fn execute(&self, input: HandleWebhookInput) -> Result<(), DomainError> {
        let provider_subscription_id = ProviderSubscriptionId::new(input.provider_subscription_id);

        match input.event {
            WebhookEvent::Purchased => Ok(()),
            WebhookEvent::Renewed { new_expires_at } => {
                handle_subscription_renewed(
                    &*self.user_repository,
                    &*self.subscription_repository,
                    &input.provider,
                    &provider_subscription_id,
                    new_expires_at,
                )
                .await
            }
            WebhookEvent::Canceled => {
                handle_subscription_expired(
                    &*self.user_repository,
                    &*self.subscription_repository,
                    &input.provider,
                    &provider_subscription_id,
                    SubscriptionStatus::Canceled,
                )
                .await
            }
            WebhookEvent::Expired => {
                handle_subscription_expired(
                    &*self.user_repository,
                    &*self.subscription_repository,
                    &input.provider,
                    &provider_subscription_id,
                    SubscriptionStatus::Expired,
                )
                .await
            }
            WebhookEvent::PastDue => {
                handle_subscription_expired(
                    &*self.user_repository,
                    &*self.subscription_repository,
                    &input.provider,
                    &provider_subscription_id,
                    SubscriptionStatus::PastDue,
                )
                .await
            }
        }
    }
}
