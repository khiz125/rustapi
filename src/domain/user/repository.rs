use crate::domain::error::DomainError;
use crate::domain::user::entity::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<User, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn list(&self) -> Result<Vec<User>, DomainError>;
}
