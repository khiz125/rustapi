use crate::domain::user::vo::{AuthMethod, PasswordHash, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthKind {
    PasswordHash,
    OAuth,
}

#[derive(Debug, Clone)]
pub struct UserAuth {
    user_id: UserId,
    kind: AuthKind,
    auth: AuthMethod,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserAuth {
    pub fn new(user_id: UserId, auth: AuthMethod) -> Self {
        let kind = match &auth {
            AuthMethod::Password { .. } => AuthKind::PasswordHash,
            AuthMethod::OAuth { .. } => AuthKind::OAuth,
        };
        let now = Utc::now();
        Self {
            user_id,
            kind,
            auth,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn kind(&self) -> &AuthKind {
        &self.kind
    }

    pub fn auth(&self) -> &AuthMethod {
        &self.auth
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn update_password(&mut self, new_password_hash: PasswordHash) {
        match &self.auth {
            AuthMethod::Password { email, .. } => {
                self.auth = AuthMethod::Password {
                    email: email.clone(),
                    password_hash: new_password_hash,
                };
                self.updated_at = Utc::now();
            }
            AuthMethod::OAuth { .. } => panic!("Cannot update password for OAuth user"),
        }
    }
}
