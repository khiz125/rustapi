use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{UserId, UserName};
use std::sync::Arc;

pub struct UpdateNameInput {
    pub user_id: i64,
    pub new_name: String,
}

#[async_trait::async_trait]
pub trait UpdateNameUsecase: Send + Sync {
    async fn execute(&self, input: UpdateNameInput) -> Result<(), DomainError>;
}

pub struct UpdateNameUsecaseImpl<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UpdateNameUsecaseImpl<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl<R> UpdateNameUsecase for UpdateNameUsecaseImpl<R>
where
    R: UserRepository + Send + Sync,
{
    async fn execute(&self, input: UpdateNameInput) -> Result<(), DomainError> {
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
