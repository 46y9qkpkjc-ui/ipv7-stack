/// IPv7 Stack - Interactive CLI Tool
///
/// A command-line interface for testing and demonstrating IPv7 protocol functionality

use clap::{Parser, Subcommand};
use ipv7_stack::{
    packet::Ipv7Packet, router::Router, spv::SpvPolicy,
    vlib::Vlib,
};

#[derive(Parser)]
#[command(name = "IPv7 Stack")]
#[command(about = "IPv7 Protocol Reference Implementation - CLI Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and display an IPv7 packet
    #[command(about = "Create a new IPv7 packet")]
    CreatePacket {
        /// Service name (e.g., 'web', 'mail')
        #[arg(short, long, default_value = "web")]
        service: String,

        /// Location (e.g., 'nyc', 'london')
        #[arg(short, long, default_value = "nyc")]
        location: String,

        /// Provider ID
        #[arg(short, long, default_value = "exampleisp")]
        provider: String,

        /// Tenant (e.g., 'home', 'office')
        #[arg(short, long, default_value = "home")]
        tenant: String,

        /// Trust level (0-255)
        #[arg(short, long, default_value = "128")]
        trust: u8,
    },

    /// Parse an IPv7 packet from hex
    #[command(about = "Parse IPv7 packet from hex string")]
    ParsePacket {
        /// Hex-encoded packet data
        #[arg(short, long)]
        hex: String,
    },

    /// Run router demo
    #[command(about = "Run interactive router demo")]
    RouterDemo,

    /// Test SPV validation
    #[command(about = "Test Source-Provider Validation")]
    TestSpv,

    /// Show version
    #[command(about = "Show version information")]
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreatePacket {
            service,
            location,
            provider,
            tenant,
            trust,
        } => {
            println!("=== Creating IPv7 Packet ===\n");

            let mut packet = Ipv7Packet::new();
            packet.header.set_trust_reputation(trust);

            let vlib = Vlib::builder()
                .service(service.clone())
                .location(location.clone())
                .provider(provider.clone())
                .tenant(tenant.clone())
                .role("user")
                .trust_level(trust)
                .reputation_scope("global")
                .build()
                .expect("Failed to build VLIB");

            packet.set_vlib(vlib).expect("Failed to set VLIB");
            packet.set_payload(b"IPv7 test packet".to_vec());

            println!("✓ Packet created successfully");
            println!("\nPacket Details:");
            println!("  Service: {}", service);
            println!("  Location: {}", location);
            println!("  Provider: {}", provider);
            println!("  Tenant: {}", tenant);
            println!("  Trust Level: {}", trust);

            match packet.to_bytes() {
                Ok(bytes) => {
                    println!("  Size: {} bytes", bytes.len());
                    println!("\n  Hex (first 64 bytes): {}...", &hex::encode(&bytes[..64.min(bytes.len())]));
                }
                Err(e) => println!("  Error serializing: {}", e),
            }
        }

        Commands::ParsePacket { hex } => {
            println!("=== Parsing IPv7 Packet ===\n");

            match hex::decode(&hex) {
                Ok(bytes) => {
                    match Ipv7Packet::from_bytes(&bytes) {
                        Ok(packet) => {
                            println!("✓ Packet parsed successfully");
                            println!("\nPacket Details:");
                            println!("  Hop Limit: {}", packet.header.get_hop_limit());
                            println!("  Trust Level: {}", packet.header.get_trust_reputation());

                            if let Some(vlib) = packet.get_vlib() {
                                println!("  Provider: {}", vlib.provider);
                                println!("  Tenant: {}", vlib.tenant);
                                println!("  Role: {}", vlib.role);
                            }

                            println!("  Payload: {} bytes", packet.get_payload().len());
                        }
                        Err(e) => println!("✗ Error parsing packet: {}", e),
                    }
                }
                Err(e) => println!("✗ Error decoding hex: {}", e),
            }
        }

        Commands::RouterDemo => {
            println!("=== IPv7 Router Demo ===\n");

            let mut router = Router::new("demo_router");
            router.register_provider("exampleisp", &[1, 2, 3, 4, 5, 6, 7, 8]);

            // Create test packet
            let mut packet = Ipv7Packet::new();
            packet.header.set_trust_reputation(180);

            let vlib = Vlib::builder()
                .service("web")
                .location("nyc")
                .provider("exampleisp")
                .tenant("home")
                .role("user")
                .trust_level(180)
                .reputation_scope("global")
                .build()
                .expect("Failed to build VLIB");

            packet.set_vlib(vlib).expect("Failed to set VLIB");

            let policy = SpvPolicy::new()
                .require()
                .allow_providers(vec!["exampleisp".to_string()])
                .min_trust_level(100)
                .drop_on_failure(true);

            router.set_spv_policy(policy);

            println!("Processing packet...");
            match router.process_packet(&packet) {
                Ok(accepted) => {
                    println!("Result: {}", if accepted { "ACCEPTED" } else { "REJECTED" });
                }
                Err(e) => println!("Error: {}", e),
            }

            println!("\n");
            router.print_stats();
        }

        Commands::TestSpv => {
            println!("=== Testing SPV Validation ===\n");

            use ipv7_stack::spv::SourceProviderValidator;

            let mut validator = SourceProviderValidator::new();
            validator.register_provider("exampleisp", &[1, 2, 3, 4, 5, 6, 7, 8]);

            let mut packet = Ipv7Packet::new();
            let vlib = Vlib::builder()
                .service("web")
                .location("nyc")
                .provider("exampleisp")
                .tenant("home")
                .role("user")
                .trust_level(150)
                .reputation_scope("global")
                .build()
                .expect("Failed to build VLIB");

            packet.set_vlib(vlib).expect("Failed to set VLIB");

            match validator.validate(&packet) {
                Ok(valid) => {
                    println!("✓ SPV Validation: {}", if valid { "PASS" } else { "FAIL" });
                }
                Err(e) => println!("✗ SPV Error: {}", e),
            }
        }

        Commands::Version => {
            println!("IPv7 Stack v0.1.0");
            println!("IETF Draft: draft-subbiah-ipv7-00");
            println!("Author: Arunkumar Subbiah");
        }
    }
}
