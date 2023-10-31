pub mod password {

    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };

    use crate::errors::AppError;

    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let weak_password = password.as_bytes();
        let password_hash = argon2
            .hash_password(weak_password, &salt)
            .map_err(|err| AppError::HashError(Some(format!("{}", err))))?
            .to_string();

        Ok(password_hash)
    }

    #[allow(dead_code)]
    pub fn verify(password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|err| AppError::HashError(Some(format!("{}", err))))?;
            
        let pass_bytes = password.as_bytes();

        let matched = Argon2::default()
            .verify_password(pass_bytes, &parsed_hash)
            .is_ok();

        Ok(matched)
    }
}
