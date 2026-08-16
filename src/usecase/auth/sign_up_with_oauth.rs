use crate::domain::error::DomainError;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::user::NewUser;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{Email, OAuthProvider, ProviderUserId, UserId, UserName};
use crate::usecase::auth::token::{
    generate_refresh_token, hash_refresh_token, issue_token, issue_tokens,
};

use chrono::Utc;
use std::sync::Arc;

pub struct SignUpWithOAuthInput {
    pub provider: OAuthProvider,
    pub provider_user_id: String,
    pub name: String,
    pub email: Option<String>,
}

pub struct SignUpWithOAuthOutput {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub is_new_user: bool,
}

pub struct SignUpWithOAuthUsecase<R: UserRepository, RT: RefreshTokenRepository> {
    user_repository: Arc<R>,
    refresh_token_repository: Arc<RT>,
    jwt_secret: String,
}

impl<R: UserRepository, RT: RefreshTokenRepository> SignUpWithOAuthUsecase<R, RT> {
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
        input: SignUpWithOAuthInput,
    ) -> Result<SignUpWithOAuthOutput, DomainError> {
        let provider_user_id = ProviderUserId::new(input.provider_user_id);

        if let Some(existing) = self
            .user_repository
            .find_by_provider(&input.provider, &provider_user_id)
            .await?
        {
            let (access_token, refresh_token) = issue_tokens(
                existing.id,
                &self.jwt_secret,
                &*self.refresh_token_repository,
            )
            .await?;

            return Ok(SignUpWithOAuthOutput {
                user_id: existing.id.value(),
                access_token,
                refresh_token,
                is_new_user: false,
            });
        }

        let name = UserName::new(input.name)?;
        let email = input.email.map(Email::new).transpose()?;
        let new_user = NewUser::new_oauth(name, email, input.provider, provider_user_id);
        let created = self.user_repository.create(new_user).await?;

        let (access_token, refresh_token) = issue_tokens(
            created.id,
            &self.jwt_secret,
            &*self.refresh_token_repository,
        )
        .await?;

        Ok(SignUpWithOAuthOutput {
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
    use crate::domain::user::vo::UserId;
    use crate::domain::user::{repository::MockUserRepository, user_auth::UserAuth};

    use std::sync::Arc;

    fn make_usecase(
        mock: MockUserRepository,
        mock_rt: MockRefreshTokenRepository,
    ) -> SignUpWithOAuthUsecase<MockUserRepository, MockRefreshTokenRepository> {
        SignUpWithOAuthUsecase::new(Arc::new(mock), Arc::new(mock_rt), "test_secret".to_string())
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
    async fn new_user_is_created() {
        let mut mock = MockUserRepository::new();
        let mut mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_provider().returning(|_, _| Ok(None));
        mock.expect_create()
            .returning(|new_user| Ok(to_user(new_user, 2)));
        mock_rt
            .expect_create()
            .returning(|_, _, _| Ok(make_refresh_token()));

        let result = make_usecase(mock, mock_rt)
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
        let mut mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_provider()
            .returning(|_, _| Ok(Some(make_existing_user())));
        mock_rt
            .expect_create()
            .returning(|_, _, _| Ok(make_refresh_token()));

        let result = make_usecase(mock, mock_rt)
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
        let mock_rt = MockRefreshTokenRepository::new();

        mock.expect_find_by_provider().returning(|_, _| Ok(None));

        let result = make_usecase(mock, mock_rt)
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
