use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::infra::oauth::google::GoogleOAuthClient;
use crate::usecase::auth::login_with_email::LoginWithEmailUsecase;
use crate::usecase::auth::logout::LogoutUsecase;
use crate::usecase::auth::refresh_token::RefreshTokenUsecase;
use crate::usecase::auth::sign_up_with_email::SignUpWithEmailUsecase;
use crate::usecase::auth::sign_up_with_mobile_device::SignUpWithMobileDeviceUsecase;
use crate::usecase::auth::sign_up_with_oauth::SignUpWithOAuthUsecase;
use crate::usecase::user::get_me::GetMeUsecase;
use crate::usecase::user::update_name::UpdateNameUsecase;
use crate::usecase::user::update_password::UpdatePasswordUsecase;
use crate::usecase::user::update_plan::UpdatePlanUsecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState<R, RT>
where
    R: UserRepository + Clone,
    RT: RefreshTokenRepository + Clone,
{
    pub sign_up: Arc<SignUpWithEmailUsecase<R, RT>>,
    pub login: Arc<LoginWithEmailUsecase<R, RT>>,
    pub logout: Arc<LogoutUsecase<RT>>,
    pub refresh_token: Arc<RefreshTokenUsecase<R, RT>>,
    pub update_password: Arc<UpdatePasswordUsecase<R>>,
    pub update_name: Arc<UpdateNameUsecase<R>>,
    pub update_plan: Arc<UpdatePlanUsecase<R>>,
    pub get_me: Arc<GetMeUsecase<R>>,
    pub sign_up_with_oauth: Arc<SignUpWithOAuthUsecase<R, RT>>,
    pub sign_up_with_mobile_device: Arc<SignUpWithMobileDeviceUsecase<R, RT>>,
    pub google_oauth_client: Arc<GoogleOAuthClient>,
}
