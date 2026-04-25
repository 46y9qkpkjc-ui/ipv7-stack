//! IPv7 Router - Packet processing and policy enforcement
//!
//! The router implements the three-stage processing model:
//! 1. Fast Path: Check trust/reputation octet for rate limiting
//! 2. Validation Path: Perform SPV on origin signature
//! 3. Routing Path: Apply forwarding policy based on identity

use crate::error::Result;
use crate::packet::Ipv7Packet;
use crate::reputation::{ReputationDatabase, ReputationScope, TrustLevel};
use crate::spv::{SourceProviderValidator, SpvPolicy};
use tracing::{debug, warn};

/// IPv7 Router with packet processing pipeline
pub struct Router {
    /// Router identifier
    pub id: String,
    /// SPV validator
    spv_validator: SourceProviderValidator,
    /// SPV policy
    spv_policy: SpvPolicy,
    /// Reputation database
    reputation_db: ReputationDatabase,
    /// Statistics
    pub stats: RouterStats,
}

/// Router statistics
#[derive(Clone, Debug, Default)]
pub struct RouterStats {
    /// Total packets processed
    pub packets_processed: u64,
    /// Packets accepted
    pub packets_accepted: u64,
    /// Packets dropped due to low trust
    pub packets_dropped_low_trust: u64,
    /// Packets failed SPV
    pub packets_failed_spv: u64,
    /// Packets failed policy
    pub packets_failed_policy: u64,
}

impl Router {
    /// Create a new router
    pub fn new(id: &str) -> Self {
        Router {
            id: id.to_string(),
            spv_validator: SourceProviderValidator::new(),
            spv_policy: SpvPolicy::new(),
            reputation_db: ReputationDatabase::new(),
            stats: RouterStats::default(),
        }
    }

    /// Register a provider with the router
    pub fn register_provider(&mut self, provider_id: &str, public_key: &[u8]) {
        self.spv_validator.register_provider(provider_id, public_key);
        debug!("Registered provider: {}", provider_id);
    }

    /// Set SPV policy
    pub fn set_spv_policy(&mut self, policy: SpvPolicy) {
        self.spv_policy = policy;
    }

    /// Process a packet through the three-stage pipeline
    pub fn process_packet(&mut self, packet: &Ipv7Packet) -> Result<bool> {
        self.stats.packets_processed += 1;

        // Stage 1: Fast Path - Check trust/reputation
        let trust_level = packet.header.get_trust_reputation();
        debug!("Stage 1 - Fast Path: trust_level={}", trust_level);

        if trust_level < self.spv_policy.min_trust_level {
            warn!(
                "Packet dropped: trust level {} below threshold {}",
                trust_level, self.spv_policy.min_trust_level
            );
            self.stats.packets_dropped_low_trust += 1;
            if self.spv_policy.drop_on_failure {
                return Ok(false);
            }
        }

        // Stage 2: Validation Path - SPV
        debug!("Stage 2 - Validation Path: Performing SPV");
        if self.spv_policy.require_spv {
            match self.spv_validator.validate(packet) {
                Ok(valid) => {
                    if !valid {
                        warn!("SPV validation failed");
                        self.stats.packets_failed_spv += 1;
                        if self.spv_policy.drop_on_failure {
                            return Ok(false);
                        }
                    }
                }
                Err(e) => {
                    warn!("SPV error: {}", e);
                    self.stats.packets_failed_spv += 1;
                    if self.spv_policy.drop_on_failure {
                        return Err(e);
                    }
                }
            }
        }

        // Stage 3: Routing Path - Policy enforcement
        debug!("Stage 3 - Routing Path: Applying policy");
        match self.spv_policy.passes_policy(packet) {
            Ok(true) => {
                debug!("Packet accepted");
                self.stats.packets_accepted += 1;

                // Record success in reputation database
                if let Some(vlib) = packet.get_vlib() {
                    let identity = format!("{}/{}", vlib.provider, vlib.tenant);
                    self.reputation_db.record_success(
                        identity,
                        ReputationScope::from_str(&vlib.reputation_scope),
                    );
                }

                Ok(true)
            }
            Ok(false) => {
                debug!("Packet rejected by policy");
                self.stats.packets_failed_policy += 1;
                Ok(false)
            }
            Err(e) => {
                warn!("Policy check error: {}", e);
                self.stats.packets_failed_policy += 1;
                Err(e)
            }
        }
    }

    /// Get router statistics
    pub fn get_stats(&self) -> &RouterStats {
        &self.stats
    }

    /// Get reputation database
    pub fn get_reputation_db(&self) -> &ReputationDatabase {
        &self.reputation_db
    }

    /// Get mutable reputation database
    pub fn get_reputation_db_mut(&mut self) -> &mut ReputationDatabase {
        &mut self.reputation_db
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = RouterStats::default();
    }

    /// Get acceptance rate
    pub fn acceptance_rate(&self) -> f32 {
        if self.stats.packets_processed == 0 {
            return 0.0;
        }
        self.stats.packets_accepted as f32 / self.stats.packets_processed as f32
    }

    /// Print statistics
    pub fn print_stats(&self) {
        println!("Router {} Statistics:", self.id);
        println!("  Total Packets: {}", self.stats.packets_processed);
        println!("  Accepted: {}", self.stats.packets_accepted);
        println!("  Dropped (Low Trust): {}", self.stats.packets_dropped_low_trust);
        println!("  Failed SPV: {}", self.stats.packets_failed_spv);
        println!("  Failed Policy: {}", self.stats.packets_failed_policy);
        println!(
            "  Acceptance Rate: {:.2}%",
            self.acceptance_rate() * 100.0
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vlib::Vlib;

    #[test]
    fn test_router_creation() {
        let router = Router::new("router1");
        assert_eq!(router.id, "router1");
        assert_eq!(router.stats.packets_processed, 0);
    }

    #[test]
    fn test_router_provider_registration() {
        let mut router = Router::new("router1");
        let pubkey = vec![1, 2, 3, 4];
        router.register_provider("isp1", &pubkey);

        // Router should now have isp1 registered
        assert!(!router.spv_validator.get_providers().is_empty());
    }

    #[test]
    fn test_packet_processing() {
        let mut router = Router::new("router1");
        let mut packet = Ipv7Packet::new();

        let vlib = Vlib::builder()
            .service("web")
            .location("nyc")
            .provider("isp1")
            .tenant("home")
            .role("user")
            .trust_level(150)
            .reputation_scope("local")
            .build()
            .unwrap();

        packet.set_vlib(vlib).unwrap();

        // Without SPV requirement, packet should pass
        router.spv_policy.require_spv = false;
        let result = router.process_packet(&packet).unwrap();
        assert!(result);
        assert_eq!(router.stats.packets_processed, 1);
    }
}
