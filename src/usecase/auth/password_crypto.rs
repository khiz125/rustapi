use crate::domain::error::DomainError;
use crate::domain::user::vo::PasswordHash;
use argon2::password_hash::PasswordHash as Argon2Hash;
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub fn hash_password(raw: &str) -> Result<PasswordHash, DomainError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(raw.as_bytes(), &salt)
        .map_err(|e| DomainError::Unexpected(e.to_string()))?
        .to_string();

    Ok(PasswordHash::new(hash))
}

pub fn verify_password(raw: &str, hash: &str) -> Result<(), DomainError> {
    let parsed = Argon2Hash::new(hash).map_err(|e| DomainError::Unexpected(e.to_string()))?;
    Argon2::default()
        .verify_password(raw.as_bytes(), &parsed)
        .map_err(|_| DomainError::IncorrectPassword)
}
