//! IPv7 Stack - Identity-Centric Network Protocol
//!
//! This crate implements the IPv7 protocol specification as defined in draft-subbiah-ipv7-00.
//!
//! IPv7 extends the Internet Protocol model with identity-carrying addresses and origin
//! validation mechanisms to mitigate abuse of residential proxy infrastructure.
//!
//! # Core Components
//!
//! - **Packet**: IPv7 packet structure with fixed header and variable-length identity block
//! - **VLIB**: Variable-Length Identity Block containing EIT, provider, tenant, role, and signature
//! - **SPV**: Source-Provider Validation for cryptographic origin verification
//! - **Router**: Packet processing and policy enforcement
//! - **Reputation**: Trust level and reputation scoring
//!
//! # Example
//!
//! ```rust,ignore
//! use ipv7_stack::packet::Ipv7Packet;
//! use ipv7_stack::vlib::Vlib;
//! use ipv7_stack::spv::SourceProviderValidator;
//!
//! // Create an IPv7 packet
//! let mut packet = Ipv7Packet::new();
//! packet.set_destination("web.nyc.exampleisp.home");
//!
//! // Build VLIB with identity information
//! let vlib = Vlib::builder()
//!     .provider("exampleisp")
//!     .tenant("home")
//!     .role("user")
//!     .build();
//!
//! // Validate source/provider binding
//! let validator = SourceProviderValidator::new();
//! let is_valid = validator.validate(&packet, &vlib);
//! ```

pub mod error;
pub mod packet;
pub mod vlib;
pub mod spv;
pub mod router;
pub mod reputation;
pub mod identity;
pub mod crypto;

pub use error::{Ipv7Error, Result};
pub use packet::Ipv7Packet;
pub use vlib::Vlib;
pub use spv::SourceProviderValidator;
pub use router::Router;
pub use reputation::{TrustLevel, ReputationScope};
pub use identity::Identity;

/// IPv7 protocol version
pub const IPV7_VERSION: u8 = 7;

/// IPv7 fixed header size in bytes
pub const IPV7_HEADER_SIZE: usize = 40;

/// Maximum VLIB size in bytes
pub const MAX_VLIB_SIZE: usize = 256;

/// Maximum IPv7 packet size (same as IPv6)
pub const MAX_PACKET_SIZE: usize = 65535;
