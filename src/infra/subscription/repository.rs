use crate::domain::error::DomainError;
use crate::domain::subscription::Subscription;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::vo::{
    ProviderSubscriptionId, SubscriptionId, SubscriptionProvider, SubscriptionStatus,
};
use crate::domain::user::vo::{UserId, UserPlan};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgSubscriptionRepository {
    pool: PgPool,
}

impl PgSubscriptionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

struct SubscriptionRow {
    id: i64,
    user_id: i64,
    provider: String,
    provider_subscription_id: String,
    status: String,
    plan: String,
    started_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

fn to_domain(row: &SubscriptionRow) -> Result<Subscription, DomainError> {
    let provider = SubscriptionProvider::from_str(&row.provider)
        .ok_or_else(|| DomainError::Unexpected(format!("unknow provider: {}", row.provider)))?;
    let status = SubscriptionStatus::from_str(&row.status)
        .ok_or_else(|| DomainError::Unexpected(format!("unknown status: {}", row.status)))?;
    let plan = UserPlan::from_str(&row.plan)
        .ok_or_else(|| DomainError::Unexpected(format!("unknown plan: {}", row.plan)))?;

    Ok(Subscription {
        id: SubscriptionId::new(row.id),
        user_id: UserId::new(row.user_id),
        provider,
        provider_subscription_id: ProviderSubscriptionId::new(row.provider_subscription_id.clone()),
        status,
        plan,
        started_at: row.started_at,
        expires_at: row.expires_at,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

#[async_trait]
impl SubscriptionRepository for PgSubscriptionRepository {
    async fn find_by_provider_subscription_id(
        &self,
        provider: &SubscriptionProvider,
        provider_subscription_id: &ProviderSubscriptionId,
    ) -> Result<Option<Subscription>, DomainError> {
        let row = sqlx::query!(
            r#"
            SELECT
                id,
                user_id,
                provider::text AS "provider!: String",
                provider_subscription_id,
                status::text AS "status!: String",
                plan::text AS "plan!: String",
                started_at,
                expires_at,
                created_at,
                updated_at
            FROM subscriptions
            WHERE provider = $1::text::subscription_provider
              AND provider_subscription_id = $2
            "#,
            provider.as_str(),
            provider_subscription_id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        to_domain(&SubscriptionRow {
            id: row.id,
            user_id: row.user_id,
            provider: row.provider,
            provider_subscription_id: row.provider_subscription_id,
            status: row.status,
            plan: row.plan,
            started_at: row.started_at,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .map(Some)
    }

    async fn find_active_by_user_id(
        &self,
        user_id: UserId,
    ) -> Result<Option<Subscription>, DomainError> {
        let row = sqlx::query!(
            r#"
            SELECT
                id,
                user_id,
                provider::text AS "provider!: String",
                provider_subscription_id,
                status::text AS "status!: String",
                plan::text AS "plan!: String",
                started_at,
                expires_at,
                created_at,
                updated_at
            FROM subscriptions
            WHERE user_id = $1
                AND status = 'active'
                AND expires_at > now()
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let Some(row) = row else { return Ok(None) };

        to_domain(&SubscriptionRow {
            id: row.id,
            user_id: row.user_id,
            provider: row.provider,
            provider_subscription_id: row.provider_subscription_id,
            status: row.status,
            plan: row.plan,
            started_at: row.started_at,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .map(Some)
    }

    async fn create(
        &self,
        user_id: UserId,
        provider: SubscriptionProvider,
        provider_subscription_id: ProviderSubscriptionId,
        plan: UserPlan,
        expires_at: DateTime<Utc>,
    ) -> Result<Subscription, DomainError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO subscriptions (
                user_id,
                provider,
                provider_subscription_id,
                plan,
                expires_at
            )
            VALUES (
                $1,
                $2::text::subscription_provider,
                $3,
                $4::text::user_plan,
                $5
            )
            RETURNING
                id,
                user_id,
                provider::text AS "provider!: String",
                provider_subscription_id,
                status::text AS "status!: String",
                plan::text AS "plan!: String",
                started_at,
                expires_at,
                created_at,
                updated_at
            "#,
            user_id.value(),
            provider.as_str(),
            provider_subscription_id.value(),
            plan.as_str(),
            expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        to_domain(&SubscriptionRow {
            id: row.id,
            user_id: row.user_id,
            provider: row.provider,
            provider_subscription_id: row.provider_subscription_id,
            status: row.status,
            plan: row.plan,
            started_at: row.started_at,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    async fn update_status(
        &self,
        provider: &SubscriptionProvider,
        provider_subscription_id: &ProviderSubscriptionId,
        status: SubscriptionStatus,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE subscriptions
            SET status = $1::text::subscription_status,
                expires_at = COALESCE($2, expires_at),
                updated_at = now()
            WHERE provider = $3::text::subscription_provider
                AND provider_subscription_id = $4
            "#,
            status.as_str(),
            expires_at,
            provider.as_str(),
            provider_subscription_id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
