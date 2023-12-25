use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use serde::Deserialize;
use serde::Serialize;

use crate::core::shared::model::DomainError;
use super::UserPlaintextPassword;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserEncryptedPassword(String);

fn get_argon2_context<'a>() -> Argon2<'a> {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::DEFAULT
    )
}

impl UserEncryptedPassword {
    pub fn encrypted_value(&self) -> &str {
        &self.0
    }

    pub fn from_unvalidated(value: String) -> Self {
        Self(value)
    }

    pub fn new(
        plaintext_password: UserPlaintextPassword
    ) -> Result<Self, DomainError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2_context = get_argon2_context();
        let password_hash = argon2_context
            .hash_password(
                plaintext_password.plaintext_value().as_bytes(),
                &salt
            )
            .unwrap()
            .to_string();

        Ok(Self(password_hash))
    }

    pub fn verify(&self, plaintext_password: UserPlaintextPassword) -> bool {
        let argon2_context = get_argon2_context();
        let parsed_hash = PasswordHash::new(&self.0).unwrap();
        argon2_context
            .verify_password(
                plaintext_password.plaintext_value().as_bytes(),
                &parsed_hash
            )
            .is_ok()
    }
}
