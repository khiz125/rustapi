use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::auth::sign_up_with_mobile_device::SignUpWithMobileDeviceInput;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MobileDeviceRequest {
    pub device_id: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct MobileDeviceResponse {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub is_new_user: bool,
}

pub async fn sign_up_with_mobile_device(
    State(state): State<AppState>,
    Json(body): Json<MobileDeviceRequest>,
) -> Result<impl IntoResponse, AppError> {
    let output = state
        .sign_up_with_mobile_device
        .execute(SignUpWithMobileDeviceInput {
            device_id: body.device_id,
            name: body.name,
        })
        .await?;

    Ok(Json(MobileDeviceResponse {
        user_id: output.user_id,
        access_token: output.access_token,
        refresh_token: output.refresh_token,
        is_new_user: output.is_new_user,
    }))
}
