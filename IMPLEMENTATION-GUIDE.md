# IPv7 Stack Implementation Guide

## Project Structure

```
ipv7-stack/
├── Cargo.toml                 # Project manifest and dependencies
├── Cargo.lock                 # Dependency lock file
├── README.md                  # Project overview
├── LICENSE                    # MIT License
├── CONTRIBUTING.md            # Contribution guidelines
├── IMPLEMENTATION-GUIDE.md    # This file
├── .gitignore                 # Git ignore rules
│
├── src/
│   ├── lib.rs                 # Library entry point and re-exports
│   ├── main.rs                # CLI application
│   │
│   ├── packet.rs              # IPv7 packet structure (40-byte fixed header + VLIB)
│   ├── vlib.rs                # Variable-Length Identity Block
│   ├── identity.rs            # Identity representation and parsing
│   │
│   ├── spv.rs                 # Source-Provider Validation
│   ├── crypto.rs              # Cryptographic operations (Ed25519, SHA-256, EIT)
│   ├── reputation.rs          # Trust levels and reputation database
│   │
│   ├── router.rs              # Packet processing router (3-stage pipeline)
│   ├── error.rs               # Error types
│
├── examples/
│   ├── packet_parser.rs       # Example: Create and parse packets
│   ├── spv_validation.rs      # Example: Validate source/provider binding
│   ├── router_demo.rs         # Example: Router 3-stage processing
│
├── tests/                     # Integration tests (future)
└── target/                    # Build artifacts (generated)
```

## Module Architecture

### Core Protocol Modules

#### `packet.rs` - IPv7 Packet Structure
**Responsibility**: Represent and serialize IPv7 packets

- **Ipv7Header**: Fixed 40-byte header
  - Version (3 bits) and traffic class (5 bits)
  - Flow label (24 bits)
  - Payload length (16 bits)
  - Next header type (8 bits)
  - Hop limit (8 bits)
  - Source address (8 bytes)
  - Destination address (8 bytes)
  - Trust/Reputation octet (8 bits)
  - Reserved (6 bytes)

- **Ipv7Packet**: Complete packet with header, VLIB, and payload
  - Methods: `new()`, `set_vlib()`, `set_payload()`, `to_bytes()`, `from_bytes()`

#### `vlib.rs` - Variable-Length Identity Block
**Responsibility**: Encode hierarchical identity information

Structure:
```
[EIT]/service.location.provider.tenant.role.trustlevel.reputationscope.[Origin_Signature]
```

Example:
```
eit_7f3a9c2b/web.nyc.exampleisp.home.guest.medium.global.ed25519sig_...
```

- **Vlib**: Identity block container
  - Builder pattern for creation
  - JSON serialization for transport
  - Methods: `to_bytes()`, `from_bytes()`, `to_identity()`

#### `identity.rs` - Identity Management
**Responsibility**: Parse and represent identities

- **Identity**: Structured representation of IPv7 identity
  - Components: EIT, service, location, provider, tenant, role, trust_level, reputation_scope
  - Parsing: `from_str()` for string representation
  - Serialization: `to_string()` for display
  - Builder pattern for construction

### Security Modules

#### `spv.rs` - Source-Provider Validation
**Responsibility**: Validate source/provider binding

- **SourceProviderValidator**: Core validation mechanism
  - Provider key registry
  - Signature verification
  - Methods: `register_provider()`, `validate()`, `validate_with_signature()`

- **SpvPolicy**: Policy enforcement rules
  - Required vs optional SPV
  - Provider whitelisting
  - Trust level thresholds
  - Drop on failure behavior
  - Method: `passes_policy()`

#### `crypto.rs` - Cryptographic Operations
**Responsibility**: Cryptographic primitives

- **OriginSignature**: Ed25519 signature container
  - Sign with private key
  - Verify with public key
  - Timestamp tracking

- **EphemeralIdentityToken**: Time-bound session tokens
  - Random token generation
  - TTL (time-to-live) support
  - Validity checking
  - Hex encoding

Utilities:
- `hash_sha256()`: SHA-256 hashing
- `generate_keypair()`: Ed25519 key pair generation

#### `reputation.rs` - Trust and Reputation
**Responsibility**: Track and manage reputation

- **TrustLevel**: Enum with levels (VeryLow, Low, Medium, High, VeryHigh)
  - Values: 0, 64, 128, 192, 255
  - Conversion from u8

- **ReputationScope**: Geographic/organizational scope (Local, Regional, Global)

- **ReputationEntry**: Track identity reputation
  - Success/violation counting
  - Trust level calculation
  - Metadata storage

- **ReputationDatabase**: In-memory reputation store
  - CRUD operations
  - Bulk operations
  - Statistics

### Processing Modules

#### `router.rs` - Packet Processing
**Responsibility**: Implement three-stage processing pipeline

Three-Stage Pipeline:

1. **Fast Path**: Trust/Reputation checking
   - Check trust level octet
   - Compare against threshold
   - Make drop/accept decision

2. **Validation Path**: SPV
   - Verify origin signature
   - Check provider registration
   - Validate cryptographic binding

