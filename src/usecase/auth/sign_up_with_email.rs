use crate::domain::error::DomainError;
use crate::domain::user::NewUser;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{Email, UserName};
use crate::usecase::auth::password_crypto::hash_password;
use crate::usecase::auth::token::issue_token;

use std::sync::Arc;

pub struct SignUpWithEmailInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct SignUpWithEmailOutput {
    pub user_id: i64,
    pub token: String,
}

pub struct SignUpWithEmailUsecase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_secret: String,
}

impl<R: UserRepository> SignUpWithEmailUsecase<R> {
    pub fn new(user_repository: Arc<R>, jwt_secret: String) -> Self {
        Self {
            user_repository,
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
        let token = issue_token(created.id.value(), &self.jwt_secret)?;

        Ok(SignUpWithEmailOutput {
            user_id: created.id.value(),
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
    use crate::domain::user::vo::{Email, UserId};
    use crate::usecase::auth::sign_up_with_email::SignUpWithEmailInput;
    use std::sync::Arc;

    fn make_usecase(mock: MockUserRepository) -> SignUpWithEmailUsecase<MockUserRepository> {
        SignUpWithEmailUsecase::new(Arc::new(mock), "test_secret".to_string())
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

    #[tokio::test]
    async fn success() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_email().returning(|_| Ok(None));
        mock.expect_create()
            .returning(|new_user| Ok(to_user(new_user)));

        let result = make_usecase(mock)
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
        mock.expect_find_by_email()
            .returning(|_| Ok(Some(make_existing_user())));

        let result = make_usecase(mock)
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
        let result = make_usecase(MockUserRepository::new())
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
        let result = make_usecase(MockUserRepository::new())
            .execute(SignUpWithEmailInput {
                name: "".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::InvalidUserName(_))));
    }
}
