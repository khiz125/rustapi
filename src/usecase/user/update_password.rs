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

        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let current_hash = match &user.auth.auth_method {
            AuthMethod::Password { password_hash, .. } => password_hash.value().to_string(),
            AuthMethod::OAuth { .. } => return Err(DomainError::NotPasswordAuthUser),
        };

        verify_password(&input.current_password, &current_hash)?;

        let new_hash = hash_password(&input.new_password)?;

        self.user_repository
            .update_password(user_id, new_hash)
            .await?;

        Ok(())
    }
}
