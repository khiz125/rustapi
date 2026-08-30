use crate::domain::error::DomainError;
use crate::domain::refresh_token;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::NewUser;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{Email, UserName};
use crate::usecase::auth::password_crypto::hash_password;
use crate::usecase::auth::token::{
    generate_refresh_token, hash_refresh_token, issue_token, issue_tokens,
};

use chrono::Utc;
use std::sync::Arc;

pub struct SignUpWithEmailInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct SignUpWithEmailOutput {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
}

pub struct SignUpWithEmailUsecase<R: UserRepository, RT: RefreshTokenRepository> {
    user_repository: Arc<R>,
    refresh_token_repository: Arc<RT>,
    jwt_secret: String,
}

impl<R: UserRepository, RT: RefreshTokenRepository> SignUpWithEmailUsecase<R, RT> {
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
        input: SignUpWithEmailInput,
    ) -> Result<SignUpWithEmailOutput, DomainError> {
        let name = UserName::new(input.name)?;
        let email = Email::new(input.email)?;

        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(DomainError::EmailAlreadyExists);
        }

        let password_hash = hash_password(&input.password)?;
        let new_user = NewUser::new_password(name, email, password_hash);
        let created = self.user_repository.create(new_user).await?;

        let (access_token, refresh_token) = issue_tokens(
            created.id,
            &self.jwt_secret,
            created.plan.as_str(),
            &*self.refresh_token_repository,
        )
        .await?;

        Ok(SignUpWithEmailOutput {
            user_id: created.id.value(),
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
    use crate::domain::user::vo::{Email, UserId, UserPlan};
    use crate::usecase::auth::sign_up_with_email::SignUpWithEmailInput;
    use std::sync::Arc;

    fn make_usecase(
        mock: MockUserRepository,
        mock_rt: MockRefreshTokenRepository,
    ) -> SignUpWithEmailUsecase<MockUserRepository, MockRefreshTokenRepository> {
        SignUpWithEmailUsecase::new(Arc::new(mock), Arc::new(mock_rt), "test_secret".to_string())
    }

    fn to_user(new_user: NewUser) -> User {
        let user_id = UserId::new(1);
        User {
            id: user_id,
            name: new_user.name,
            auth: UserAuth {
                user_id,
                auth_method: new_user.auth.method,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            plan: UserPlan::Free,
            plan_expires_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn make_existing_user() -> User {
        let name = UserName::new("testname".to_string()).unwrap();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password_hash = hash_password("hash").unwrap();
        to_user(NewUser::new_password(name, email, password_hash))
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

    #[tokio::test]
    async fn success() {
        let mut mock = MockUserRepository::new();
        let mut mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_email().returning(|_| Ok(None));
        mock.expect_create()
            .returning(|new_user| Ok(to_user(new_user)));
        mock_rt
            .expect_create()
            .returning(|_, _, _| Ok(make_refresh_token()));

        let result = make_usecase(mock, mock_rt)
            .execute(SignUpWithEmailInput {
                name: "testname".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn email_already_exists() {
        let mut mock = MockUserRepository::new();
        let mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_email()
            .returning(|_| Ok(Some(make_existing_user())));

        let result = make_usecase(mock, mock_rt)
            .execute(SignUpWithEmailInput {
                name: "testname".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::EmailAlreadyExists)));
    }

    #[tokio::test]
    async fn invalid_email() {
        let result = make_usecase(MockUserRepository::new(), MockRefreshTokenRepository::new())
            .execute(SignUpWithEmailInput {
                name: "testuser".to_string(),
                email: "invalid_email".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::InvalidEmail(_))));
    }

    #[tokio::test]
    async fn invalid_name() {
        let result = make_usecase(MockUserRepository::new(), MockRefreshTokenRepository::new())
            .execute(SignUpWithEmailInput {
                name: "".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::InvalidUserName(_))));
    }
}
