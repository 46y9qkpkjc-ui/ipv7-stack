//! IPv7 Router Demo Example
//!
//! Demonstrates the three-stage packet processing pipeline:
//! 1. Fast Path (trust/reputation checking)
//! 2. Validation Path (SPV)
//! 3. Routing Path (policy enforcement)

use ipv7_stack::packet::Ipv7Packet;
use ipv7_stack::reputation::ReputationScope;
use ipv7_stack::router::Router;
use ipv7_stack::spv::SpvPolicy;
use ipv7_stack::vlib::Vlib;

fn main() {
    println!("=== IPv7 Router Demo: Three-Stage Processing ===\n");

    // Create a router
    let mut router = Router::new("edge_router_1");
    println!("✓ Created router: {}\n", router.id);

    // Set SPV policy
    let policy = SpvPolicy::new()
        .require()
        .allow_providers(vec!["exampleisp".to_string(), "isp2".to_string()])
        .min_trust_level(100)
        .drop_on_failure(true);

    router.set_spv_policy(policy);
    println!("✓ Configured SPV Policy");
    println!("  Required: Yes");
    println!("  Allowed Providers: exampleisp, isp2");
    println!("  Min Trust Level: 100");
    println!("  Drop on Failure: Yes\n");

    // Register providers
    router.register_provider("exampleisp", &[1, 2, 3, 4, 5, 6, 7, 8]);
    router.register_provider("isp2", &[8, 7, 6, 5, 4, 3, 2, 1]);
    println!("✓ Registered providers: exampleisp, isp2\n");

    // Test Case 1: High-trust packet from authorized provider
    println!("=== Test Case 1: High-Trust Packet ===");
    let mut packet1 = Ipv7Packet::new();
    packet1.header.set_trust_reputation(200);
    let vlib1 = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("exampleisp")
        .tenant("home")
        .role("user")
        .trust_level(200)
        .reputation_scope("global")
        .build()
        .unwrap();
    packet1.set_vlib(vlib1).unwrap();
    packet1.set_payload(b"High-trust traffic".to_vec());

    println!("Processing packet from exampleisp with trust_level=200...");
    match router.process_packet(&packet1) {
        Ok(accepted) => {
            if accepted {
                println!("✓ ACCEPTED\n");
            } else {
                println!("✗ REJECTED\n");
            }
        }
        Err(e) => println!("✗ ERROR: {}\n", e),
    }

    // Test Case 2: Low-trust packet
    println!("=== Test Case 2: Low-Trust Packet ===");
    let mut packet2 = Ipv7Packet::new();
    packet2.header.set_trust_reputation(50); // Below threshold
    let vlib2 = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("exampleisp")
        .tenant("home")
        .role("user")
        .trust_level(50)
        .reputation_scope("global")
        .build()
        .unwrap();
    packet2.set_vlib(vlib2).unwrap();
    packet2.set_payload(b"Low-trust traffic".to_vec());

    println!("Processing packet with trust_level=50 (below 100 threshold)...");
    match router.process_packet(&packet2) {
        Ok(accepted) => {
            if accepted {
                println!("✓ ACCEPTED\n");
            } else {
                println!("✗ REJECTED (trust level too low)\n");
            }
        }
        Err(e) => println!("✗ ERROR: {}\n", e),
    }

    // Test Case 3: Unknown provider
    println!("=== Test Case 3: Unknown Provider ===");
    let mut packet3 = Ipv7Packet::new();
    packet3.header.set_trust_reputation(200);
    let vlib3 = Vlib::builder()
        .service("web")
        .location("nyc")
        .provider("unknownisp")
        .tenant("home")
        .role("user")
        .trust_level(200)
        .reputation_scope("global")
        .build()
        .unwrap();
    packet3.set_vlib(vlib3).unwrap();
    packet3.set_payload(b"Unknown provider traffic".to_vec());

    println!("Processing packet from unknownisp...");
    match router.process_packet(&packet3) {
        Ok(accepted) => {
            if accepted {
                println!("✓ ACCEPTED\n");
            } else {
                println!("✗ REJECTED (unknown provider)\n");
            }
        }
        Err(e) => println!("✗ ERROR: {}\n", e),
    }

    // Test Case 4: Authorized provider isp2
    println!("=== Test Case 4: Packet from isp2 ===");
    let mut packet4 = Ipv7Packet::new();
    packet4.header.set_trust_reputation(180);
    let vlib4 = Vlib::builder()
        .service("mail")
        .location("london")
        .provider("isp2")
        .tenant("office")
        .role("admin")
        .trust_level(180)
        .reputation_scope("global")
        .build()
        .unwrap();
    packet4.set_vlib(vlib4).unwrap();
    packet4.set_payload(b"Email traffic from isp2".to_vec());

    println!("Processing packet from isp2 with trust_level=180...");
    match router.process_packet(&packet4) {
        Ok(accepted) => {
            if accepted {
                println!("✓ ACCEPTED\n");
            } else {
                println!("✗ REJECTED\n");
            }
        }
        Err(e) => println!("✗ ERROR: {}\n", e),
    }

    // Print router statistics
    println!("=== Router Statistics ===");
    router.print_stats();
    println!();
    println!("Breakdown:");
    println!("  ✓ Accepted: {} packets", router.get_stats().packets_accepted);
    println!(
        "  ✗ Dropped (Low Trust): {} packets",
        router.get_stats().packets_dropped_low_trust
    );
    println!(
        "  ✗ Failed Policy: {} packets",
        router.get_stats().packets_failed_policy
    );

    // Demonstrate reputation tracking
    println!("\n=== Reputation Database ===");
    let rep_db = router.get_reputation_db();
    println!("Tracked identities: {}", rep_db.len());
    for (identity, entry) in rep_db.entries() {
        println!("  {}: trust_level={}, success_rate={:.1}%",
                 identity, entry.trust_level.as_u8(),
                 entry.success_rate() * 100.0);
    }

    println!("\n✓ Router demo completed!");
}
