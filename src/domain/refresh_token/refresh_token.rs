use crate::domain::refresh_token::vo::{RefreshTokenId, TokenHash};
use crate::domain::user::vo::UserId;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: RefreshTokenId,
    pub user_id: UserId,
    pub token_hash: TokenHash,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl RefreshToken {
    pub fn is_valid(&self) -> bool {
        self.revoked_at.is_none() && self.expires_at > Utc::now()
    }
}

