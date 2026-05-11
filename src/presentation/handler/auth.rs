use crate::domain::user::repository::UserRepository;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::auth::login_with_email::LoginWithEmailInput;
use crate::usecase::auth::sign_up_with_email::SignUpWithEmailInput;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub user_id: i64,
    pub token: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: i64,
    pub token: String,
}

pub async fn sign_up<R: UserRepository + Clone>(
    State(state): State<AppState<R>>,
    Json(body): Json<SignUpRequest>,
) -> Result<impl IntoResponse, AppError> {
    let output = state
        .sign_up
        .execute(SignUpWithEmailInput {
            name: body.name,
            email: body.email,
            password: body.password,
        })
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(SignUpResponse {
            user_id: output.user_id,
            token: output.token,
        }),
    ))
}

pub async fn login<R: UserRepository + Clone>(
    State(state): State<AppState<R>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let output = state
        .login
        .execute(LoginWithEmailInput {
            email: body.email,
            password: body.password,
        })
        .await?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            user_id: output.user_id,
            token: output.token,
        }),
    ))
}

pub async fn logout() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
