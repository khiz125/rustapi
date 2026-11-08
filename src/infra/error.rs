use crate::domain::repo_error::RepositoryError;

impl From<sqlx::Error> for RepositoryError {
    fn from(e: sqlx::Error) -> Self {
        RepositoryError::DatabaseError(e.to_string())
    }
}
