use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::auth::login_with_email::LoginWithEmailInput;
use crate::usecase::auth::refresh_token::RefreshTokenInput;
use crate::usecase::auth::sign_up_with_email::SignUpWithEmailInput;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
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
pub struct WebLoginResponse {
    pub user_id: i64,
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn web_login<R, RT>(
    State(state): State<AppState<R, RT>>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone,
    RT: RefreshTokenRepository + Clone,
{
    let output = state
        .login
        .execute(LoginWithEmailInput {
            email: body.email,
            password: body.password,
        })
        .await?;

    let cookie = Cookie::build(("refresh_token", output.refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/web/auth/refresh")
        .max_age(time::Duration::days(30))
        .build();

    Ok((
        jar.add(cookie),
        Json(WebLoginResponse {
            user_id: output.user_id,
            access_token: output.access_token,
        }),
    ))
}

pub async fn mobile_refresh<R, RT>(
    State(state): State<AppState<R, RT>>,
    Json(body): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
{
    let output = state
        .refresh_token
        .execute(RefreshTokenInput {
            refresh_token: body.refresh_token,
        })
        .await?;

    Ok(Json(RefreshResponse {
        user_id: output.user_id,
        access_token: output.access_token,
        refresh_token: output.refresh_token,
    }))
}

pub async fn web_refresh<R, RT>(
    State(state): State<AppState<R, RT>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone,
    RT: RefreshTokenRepository + Clone,
{
    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or(crate::domain::error::DomainError::Unauthorized)?;

    let output = state
        .refresh_token
        .execute(RefreshTokenInput { refresh_token })
        .await?;

    let cookie = Cookie::build(("refresh_token", output.refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/web/auth/refresh")
        .max_age(time::Duration::days(30))
        .build();

    Ok((
        jar.add(cookie),
        Json(WebLoginResponse {
            user_id: output.user_id,
            access_token: output.access_token,
        }),
    ))
}

pub async fn sign_up<R, RT>(
    State(state): State<AppState<R, RT>>,
    body: Result<Json<SignUpRequest>, axum::extract::rejection::JsonRejection>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone,
    RT: RefreshTokenRepository + Clone,
{
    let Json(body) = body.map_err(|e| DomainError::InvalidRequest(e.to_string()))?;
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

pub async fn logout() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
