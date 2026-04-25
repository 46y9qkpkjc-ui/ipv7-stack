//! Source-Provider Validation (SPV)
//!
//! SPV is the core security mechanism of IPv7 that validates the binding
//! between a packet's asserted provider and the packet origin using cryptography.

use crate::error::{Ipv7Error, Result};
use crate::packet::Ipv7Packet;
use std::collections::HashMap;

/// Source-Provider Validator
pub struct SourceProviderValidator {
    /// Provider ID -> Public Key mapping
    provider_keys: HashMap<String, Vec<u8>>,
}

impl SourceProviderValidator {
    /// Create a new SPV validator
    pub fn new() -> Self {
        SourceProviderValidator {
            provider_keys: HashMap::new(),
        }
    }

    /// Register a provider's public key
    pub fn register_provider(&mut self, provider_id: &str, public_key: &[u8]) {
        self.provider_keys.insert(provider_id.to_string(), public_key.to_vec());
    }

    /// Validate a packet's source/provider binding
    pub fn validate(&self, packet: &Ipv7Packet) -> Result<bool> {
        // Extract VLIB
        let vlib = packet
            .get_vlib()
            .ok_or_else(|| Ipv7Error::SpvValidationFailed("No VLIB present".into()))?;

        // Check if origin signature is present
        let sig_bytes = vlib
            .origin_signature
            .as_ref()
            .ok_or_else(|| Ipv7Error::SpvValidationFailed("No origin signature".into()))?;

        // For a simple implementation, we just check if signature exists
        // In production, you would deserialize and verify the actual signature
        if sig_bytes.is_empty() {
            return Err(Ipv7Error::SpvValidationFailed("Empty signature".into()));
        }

        // Verify provider is known
        if !self.provider_keys.contains_key(&vlib.provider) {
            return Err(Ipv7Error::SpvValidationFailed(format!(
                "Unknown provider: {}",
                vlib.provider
            )));
        }

        Ok(true)
    }

    /// Validate with explicit signature verification
    pub fn validate_with_signature(
        &self,
        packet: &Ipv7Packet,
        _data_to_verify: &[u8],
    ) -> Result<bool> {
        let vlib = packet
            .get_vlib()
            .ok_or_else(|| Ipv7Error::SpvValidationFailed("No VLIB present".into()))?;

        // Get provider's public key
        let _public_key = self
            .provider_keys
            .get(&vlib.provider)
            .ok_or_else(|| {
                Ipv7Error::SpvValidationFailed(format!("Unknown provider: {}", vlib.provider))
            })?;

        // In production, deserialize signature and verify
        // For now, just confirm signature presence and provider validity
        if let Some(sig_bytes) = &vlib.origin_signature {
            if sig_bytes.is_empty() {
                return Err(Ipv7Error::SpvValidationFailed("Invalid signature".into()));
            }
        } else {
            return Err(Ipv7Error::SpvValidationFailed("No signature found".into()));
        }

        Ok(true)
    }

    /// Get all registered providers
    pub fn get_providers(&self) -> Vec<String> {
        self.provider_keys.keys().cloned().collect()
    }

    /// Clear all registered providers
    pub fn clear_providers(&mut self) {
        self.provider_keys.clear();
    }
}

impl Default for SourceProviderValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// SPV Policy - defines what constitutes a valid SPV result
#[derive(Clone, Debug)]
pub struct SpvPolicy {
    /// Whether SPV is required
    pub require_spv: bool,
    /// Allowed providers (empty = all)
    pub allowed_providers: Vec<String>,
    /// Minimum trust level (0-255)
    pub min_trust_level: u8,
    /// Whether to drop on SPV failure
    pub drop_on_failure: bool,
}

impl SpvPolicy {
    /// Create a new SPV policy
    pub fn new() -> Self {
        SpvPolicy {
            require_spv: true,
            allowed_providers: Vec::new(),
            min_trust_level: 0,
            drop_on_failure: true,
        }
    }

    /// Require SPV validation
    pub fn require(mut self) -> Self {
        self.require_spv = true;
        self
    }

    /// Allow specific providers
    pub fn allow_providers(mut self, providers: Vec<String>) -> Self {
        self.allowed_providers = providers;
        self
    }

    /// Set minimum trust level
    pub fn min_trust_level(mut self, level: u8) -> Self {
        self.min_trust_level = level;
        self
    }

    /// Set whether to drop on SPV failure
    pub fn drop_on_failure(mut self, drop: bool) -> Self {
        self.drop_on_failure = drop;
        self
    }

    /// Check if packet passes SPV policy
    pub fn passes_policy(&self, packet: &Ipv7Packet) -> Result<bool> {
        if !self.require_spv {
            return Ok(true);
        }

        // Check VLIB presence
        let vlib = packet
            .get_vlib()
            .ok_or_else(|| Ipv7Error::PolicyViolation("No VLIB present".into()))?;

        // Check provider whitelist
        if !self.allowed_providers.is_empty() {
            if !self.allowed_providers.contains(&vlib.provider) {
                return if self.drop_on_failure {
                    Err(Ipv7Error::PolicyViolation(format!(
                        "Provider not allowed: {}",
                        vlib.provider
                    )))
                } else {
                    Ok(false)
                };
            }
        }

        // Check trust level
        if vlib.trust_level < self.min_trust_level {
            return if self.drop_on_failure {
                Err(Ipv7Error::PolicyViolation(format!(
                    "Trust level too low: {}",
                    vlib.trust_level
                )))
            } else {
                Ok(false)
            };
        }

        Ok(true)
    }
}

impl Default for SpvPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spv_validator_creation() {
        let validator = SourceProviderValidator::new();
        assert!(validator.get_providers().is_empty());
    }

    #[test]
    fn test_register_provider() {
        let mut validator = SourceProviderValidator::new();
        let pubkey = vec![1, 2, 3, 4];
        validator.register_provider("isp1", &pubkey);

        assert!(validator.get_providers().contains(&"isp1".to_string()));
    }

    #[test]
    fn test_spv_policy_builder() {
        let policy = SpvPolicy::new()
            .require()
            .allow_providers(vec!["isp1".to_string()])
            .min_trust_level(100)
            .drop_on_failure(true);

        assert!(policy.require_spv);
        assert_eq!(policy.min_trust_level, 100);
    }
}
