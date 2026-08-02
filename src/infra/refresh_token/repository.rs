use crate::domain::error::DomainError;
use crate::domain::refresh_token::RefreshToken;
use crate::domain::refresh_token::repository::RefreshTokenRepository;
use crate::domain::refresh_token::vo::{RefreshTokenId, TokenHash};
use crate::domain::user::vo::UserId;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgRefreshTokenRepository {
    pool: PgPool,
}

impl PgRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for PgRefreshTokenRepository {
    async fn find_by_hash(
        &self,
        token_hash: &TokenHash,
    ) -> Result<Option<RefreshToken>, DomainError> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at, revoked_at
            FROM refresh_tokens
            WHERE token_hash = $1
            "#,
            token_hash.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        Ok(Some(RefreshToken {
            id: RefreshTokenId::new(row.id),
            user_id: UserId::new(row.user_id),
            token_hash: TokenHash::from_hash(row.token_hash),
            expires_at: row.expires_at,
            created_at: row.created_at,
            revoked_at: row.revoked_at,
        }))
    }

    async fn create(
        &self,
        user_id: UserId,
        token_hash: TokenHash,
        expires_at: DateTime<Utc>,
    ) -> Result<RefreshToken, DomainError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
            Values ($1, $2, $3)
            RETURNING id, user_id, token_hash, expires_at, created_at, revoked_at
            "#,
            user_id.value(),
            token_hash.value(),
            expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(RefreshToken {
            id: RefreshTokenId::new(row.id),
            user_id: UserId::new(row.user_id),
            token_hash: TokenHash::from_hash(row.token_hash),
            expires_at: row.expires_at,
            created_at: row.created_at,
            revoked_at: row.revoked_at,
        })
    }

    async fn revoke(&self, id: RefreshTokenId) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = now()
            WHERE id = $1
            "#,
            id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }

    async fn revoke_all_for_user(&self, user_id: UserId) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = now()
            WHERE user_id = $1
              AND revoked_at IS NULL
            "#,
            user_id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
