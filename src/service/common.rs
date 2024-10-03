use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::service::{ServiceError, ServiceResult};

pub fn hash(value: String) -> ServiceResult<String> {
    let argon = Argon2::default();
    let salt: SaltString = SaltString::generate(OsRng);

    match argon.hash_password(value.as_bytes(), &salt) {
        Ok(value) => Ok(value.to_string()),
        Err(_) => Err(ServiceError::Hash),
    }
}

pub fn verify_hash(value: String, hash: String) -> ServiceResult<bool> {
    let argon = Argon2::default();

    let hash = match PasswordHash::new(hash.as_str()) {
        Ok(value) => value,
        Err(_) => return Err(ServiceError::Hash),
    };

    Ok(argon.verify_password(value.as_bytes(), &hash).is_ok())
}
