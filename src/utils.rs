pub mod password {

    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };

    pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let weak_password = password.as_bytes();
        let password_hash = argon2.hash_password(weak_password, &salt)?.to_string();

        Ok(password_hash)
    }
    
    pub fn verify(password: &str, hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let parsed_hash = PasswordHash::new(hash)?;
        let pass_bytes = password.as_bytes();

        let matched = Argon2::default()
            .verify_password(pass_bytes, &parsed_hash)
            .is_ok();

        Ok(matched)
    }
}
