use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::middleware::auth::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::user::update_name::UpdateNameInput;
use crate::usecase::user::update_password::UpdatePasswordInput;
use crate::{domain::error::DomainError, usecase::user::get_me::GetMeInput};
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetMeResponse {
    pub user_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub auth_kind: String,
    pub plan: String,
    pub plan_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct UpdateNameRequest {
    pub new_name: String,
}

pub async fn get_me<R, RT>(
    State(state): State<AppState<R, RT>>,
    AuthUser(claims): AuthUser,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone + Send + Sync + 'static,
    RT: RefreshTokenRepository + Clone + Send + Sync + 'static,
{
    let output = state
        .get_me
        .execute(GetMeInput {
            user_id: claims.sub,
        })
        .await?;

    Ok(Json(GetMeResponse {
        user_id: output.user_id,
        name: output.name,
        email: output.email,
        auth_kind: output.auth_kind,
        plan: output.plan,
        plan_expires_at: output.plan_expires_at,
        created_at: output.created_at,
    }))
}

pub async fn update_password<R, RT>(
    State(state): State<AppState<R, RT>>,
    AuthUser(claims): AuthUser,
    body: Result<Json<UpdatePasswordRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone,
    RT: RefreshTokenRepository + Clone,
{
    let Json(body) = body.map_err(|e| DomainError::InvalidRequest(e.to_string()))?;

    state
        .update_password
        .execute(UpdatePasswordInput {
            user_id: claims.sub,
            current_password: body.current_password,
            new_password: body.new_password,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_name<R, RT>(
    State(state): State<AppState<R, RT>>,
    AuthUser(claims): AuthUser,
    body: Result<Json<UpdateNameRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError>
where
    R: UserRepository + Clone,
    RT: RefreshTokenRepository + Clone,
{
    let Json(body) = body.map_err(|e| DomainError::InvalidRequest(e.to_string()))?;

    state
        .update_name
        .execute(UpdateNameInput {
            user_id: claims.sub,
            new_name: body.new_name,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
