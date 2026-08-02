use crate::domain::error::DomainError;
use crate::domain::user::vo::device_id::DeviceId;
use crate::domain::user::vo::email::Email;
use crate::domain::user::vo::oauth_provider::OAuthProvider;
use crate::domain::user::vo::password_hash::PasswordHash;
use crate::domain::user::vo::provider_user_id::ProviderUserId;
use crate::domain::user::vo::user_id::UserId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]

pub enum AuthMethod {
    Password {
        email: Email,
        password_hash: PasswordHash,
    },
    OAuth {
        email: Option<Email>,
        provider: OAuthProvider,
        provider_user_id: ProviderUserId,
    },
    MobileDevice {
        device_id: DeviceId,
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
        email: Option<Email>,
        provider: OAuthProvider,
        provider_user_id: ProviderUserId,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            auth_method: AuthMethod::OAuth {
                email,
                provider,
                provider_user_id,
            },
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_mobile_device(user_id: UserId, device_id: DeviceId) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            auth_method: AuthMethod::MobileDevice { device_id },
            created_at: now,
            updated_at: now,
        }
    }

    pub fn email(&self) -> Option<&Email> {
        match &self.auth_method {
            AuthMethod::Password { email, .. } => Some(email),
            AuthMethod::OAuth { email, .. } => email.as_ref(),
            AuthMethod::MobileDevice { .. } => None,
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
            AuthMethod::MobileDevice { .. } => Err(DomainError::NotPasswordAuthUser),
        }
    }
}
