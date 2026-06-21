use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::OAuthProvider;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::auth::sign_up_with_oauth::SignUpWithOAuthInput;
use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}

#[derive(Serialize)]
pub struct OAuthResponse {
    pub user_id: i64,
    pub token: String,
    pub is_new_user: bool,
}

pub async fn google_redirect<R: UserRepository + Clone>(
    State(state): State<AppState<R>>,
) -> impl IntoResponse {
    Redirect::temporary(&state.google_oauth_client.authorization_url())
}

pub async fn google_callback<R: UserRepository + Clone>(
    State(state): State<AppState<R>>,
    Query(query): Query<CallbackQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user_info = state
        .google_oauth_client
        .fetch_user_info(&query.code)
        .await?;

    let output = state
        .sign_up_with_oauth
        .execute(SignUpWithOAuthInput {
            provider: OAuthProvider::Google,
            provider_user_id: user_info.sub,
            name: user_info.name.unwrap_or_else(|| "Google user".to_string()),
            email: user_info.email,
        })
        .await?;

    Ok(Json(OAuthResponse {
        user_id: output.user_id,
        token: output.token,
        is_new_user: output.is_new_user,
    }))
}
