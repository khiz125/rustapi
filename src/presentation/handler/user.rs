use crate::domain::user::repository::UserRepository;
use crate::middleware::auth::AuthUser;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::user::update_password::UpdatePasswordInput;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub async fn update_password<R: UserRepository + Clone>(
    State(state): State<AppState<R>>,
    AuthUser(claims): AuthUser,
    Json(body): Json<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
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
