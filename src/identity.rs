//! IPv7 Identity representation and parsing

use crate::error::{Ipv7Error, Result};
use serde::{Deserialize, Serialize};

/// IPv7 Identity format: \[EIT\]/service.location.provider.tenant.role.trustlevel.reputationscope
///
/// Example: eit_7f3a9c2b/web.nyc.exampleisp.home.guest.medium.local
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Identity {
    /// Ephemeral Identity Token (time-bound, session-specific)
    pub eit: String,
    /// Service identifier (e.g., "web", "mail", "ssh")
    pub service: String,
    /// Geographic location or network zone (e.g., "nyc", "london")
    pub location: String,
    /// Provider/ISP identifier
    pub provider: String,
    /// Tenant identifier (e.g., "home", "office", "datacenter")
    pub tenant: String,
    /// Role or access level (e.g., "guest", "user", "admin")
    pub role: String,
    /// Trust level (e.g., "low", "medium", "high")
    pub trust_level: String,
    /// Reputation scope (e.g., "local", "global", "regional")
    pub reputation_scope: String,
}

impl Identity {
    /// Create a new IPv7 identity
    pub fn new(
        eit: String,
        service: String,
        location: String,
        provider: String,
        tenant: String,
        role: String,
        trust_level: String,
        reputation_scope: String,
    ) -> Self {
        Identity {
            eit,
            service,
            location,
            provider,
            tenant,
            role,
            trust_level,
            reputation_scope,
        }
    }

    /// Parse identity from string format: \[EIT\]/service.location.provider.tenant.role.trustlevel.reputationscope
    pub fn from_str(identity_str: &str) -> Result<Self> {
        // Split EIT from rest
        let parts: Vec<&str> = identity_str.split('/').collect();
        if parts.len() != 2 {
            return Err(Ipv7Error::InvalidIdentity(
                "Identity must contain EIT/components format".to_string(),
            ));
        }

        let eit = parts[0].to_string();
        let components: Vec<&str> = parts[1].split('.').collect();

        if components.len() != 7 {
            return Err(Ipv7Error::InvalidIdentity(
                format!("Expected 7 components, got {}", components.len()),
            ));
        }

        Ok(Identity {
            eit,
            service: components[0].to_string(),
            location: components[1].to_string(),
            provider: components[2].to_string(),
            tenant: components[3].to_string(),
            role: components[4].to_string(),
            trust_level: components[5].to_string(),
            reputation_scope: components[6].to_string(),
        })
    }

    /// Convert identity to string representation
    pub fn to_string(&self) -> String {
        format!(
            "{}/{}.{}.{}.{}.{}.{}.{}",
            self.eit,
            self.service,
            self.location,
            self.provider,
            self.tenant,
            self.role,
            self.trust_level,
            self.reputation_scope
        )
    }

    /// Create an identity builder
    pub fn builder() -> IdentityBuilder {
        IdentityBuilder::default()
    }
}

/// Builder for creating IPv7 identities
#[derive(Default)]
pub struct IdentityBuilder {
    eit: Option<String>,
    service: Option<String>,
    location: Option<String>,
    provider: Option<String>,
    tenant: Option<String>,
    role: Option<String>,
    trust_level: Option<String>,
    reputation_scope: Option<String>,
}

impl IdentityBuilder {
    pub fn eit(mut self, eit: impl Into<String>) -> Self {
        self.eit = Some(eit.into());
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

    pub fn trust_level(mut self, trust_level: impl Into<String>) -> Self {
        self.trust_level = Some(trust_level.into());
        self
    }

    pub fn reputation_scope(mut self, reputation_scope: impl Into<String>) -> Self {
        self.reputation_scope = Some(reputation_scope.into());
        self
    }

    pub fn build(self) -> Result<Identity> {
        Ok(Identity {
            eit: self.eit.ok_or_else(|| Ipv7Error::InvalidIdentity("Missing EIT".into()))?,
            service: self
                .service
                .ok_or_else(|| Ipv7Error::InvalidIdentity("Missing service".into()))?,
            location: self
                .location
                .ok_or_else(|| Ipv7Error::InvalidIdentity("Missing location".into()))?,
            provider: self
                .provider
                .ok_or_else(|| Ipv7Error::InvalidIdentity("Missing provider".into()))?,
            tenant: self
                .tenant
                .ok_or_else(|| Ipv7Error::InvalidIdentity("Missing tenant".into()))?,
            role: self
                .role
                .ok_or_else(|| Ipv7Error::InvalidIdentity("Missing role".into()))?,
            trust_level: self
                .trust_level
                .ok_or_else(|| Ipv7Error::InvalidIdentity("Missing trust_level".into()))?,
            reputation_scope: self.reputation_scope.ok_or_else(|| {
                Ipv7Error::InvalidIdentity("Missing reputation_scope".into())
            })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_parsing() {
        let id_str = "eit_7f3a9c2b/web.nyc.exampleisp.home.guest.medium.local";
        let identity = Identity::from_str(id_str).unwrap();
        assert_eq!(identity.eit, "eit_7f3a9c2b");
        assert_eq!(identity.service, "web");
        assert_eq!(identity.provider, "exampleisp");
    }

    #[test]
    fn test_identity_builder() {
        let identity = Identity::builder()
            .eit("eit_test")
            .service("web")
            .location("nyc")
            .provider("isp1")
            .tenant("home")
            .role("user")
            .trust_level("medium")
            .reputation_scope("local")
            .build()
            .unwrap();

        assert_eq!(identity.eit, "eit_test");
        assert_eq!(identity.provider, "isp1");
    }
}
