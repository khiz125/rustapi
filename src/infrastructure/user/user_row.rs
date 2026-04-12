use crate::domain::error::DomainError;
use crate::domain::types::UtcDateTime;

use crate::domain::user::User;
use crate::domain::user::user_auth::{AuthMethod, UserAuth};

use crate::domain::user::vo::email::Email;
use crate::domain::user::vo::oauth_provider::OAuthProvider;
use crate::domain::user::vo::password_hash::PasswordHash;
use crate::domain::user::vo::provider_user_id::ProviderUserId;
use crate::domain::user::vo::user_id::UserId;
use crate::domain::user::vo::user_name::UserName;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum AuthRow {
    Password {
        email: String,
        password_hash: String,
        created_at: UtcDateTime,
        updated_at: UtcDateTime,
    },
    OAuth {
        provider: OAuthProvider,
        provider_user_id: String,
        created_at: UtcDateTime,
        updated_at: UtcDateTime,
    },
}

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct UserRow {
    pub id: i64,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub kind: AuthRow,
}

#[allow(dead_code)]
impl UserRow {
    pub fn into_domain(self) -> Result<User, DomainError> {
        let user_id = UserId::new(self.id);
        let name = UserName::new(self.name).map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let (auth_method, auth_created_at, auth_updated_at) = match self.auth {
            AuthRow::Password {
                email,
                password_hash,
                created_at,
                updated_at,
            } => {
                let email =
                    Email::new(email).map_err(|e| DomainError::Unexpected(e.to_string()))?;
                let password_hash = PasswordHash::new(password_hash);
                (
                    AuthMethod::Password {
                        email,
                        password_hash,
                    },
                    created_at,
                    updated_at,
                )
            }
            AuthRow::OAuth {
                provider,
                provider_user_id,
                created_at,
                updated_at,
            } => {
                let provider_user_id = ProviderUserId::new(provider_user_id);
                (
                    AuthMethod::OAuth {
                        provider,
                        provider_user_id,
                    },
                    created_at,
                    updated_at,
                )
            }
        };

        Ok(User {
            id: user_id,
            name,
            auth: UserAuth {
                user_id,
                auth_method,
                created_at: auth_created_at,
                updated_at: auth_updated_at,
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

impl AuthRow {
    pub fn from_row(
        kind: &str,
        email: Option<String>,
        password_hash: Option<String>,
        provider: Option<String>,
        provider_user_id: Option<String>,
        created_at: UtcDateTime,
        updated_at: UtcDateTime,
    ) -> Result<Self, DomainError> {
        match kind {
            "password_hash" => Ok(AuthRow::Password {
                email: email.ok_or_else(|| DomainError::Unexpected("email is null".into()))?,
                password_hash: password_hash
                    .ok_or_else(|| DomainError::Unexpected("password_hash is null".into()))?,
                created_at,
                updated_at,
            }),
            "oauth" => Ok(AuthRow::OAuth {
                provider: OAuthProvider::from_str(
                    &provider.ok_or_else(|| DomainError::Unexpected("provider is null".into()))?,
                )
                .ok_or_else(|| DomainError::Unexpected("invalid OAuth provider".into()))?,
                provider_user_id: provider_user_id
                    .ok_or_else(|| DomainError::Unexpected("provider_user_id is null".into()))?,
                created_at,
                updated_at,
            }),
            other => Err(DomainError::Unexpected(format!(
                "unknown auth kind: {}",
                other
            ))),
        }
    }
}
