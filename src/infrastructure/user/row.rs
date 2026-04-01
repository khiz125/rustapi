use crate::domain::types::UtcDateTime;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct UserRow {
    pub id: i64,
    pub name: String,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
    pub kind: String,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub provider: Option<String>,
    pub provider_user_id: Option<String>,
    pub auth_created_at: UtcDateTime,
    pub auth_updated_at: UtcDateTime,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct AuthRow {
    pub kind: String,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub provider: Option<String>,
    pub provider_user_id: Option<String>,
    pub created_at: UtcDateTime,
    pub updated_at: UtcDateTime,
}
