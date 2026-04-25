//! Variable-Length Identity Block (VLIB) for IPv7
//!
//! The VLIB contains hierarchical identity information including:
//! - Ephemeral Identity Token (EIT)
//! - Service, location, provider, tenant identifiers
//! - Role and policy information
//! - Origin Signature for cryptographic verification

use crate::error::{Ipv7Error, Result};
use crate::identity::Identity;
use serde::{Deserialize, Serialize};

/// Variable-Length Identity Block
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vlib {
    /// Ephemeral Identity Token
    pub eit: Option<Vec<u8>>,
    /// Service identifier
    pub service: String,
    /// Geographic location or zone
    pub location: String,
    /// Provider/ISP identifier
    pub provider: String,
    /// Tenant identifier
    pub tenant: String,
    /// Role or access level
    pub role: String,
    /// Trust level (0-255, higher = more trusted)
    pub trust_level: u8,
    /// Reputation scope (local, regional, global)
    pub reputation_scope: String,
    /// Rate limit token (optional)
    pub rate_limit_token: Option<Vec<u8>>,
    /// Origin Signature (serialized)
    pub origin_signature: Option<Vec<u8>>,
}

impl Vlib {
    /// Create a new VLIB
    pub fn new(
        service: String,
        location: String,
        provider: String,
        tenant: String,
        role: String,
        trust_level: u8,
        reputation_scope: String,
    ) -> Self {
        Vlib {
            eit: None,
            service,
            location,
            provider,
            tenant,
            role,
            trust_level,
            reputation_scope,
            rate_limit_token: None,
            origin_signature: None,
        }
    }

    /// Set Ephemeral Identity Token
    pub fn with_eit(mut self, eit: Vec<u8>) -> Self {
        self.eit = Some(eit);
        self
    }

    /// Set Origin Signature
    pub fn with_signature(mut self, signature: Vec<u8>) -> Self {
        self.origin_signature = Some(signature);
        self
    }

    /// Set Rate Limit Token
    pub fn with_rate_limit_token(mut self, token: Vec<u8>) -> Self {
        self.rate_limit_token = Some(token);
        self
    }

    /// Convert to bytes for transmission
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let json = serde_json::to_vec(self)
            .map_err(|e| Ipv7Error::SerializationError(e.to_string()))?;
        Ok(json)
    }

    /// Parse from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let vlib = serde_json::from_slice(data)
            .map_err(|e| Ipv7Error::ParseError(e.to_string()))?;
        Ok(vlib)
    }

    /// Create a VLIB builder
    pub fn builder() -> VlibBuilder {
        VlibBuilder::default()
    }

    /// Convert VLIB to Identity
    pub fn to_identity(&self) -> Result<Identity> {
        let eit_str = if let Some(eit) = &self.eit {
            format!("eit_{}", hex::encode(eit))
        } else {
            "eit_default".to_string()
        };

        Ok(Identity::new(
            eit_str,
            self.service.clone(),
            self.location.clone(),
            self.provider.clone(),
            self.tenant.clone(),
            self.role.clone(),
            format!("{:x}", self.trust_level),
            self.reputation_scope.clone(),
        ))
    }

    /// Get VLIB size in bytes
    pub fn size(&self) -> Result<usize> {
        Ok(self.to_bytes()?.len())
    }
}

/// Builder for VLIB
#[derive(Default)]
pub struct VlibBuilder {
    eit: Option<Vec<u8>>,
    service: Option<String>,
    location: Option<String>,
    provider: Option<String>,
    tenant: Option<String>,
    role: Option<String>,
    trust_level: Option<u8>,
    reputation_scope: Option<String>,
    rate_limit_token: Option<Vec<u8>>,
    origin_signature: Option<Vec<u8>>,
}

impl VlibBuilder {
    pub fn eit(mut self, eit: Vec<u8>) -> Self {
        self.eit = Some(eit);
        self
    }

    pub fn service(mut self, service: impl Into<String>) -> Self {
        self.service = Some(service.into());
        self
    }

    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
        self.tenant = Some(tenant.into());
        self
    }

    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn trust_level(mut self, trust_level: u8) -> Self {
        self.trust_level = Some(trust_level);
        self
    }

    pub fn reputation_scope(mut self, reputation_scope: impl Into<String>) -> Self {
        self.reputation_scope = Some(reputation_scope.into());
        self
    }

    pub fn rate_limit_token(mut self, token: Vec<u8>) -> Self {
        self.rate_limit_token = Some(token);
        self
    }

    pub fn origin_signature(mut self, signature: Vec<u8>) -> Self {
        self.origin_signature = Some(signature);
        self
    }

    pub fn build(self) -> Result<Vlib> {
        Ok(Vlib {
            eit: self.eit,
            service: self
                .service
                .ok_or_else(|| Ipv7Error::InvalidVlib("Missing service".into()))?,
            location: self
                .location
                .ok_or_else(|| Ipv7Error::InvalidVlib("Missing location".into()))?,
            provider: self
                .provider
                .ok_or_else(|| Ipv7Error::InvalidVlib("Missing provider".into()))?,
            tenant: self
                .tenant
                .ok_or_else(|| Ipv7Error::InvalidVlib("Missing tenant".into()))?,
            role: self
                .role
                .ok_or_else(|| Ipv7Error::InvalidVlib("Missing role".into()))?,
            trust_level: self.trust_level.unwrap_or(128),
            reputation_scope: self
                .reputation_scope
                .ok_or_else(|| Ipv7Error::InvalidVlib("Missing reputation_scope".into()))?,
            rate_limit_token: self.rate_limit_token,
            origin_signature: self.origin_signature,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlib_creation() {
        let vlib = Vlib::new(
            "web".to_string(),
            "nyc".to_string(),
            "exampleisp".to_string(),
            "home".to_string(),
            "guest".to_string(),
            128,
            "local".to_string(),
        );
        assert_eq!(vlib.service, "web");
        assert_eq!(vlib.provider, "exampleisp");
    }

    #[test]
    fn test_vlib_builder() {
        let vlib = Vlib::builder()
            .service("web")
            .location("nyc")
            .provider("isp1")
            .tenant("home")
            .role("user")
            .trust_level(200)
            .reputation_scope("global")
            .build()
            .unwrap();

        assert_eq!(vlib.trust_level, 200);
    }

    #[test]
    fn test_vlib_serialization() {
        let vlib = Vlib::builder()
            .service("web")
            .location("nyc")
            .provider("isp1")
            .tenant("home")
            .role("user")
            .trust_level(100)
            .reputation_scope("local")
            .build()
            .unwrap();

        let bytes = vlib.to_bytes().unwrap();
        let decoded = Vlib::from_bytes(&bytes).unwrap();
        assert_eq!(vlib.provider, decoded.provider);
    }
}
