use crate::domain::error::DomainError;
use crate::presentation::error::AppError;
use crate::usecase::auth::token::Claims;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::env;

pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(DomainError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(DomainError::Unauthorized)?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| DomainError::Unexpected("JWT_SECRET not set".into()))?;

        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| DomainError::Unauthorized)?
        .claims;

        Ok(AuthUser(claims))
    }
}
