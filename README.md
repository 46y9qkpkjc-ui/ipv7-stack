# IPv7 Stack - Rust Implementation

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
[![IETF Draft](https://img.shields.io/badge/IETF-draft--subbiah--ipv7--00-brightgreen)](https://datatracker.ietf.org/doc/draft-subbiah-ipv7-00/)

A comprehensive Rust implementation of the **IPv7 (Identity-Centric Network Protocol)**, as specified in [draft-subbiah-ipv7-00](https://datatracker.ietf.org/doc/draft-subbiah-ipv7-00/).

## Overview

IPv7 is an identity-centric network protocol that extends the Internet Protocol model with cryptographic origin validation and policy enforcement to mitigate residential proxy exploitation and improve network security for IoT devices.

### Key Features

✅ **Identity-Centric Addressing** - Hierarchical identity strings instead of purely numerical addresses  
✅ **Source-Provider Validation (SPV)** - Cryptographic verification of origin/provider binding  
✅ **Ephemeral Identity Tokens (EIT)** - Time-bound session-specific anonymity masking  
✅ **Trust & Reputation Scoring** - Network-layer trust signaling and filtering  
✅ **Three-Stage Processing Pipeline** - Fast path, validation, and routing layers  
✅ **Ed25519 Cryptography** - Modern elliptic curve signature verification  
✅ **Variable-Length Identity Block (VLIB)** - Flexible identity component encoding  

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Cargo

### Installation

```bash
git clone https://github.com/arunkumarsubbiah/ipv7-stack.git
cd ipv7-stack
cargo build --release
```

### Running Examples

```bash
# Parse and serialize IPv7 packets
cargo run --example packet_parser

# Demonstrate SPV validation
cargo run --example spv_validation

# See the router in action
cargo run --example router_demo
```

## Architecture

### Packet Structure

```
┌─────────────────────────────────────────────────┐
│ IPv7 Fixed Header (40 bytes)                    │
├─────────────────────────────────────────────────┤
│ Variable-Length Identity Block (VLIB)           │
│ ┌─────────────────────────────────────────────┐ │
│ │ EIT (Ephemeral Identity Token)              │ │
│ │ Service . Location . Provider . Tenant      │ │
│ │ Role . Trust Level . Reputation Scope       │ │
│ │ Origin Signature (Ed25519)                  │ │
│ └─────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────┤
│ Payload (variable)                              │
└─────────────────────────────────────────────────┘
```

### Three-Stage Processing

```
Incoming Packet
       │
       ▼
┌─────────────────────────────────┐
│ Stage 1: Fast Path              │
│ - Check Trust/Reputation octet  │
│ - Rate limiting decision        │
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│ Stage 2: Validation Path        │
│ - Perform SPV                   │
│ - Verify Origin Signature       │
│ - Authenticate provider         │
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│ Stage 3: Routing Path           │
│ - Apply policy rules            │
│ - Check reputation              │
│ - Make routing decision         │
└─────────────────────────────────┘
       │
       ▼
    Accept/Drop
```

## Core Modules

### `packet.rs` - IPv7 Packet Structure
- Fixed 40-byte header
- Variable-length VLIB
- Serialization/deserialization

### `vlib.rs` - Variable-Length Identity Block
- Identity component encoding
- Builder pattern for construction
- JSON serialization support

### `spv.rs` - Source-Provider Validation
- Provider key registration
- SPV policy enforcement
- Signature verification framework

### `crypto.rs` - Cryptographic Operations
- Ed25519 signing and verification
- Ephemeral Identity Token generation
- SHA-256 hashing

### `identity.rs` - Identity Management
- Identity parsing and formatting
- Hierarchical identity representation
- Builder pattern for creation

### `reputation.rs` - Trust and Reputation
- Trust level scoring (0-255)
- Reputation database
- Success/violation tracking

### `router.rs` - Packet Processing
- Three-stage processing pipeline
- Policy enforcement
- Router statistics and metrics

## Usage Examples

### Creating an IPv7 Packet

```rust
use ipv7_stack::packet::Ipv7Packet;
use ipv7_stack::vlib::Vlib;

// Create packet
let mut packet = Ipv7Packet::new();

// Create VLIB
let vlib = Vlib::builder()
    .service("web")
    .location("nyc")
    .provider("exampleisp")
    .tenant("home")
    .role("user")
    .trust_level(200)
    .reputation_scope("global")
    .build()?;

// Set VLIB and payload
packet.set_vlib(vlib)?;
packet.set_payload(b"Hello IPv7".to_vec());
```

### Validating Source/Provider Binding

```rust
use ipv7_stack::spv::{SourceProviderValidator, SpvPolicy};

let mut validator = SourceProviderValidator::new();
validator.register_provider("exampleisp", &public_key);

let policy = SpvPolicy::new()
    .require()
    .min_trust_level(100);

match validator.validate(&packet) {
    Ok(valid) => println!("SPV: {}", if valid { "PASS" } else { "FAIL" }),
    Err(e) => println!("Error: {}", e),
}
```

### Processing Packets with Router

```rust
use ipv7_stack::router::Router;

let mut router = Router::new("edge_router_1");
router.register_provider("exampleisp", &public_key);

let result = router.process_packet(&packet)?;
println!("Packet: {}", if result { "ACCEPTED" } else { "REJECTED" });
println!("Stats: {}", router.get_stats().packets_processed);
```

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Security Considerations

- **Signature Verification**: Always verify origin signatures using registered provider keys
- **Key Management**: Implement proper key rotation and storage policies
- **EIT Expiration**: Check ephemeral token validity before processing
- **Rate Limiting**: Use trust level for congestion-aware rate limiting
- **Policy Enforcement**: Apply strict SPV policies in production environments

## Integration with Linux Kernel

This reference implementation can be extended to integrate with Linux kernel networking:

1. **As an eBPF program** - Attach to tc (traffic control) for packet filtering
2. **As a kernel module** - Implement IPv7 protocol handler in kernel
3. **As a userspace daemon** - Process packets at application layer
4. **With Rust-for-Linux** - Native kernel module in Rust

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## References

- [IETF Draft](https://datatracker.ietf.org/doc/draft-subbiah-ipv7-00/)
- [RFC 2119 - Key Words for Use in RFCs](https://tools.ietf.org/html/rfc2119)
- [Ed25519 - EdDSA signature scheme](https://tools.ietf.org/html/rfc8032)
- [Rust Cryptography](https://docs.rs/ed25519-dalek/)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Arunkumar Subbiah  
Email: arunkumar.subbiah@apexadversary.com

## Citation

If you use this implementation in your research or projects, please cite the IETF draft:

```
Subbiah, A., "IPv7: Identity-Centric Network Protocol for Security,
Proxy Mitigation, and Operability", draft-subbiah-ipv7-00, April 2026.
```

---

**Status**: Reference Implementation for IETF Standards Track Proposal  
**Last Updated**: April 2026  
**Rust Edition**: 2021
