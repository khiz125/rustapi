use crate::presentation::handler::{auth, mobile, oauth, subscription, user, webhook};
use crate::presentation::state::AppState;
use axum::Router;
use axum::routing::{get, post, put};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/auth/signup", post(auth::sign_up))
        .route("/mobile/auth/logout", post(auth::mobile_logout))
        .route("/web/auth/logout", post(auth::web_logout))
        .route("/auth/google", get(oauth::google_redirect))
        .route("/auth/google/callback", get(oauth::google_callback))
        // mobile
        .route(
            "/mobile/auth/signin",
            post(mobile::sign_up_with_mobile_device),
        )
        .route("/mobile/auth/refresh", post(auth::mobile_refresh))
        //web
        .route("/web/auth/login", post(auth::web_login))
        .route("/web/auth/refresh", post(auth::web_refresh))
        // user handle
        .route("/users/me", get(user::get_me))
        .route("/users/me/password", put(user::update_password))
        .route("/users/me/name", put(user::update_name))
        .route(
            "/subscriptions/verify/google-play",
            post(subscription::verify_google_play),
        )
        .route(
            "/subscriptions/verify/app-store",
            post(subscription::verify_app_store),
        )
        .route(
            "/subscriptions/verify/stripe",
            post(subscription::verify_stripe),
        )
        .route(
            "/subscriptions/verify/paypay",
            post(subscription::verify_paypay),
        )
        .route("/webhooks/google-play", post(webhook::google_play_webhook))
        .route("/webhooks/app-store", post(webhook::app_store_webhook))
        .route("/webhooks/stripe", post(webhook::stripe_webhook))
        .route("/webhooks/paypay", post(webhook::paypay_webhook))
        .with_state(state)
}
