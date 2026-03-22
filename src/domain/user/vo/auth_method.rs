use crate::domain::user::vo::{Email, OAuthProvider, PasswordHash, ProviderUserId};

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
