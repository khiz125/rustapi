use crate::domain::error::DomainError;
use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::user_auth::UserAuth;
use crate::domain::user::vo::{AuthMethod, OAuthProvider, PasswordHash, UserId, UserName};

use sqlx::{PgPool, Row};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError> {
        let user_row =
            sqlx::query(r#"SELECT id, name, created_at, updated_at FROM users WHERE id = $1"#)
                .bind(id.value() as i64)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| DomainError::UnExpected(e.to_string()))?;

        let user_row = match user_row {
            Some(r) => r,
            None => return Ok(None),
        };

        let user_id = UserId::new(user_row.get::<i64, _>("id") as i32);
        let user_name = UserName::new(user_row.get::<String, _>("name"))
            .map_err(|_| DomainError::InvalidUserName)?;

        // user_auth テーブル取得（存在しない場合もあるので fetch_optional）
        let auth_row = sqlx::query(
            r#"SELECT kind, email, password_hash, provider, provider_user_id FROM user_auth WHERE user_id = $1"#,
        )
        .bind(user_id.value() as i64)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::UnExpected(e.to_string()))?;

        let auth = if let Some(a) = auth_row {
            match a.get::<String, _>("kind").as_str() {
                "password_hash" => AuthMethod::Password {
                    email: crate::domain::user::vo::Email::new(
                        a.get::<Option<String>, _>("email")
                            .ok_or_else(|| DomainError::UnExpected("email is null".to_string()))?,
                    )
                    .map_err(|_| DomainError::UnExpected("invalid email".to_string()))?,
                    password_hash: PasswordHash::new(
                        a.get::<Option<String>, _>("password_hash").ok_or_else(|| {
                            DomainError::UnExpected("password_hash is null".to_string())
                        })?,
                    ),
                },
                "oauth" => AuthMethod::OAuth {
                    provider: match a
                        .get::<Option<String>, _>("provider")
                        .ok_or_else(|| DomainError::UnExpected("provider is null".to_string()))?
                        .as_str()
                    {
                        "Google" => OAuthProvider::Google,
                        "Apple" => OAuthProvider::Apple,
                        other => {
                            return Err(DomainError::UnExpected(format!(
                                "invalid oauth provider: {}",
                                other
                            )));
                        }
                    },
                    provider_user_id: crate::domain::user::vo::ProviderUserId::new(
                        a.get::<Option<String>, _>("provider_user_id")
                            .ok_or_else(|| {
                                DomainError::UnExpected("provider_user_id is null".to_string())
                            })?,
                    ),
                },
                other => {
                    return Err(DomainError::UnExpected(format!(
                        "invalid auth kind: {}",
                        other
                    )));
                }
            }
        } else {
            return Ok(Some(User::new(user_id, user_name, None)));
        };

        // UserAuth 作成
        let user_auth = UserAuth::new(user_id, auth);

        // User 作成
        let user = User::new(user_id, user_name, Some(user_auth));

        Ok(Some(user))
    }
    async fn create(&self, user: User) -> Result<User, DomainError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DomainError::UnExpected(e.to_string()))?;

        let row = sqlx::query(
            r#"
            insert into users (name)
            value ($1)
                returinig id, name, created_at, updated_at
            "#,
        )
        .bind(user.name().value())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| DomainError::InvalidUserName)?;

        let user_id = UserId::new(row.get::<i64, _>("id") as i32);
        let user_name = UserName::new(row.get::<String, _>("name"))
            .map_err(|_| DomainError::InvalidUserName)?;

        if let Some(auth) = user.auth() {
            match auth.auth() {
                AuthMethod::Password {
                    email,
                    password_hash,
                } => {
                    sqlx::query(
                        r#"
                    insert into user_auth
                    (user_id, kind, email, password_hash)
                    values ($1, $2, $3, $4)
                    "#,
                    )
                    .bind(user_id.value() as i64)
                    .bind("password_hash")
                    .bind(email.value())
                    .bind(password_hash.value())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| DomainError::UnExpected(e.to_string()))?;
                }

                AuthMethod::OAuth {
                    provider,
                    provider_user_id,
                } => {
                    let provider_str = match provider {
                        OAuthProvider::Google => "Google",
                        OAuthProvider::Apple => "Apple",
                    };

                    sqlx::query(
                        r#"
                    insert into user_auth
                    (user_id, kind, provider, provider_user_id)
                    values ($1, $2, $3, $4)
                    "#,
                    )
                    .bind(user_id.value() as i64)
                    .bind("oauth")
                    .bind(provider_str)
                    .bind(provider_user_id.value())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| DomainError::UnExpected(e.to_string()))?;
                }
            }
        }

        tx.commit()
            .await
            .map_err(|e| DomainError::UnExpected(e.to_string()))?;

        let created_user = User::new(user_id, user_name, user.auth().cloned());

        Ok(created_user)
    }
}
