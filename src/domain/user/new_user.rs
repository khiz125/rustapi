use crate::domain::user::user_auth::AuthMethod;
use crate::domain::user::vo::{Email, OAuthProvider, PasswordHash, ProviderUserId, UserName};

#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: UserName,
    pub auth: NewUserAuth,
}

#[derive(Debug, Clone)]
pub struct NewUserAuth {
    pub method: AuthMethod,
}

impl NewUser {
    pub fn new_password(name: UserName, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            name,
            auth: NewUserAuth {
                method: AuthMethod::Password {
                    email,
                    password_hash,
                },
            },
        }
    }

    pub fn new_oauth(
        name: UserName,
        provider: OAuthProvider,
        provider_user_id: ProviderUserId,
    ) -> Self {
        Self {
            name,
            auth: NewUserAuth {
                method: AuthMethod::OAuth {
                    provider,
                    provider_user_id,
                },
            },
        }
    }
}
