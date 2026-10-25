use crate::infra::oauth::google::GoogleOAuthClient;
use crate::usecase::auth::login_with_email::LoginWithEmailUsecase;
use crate::usecase::auth::logout::LogoutUsecase;
use crate::usecase::auth::refresh_token::RefreshTokenUsecase;
use crate::usecase::auth::sign_up_with_email::SignUpWithEmailUsecase;
use crate::usecase::auth::sign_up_with_mobile_device::SignUpWithMobileDeviceUsecase;
use crate::usecase::auth::sign_up_with_oauth::SignUpWithOAuthUsecase;
use crate::usecase::subscription::handle_webhook::HandleWebhookUsecase;
use crate::usecase::subscription::verify_subscription::VerifySubscriptionUsecase;
use crate::usecase::user::get_me::GetMeUsecase;
use crate::usecase::user::update_name::UpdateNameUsecase;
use crate::usecase::user::update_password::UpdatePasswordUsecase;
use crate::usecase::user::update_plan::UpdatePlanUsecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub sign_up: Arc<dyn SignUpWithEmailUsecase>,
    pub login: Arc<dyn LoginWithEmailUsecase>,
    pub logout: Arc<dyn LogoutUsecase>,
    pub refresh_token: Arc<dyn RefreshTokenUsecase>,
    pub update_password: Arc<dyn UpdatePasswordUsecase>,
    pub update_name: Arc<dyn UpdateNameUsecase>,
    pub update_plan: Arc<dyn UpdatePlanUsecase>,
    pub get_me: Arc<dyn GetMeUsecase>,
    pub sign_up_with_oauth: Arc<dyn SignUpWithOAuthUsecase>,
    pub sign_up_with_mobile_device: Arc<dyn SignUpWithMobileDeviceUsecase>,
    pub google_oauth_client: Arc<GoogleOAuthClient>,
    pub verify_subscription: Arc<dyn VerifySubscriptionUsecase>,
    pub handle_webhook: Arc<dyn HandleWebhookUsecase>,
}
