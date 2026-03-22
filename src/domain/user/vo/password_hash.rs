#[derive(Debug, Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
