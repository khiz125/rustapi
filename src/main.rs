mod domain;
mod infra;
mod middleware;
mod presentation;
mod usecase;

use infra::user::repository::PgUserRepository;
use presentation::router::create_router;
use presentation::state::AppState;
use usecase::auth::login_with_email::LoginWithEmailUsecase;
use usecase::auth::sign_up_with_email::SignUpWithEmailUsecase;
use usecase::user::update_password::UpdatePasswordUsecase;

use std::env;
use std::sync::Arc;

use crate::infra::database::connection::create_pool;
use crate::usecase::user::update_name::UpdateNameUsecase;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    let user_repository = Arc::new(PgUserRepository::new(pool));
    let state = AppState {
        sign_up: Arc::new(SignUpWithEmailUsecase::new(
            user_repository.clone(),
            jwt_secret.clone(),
        )),
        login: Arc::new(LoginWithEmailUsecase::new(
            user_repository.clone(),
            jwt_secret.clone(),
        )),
        update_password: Arc::new(UpdatePasswordUsecase::new(user_repository.clone())),
        update_name: Arc::new(UpdateNameUsecase::new(user_repository.clone())),
    };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind");

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
