#[derive(Debug)]
pub enum DomainError {
    InvalidPassword,
    InvalidUserName,
    InvalidEmail,
    UserNotFound,
    UnExpected(String),
}
