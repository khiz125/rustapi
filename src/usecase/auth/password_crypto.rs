use crate::domain::error::DomainError;
use crate::domain::user::vo::PasswordHash;
use crate::usecase::error::UsecaseError;
use argon2::password_hash::{Error as Argon2Error, PasswordHash as Argon2Hash};
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub fn hash_password(raw: &str) -> Result<PasswordHash, UsecaseError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(raw.as_bytes(), &salt)
        .map_err(|e| UsecaseError::Crypto(e.to_string()))?
        .to_string();

    Ok(PasswordHash::new(hash))
}

pub fn verify_password(raw: &str, hash: &str) -> Result<(), UsecaseError> {
    let parsed = Argon2Hash::new(hash).map_err(|e| UsecaseError::Crypto(e.to_string()))?;
    match Argon2::default().verify_password(raw.as_bytes(), &parsed) {
        Ok(()) => Ok(()),
        Err(Argon2Error::Password) => Err(UsecaseError::Domain(DomainError::IncorrectPassword)),
        Err(e) => Err(UsecaseError::Crypto(e.to_string())),
    }
}
