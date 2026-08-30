use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::AuthMethod;
use crate::domain::user::vo::UserId;
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub struct GetMeInput {
    pub user_id: i64,
}

pub struct GetMeOutput {
    pub user_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub auth_kind: String,
    pub plan: String,
    pub plan_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub struct GetMeUsecase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> GetMeUsecase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: GetMeInput) -> Result<GetMeOutput, DomainError> {
        let user = self
            .user_repository
            .find_by_id(UserId::new(input.user_id))
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let email = user.auth.email().map(|e| e.value().to_string());

        let auth_kind = match &user.auth.auth_method {
            AuthMethod::Password { .. } => "password_hash",
            AuthMethod::OAuth { .. } => "oauth",
            AuthMethod::MobileDevice { .. } => "mobile_device",
        }
        .to_string();

        Ok(GetMeOutput {
            user_id: user.id.value(),
            name: user.name.value().to_string(),
            email,
            auth_kind,
            plan: user.plan.as_str().to_string(),
            plan_expires_at: user.plan_expires_at,
            created_at: user.created_at,
        })
    }
}
