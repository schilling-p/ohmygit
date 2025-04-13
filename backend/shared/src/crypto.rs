use argon2::{
    password_hash::rand_core::OsRng,
    password_hash::Error as PasswordHashError,
    password_hash::PasswordHash,
    password_hash::PasswordHasher,
    password_hash::SaltString,
    Argon2, PasswordVerifier,
};

pub fn hash_password(password: &str) -> Result<String, PasswordHashError> {
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), PasswordHashError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;

    argon2.verify_password(password.as_bytes(), &parsed_hash)
}
