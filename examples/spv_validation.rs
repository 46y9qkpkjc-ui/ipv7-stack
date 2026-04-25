//! IPv7 Source-Provider Validation (SPV) Example
//!
//! Demonstrates registering providers and validating source/provider binding

use ipv7_stack::packet::Ipv7Packet;
use ipv7_stack::spv::{SourceProviderValidator, SpvPolicy};
use ipv7_stack::vlib::Vlib;
use ipv7_stack::crypto;

fn main() {
    println!("=== IPv7 Source-Provider Validation Example ===\n");

    // Create a validator
    let mut validator = SourceProviderValidator::new();
    println!("✓ Created SPV validator");

    // Generate keys for a provider
    let (private_key, public_key) = crypto::generate_keypair();
    println!("✓ Generated Ed25519 keypair");

    // Register provider
    validator.register_provider("exampleisp", &public_key);
    println!("✓ Registered provider: exampleisp");
    println!("  Public Key: {}", hex::encode(&public_key));

    // Create SPV policy
    let policy = SpvPolicy::new()
        .require()
        .allow_providers(vec!["exampleisp".to_string()])
        .min_trust_level(100)
        .drop_on_failure(true);

    println!("\n=== SPV Policy ===");
    println!("  Require SPV: {}", policy.require_spv);
    println!("  Allowed Providers: {:?}", policy.allowed_providers);
    println!("  Min Trust Level: {}", policy.min_trust_level);
    println!("  Drop on Failure: {}", policy.drop_on_failure);

    // Create a packet
    let mut packet = Ipv7Packet::new();
    packet.header.set_trust_reputation(150);
    println!("\n✓ Created packet with trust level 150");

    // Create VLIB with provider signature
    let mut vlib = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("exampleisp")
        .tenant("home")
        .role("user")
        .trust_level(150)
        .reputation_scope("global")
        .build()
        .unwrap();

    // Add signature (simulated)
    let signature_bytes = crypto::hash_sha256(b"packet_data");
    vlib = vlib.with_signature(signature_bytes);
    println!("✓ Added origin signature to VLIB");

    packet.set_vlib(vlib).unwrap();
    println!("✓ Set VLIB in packet");

    // Validate
    println!("\n=== Validation Results ===");
    match validator.validate(&packet) {
        Ok(valid) => println!("✓ SPV Validation: {}", if valid { "PASS" } else { "FAIL" }),
        Err(e) => println!("✗ SPV Validation Error: {}", e),
    }

    // Check policy
    match policy.passes_policy(&packet) {
        Ok(passes) => {
            println!("✓ Policy Check: {}", if passes { "PASS" } else { "FAIL" })
        }
        Err(e) => println!("✗ Policy Check Error: {}", e),
    }

    // Test with low trust level
    println!("\n=== Testing Low Trust Level ===");
    let mut low_trust_packet = Ipv7Packet::new();
    low_trust_packet.header.set_trust_reputation(50); // Below threshold
    let low_vlib = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("exampleisp")
        .tenant("home")
        .role("user")
        .trust_level(50)
        .reputation_scope("global")
        .build()
        .unwrap();
    low_trust_packet.set_vlib(low_vlib).unwrap();

    match policy.passes_policy(&low_trust_packet) {
        Ok(passes) => println!("Policy Check: {}", if passes { "PASS" } else { "FAIL" }),
        Err(e) => println!("✓ Expected Policy Violation: {}", e),
    }

    // Test with unknown provider
    println!("\n=== Testing Unknown Provider ===");
    let mut unknown_provider_packet = Ipv7Packet::new();
    unknown_provider_packet.header.set_trust_reputation(150);
    let unknown_vlib = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("unknownisp")
        .tenant("home")
        .role("user")
        .trust_level(150)
        .reputation_scope("global")
        .build()
        .unwrap();
    unknown_provider_packet.set_vlib(unknown_vlib).unwrap();

    match policy.passes_policy(&unknown_provider_packet) {
        Ok(passes) => println!("Policy Check: {}", if passes { "PASS" } else { "FAIL" }),
        Err(e) => println!("✓ Expected Provider Check to Fail: {}", e),
    }

    println!("\n✓ SPV validation examples completed!");
}
