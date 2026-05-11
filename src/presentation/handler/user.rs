use crate::domain::user::repository::UserRepository;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::auth::token::Claims;
use crate::usecase::user::update_password::UpdatePasswordInput;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub async fn update_password<R: UserRepository + Clone>(
    State(state): State<AppState<R>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(body): Json<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token_data = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| crate::domain::error::DomainError::IncorrectPassword)?;

    state
        .update_password
        .execute(UpdatePasswordInput {
            user_id: token_data.claims.sub,
            current_password: body.current_password,
            new_password: body.new_password,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
