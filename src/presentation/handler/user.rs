use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::middleware::auth::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::user::update_name::UpdateNameInput;
use crate::usecase::user::update_password::UpdatePasswordInput;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct UpdateNameRequest {
    pub new_name: String,
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
