use super::user_row::{AuthRow, UserRow};
use crate::domain::error::DomainError;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::{AuthMethod, UserAuth};
use crate::domain::user::vo::UserName;
use crate::domain::user::vo::{
    email::Email, oauth_provider::OAuthProvider, password_hash::PasswordHash,
    provider_user_id::ProviderUserId, user_id::UserId,
};
use crate::domain::user::{NewUser, User};

use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError> {
        let row = sqlx::query!(
            r#"
              SELECT
                u.id,
                u.name,
                u.created_at,
                u.updated_at,
                a.kind::text as "kind!: String",
                a.email,
                a.password_hash,
                a.provider,
                a.provider_user_id,
                a.created_at as auth_created_at,
                a.updated_at as auth_updated_at
              FROM users u
              INNER JOIN user_auth a ON a.user_id = u.id
              WHERE u.id = $1
            "#,
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let user_row = UserRow {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
            kind: AuthRow::from_row(
                &row.kind,
                row.email,
                row.password_hash,
                row.provider,
                row.provider_user_id,
                row.auth_created_at,
                row.auth_updated_at,
            )?,
        };
        Ok(Some(user_row.into_domain()?))
    }
    async fn find_by_provider(
        &self,
        provider: &OAuthProvider,
        provider_user_id: &ProviderUserId,
    ) -> Result<Option<User>, DomainError> {
        let row = sqlx::query!(
            r#"
        SELECT
            u.id,
            u.name,
            u.created_at,
            u.updated_at,
            a.kind::text        AS "kind!: String",
            a.email,
            a.password_hash,
            a.provider,
            a.provider_user_id,
            a.created_at        AS auth_created_at,
            a.updated_at        AS auth_updated_at
        FROM users u
        INNER JOIN user_auth a ON a.user_id = u.id
        WHERE a.provider = $1
          AND a.provider_user_id = $2
        "#,
            provider.as_str(),
            provider_user_id.value(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let user_row = UserRow {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
            kind: AuthRow::from_row(
                &row.kind,
                row.email,
                row.password_hash,
                row.provider,
                row.provider_user_id,
                row.auth_created_at,
                row.auth_updated_at,
            )?,
        };

        Ok(Some(user_row.into_domain()?))
    }
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let row = sqlx::query!(
            r#"
        SELECT
            u.id,
            u.name,
            u.created_at,
            u.updated_at,
            a.kind::text        AS "kind!: String",
            a.email,
            a.password_hash,
            a.provider,
            a.provider_user_id,
            a.created_at        AS auth_created_at,
            a.updated_at        AS auth_updated_at
        FROM users u
        INNER JOIN user_auth a ON a.user_id = u.id
        WHERE a.email = $1
        "#,
            email.value(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let user_row = UserRow {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
            kind: AuthRow::from_row(
                &row.kind,
                row.email,
                row.password_hash,
                row.provider,
                row.provider_user_id,
                row.auth_created_at,
                row.auth_updated_at,
            )?,
        };

        Ok(Some(user_row.into_domain()?))
    }
    async fn create(&self, new_user: NewUser) -> Result<User, DomainError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let user_row = sqlx::query!(
            r#"
              INSERT INTO users (name)
              VALUES ($1)
              RETURNING id, name, created_at, updated_at
            "#,
            new_user.name.value()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let auth_row: AuthRow = match &new_user.auth.method {
            AuthMethod::Password {
                email,
                password_hash,
            } => {
                let row = sqlx::query!(
                    r#"
                    INSERT INTO user_auth (user_id, kind, email, password_hash)
                    VALUES ($1, 'password_hash', $2, $3)
                    RETURNING
                        user_id,
                        kind::text AS "kind!: String",
                        email,
                        password_hash,
                        provider,
                        provider_user_id,
                        created_at,
                        updated_at
                    "#,
                    user_row.id,
                    email.value(),
                    password_hash.value()
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| DomainError::Unexpected(e.to_string()))?;

                AuthRow::Password {
                    email: row
                        .email
                        .ok_or_else(|| DomainError::Unexpected("email is null".into()))?,
                    password_hash: row
                        .password_hash
                        .ok_or_else(|| DomainError::Unexpected("password_hash is null".into()))?,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            }

            AuthMethod::OAuth {
                email,
                provider,
                provider_user_id,
            } => {
                let row = sqlx::query!(
                    r#"
                INSERT INTO user_auth (user_id, kind, email, provider, provider_user_id)
                VALUES ($1, 'oauth', $2, $3, $4)
                    RETURNING
                      user_id,
                      kind::text AS "kind!: String",
                      email,
                      password_hash,
                      provider,
                      provider_user_id,
                      created_at,
                      updated_at
                "#,
                    user_row.id,
                    email.as_ref().map(|e| e.value()),
                    provider.as_str(),
                    provider_user_id.value()
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| DomainError::Unexpected(e.to_string()))?;

                AuthRow::OAuth {
                    email: row.email,
                    provider: OAuthProvider::from_str(
                        &row.provider
                            .ok_or_else(|| DomainError::Unexpected("provider is null".into()))?,
                    )
                    .ok_or_else(|| DomainError::Unexpected("invalid OAuth provider".into()))?,
                    provider_user_id: row.provider_user_id.ok_or_else(|| {
                        DomainError::Unexpected("provider_user_id is null".into())
                    })?,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            }
        };

        tx.commit()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let user_row_dto = UserRow {
            id: user_row.id,
            name: user_row.name,
            created_at: user_row.created_at,
            updated_at: user_row.updated_at,
            kind: auth_row,
        };

        user_row_dto.into_domain()
    }

    async fn save_auth(&self, user_auth: &UserAuth) -> Result<(), DomainError> {
        match &user_auth.auth_method {
            AuthMethod::Password { password_hash, .. } => {
                sqlx::query!(
                    r#"
                UPDATE user_auth
                SET password_hash = $1,
                    updated_at = $2
                WHERE user_id = $3
                "#,
                    password_hash.value(),
                    user_auth.updated_at,
                    user_auth.user_id.value()
                )
                .execute(&self.pool)
                .await
                .map_err(|e| DomainError::Unexpected(e.to_string()))?;
            }
            AuthMethod::OAuth { .. } => {
                return Err(DomainError::Unexpected(
                    "cannot save OAuth auth medhod".into(),
                ));
            }
        }
        Ok(())
    }

    async fn update_name(&self, user_id: UserId, new_name: UserName) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET name = $1,
            updated_at = now()
            WHERE id = $2
            "#,
            new_name.value(),
            user_id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
