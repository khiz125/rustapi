use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{UserId, UserName};
use std::sync::Arc;

pub struct UpdateNameInput {
    pub user_id: i64,
    pub new_name: String,
}

pub struct UpdateNameUsecase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UpdateNameUsecase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: UpdateNameInput) -> Result<(), DomainError> {
        let user_id = UserId::new(input.user_id);

        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let new_name = UserName::new(input.new_name)?;

        self.user_repository.update_name(user.id, new_name).await?;

        Ok(())
    }
}
