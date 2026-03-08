use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError::InvalidUserName);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
