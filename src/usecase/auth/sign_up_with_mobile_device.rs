use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::new_user::NewUser;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{DeviceId, UserName};
use crate::usecase::auth::token::{generate_refresh_token, hash_refresh_token, issue_token};
use chrono::Utc;
use std::sync::Arc;

pub struct SignUpWithMobileDeviceInput {
    pub device_id: String,
    pub name: Option<String>,
}

pub struct SignUpWithMobileDeviceOutput {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub is_new_user: bool,
}

pub struct SignUpWithMobileDeviceUsecase<R: UserRepository, RT: RefreshTokenRepository> {
    user_repository: Arc<R>,
    refresh_token_repository: Arc<RT>,
    jwt_token: String,
}

impl<R: UserRepository, RT: RefreshTokenRepository> SignUpWithMobileDeviceUsecase<R, RT> {
    pub fn new(
        user_repository: Arc<R>,
        refresh_token_repository: Arc<RT>,
        jwt_token: String,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            jwt_token,
        }
    }

    pub async fn execute(
        &self,
        input: SignUpWithMobileDeviceInput,
    ) -> Result<SignUpWithMobileDeviceOutput, DomainError> {
        let device_id = DeviceId::new(input.device_id);
        {
            if let Some(existing) = self.user_repository.find_by_device_id(&device_id).await? {
                let access_token = issue_token(existing.id.value(), &self.jwt_token)?;
                let refresh_token = generate_refresh_token();
                let token_hash = hash_refresh_token(&refresh_token);
                let expires_at = Utc::now() + chrono::Duration::days(30);

                self.refresh_token_repository
                    .create(existing.id, token_hash, expires_at)
                    .await?;

                return Ok(SignUpWithMobileDeviceOutput {
                    user_id: existing.id.value(),
                    access_token,
                    refresh_token,
                    is_new_user: false,
                });
            }
        }

        let name = input
            .name
            .unwrap_or_else(|| format!("user_{}", &device_id.value()[..8]));
        let user_name = UserName::new(name)?;
        let new_user = NewUser::new_mobile_device(user_name, device_id);
        let created = self.user_repository.create(new_user).await?;
        let access_token = issue_token(created.id.value(), &self.jwt_token)?;
        let refresh_token = generate_refresh_token();
        let token_hash = hash_refresh_token(&refresh_token);
        let expires_at = Utc::now() + chrono::Duration::days(30);

        self.refresh_token_repository
            .create(created.id, token_hash, expires_at)
            .await?;

        Ok(SignUpWithMobileDeviceOutput {
            user_id: created.id.value(),
            access_token,
            refresh_token,
            is_new_user: true,
        })
    }
}
