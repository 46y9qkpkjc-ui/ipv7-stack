//! IPv7 Packet structure and processing
//!
//! Fixed 40-byte header followed by variable-length VLIB

use crate::error::{Ipv7Error, Result};
use crate::vlib::Vlib;
use bytes::{Buf, BufMut, BytesMut};

/// IPv7 Fixed Header (40 bytes)
#[derive(Clone, Debug)]
pub struct Ipv7Header {
    /// Version (7) and traffic class
    version_traffic: u8,
    /// Flow label (24 bits)
    flow_label: [u8; 3],
    /// Payload length
    payload_length: u16,
    /// Next header type
    next_header: u8,
    /// Hop limit
    hop_limit: u8,
    /// Source address (64 bits = 8 bytes)
    source: [u8; 8],
    /// Destination address (64 bits = 8 bytes)
    destination: [u8; 8],
    /// Trust/Reputation octet
    trust_reputation: u8,
    /// Reserved (15 bytes) - for future extensions and padding to 40 bytes
    reserved: [u8; 15],
}

impl Ipv7Header {
    /// Create a new IPv7 header
    pub fn new() -> Self {
        Ipv7Header {
            version_traffic: 0x70, // Version 7, traffic class 0
            flow_label: [0, 0, 0],
            payload_length: 0,
            next_header: 0,
            hop_limit: 64,
            source: [0; 8],
            destination: [0; 8],
            trust_reputation: 128, // Medium trust
            reserved: [0; 15],
        }
    }

    /// Set source address
    pub fn set_source(&mut self, addr: &[u8]) -> Result<()> {
        if addr.len() != 8 {
            return Err(Ipv7Error::InvalidPacket("Source address must be 8 bytes".into()));
        }
        self.source.copy_from_slice(addr);
        Ok(())
    }

    /// Set destination address
    pub fn set_destination(&mut self, addr: &[u8]) -> Result<()> {
        if addr.len() != 8 {
            return Err(Ipv7Error::InvalidPacket(
                "Destination address must be 8 bytes".into(),
            ));
        }
        self.destination.copy_from_slice(addr);
        Ok(())
    }

    /// Set hop limit
    pub fn set_hop_limit(&mut self, limit: u8) {
        self.hop_limit = limit;
    }

    /// Get hop limit
    pub fn get_hop_limit(&self) -> u8 {
        self.hop_limit
    }

    /// Set trust/reputation level
    pub fn set_trust_reputation(&mut self, level: u8) {
        self.trust_reputation = level;
    }

    /// Get trust/reputation level
    pub fn get_trust_reputation(&self) -> u8 {
        self.trust_reputation
    }

    /// Serialize header to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = BytesMut::with_capacity(40);
        bytes.put_u8(self.version_traffic);
        bytes.put_slice(&self.flow_label);
        bytes.put_u16(self.payload_length);
        bytes.put_u8(self.next_header);
        bytes.put_u8(self.hop_limit);
        bytes.put_slice(&self.source);
        bytes.put_slice(&self.destination);
        bytes.put_u8(self.trust_reputation);
        bytes.put_slice(&self.reserved);
        bytes.to_vec()
    }

    /// Parse header from bytes
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize)> {
        if data.len() < 40 {
            return Err(Ipv7Error::InvalidPacket("Header too short".into()));
        }

        let mut cursor = &data[..];
        let version_traffic = cursor.get_u8();
        let flow_label = [cursor.get_u8(), cursor.get_u8(), cursor.get_u8()];
        let payload_length = cursor.get_u16();
        let next_header = cursor.get_u8();
        let hop_limit = cursor.get_u8();

        let mut source = [0u8; 8];
        cursor.copy_to_slice(&mut source);

        let mut destination = [0u8; 8];
        cursor.copy_to_slice(&mut destination);

        let trust_reputation = cursor.get_u8();
        let mut reserved = [0u8; 15];
        cursor.copy_to_slice(&mut reserved);

        Ok((
            Ipv7Header {
                version_traffic,
                flow_label,
                payload_length,
                next_header,
                hop_limit,
                source,
                destination,
                trust_reputation,
                reserved,
            },
            40,
        ))
    }
}

/// IPv7 Packet (Header + VLIB)
#[derive(Clone, Debug)]
pub struct Ipv7Packet {
    pub header: Ipv7Header,
    pub vlib: Option<Vlib>,
    pub payload: Vec<u8>,
}

impl Ipv7Packet {
    /// Create a new IPv7 packet
    pub fn new() -> Self {
        Ipv7Packet {
            header: Ipv7Header::new(),
            vlib: None,
            payload: Vec::new(),
        }
    }

    /// Set VLIB
    pub fn set_vlib(&mut self, vlib: Vlib) -> Result<()> {
        self.vlib = Some(vlib);
        Ok(())
    }

    /// Get VLIB
    pub fn get_vlib(&self) -> Option<&Vlib> {
        self.vlib.as_ref()
    }

    /// Set payload
    pub fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
        self.header.payload_length = (self.payload.len() as u16).saturating_add(
            self.vlib
                .as_ref()
                .map(|v| v.to_bytes().map(|b| b.len() as u16).unwrap_or(0))
                .unwrap_or(0),
        );
    }

    /// Get payload
    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }

    /// Serialize packet to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Add header
        bytes.extend_from_slice(&self.header.to_bytes());

        // Add VLIB if present
        if let Some(vlib) = &self.vlib {
            bytes.extend_from_slice(&vlib.to_bytes()?);
        }

        // Add payload
        bytes.extend_from_slice(&self.payload);

        Ok(bytes)
    }

    /// Parse packet from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let (header, header_size) = Ipv7Header::from_bytes(data)?;

        let mut offset = header_size;
        let mut vlib = None;

        // Try to parse VLIB
        if offset < data.len() {
            // Simple heuristic: if remaining data starts with '{' it's likely JSON VLIB
            if data[offset] == b'{' {
                if let Ok(v) = Vlib::from_bytes(&data[offset..]) {
                    let vlib_bytes = v.to_bytes()?;
                    offset += vlib_bytes.len();
                    vlib = Some(v);
                }
            }
        }

        let payload = if offset < data.len() {
            data[offset..].to_vec()
        } else {
            Vec::new()
        };

        Ok(Ipv7Packet {
            header,
            vlib,
            payload,
        })
    }

    /// Get packet size
    pub fn size(&self) -> Result<usize> {
        let mut size = 40; // Header
        if let Some(vlib) = &self.vlib {
            size += vlib.to_bytes()?.len();
        }
        size += self.payload.len();
        Ok(size)
    }
}

impl Default for Ipv7Packet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_serialization() {
        let header = Ipv7Header::new();
        let bytes = header.to_bytes();
        assert_eq!(bytes.len(), 40);
    }

    #[test]
    fn test_packet_creation() {
        let packet = Ipv7Packet::new();
        assert_eq!(packet.header.hop_limit, 64);
    }

    #[test]
    fn test_packet_with_vlib() {
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
        assert!(packet.get_vlib().is_some());
    }
}
