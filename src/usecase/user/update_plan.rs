use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::vo::{UserId, UserPlan};
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub struct UpdatePlanInput {
    pub user_id: i64,
    pub plan: String,
    pub plan_expires_at: Option<DateTime<Utc>>,
}

pub struct UpdatePlanUsecase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UpdatePlanUsecase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: UpdatePlanInput) -> Result<(), DomainError> {
        let user_id = UserId::new(input.user_id);

        self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let plan = UserPlan::from_str(&input.plan)
            .ok_or_else(|| DomainError::Unexpected(format!("unknown plan: {}", input.plan)))?;

        self.user_repository
            .update_plan(user_id, plan, input.plan_expires_at)
            .await?;

        Ok(())
    }
}
