use crate::domain::user::repository::UserRepository;
use crate::presentation::error::AppError;
use crate::presentation::state::AppState;
use crate::usecase::auth::login_with_email::LoginWithEmailInput;
use crate::usecase::auth::sign_up_with_email::SignUpWithEmailInput;
use axum::extract::State;
