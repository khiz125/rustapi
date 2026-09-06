use crate::domain::subscription::vo::{
    ProviderSubscriptionId, SubscriptionId, SubscriptionProvider, SubscriptionStatus,
};
use crate::domain::user::vo::{UserId, UserPlan};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub user_id: UserId,
    pub provider: SubscriptionProvider,
    pub provider_subscription_id: ProviderSubscriptionId,
    pub status: SubscriptionStatus,
    pub plan: UserPlan,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    pub fn is_active(&self) -> bool {
        self.status == SubscriptionStatus::Active && self.expires_at > Utc::now()
    }
}