3. **Routing Path**: Policy enforcement
   - Apply SPV policy rules
   - Check provider whitelist
   - Verify trust thresholds

- **Router**: Main processor
  - Methods: `process_packet()`, `register_provider()`, `set_spv_policy()`
  - Statistics: packets processed, accepted, dropped
  - Metrics: acceptance rate, provider info

- **RouterStats**: Statistics container
  - Counters for each rejection reason
  - Used for monitoring and debugging

### Utility Modules

#### `error.rs` - Error Handling
**Responsibility**: Define all error types

- **Ipv7Error**: Enum with variants for each error condition
  - InvalidPacket, InvalidVlib, SignatureVerificationFailed, etc.
  - Implements `std::error::Error` trait
  - Display formatting for debugging

- **Result<T>**: Type alias for `Result<T, Ipv7Error>`

### CLI Module

#### `main.rs` - Command-Line Interface
**Responsibility**: Provide interactive testing tool

Commands:
- `create-packet`: Create and display IPv7 packet
- `parse-packet`: Parse hex-encoded packet
- `router-demo`: Run router demonstration
- `test-spv`: Test SPV validation
- `version`: Show version info

Uses clap for argument parsing

## Data Flow

### Packet Reception and Processing

```
Raw Packet Bytes
       │
       ▼
    Header Parsing (40 bytes)
       │
       ▼
    VLIB Parsing (variable length, JSON)
       │
       ▼
    Router::process_packet()
       │
       ├─▶ Stage 1: Check Trust Level
       │
       ├─▶ Stage 2: SPV Validation
       │       ├─▶ Verify Provider Registration
       │       └─▶ Check Signature (if present)
       │
       └─▶ Stage 3: Policy Enforcement
               ├─▶ Check Provider Whitelist
               ├─▶ Check Trust Threshold
               └─▶ Update Reputation Database
       │
       ▼
    Decision: Accept/Drop
       │
       ▼
    Statistics Update
```

### Packet Creation

```
IPv7Packet::new()
       │
       ▼
  Set Header Fields
  ├─ Source Address
  ├─ Destination Address
  ├─ Hop Limit
  └─ Trust Level
       │
       ▼
  Build VLIB
  ├─ Service
  ├─ Location
  ├─ Provider
  ├─ Tenant
  ├─ Role
  ├─ Trust Level
  └─ Reputation Scope
       │
       ▼
  Set Payload
       │
       ▼
  Serialize to Bytes
       │
       ▼
  Ready for Transmission
```

## Dependency Graph

```
ipv7_stack
├── bytes (serialization)
├── serde (serialization)
├── serde_json (JSON support)
├── ed25519-dalek (cryptography)
├── sha2 (hashing)
├── hex (hex encoding)
├── rand (random generation)
├── thiserror (error handling)
├── tokio (async runtime, optional)
└── clap (CLI parsing)
```

## Building and Testing

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_spv_validation

# Test specific module
cargo test --lib packet
```

### Examples

```bash
# Run example
cargo run --example packet_parser

# Run with output
cargo run --example router_demo -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Check with clippy
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open
```

## Integration Points

### Future Kernel Integration

#### As eBPF Program
```rust
// Attach to tc (traffic control)
// Filter IPv7 packets based on SPV and reputation
// Rate limit based on trust level
```

#### As Netfilter Module
```rust
// IPv7-aware netfilter hooks
// Integration with nftables rules
// Performance optimizations with native kernel code
```

#### As Userspace Daemon
```rust
// Tokio-based async packet processing
// gRPC interface for external tools
// Integration with existing network infrastructure
```

## Performance Considerations

### Optimizations
- Header parsing (pre-allocated buffers)
- Reputation database (HashMap for O(1) lookup)
- SPV validation (cached provider keys)
- Trust level fast path (early exit)

### Scalability
- In-memory reputation database (consider persistent storage)
- Batch processing support
- Async processing with Tokio
- Lock-free data structures for multi-threaded use

## Security Hardening

### Cryptography
- Ed25519 for signature verification
- SHA-256 for hashing
- Random token generation with crypto-grade RNG

### Validation
- Strict header parsing
- VLIB component validation
- Provider key verification
- Signature verification before acceptance

### Resource Limits
- Maximum packet size (65535 bytes)
- Maximum VLIB size (256 bytes)
- Rate limiting via trust level
- Reputation tracking for suspicious patterns

## Next Steps

1. **Kernel Integration**: Develop eBPF/netfilter modules
2. **Performance**: Benchmark and optimize hot paths
3. **Persistence**: Add reputation database persistence
4. **Observability**: Add tracing and metrics
5. **Distribution**: Package for major Linux distributions
6. **Community**: Seek feedback and contributions

## References

- [IETF Draft](https://datatracker.ietf.org/doc/draft-subbiah-ipv7-00/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [ed25519-dalek](https://docs.rs/ed25519-dalek/)
- [Tokio Guide](https://tokio.rs/)

---

**Last Updated**: April 2026  
**Status**: Reference Implementation v0.1.0  
**Author**: Arunkumar Subbiah
