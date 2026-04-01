use crate::domain::error::DomainError;
use crate::domain::user::User;
use crate::domain::user::vo::{email::Email, user_id::UserId};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError>;
    //async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
    /*
    async fn update_name(&self, user_id: UserId, new_name: UserName) -> Result<(), DomainError>;
    async fn update_password(
        &self,
        user_id: UserId,
        new_password_hash: PasswordHash,
    ) -> Result<(), DomainError>;
    */
}
