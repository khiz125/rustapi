use super::row::{AuthRow, UserRow};
use crate::domain::error::DomainError;
use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::{AuthMethod, UserAuth};
use crate::domain::user::vo::{
    email::Email, oauth_provider::OAuthProvider, password_hash::PasswordHash,
    provider_user_id::ProviderUserId, user_id::UserId, user_name::UserName,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[allow(dead_code)]
fn to_domain(user_row: UserRow) -> Result<User, DomainError> {
    let user_id = UserId::new(user_row.id);
    let name = UserName::new(user_row.name).map_err(|e| DomainError::Unexpected(e.to_string()))?;

    let method = match user_row.kind.as_str() {
        "password_hash" => {
            let email = Email::new(user_row.email.ok_or_else(|| {
                DomainError::Unexpected("email is null for password_hash".into())
            })?)
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;
            let password_hash = PasswordHash::new(
                user_row
                    .password_hash
                    .ok_or_else(|| DomainError::Unexpected("password_hash is null".into()))?,
            );
            AuthMethod::Password {
                email,
                password_hash,
            }
        }
        "oauth" => {
            let provider_str = user_row
                .provider
                .ok_or_else(|| DomainError::Unexpected("provider is null".into()))?;
            let provider = OAuthProvider::from_str(&provider_str)
                .ok_or_else(|| DomainError::Unexpected("provider_user_id is null".into()))?;
            let provider_user_id = ProviderUserId::new(
                user_row
                    .provider_user_id
                    .ok_or_else(|| DomainError::Unexpected("provider_user_id is null".into()))?,
            );
            AuthMethod::OAuth {
                provider,
                provider_user_id,
            }
        }
        other => {
            return Err(DomainError::Unexpected(format!(
                "unknown auth kind: {}",
                other
            )));
        }
    };

    Ok(User {
        id: user_id,
        name,
        auth: UserAuth {
            user_id,
            method,
            created_at: user_row.auth_created_at,
            updated_at: user_row.auth_updated_at,
        },
        created_at: user_row.created_at,
        updated_at: user_row.updated_at,
    })
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError> {
        let user_row = sqlx::query!(
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

        let Some(user_row) = user_row else {
            return Ok(None);
        };

        Ok(Some(to_domain(UserRow {
            id: user_row.id,
            name: user_row.name,
            created_at: user_row.created_at,
            updated_at: user_row.updated_at,
            kind: user_row.kind,
            email: user_row.email,
            password_hash: user_row.password_hash,
            provider: user_row.provider,
            provider_user_id: user_row.provider_user_id,
            auth_created_at: user_row.auth_created_at,
            auth_updated_at: user_row.auth_updated_at,
        })?))
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
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
            user.name.value()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let auth_row: AuthRow = match &user.auth.method {
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

                AuthRow {
                    kind: row.kind,
                    email: row.email,
                    password_hash: row.password_hash,
                    provider: row.provider,
                    provider_user_id: row.provider_user_id,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            }

            AuthMethod::OAuth {
                provider,
                provider_user_id,
            } => {
                let row = sqlx::query!(
                    r#"
                INSERT INTO user_auth (user_id, kind, provider, provider_user_id)
                VALUES ($1, 'oauth', $2, $3)
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
                    provider.as_str(),
                    provider_user_id.value()
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| DomainError::Unexpected(e.to_string()))?;

                AuthRow {
                    kind: row.kind,
                    email: row.email,
                    password_hash: row.password_hash,
                    provider: row.provider,
                    provider_user_id: row.provider_user_id,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            }
        };

        tx.commit()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        to_domain(UserRow {
            id: user_row.id,
            name: user_row.name,
            created_at: user_row.created_at,
            updated_at: user_row.updated_at,
            kind: auth_row.kind,
            email: auth_row.email,
            password_hash: auth_row.password_hash,
            provider: auth_row.provider,
            provider_user_id: auth_row.provider_user_id,
            auth_created_at: auth_row.created_at,
            auth_updated_at: auth_row.updated_at,
        })
    }
}
