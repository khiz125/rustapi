pub mod claims;
pub mod jwt_secret;

pub use claims::Claims;
pub use jwt_secret::JwtSecret;

use crate::domain::error::DomainError;
use jsonwebtoken::{DecodingKey, Validation, decode};

pub fn verify_token(token: &str, secret: &JwtSecret) -> Result<Claims, DomainError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| DomainError::Unauthorized)
}
