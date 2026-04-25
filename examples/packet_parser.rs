//! IPv7 Packet Parser Example
//!
//! Demonstrates parsing and serializing IPv7 packets with VLIB

use ipv7_stack::packet::Ipv7Packet;
use ipv7_stack::vlib::Vlib;

fn main() {
    println!("=== IPv7 Packet Parser Example ===\n");

    // Create an IPv7 packet
    let mut packet = Ipv7Packet::new();
    println!("✓ Created new IPv7 packet");

    // Set source and destination addresses
    let source = [192, 168, 1, 1, 0, 0, 0, 1];
    let dest = [192, 168, 1, 100, 0, 0, 0, 2];
    packet.header.set_source(&source).unwrap();
    packet.header.set_destination(&dest).unwrap();
    println!("✓ Set source and destination addresses");

    // Set hop limit and trust level
    packet.header.set_hop_limit(64);
    packet.header.set_trust_reputation(200);
    println!("✓ Set hop limit (64) and trust level (200)");

    // Create and set VLIB
    let vlib = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("exampleisp")
        .tenant("home")
        .role("user")
        .trust_level(200)
        .reputation_scope("global")
        .build()
        .unwrap();

    println!("\n=== VLIB Details ===");
    println!("  Service: {}", vlib.service);
    println!("  Location: {}", vlib.location);
    println!("  Provider: {}", vlib.provider);
    println!("  Tenant: {}", vlib.tenant);
    println!("  Role: {}", vlib.role);
    println!("  Trust Level: {}", vlib.trust_level);
    println!("  Reputation Scope: {}", vlib.reputation_scope);

    packet.set_vlib(vlib).unwrap();
    println!("\n✓ Set VLIB");

    // Set payload
    let payload = b"Hello, IPv7!";
    packet.set_payload(payload.to_vec());
    println!("✓ Set payload ({} bytes)", payload.len());

    // Serialize to bytes
    let bytes = packet.to_bytes().unwrap();
    println!("\n=== Packet Serialization ===");
    println!("Total packet size: {} bytes", bytes.len());
    println!("First 40 bytes (header): {}", hex::encode(&bytes[..40.min(bytes.len())]));

    // Parse from bytes
    let parsed_packet = Ipv7Packet::from_bytes(&bytes).unwrap();
    println!("\n=== Parsed Packet ===");
    println!("Hop Limit: {}", parsed_packet.header.get_hop_limit());
    println!("Trust Level: {}", parsed_packet.header.get_trust_reputation());

    if let Some(vlib) = parsed_packet.get_vlib() {
        println!("VLIB Provider: {}", vlib.provider);
        println!("VLIB Tenant: {}", vlib.tenant);
    }

    println!("Payload: {}", String::from_utf8_lossy(parsed_packet.get_payload()));

    println!("\n✓ Successfully parsed and serialized IPv7 packet!");
}
