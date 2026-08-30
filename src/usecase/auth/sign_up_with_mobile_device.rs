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
                let access_token =
                    issue_token(existing.id.value(), &self.jwt_token, existing.plan.as_str())?;
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
        let access_token = issue_token(created.id.value(), &self.jwt_token, created.plan.as_str())?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::refresh_token::RefreshToken;
    use crate::domain::refresh_token::repository::MockRefreshTokenRepository;
    use crate::domain::refresh_token::vo::{RefreshTokenId, TokenHash};
    use crate::domain::user::User;
    use crate::domain::user::repository::MockUserRepository;
    use crate::domain::user::user_auth::UserAuth;
    use crate::domain::user::vo::{DeviceId, UserId, UserPlan};
    use chrono::Utc;
    use std::sync::Arc;

    fn make_usecase(
        mock: MockUserRepository,
        mock_rt: MockRefreshTokenRepository,
    ) -> SignUpWithMobileDeviceUsecase<MockUserRepository, MockRefreshTokenRepository> {
        SignUpWithMobileDeviceUsecase::new(
            Arc::new(mock),
            Arc::new(mock_rt),
            "test_secret".to_string(),
        )
    }

    fn make_refresh_token() -> RefreshToken {
        RefreshToken {
            id: RefreshTokenId::new(1),
            user_id: UserId::new(1),
            token_hash: TokenHash::from_hash("hash"),
            expires_at: Utc::now() + chrono::Duration::days(30),
            created_at: Utc::now(),
            revoked_at: None,
        }
    }

    fn to_user(new_user: NewUser, id: i64) -> User {
        let user_id = UserId::new(id);
        User {
            id: user_id,
            name: new_user.name,
            auth: UserAuth {
                user_id,
                auth_method: new_user.auth.method,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            plan: UserPlan::Free,
            plan_expires_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn make_existing_user() -> User {
        let name = UserName::new("user_test1234").unwrap();
        let device_id = DeviceId::new("test-device-id-12345678");
        let new_user = NewUser::new_mobile_device(name, device_id);
        to_user(new_user, 1)
    }

    #[tokio::test]
    async fn new_user_is_created() {
        let mut mock = MockUserRepository::new();
        let mut mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_device_id().returning(|_| Ok(None));
        mock.expect_create()
            .returning(|new_user| Ok(to_user(new_user, 2)));
        mock_rt
            .expect_create()
            .returning(|_, _, _| Ok(make_refresh_token()));

        let result = make_usecase(mock, mock_rt)
            .execute(SignUpWithMobileDeviceInput {
                device_id: "test-device-id-123".to_string(),
                name: None,
            })
            .await
            .unwrap();

        assert_eq!(result.user_id, 2);
        assert!(result.is_new_user);
    }

    #[tokio::test]
    async fn existing_user_logs_in() {
        let mut mock = MockUserRepository::new();
        let mut mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_device_id()
            .returning(|_| Ok(Some(make_existing_user())));
        mock_rt
            .expect_create()
            .returning(|_, _, _| Ok(make_refresh_token()));

        let result = make_usecase(mock, mock_rt)
            .execute(SignUpWithMobileDeviceInput {
                device_id: "test-device-id-123".to_string(),
                name: None,
            })
            .await
            .unwrap();

        assert_eq!(result.user_id, 1);
        assert!(!result.is_new_user);
    }
}
