//! This module contains types related to encryption and secrets management.
//! These types are used throughout the application to and to ensure that sensitive data is protected from unauthorized access.

use std::path::Path;

use async_trait::async_trait;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use rand::{thread_rng, Rng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::data_source::{DataSourceError, PasswordDataSource};

const SECRET_LEN: usize = 32;
const HASH_LEN: usize = 32;

/// The `EncryptedSecret` struct is used to represent a secret that has been encrypted with a user's password.
/// It consists of a salt, a password hash, and the encrypted secret itself.
///
/// The salt is a random value that is used to salt the password hash.
/// This helps to protect against [dictionary attacks](https://en.wikipedia.org/wiki/Dictionary_attack),
/// as the same password will result in a different hash when salted with a different value.
///
/// The password hash is a [Sha256](https://en.wikipedia.org/wiki/SHA-2) hash of the user's password, salted with the salt value.
/// This is used to verify the user's password when decrypting the secret.
///
/// The encrypted secret is the secret value itself,
/// encrypted with a key derived from the user's password and the salt using [ChaCha20-Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305).
#[derive(Serialize, Deserialize)]
pub struct EncryptedSecret {
    salt: [u8; 12],
    password_hash: [u8; HASH_LEN],
    encrypted_secret: Vec<u8>,
}

impl EncryptedSecret {
    /// Encrypts a secret with a password.
    ///
    /// This function takes a password and a secret value as input,
    /// and returns an `EncryptedSecret` containing the encrypted secret.
    /// The secret is encrypted using a key derived from the password and a randomly generated salt.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to use for encryption.
    /// * `secret` - The secret value to encrypt.
    pub fn encrypt(password: &str, secret: &Secret) -> Self {
        let salt: [u8; 12] = thread_rng().gen();
        let password_hash = Self::hash_password(password, &salt);

        let cipher = Self::create_cipher(password);
        let nonce = Nonce::from_slice(&salt);

        let encrypted_secret = cipher.encrypt(nonce, secret.bytes.as_ref()).unwrap();

        Self {
            salt,
            password_hash,
            encrypted_secret,
        }
    }

    fn create_cipher(password: &str) -> ChaCha20Poly1305 {
        let padded_password = format!("{password:<32}");
        let key = Key::from_slice(padded_password.as_bytes());
        ChaCha20Poly1305::new(key)
    }

    fn hash_password(password: &str, salt: &[u8]) -> [u8; HASH_LEN] {
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(password);
        hasher.finalize().try_into().unwrap()
    }

    /// Check if the password would decrypt the secret.
    pub fn valid_password(&self, password: &str) -> bool {
        let hash = Self::hash_password(password, &self.salt);
        hash == self.password_hash
    }

    /// Decrypt the secret.
    ///
    /// Returns None if the password is invalid.
    /// Returns the original secret if it is correct.
    pub fn decrypt(&self, password: &str) -> Option<Secret> {
        let cipher = Self::create_cipher(password);
        let nonce = Nonce::from_slice(&self.salt);

        if let Ok(raw) = cipher.decrypt(nonce, self.encrypted_secret.as_ref()) {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(raw.as_slice());
            Some(Secret::from_bytes(bytes))
        } else {
            None
        }
    }
}

/// The `Secret` struct is used to represent a secret value.
/// It consists of a fixed-length array of bytes.
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct Secret {
    bytes: [u8; SECRET_LEN],
}

impl Secret {
    pub fn random() -> Self {
        let mut bytes = [0_u8; SECRET_LEN];

        thread_rng().fill_bytes(&mut bytes);

        Self { bytes }
    }

    pub fn from_bytes(bytes: [u8; SECRET_LEN]) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8; SECRET_LEN] {
        &self.bytes
    }

    pub fn derive_for_token_secret(&self) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(self.bytes);
        hasher.update("token-secret".as_bytes());
        let derived_key = hasher.finalize();

        Self::from_bytes(derived_key.try_into().unwrap())
    }

    pub fn derive_from_uuid(&self, uuid: &Uuid) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(self.bytes);
        hasher.update(uuid);
        let derived_key = hasher.finalize();

        Self::from_bytes(derived_key.try_into().unwrap())
    }
}

