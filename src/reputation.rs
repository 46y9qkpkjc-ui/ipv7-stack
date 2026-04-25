//! Trust and Reputation Scoring for IPv7
//!
//! Provides reputation-based packet filtering and trust-level management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Trust levels in IPv7
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustLevel {
    VeryLow = 0,
    Low = 64,
    Medium = 128,
    High = 192,
    VeryHigh = 255,
}

impl TrustLevel {
    /// Convert from u8 value
    pub fn from_u8(value: u8) -> Self {
        match value {
            0..=32 => TrustLevel::VeryLow,
            33..=96 => TrustLevel::Low,
            97..=160 => TrustLevel::Medium,
            161..=224 => TrustLevel::High,
            225..=255 => TrustLevel::VeryHigh,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            TrustLevel::VeryLow => "very_low",
            TrustLevel::Low => "low",
            TrustLevel::Medium => "medium",
            TrustLevel::High => "high",
            TrustLevel::VeryHigh => "very_high",
        }
    }

    /// Get as u8
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Reputation scope (geographic/organizational scope of reputation)
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReputationScope {
    Local,
    Regional,
    Global,
}

impl ReputationScope {
    /// Get from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "regional" => ReputationScope::Regional,
            "global" => ReputationScope::Global,
            _ => ReputationScope::Local,
        }
    }

    /// Get as string
    pub fn as_str(&self) -> &'static str {
        match self {
            ReputationScope::Local => "local",
            ReputationScope::Regional => "regional",
            ReputationScope::Global => "global",
        }
    }
}

impl std::fmt::Display for ReputationScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Reputation entry for an identity
#[derive(Clone, Debug)]
pub struct ReputationEntry {
    /// Identity identifier
    pub identity: String,
    /// Current trust level
    pub trust_level: TrustLevel,
    /// Scope of reputation
    pub scope: ReputationScope,
    /// Last updated timestamp
    pub last_updated: u64,
    /// Number of violations
    pub violation_count: u32,
    /// Number of successful transactions
    pub success_count: u32,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl ReputationEntry {
    /// Create a new reputation entry
    pub fn new(identity: String, scope: ReputationScope) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        ReputationEntry {
            identity,
            trust_level: TrustLevel::Medium,
            scope,
            last_updated: now,
            violation_count: 0,
            success_count: 0,
            metadata: HashMap::new(),
        }
    }

    /// Record a violation
    pub fn record_violation(&mut self) {
        self.violation_count += 1;
        self.update_trust_level();
        self.update_timestamp();
    }

    /// Record a success
    pub fn record_success(&mut self) {
        self.success_count += 1;
        self.update_trust_level();
        self.update_timestamp();
    }

    /// Update trust level based on violations and successes
    fn update_trust_level(&mut self) {
        if self.success_count == 0 {
            self.trust_level = TrustLevel::VeryLow;
            return;
        }

        let ratio = self.violation_count as f32 / (self.success_count as f32);

        self.trust_level = if ratio > 0.5 {
            TrustLevel::VeryLow
        } else if ratio > 0.3 {
            TrustLevel::Low
        } else if ratio > 0.1 {
            TrustLevel::Medium
        } else if ratio > 0.01 {
            TrustLevel::High
        } else {
            TrustLevel::VeryHigh
        };
    }

    /// Update last modified timestamp
    fn update_timestamp(&mut self) {
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Get success rate
    pub fn success_rate(&self) -> f32 {
        if self.success_count == 0 {
            return 0.0;
        }
        self.success_count as f32 / (self.success_count as f32 + self.violation_count as f32)
    }
}

/// Reputation Database
pub struct ReputationDatabase {
    entries: HashMap<String, ReputationEntry>,
}

impl ReputationDatabase {
    /// Create a new reputation database
    pub fn new() -> Self {
        ReputationDatabase {
            entries: HashMap::new(),
        }
    }

    /// Get or create a reputation entry
    pub fn get_or_create(&mut self, identity: String, scope: ReputationScope) -> &mut ReputationEntry {
        self.entries
            .entry(identity.clone())
            .or_insert_with(|| ReputationEntry::new(identity, scope))
    }

    /// Get a reputation entry
    pub fn get(&self, identity: &str) -> Option<&ReputationEntry> {
        self.entries.get(identity)
    }

    /// Get mutable reputation entry
    pub fn get_mut(&mut self, identity: &str) -> Option<&mut ReputationEntry> {
        self.entries.get_mut(identity)
    }

    /// Record violation for an identity
    pub fn record_violation(&mut self, identity: String, scope: ReputationScope) {
        self.get_or_create(identity, scope).record_violation();
    }

    /// Record success for an identity
    pub fn record_success(&mut self, identity: String, scope: ReputationScope) {
        self.get_or_create(identity, scope).record_success();
    }

    /// Get trust level for identity
    pub fn get_trust_level(&self, identity: &str) -> Option<TrustLevel> {
        self.entries.get(identity).map(|e| e.trust_level)
    }

    /// Get all entries
    pub fn entries(&self) -> &HashMap<String, ReputationEntry> {
        &self.entries
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get count of tracked identities
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for ReputationDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::VeryLow < TrustLevel::Low);
        assert!(TrustLevel::High > TrustLevel::Medium);
    }

    #[test]
    fn test_reputation_entry() {
        let mut entry = ReputationEntry::new("test_id".to_string(), ReputationScope::Local);
        entry.record_success();
        entry.record_success();
        entry.record_violation();

        assert!(entry.success_rate() > 0.5);
    }

    #[test]
    fn test_reputation_database() {
        let mut db = ReputationDatabase::new();
        db.record_success("id1".to_string(), ReputationScope::Global);
        db.record_success("id1".to_string(), ReputationScope::Global);
        db.record_violation("id1".to_string(), ReputationScope::Global);

        assert!(db.get("id1").is_some());
        assert!(db.get_trust_level("id1").is_some());
    }
}
