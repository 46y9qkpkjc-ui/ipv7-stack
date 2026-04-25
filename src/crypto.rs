//! Cryptographic operations for IPv7

use crate::error::{Ipv7Error, Result};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature};
use rand::Rng;
use sha2::{Sha256, Digest};
use std::str::FromStr;

/// Origin Signature using Ed25519
#[derive(Clone, Debug)]
pub struct OriginSignature {
    /// Raw signature bytes
    pub signature: Vec<u8>,
    /// Public key of the signer
    pub public_key: Vec<u8>,
    /// Timestamp of signature
    pub timestamp: u64,
}

impl OriginSignature {
    /// Create a new origin signature
    pub fn new(signature: Vec<u8>, public_key: Vec<u8>, timestamp: u64) -> Self {
        OriginSignature {
            signature,
            public_key,
            timestamp,
        }
    }

    /// Sign data with Ed25519 private key
    pub fn sign(data: &[u8], private_key: &[u8; 32]) -> Result<OriginSignature> {
        let signing_key = SigningKey::from_bytes(private_key);
        let signature = signing_key.sign(data);
        let public_key = signing_key.verifying_key();

        Ok(OriginSignature {
            signature: signature.to_bytes().to_vec(),
            public_key: public_key.as_bytes().to_vec(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    /// Verify signature with public key
    pub fn verify(&self, data: &[u8]) -> Result<bool> {
        let public_key_bytes: [u8; 32] = self
            .public_key
            .as_slice()
            .try_into()
            .map_err(|_| Ipv7Error::CryptoError("Invalid public key length".into()))?;

        let signature_bytes: [u8; 64] = self
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| Ipv7Error::CryptoError("Invalid signature length".into()))?;

        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|e| Ipv7Error::CryptoError(format!("Invalid public key: {}", e)))?;

        let signature = Signature::from_bytes(&signature_bytes);

        match verifying_key.verify(data, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get signature as hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.signature)
    }

    /// Get public key as hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(&self.public_key)
    }
}

/// Ephemeral Identity Token (EIT) for session-specific anonymity
#[derive(Clone, Debug)]
pub struct EphemeralIdentityToken {
    /// Token value
    pub token: Vec<u8>,
    /// Creation timestamp
    pub created_at: u64,
    /// Expiration timestamp
    pub expires_at: u64,
    /// Associated provider ID
    pub provider_id: String,
}

impl EphemeralIdentityToken {
    /// Create a new EIT with default 1-hour expiration
    pub fn new(provider_id: String) -> Self {
        let mut rng = rand::thread_rng();
        let mut token = vec![0u8; 32];
        rng.fill(&mut token[..]);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        EphemeralIdentityToken {
            token,
            created_at: now,
            expires_at: now + 3600, // 1 hour
            provider_id,
        }
    }

    /// Create EIT with custom expiration
    pub fn with_ttl(provider_id: String, ttl_seconds: u64) -> Self {
        let mut rng = rand::thread_rng();
        let mut token = vec![0u8; 32];
        rng.fill(&mut token[..]);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        EphemeralIdentityToken {
            token,
            created_at: now,
            expires_at: now + ttl_seconds,
            provider_id,
        }
    }

    /// Check if token is still valid
    pub fn is_valid(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now < self.expires_at
    }

    /// Get token as hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.token)
    }

    /// Get time until expiration in seconds
    pub fn ttl(&self) -> i64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        self.expires_at as i64 - now
    }
}

/// Hash data using SHA-256
pub fn hash_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Generate a new Ed25519 key pair
pub fn generate_keypair() -> ([u8; 32], [u8; 32]) {
    let mut rng = rand::thread_rng();
    let mut private_key = [0u8; 32];
    rng.fill(&mut private_key);

    let signing_key = SigningKey::from_bytes(&private_key);
    let verifying_key = signing_key.verifying_key();

    (private_key, verifying_key.to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let (private_key, _) = generate_keypair();
        let data = b"test data";

        let signature = OriginSignature::sign(data, &private_key).unwrap();
        assert!(signature.verify(data).unwrap());
    }

    #[test]
    fn test_eit_validity() {
        let eit = EphemeralIdentityToken::new("test_provider".into());
        assert!(eit.is_valid());
        assert!(eit.ttl() > 0);
    }

    #[test]
    fn test_hash_sha256() {
        let data = b"test";
        let hash = hash_sha256(data);
        assert_eq!(hash.len(), 32);
    }
}