/// The `EncryptionMetadata` struct is used to store encryption metadata for a bucket.
#[derive(Serialize, Deserialize)]
pub struct EncryptionMetadata {
    encrypted_secrets: Vec<EncryptedSecret>,
}

impl EncryptionMetadata {
    pub fn new(secret: EncryptedSecret) -> Self {
        Self {
            encrypted_secrets: vec![secret],
        }
    }

    /// Read the secret from a json file.
    pub async fn from_file(path: &Path) -> std::io::Result<Self> {
        let json = tokio::fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&json)?)
    }

    /// Save this encrypted secret to a json file.
    /// No sensitive data will be stored in this file.
    ///
    /// If the file does not exists, it will be created.
    /// If it doesn't the original will be truncated.
    /// An error will be returned if the file cannot be created or written to.
    pub async fn save_to(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string(self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    pub fn find_encrypted_secret_by_password(&self, password: &str) -> Option<&EncryptedSecret> {
        self.encrypted_secrets
            .iter()
            .find(|e| e.valid_password(password))
    }

    pub fn decrypt_secret(&self, password: &str) -> Option<Secret> {
        self.find_encrypted_secret_by_password(password)?
            .decrypt(password)
    }
}

#[async_trait]
impl PasswordDataSource for EncryptionMetadata {
    async fn validate_password(
        &self,
        password: Option<&str>,
    ) -> Result<Option<[u8; 32]>, DataSourceError> {
        let Some(password) = password else {
            return Ok(None);
        };

        let Some(secret) = self.find_encrypted_secret_by_password(password) else {
            return Ok(None);
        };

        Ok(secret
            .decrypt(password)
            .map(|secret| secret.derive_for_token_secret().bytes().clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let secret = Secret::random();
        let encrypted_secret = EncryptedSecret::encrypt("admin", &secret);

        assert!(encrypted_secret.valid_password("admin"));
    }

    #[test]
    fn test_decrypt() {
        let secret = Secret::random();
        let encrypted_secret = EncryptedSecret::encrypt("admin", &secret);

        assert_eq!(None, encrypted_secret.decrypt("welcome"));
        assert_eq!(
            secret.bytes,
            encrypted_secret.decrypt("admin").unwrap().bytes
        );
    }

    #[test]
    fn encrypt_and_decrypt() {
        let secret = Secret::random();
        let encrypted = EncryptedSecret::encrypt("admin", &secret);

        let decrypted = encrypted.decrypt("admin").unwrap();

        assert_eq!(secret.bytes, decrypted.bytes);
    }

    #[test]
    fn check_password_valid() {
        let secret = Secret::random();
        let encrypted = EncryptedSecret::encrypt("admin", &secret);

        assert!(encrypted.valid_password("admin"));
    }

    #[test]
    fn hash_must_be_different_each_time() {
        let secret = Secret::random();
        let encrypted1 = EncryptedSecret::encrypt("admin", &secret);
        let encrypted2 = EncryptedSecret::encrypt("admin", &secret);

        assert_ne!(encrypted1.password_hash, encrypted2.password_hash);
    }

    #[test]
    fn encrypted_message_must_be_different_each_time() {
        let secret = Secret::random();
        let encrypted1 = EncryptedSecret::encrypt("admin", &secret);
        let encrypted2 = EncryptedSecret::encrypt("admin", &secret);

        assert_ne!(encrypted1.encrypted_secret, encrypted2.encrypted_secret);
    }

    #[test]
    fn encrypt_should_change_the_secret() {
        let secret = Secret::random();
        let encrypted_secret = EncryptedSecret::encrypt("password123", &secret);

        // Check that the encrypted secret is not the same as the original secret
        assert_ne!(secret.bytes(), encrypted_secret.encrypted_secret.as_slice());
    }
}
