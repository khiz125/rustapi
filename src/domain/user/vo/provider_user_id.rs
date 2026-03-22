#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderUserId(String);

impl ProviderUserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
