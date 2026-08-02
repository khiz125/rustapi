use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::AuthMethod;
use crate::domain::user::vo::UserId;
use crate::usecase::auth::password_crypto::{hash_password, verify_password};
use std::sync::Arc;

pub struct UpdatePasswordInput {
    pub user_id: i64,
    pub current_password: String,
    pub new_password: String,
}

pub struct UpdatePasswordUsecase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UpdatePasswordUsecase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: UpdatePasswordInput) -> Result<(), DomainError> {
        let user_id = UserId::new(input.user_id);

        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let current_hash = match &user.auth.auth_method {
            AuthMethod::Password { password_hash, .. } => password_hash.value().to_string(),
            AuthMethod::OAuth { .. } => return Err(DomainError::NotPasswordAuthUser),
            AuthMethod::MobileDevice { .. } => return Err(DomainError::NotPasswordAuthUser),
        };

        verify_password(&input.current_password, &current_hash)?;

        let new_hash = hash_password(&input.new_password)?;

        user.auth.change_password(new_hash)?;

        self.user_repository.save_auth(&user.auth).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use crate::domain::user::repository::MockUserRepository;
    use crate::domain::user::user_auth::UserAuth;
    use crate::domain::user::vo::{Email, OAuthProvider, ProviderUserId, UserId, UserName};
    use crate::usecase::auth::password_crypto::hash_password;
    use std::sync::Arc;

    fn make_usecase(mock: MockUserRepository) -> UpdatePasswordUsecase<MockUserRepository> {
        UpdatePasswordUsecase::new(Arc::new(mock))
    }

    fn make_user(raw_password: &str) -> User {
        let user_id = UserId::new(1);
        let name = UserName::new("testname").unwrap();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password_hash = hash_password(raw_password).unwrap();
        let auth = UserAuth::new_password(user_id, email, password_hash);
        User::new(user_id, name, auth)
    }

    fn make_oauth_user() -> User {
        let user_id = UserId::new(1);
        let email = Email::new("test@example.com".to_string()).unwrap();
        let name = UserName::new("oauth_user").unwrap();
        let provider_user_id = ProviderUserId::new("google_user".to_string());
        let auth = UserAuth::new_oauth(
            user_id,
            Some(email),
            OAuthProvider::Google,
            provider_user_id,
        );
        User::new(user_id, name, auth)
    }

    #[tokio::test]
    async fn success() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .returning(|_| Ok(Some(make_user("password123"))));
        mock.expect_save_auth().returning(|_| Ok(()));

        let result = make_usecase(mock)
            .execute(UpdatePasswordInput {
                user_id: 1,
                current_password: "password123".to_string(),
                new_password: "password456".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn incorrect_password() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .returning(|_| Ok(Some(make_user("password123"))));

        let result = make_usecase(mock)
            .execute(UpdatePasswordInput {
                user_id: 1,
                current_password: "wrong_pssword".to_string(),
                new_password: "new_password".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::IncorrectPassword)));
    }

    #[tokio::test]
    async fn oauth_user_cannot_update_password() {
        let mut mock = MockUserRepository::new();
        mock.expect_find_by_id()
            .returning(|_| Ok(Some(make_oauth_user())));

        let result = make_usecase(mock)
            .execute(UpdatePasswordInput {
                user_id: 1,
                current_password: "pass".to_string(),
                new_password: "new_pass".to_string(),
            })
            .await;

        assert!(matches!(result, Err(DomainError::NotPasswordAuthUser)));
    }
}
