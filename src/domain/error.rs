#[derive(Debug)]
pub enum DomainError {
    InvalidEmail,
    InvalidUserName,
    NotFound,
    Other(String),
}
