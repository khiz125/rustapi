use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::vo::SubscriptionProvider;
use crate::domain::user::repository::UserRepository;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::subscription::handle_webhook::{HandleWebhookInput, WebhookEvent};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebhookRequest {
    pub provider_subscription_id: String,
    pub event_type: String,
    pub new_expires_at: Option<DateTime<Utc>>,
}

async fn handle_webhook<F>(
    state: AppState,
    provider: SubscriptionProvider,
    body: WebhookRequest,
    event_mapper: F,
) -> Result<impl IntoResponse, AppError>
where
    F: Fn(&str, Option<DateTime<Utc>>) -> Result<WebhookEvent, DomainError>,
{
    let event = event_mapper(&body.event_type, body.new_expires_at)?;

    state
        .handle_webhook
        .execute(HandleWebhookInput {
            provider,
            provider_subscription_id: body.provider_subscription_id,
            event,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

fn google_play_event_mapper(
    event_type: &str,
    new_expires_at: Option<DateTime<Utc>>,
) -> Result<WebhookEvent, DomainError> {
    match event_type {
        "SUBSCRIPTION_PURCHASED" => Ok(WebhookEvent::Purchased),
        "SUBSCRIPTION_RENEWED" => Ok(WebhookEvent::Renewed {
            new_expires_at: new_expires_at.ok_or_else(|| {
                DomainError::InvalidSubscription(
                    "new_expires_at is required for RENEWED event".into(),
                )
            })?,
        }),
        "SUBSCRIPTION_CANCELED" => Ok(WebhookEvent::Canceled),
        "SUBSCRIPTION_EXPIRED" => Ok(WebhookEvent::Expired),
        "SUBSCRIPTION_ON_HOLD" => Ok(WebhookEvent::PastDue),
        other => Err(DomainError::InvalidSubscription(format!(
            "unknown event type: {}",
            other
        ))),
    }
}

fn app_store_event_mapper(
    event_type: &str,
    new_expires_at: Option<DateTime<Utc>>,
) -> Result<WebhookEvent, DomainError> {
    match event_type {
        "SUBSCRIBED" => Ok(WebhookEvent::Purchased),
        "DID_RENEW" => Ok(WebhookEvent::Renewed {
            new_expires_at: new_expires_at.ok_or_else(|| {
                DomainError::InvalidSubscription(
                    "new_expires_at is required for DID_RENEW event".into(),
                )
            })?,
        }),
        "DID_CHANGE_RENEWAL_STATUS" | "DID_FAIL_TO_RENEW" => Ok(WebhookEvent::PastDue),
        "EXPIRED" => Ok(WebhookEvent::Expired),
        other => Err(DomainError::InvalidSubscription(format!(
            "unknown notification type: {}",
            other
        ))),
    }
}

fn stripe_event_mapper(
    event_type: &str,
    new_expires_at: Option<DateTime<Utc>>,
) -> Result<WebhookEvent, DomainError> {
    match event_type {
        "invoice.paid" => Ok(WebhookEvent::Renewed {
            new_expires_at: new_expires_at.ok_or_else(|| {
                DomainError::InvalidSubscription(
                    "new_expires_at is required for invoice.paid event".into(),
                )
            })?,
        }),
        "invoice.payment_failed" => Ok(WebhookEvent::PastDue),
        "customer.subscription.deleted" => Ok(WebhookEvent::Canceled),
        other => Err(DomainError::InvalidSubscription(format!(
            "unknown event type: {}",
            other
        ))),
    }
}

fn paypay_event_mapper(
    event_type: &str,
    new_expires_at: Option<DateTime<Utc>>,
) -> Result<WebhookEvent, DomainError> {
    match event_type {
        "PAYMENT_COMPLETED" => Ok(WebhookEvent::Renewed {
            new_expires_at: new_expires_at.ok_or_else(|| {
                DomainError::InvalidSubscription(
                    "new_expires_at is required for PAYMENT_COMPLETED event".into(),
                )
            })?,
        }),
        "PAYMENT_FAILED" => Ok(WebhookEvent::PastDue),
        "SUBSCRIPTION_CANCELED" => Ok(WebhookEvent::Canceled),
        other => Err(DomainError::InvalidSubscription(format!(
            "unknown event type: {}",
            other
        ))),
    }
}

pub async fn google_play_webhook(
    State(state): State<AppState>,
    Json(body): Json<WebhookRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Google Cloud Pub/Sub の署名検証
    handle_webhook(
        state,
        SubscriptionProvider::GooglePlay,
        body,
        google_play_event_mapper,
    )
    .await
}

pub async fn app_store_webhook(
    State(state): State<AppState>,
    Json(body): Json<WebhookRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: App Store の署名検証
    handle_webhook(
        state,
        SubscriptionProvider::AppStore,
        body,
        app_store_event_mapper,
    )
    .await
}

pub async fn stripe_webhook(
    State(state): State<AppState>,
    Json(body): Json<WebhookRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Stripe-Signature ヘッダーの署名検証
    handle_webhook(
        state,
        SubscriptionProvider::Stripe,
        body,
        stripe_event_mapper,
    )
    .await
}

pub async fn paypay_webhook(
    State(state): State<AppState>,
    Json(body): Json<WebhookRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: PayPay の署名検証
    handle_webhook(
        state,
        SubscriptionProvider::PayPay,
        body,
        paypay_event_mapper,
    )
    .await
}
