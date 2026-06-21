use crate::domain::error::DomainError;
use crate::domain::user::NewUser;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{Email, OAuthProvider, ProviderUserId, UserName};
use crate::usecase::auth::token::issue_token;

use std::sync::Arc;

pub struct SignUpWithOAuthInput {
    pub provider: OAuthProvider,
    pub provider_user_id: String,
    pub name: String,
    pub email: Option<String>,
}

pub struct SignUpWithOAuthOutput {
    pub user_id: i64,
    pub token: String,
    pub is_new_user: bool,
}

pub struct SignUpWithOAuthUsecase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_secret: String,
}

impl<R: UserRepository> SignUpWithOAuthUsecase<R> {
    pub fn new(user_repository: Arc<R>, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn execute(
        &self,
        input: SignUpWithOAuthInput,
    ) -> Result<SignUpWithOAuthOutput, DomainError> {
        let provider_user_id = ProviderUserId::new(input.provider_user_id);
        if let Some(existing) = self
            .user_repository
            .find_by_provider(&input.provider, &provider_user_id)
            .await?
        {
            let token = issue_token(existing.id.value(), &self.jwt_secret)?;
            return Ok(SignUpWithOAuthOutput {
                user_id: existing.id.value(),
                token,
                is_new_user: false,
            });
        }

        let name = UserName::new(input.name)?;
        let email = input.email.map(Email::new).transpose()?;
        let new_user = NewUser::new_oauth(name, email, input.provider, provider_user_id);
        let created = self.user_repository.create(new_user).await?;
        let token = issue_token(created.id.value(), &self.jwt_secret)?;

        Ok(SignUpWithOAuthOutput {
            user_id: created.id.value(),
            token,
            is_new_user: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use crate::domain::user::vo::UserId;
    use crate::domain::user::{repository::MockUserRepository, user_auth::UserAuth};

    use std::sync::Arc;

    fn make_usecase(mock: MockUserRepository) -> SignUpWithOAuthUsecase<MockUserRepository> {
        SignUpWithOAuthUsecase::new(Arc::new(mock), "test_secret".to_string())
    }

    fn to_user(new_user: NewUser, id: i64) -> User {
        let user_id = UserId::new(id);
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
        let name = UserName::new("testuser").unwrap();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let provider_user_id = ProviderUserId::new("google_123".to_string());
        let new_user =
            NewUser::new_oauth(name, Some(email), OAuthProvider::Google, provider_user_id);
        to_user(new_user, 1)
    }

    #[tokio::test]
    async fn new_user_is_created() {
        let mut mock = MockUserRepository::new();

        mock.expect_find_by_provider().returning(|_, _| Ok(None));
        mock.expect_create()
            .returning(|new_user| Ok(to_user(new_user, 2)));

        let result = make_usecase(mock)
            .execute(SignUpWithOAuthInput {
                provider: OAuthProvider::Google,
                provider_user_id: "google_456".to_string(),
                name: "testuser".to_string(),
                email: Some("test@example.com".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(result.user_id, 2);
        assert!(result.is_new_user);
    }

    #[tokio::test]
    async fn existing_user_logs_in() {
        let mut mock = MockUserRepository::new();

        mock.expect_find_by_provider()
            .returning(|_, _| Ok(Some(make_existing_user())));

        let result = make_usecase(mock)
            .execute(SignUpWithOAuthInput {
                provider: OAuthProvider::Google,
                provider_user_id: "google_123".to_lowercase(),
                name: "testuser".to_string(),
                email: Some("test@example.com".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(result.user_id, 1);
        assert!(!result.is_new_user);
    }

    #[tokio::test]
    async fn invalid_name() {
        let mut mock = MockUserRepository::new();

        mock.expect_find_by_provider().returning(|_, _| Ok(None));

        let result = make_usecase(mock)
            .execute(SignUpWithOAuthInput {
                provider: OAuthProvider::Google,
                provider_user_id: "google_456".to_string(),
                name: "".to_string(),
                email: Some("test@example.com".to_string()),
            })
            .await;

        assert!(matches!(result, Err(DomainError::InvalidUserName(_))));
    }
}
