use chrono::Utc;

use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::AuthMethod;
use crate::domain::user::vo::email::Email;
use crate::usecase::auth::password_crypto::verify_password;
use crate::usecase::auth::token::{generate_refresh_token, hash_refresh_token, issue_token};
use std::sync::Arc;

pub struct LoginWithEmailInput {
    pub email: String,
    pub password: String,
}

pub struct LoginWithEmailOutput {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
}

pub struct LoginWithEmailUsecase<R: UserRepository, RT: RefreshTokenRepository> {
    user_repository: Arc<R>,
    jwt_secret: String,
    refresh_token_repository: Arc<RT>,
}

impl<R: UserRepository, RT: RefreshTokenRepository> LoginWithEmailUsecase<R, RT> {
    pub fn new(
        user_repository: Arc<R>,
        refresh_token_repository: Arc<RT>,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            jwt_secret,
        }
    }

    pub async fn execute(
        &self,
        input: LoginWithEmailInput,
    ) -> Result<LoginWithEmailOutput, DomainError> {
        let email = Email::new(input.email)?;

        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let password_hash = match &user.auth.auth_method {
            AuthMethod::Password { password_hash, .. } => password_hash.value(),
            AuthMethod::OAuth { .. } => return Err(DomainError::NotPasswordAuthUser),
            AuthMethod::MobileDevice { .. } => return Err(DomainError::NotPasswordAuthUser),
        };

        verify_password(&input.password, password_hash)?;

        let access_token = issue_token(user.id.value(), &self.jwt_secret)?;
        let refresh_token = generate_refresh_token();
        let token_hash = hash_refresh_token(&refresh_token);
        let expires_at = Utc::now() + chrono::Duration::days(30);

        self.refresh_token_repository
            .create(user.id, token_hash, expires_at)
            .await?;

        Ok(LoginWithEmailOutput {
            user_id: user.id.value(),
            access_token,
            refresh_token,
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
    use crate::domain::user::vo::{UserId, UserName};
    use crate::usecase::auth::password_crypto::hash_password;
    use std::sync::Arc;

    fn make_usecase(
        mock: MockUserRepository,
        mock_rt: MockRefreshTokenRepository,
    ) -> LoginWithEmailUsecase<MockUserRepository, MockRefreshTokenRepository> {
        LoginWithEmailUsecase::new(Arc::new(mock), Arc::new(mock_rt), "test_secret".to_string())
    }

    fn make_user(raw_password: &str) -> User {
        let user_id = UserId::new(1);
        let name = UserName::new("testuser").unwrap();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password_hash = hash_password(raw_password).unwrap();
        let auth = UserAuth::new_password(user_id, email, password_hash);
        User::new(user_id, name, auth)
    }

    #[tokio::test]
    async fn success() {
        let mut mock = MockUserRepository::new();
        let mut mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_email()
            .returning(|_| Ok(Some(make_user("password123"))));
        mock_rt.expect_create().returning(|_, _, _| {
            Ok(RefreshToken {
                id: RefreshTokenId::new(1),
                user_id: UserId::new(1),
                token_hash: TokenHash::from_hash("hash"),
                expires_at: chrono::Utc::now() + chrono::Duration::days(30),
                created_at: chrono::Utc::now(),
                revoked_at: None,
            })
        });

        let result = make_usecase(mock, mock_rt)
            .execute(LoginWithEmailInput {
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn user_not_found() {
        let mut mock = MockUserRepository::new();
        let mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_email().returning(|_| Ok(None));

        let result = make_usecase(mock, mock_rt)
            .execute(LoginWithEmailInput {
                email: "not_exist@example.com".to_string(),
                password: "invalid_password".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::UserNotFound)));
    }

    #[tokio::test]
    async fn incorrect_password() {
        let mut mock = MockUserRepository::new();
        let mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_email()
            .returning(|_| Ok(Some(make_user("password123"))));

        let result = make_usecase(mock, mock_rt)
            .execute(LoginWithEmailInput {
                email: "test@example.com".to_string(),
                password: "wrong_password".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::IncorrectPassword)));
    }
}
