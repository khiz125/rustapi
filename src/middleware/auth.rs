use crate::domain::error::DomainError;
use crate::presentation::error::AppError;
use crate::shared::auth::{Claims, JwtSecret, verify_token};

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    JwtSecret: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let secret = JwtSecret::from_ref(state);
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::from(DomainError::Unauthorized))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::from(DomainError::Unauthorized))?;

        let claims = verify_token(token, &secret).map_err(AppError::from)?;

        Ok(AuthUser(claims))
    }
}
