use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::AuthMethod;
use crate::domain::user::vo::Email;
use crate::usecase::auth::password_crypto::verify_password;
use crate::usecase::auth::token::issue_token;
use std::sync::Arc;

pub struct LoginWithEmailInput {
    pub email: String,
    pub password: String,
}

pub struct LoginWithEmailOutput {
    pub user_id: i64,
    pub token: String,
}

pub struct LoginWithEmailUsecase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_secret: String,
}

impl<R: UserRepository> LoginWithEmailUsecase<R> {
    pub fn new(user_repository: Arc<R>, jwt_secret: String) -> Self {
        Self {
            user_repository,
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
        };

        verify_password(&input.password, password_hash)?;

        let token = issue_token(user.id.value(), &self.jwt_secret)?;

        Ok(LoginWithEmailOutput {
            user_id: user.id.value(),
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use crate::domain::user::repository::MockUserRepository;
    use crate::domain::user::user_auth::UserAuth;
    use crate::domain::user::vo::{UserId, UserName};
    use crate::usecase::auth::password_crypto::hash_password;
    use std::sync::Arc;

    fn make_usecase(mock: MockUserRepository) -> LoginWithEmailUsecase<MockUserRepository> {
        LoginWithEmailUsecase::new(Arc::new(mock), "test_secret".to_string())
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
        mock.expect_find_by_email()
            .returning(|_| Ok(Some(make_user("password123"))));

        let result = make_usecase(mock)
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
        mock.expect_find_by_email().returning(|_| Ok(None));

        let result = make_usecase(mock)
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
        mock.expect_find_by_email()
            .returning(|_| Ok(Some(make_user("password123"))));

        let result = make_usecase(mock)
            .execute(LoginWithEmailInput {
                email: "test@example.com".to_string(),
                password: "wrong_password".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::IncorrectPassword)));
    }
}
