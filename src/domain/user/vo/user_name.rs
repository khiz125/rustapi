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
