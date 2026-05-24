use crate::domain::error::DomainError;
use std::fmt;

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

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        assert_eq!(email.value(), "test@example.com");
    }

    #[test]
    fn missing_at_is_invalid() {
        let result = Email::new("invalid-email".to_string());
        assert!(matches!(result, Err(DomainError::InvalidEmail(_))));
    }

    #[test]
    fn empty_is_invalid() {
        let result = Email::new("".to_string());
        assert!(matches!(result, Err(DomainError::InvalidEmail(_))));
    }
}
