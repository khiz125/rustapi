use crate::domain::user::user_auth::UserAuth;
use crate::domain::user::vo::{UserId, UserName};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub auth: Option<UserAuth>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: UserId, name: UserName, auth: Option<UserAuth>) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            auth,
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
    pub fn auth(&self) -> Option<&UserAuth> {
        self.auth.as_ref()
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

    pub fn set_auth(&mut self, auth: UserAuth) {
        self.auth = Some(auth);
        self.updated_at = Utc::now();
    }
}
