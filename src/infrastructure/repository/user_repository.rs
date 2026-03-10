use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::error::DomainError;
use crate::domain::user::entity::User;
use crate::domain::user::repository::UserRepository;

pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn find_by_id(&self, id: i32) -> Result<User, DomainError> {
        // todo!()
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
        // todo!()
    }
}
