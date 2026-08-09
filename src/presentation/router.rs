use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::presentation::handler::{auth, mobile, oauth, user};
use crate::presentation::state::AppState;
use axum::Router;
use axum::routing::{get, post, put};

pub fn create_router<R, RT>(state: AppState<R, RT>) -> Router
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/auth/signup", post(auth::sign_up::<R, RT>))
        .route("/mobile/auth/logout", post(auth::mobile_logout::<R, RT>))
        .route("/web/auth/logout", post(auth::web_logout::<R, RT>))
        .route("/auth/google", get(oauth::google_redirect::<R, RT>))
        .route(
            "/auth/google/callback",
            get(oauth::google_callback::<R, RT>),
        )
        // mobile
        .route(
            "/mobile/auth/signin",
            post(mobile::sign_up_with_mobile_device::<R, RT>),
        )
        .route("/mobile/auth/refresh", post(auth::mobile_refresh::<R, RT>))
        //web
        .route("/web/auth/login", post(auth::web_login::<R, RT>))
        .route("/web/auth/refresh", post(auth::web_refresh::<R, RT>))
        // user handle
        .route("/users/me/password", put(user::update_password::<R, RT>))
        .route("/users/me/name", put(user::update_name::<R, RT>))
        .with_state(state)
}
