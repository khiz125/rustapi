use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let normalized_value = value.trim().to_lowercase();
        if !normalized_value.contains("@") {
            return Err(DomainError::InvalidEmail(normalized_value));
        }
        Ok(Self(normalized_value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
