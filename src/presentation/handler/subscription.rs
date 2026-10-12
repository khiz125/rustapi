use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::subscription::repository::SubscriptionRepository;
use crate::domain::subscription::vo::SubscriptionProvider;
use crate::domain::user::repository::UserRepository;
use crate::middleware::auth::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::subscription::verify_subscription::VerifySubscriptionInput;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct VerifySubscriptionRequest {
    pub provider_subscription_id: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct VerifySubscriptionResponse {
    pub subscription_id: i64,
    pub expires_at: DateTime<Utc>,
}

async fn verify_subscription<R, RT, S>(
    state: &AppState<R, RT, S>,
    user_id: i64,
    provider: SubscriptionProvider,
    body: VerifySubscriptionRequest,
) -> Result<impl IntoResponse + use<R, RT, S>, AppError>
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
    S: SubscriptionRepository + Clone + Send + Sync + 'static,
{
    let output = state
        .verify_subscription
        .execute(VerifySubscriptionInput {
            user_id,
            provider,
            provider_subscription_id: body.provider_subscription_id,
            expires_at: body.expires_at,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(VerifySubscriptionResponse {
            subscription_id: output.subscription_id,
            expires_at: output.expires_at,
        }),
    ))
}

pub async fn verify_google_play<R, RT, S>(
    State(state): State<AppState<R, RT, S>>,
    AuthUser(claims): AuthUser,
    Json(body): Json<VerifySubscriptionRequest>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
    S: SubscriptionRepository + Clone + Send + Sync + 'static,
{
    verify_subscription(&state, claims.sub, SubscriptionProvider::GooglePlay, body).await
}

pub async fn verify_app_store<R, RT, S>(
    State(state): State<AppState<R, RT, S>>,
    AuthUser(claims): AuthUser,
    Json(body): Json<VerifySubscriptionRequest>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
    S: SubscriptionRepository + Clone + Send + Sync + 'static,
{
    verify_subscription(&state, claims.sub, SubscriptionProvider::AppStore, body).await
}

pub async fn verify_stripe<R, RT, S>(
    State(state): State<AppState<R, RT, S>>,
    AuthUser(claims): AuthUser,
    Json(body): Json<VerifySubscriptionRequest>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
    S: SubscriptionRepository + Clone + Send + Sync + 'static,
{
    verify_subscription(&state, claims.sub, SubscriptionProvider::Stripe, body).await
}

pub async fn verify_paypay<R, RT, S>(
    State(state): State<AppState<R, RT, S>>,
    AuthUser(claims): AuthUser,
    Json(body): Json<VerifySubscriptionRequest>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
    S: SubscriptionRepository + Clone + Send + Sync + 'static,
{
    verify_subscription(&state, claims.sub, SubscriptionProvider::PayPay, body).await
}
