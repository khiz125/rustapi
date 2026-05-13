use crate::domain::user::repository::UserRepository;
use crate::usecase::auth::login_with_email::LoginWithEmailUsecase;
use crate::usecase::auth::sign_up_with_email::SignUpWithEmailUsecase;
use crate::usecase::user::update_name::UpdateNameUsecase;
use crate::usecase::user::update_password::UpdatePasswordUsecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState<R: UserRepository + Clone> {
    pub sign_up: Arc<SignUpWithEmailUsecase<R>>,
    pub login: Arc<LoginWithEmailUsecase<R>>,
    pub update_password: Arc<UpdatePasswordUsecase<R>>,
    pub update_name: Arc<UpdateNameUsecase<R>>,
}
