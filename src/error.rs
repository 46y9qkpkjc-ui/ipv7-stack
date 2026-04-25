//! Error types for IPv7 stack

use thiserror::Error;

/// IPv7 stack error types
#[derive(Error, Debug)]
pub enum Ipv7Error {
    #[error("Invalid packet format: {0}")]
    InvalidPacket(String),

    #[error("Invalid VLIB: {0}")]
    InvalidVlib(String),

    #[error("Signature verification failed: {0}")]
    SignatureVerificationFailed(String),

    #[error("Source-Provider Validation failed: {0}")]
    SpvValidationFailed(String),

    #[error("Invalid identity: {0}")]
    InvalidIdentity(String),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Routing error: {0}")]
    RoutingError(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Insufficient buffer: {0}")]
    InsufficientBuffer(String),

    #[error("Invalid trust level: {0}")]
    InvalidTrustLevel(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for IPv7 operations
pub type Result<T> = std::result::Result<T, Ipv7Error>;
