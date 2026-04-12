use crate::domain::{
    error::DomainError,
    user::vo::{
        email::Email, oauth_provider::OAuthProvider, password_hash::PasswordHash,
        provider_user_id::ProviderUserId, user_id::UserId,
    },
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]

pub enum AuthMethod {
    Password {
        email: Email,
        password_hash: PasswordHash,
    },
    OAuth {
        provider: OAuthProvider,
        provider_user_id: ProviderUserId,
    },
}

#[derive(Debug, Clone)]
pub struct UserAuth {
    pub user_id: UserId,
    pub auth_method: AuthMethod,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAuth {
    pub fn new_password(user_id: UserId, email: Email, password_hash: PasswordHash) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            auth_method: AuthMethod::Password {
                email,
                password_hash,
            },
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_oauth(
        user_id: UserId,
        provider: OAuthProvider,
        provider_user_id: ProviderUserId,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            auth_method: AuthMethod::OAuth {
                provider,
                provider_user_id,
            },
            created_at: now,
            updated_at: now,
        }
    }

    pub fn email(&self) -> Option<&Email> {
        match &self.auth_method {
            AuthMethod::Password { email, .. } => Some(email),
            AuthMethod::OAuth { .. } => None,
        }
    }

    pub fn change_password(&mut self, new_password_hash: PasswordHash) -> Result<(), DomainError> {
        match &self.auth_method {
            AuthMethod::Password { email, .. } => {
                self.auth_method = AuthMethod::Password {
                    email: email.clone(),
                    password_hash: new_password_hash,
                };
                self.updated_at = Utc::now();
                Ok(())
            }
            AuthMethod::OAuth { .. } => Err(DomainError::NotPasswordAuthUser),
        }
    }
}
