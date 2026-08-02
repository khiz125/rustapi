mod domain;
mod infra;
mod middleware;
mod presentation;
mod usecase;

use infra::oauth::google::GoogleOAuthClient;
use infra::refresh_token::repository::PgRefreshTokenRepository;
use infra::user::repository::PgUserRepository;
use presentation::router::create_router;
use presentation::state::AppState;
use usecase::auth::login_with_email::LoginWithEmailUsecase;
use usecase::auth::refresh_token::RefreshTokenUsecase;
use usecase::auth::sign_up_with_email::SignUpWithEmailUsecase;
use usecase::auth::sign_up_with_oauth::SignUpWithOAuthUsecase;
use usecase::user::update_password::UpdatePasswordUsecase;

use std::env;
use std::sync::Arc;

use crate::infra::database::connection::create_pool;
use crate::usecase::auth::sign_up_with_mobile_device::SignUpWithMobileDeviceUsecase;
use crate::usecase::user::update_name::UpdateNameUsecase;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let google_client_secret =
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let google_redirect_uri =
        env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");

    let google_oauth_client = Arc::new(GoogleOAuthClient::new(
        google_client_id,
        google_client_secret,
        google_redirect_uri,
    ));

    let pool = create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
    let refresh_token_repository = Arc::new(PgRefreshTokenRepository::new(pool));
    let state = AppState {
        sign_up: Arc::new(SignUpWithEmailUsecase::new(
            user_repository.clone(),
            jwt_secret.clone(),
        )),
        login: Arc::new(LoginWithEmailUsecase::new(
            user_repository.clone(),
            refresh_token_repository.clone(),
            jwt_secret.clone(),
        )),
        refresh_token: Arc::new(RefreshTokenUsecase::new(
            user_repository.clone(),
            refresh_token_repository.clone(),
            jwt_secret.clone(),
        )),
        update_password: Arc::new(UpdatePasswordUsecase::new(user_repository.clone())),
        update_name: Arc::new(UpdateNameUsecase::new(user_repository.clone())),
        sign_up_with_oauth: Arc::new(SignUpWithOAuthUsecase::new(
            user_repository.clone(),
            jwt_secret.clone(),
        )),
        sign_up_with_mobile_device: Arc::new(SignUpWithMobileDeviceUsecase::new(
            user_repository.clone(),
            refresh_token_repository.clone(),
            jwt_secret.clone(),
        )),
        google_oauth_client,
    };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind");

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
