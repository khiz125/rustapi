use crate::domain::user::repository::UserRepository;
use crate::presentation::handler::{auth, user};
use crate::presentation::state::AppState;
use axum::Router;
use axum::routing::{post, put};

pub fn create_router<R: UserRepository + Clone + 'static>(state: AppState<R>) -> Router {
    Router::new()
        .route("/auth/signup", post(auth::sign_up::<R>))
        .route("/auth/login", post(auth::login::<R>))
        .route("/auth/logout", post(auth::logout))
        .route("/users/me/password", put(user::update_password::<R>))
        .route("/users/me/name", put(user::update_name::<R>))
        .with_state(state)
}
