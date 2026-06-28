#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefreshTokenId(i64);

impl RefreshTokenId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenHash(String);

impl TokenHash {
    pub fn from_hash(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
