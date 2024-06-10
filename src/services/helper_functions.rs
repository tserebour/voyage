pub mod helpers_functions{
    use bcrypt::{hash, DEFAULT_COST,verify};

    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    pub fn verify_password(plain_password: &str, hashed_password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(plain_password, hashed_password)
    }
}