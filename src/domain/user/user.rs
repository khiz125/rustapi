use crate::domain::user::user_auth::UserAuth;
use crate::domain::user::vo::UserPlan;
use crate::domain::user::vo::{user_id::UserId, user_name::UserName};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub auth: UserAuth,
    pub plan: UserPlan,
    pub plan_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: UserId, name: UserName, auth: UserAuth, plan: UserPlan) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            auth,
            plan,
            plan_expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }
    pub fn name(&self) -> &UserName {
        &self.name
    }
    pub fn auth(&self) -> &UserAuth {
        &self.auth
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn update_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn change_name(&mut self, name: UserName) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn is_premium(&self) -> bool {
        if !self.plan.is_premium() {
            return false;
        }

        match self.plan_expires_at {
            Some(expires_at) => expires_at > Utc::now(),
            None => true,
        }
    }
}
