use crate::domain::error::DomainError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name: String = name.into();
        if name.is_empty() || name.len() > 30 {
            return Err(DomainError::InvalidUserName(Self(name)));
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let name = UserName::new("testname".to_string()).unwrap();
        assert_eq!(name.value(), "testname");
    }

    #[test]
    fn empty_is_invalid() {
        let result = UserName::new("".to_string());
        assert!(matches!(result, Err(DomainError::InvalidUserName(_))));
    }

    #[test]
    fn over_30_char_is_invalid() {
        let name = "a".repeat(31);
        let result = UserName::new(name.to_string());
        assert!(matches!(result, Err(DomainError::InvalidUserName(_))));
    }

    #[test]
    fn exactly_30_char_is_valid() {
        let name = "a".repeat(30);
        assert!(UserName::new(name.to_string()).is_ok());
    }
}
