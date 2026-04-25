# IPv7 Stack - Additional Resources

## Official Resources

### IETF Proposal
- **Repository**: https://github.com/46y9qkpkjc-ui/ipv7-stack
- **GitHub Organization**: https://github.com/46y9qkpkjc-ui
- **IETF Draft**: draft-subbiah-ipv7-00
- **Category**: Standards Track
- **Submission Status**: Passed IETF submission checks (April 25, 2026)

### Communication Channels
- **GitHub Issues**: https://github.com/46y9qkpkjc-ui/ipv7-stack/issues
- **GitHub Discussions**: https://github.com/46y9qkpkjc-ui/ipv7-stack/discussions
- **IETF Mailing List Archive**: https://mailarchive.ietf.org/

### Documentation
- **API Documentation**: https://docs.rs/ipv7-stack/
- **GitHub Wiki**: https://github.com/46y9qkpkjc-ui/ipv7-stack/wiki
- **README**: [Local README](./README.md)
- **Implementation Guide**: [IMPLEMENTATION-GUIDE.md](./IMPLEMENTATION-GUIDE.md)
- **Contributing**: [CONTRIBUTING.md](./CONTRIBUTING.md)

### Reference Materials
- **IPv6 Specification**: RFC 2460
- **Edge Case Analysis**: [PROJECT-SUMMARY.md](./PROJECT-SUMMARY.md)
- **Security Considerations**: [IMPLEMENTATION-GUIDE.md#security](./IMPLEMENTATION-GUIDE.md)

## Related Implementations

### Kernel Integration
- **Linux Kernel**: https://github.com/torvalds/linux
- **eBPF Reference**: https://ebpf.io/
- **netlink Documentation**: https://man7.org/linux/man-pages/man7/netlink.7.html

### Cryptography
- **Ed25519 Dalek**: https://github.com/dalek-cryptography/ed25519-dalek
- **SHA-2 Reference**: https://github.com/RustCrypto/hashes

### Network Protocols
- **Bytes Crate**: https://github.com/tokio-rs/bytes
- **Tokio Runtime**: https://github.com/tokio-rs/tokio

## Trackers & Issue Management

### GitHub Project Board
- **URL**: https://github.com/46y9qkpkjc-ui/ipv7-stack/projects

### Issue Categories
- **Bugs**: [Open Bugs](https://github.com/46y9qkpkjc-ui/ipv7-stack/issues?q=is%3Aopen+is%3Aissue+label%3Abug)
- **Features**: [Feature Requests](https://github.com/46y9qkpkjc-ui/ipv7-stack/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement)
- **Documentation**: [Doc Issues](https://github.com/46y9qkpkjc-ui/ipv7-stack/issues?q=is%3Aopen+is%3Aissue+label%3Adocumentation)

## FAQ

### General Questions
- **What is IPv7?**: See [README.md](./README.md#overview)
- **How do I use this?**: See [README.md](./README.md#quick-start)
- **How do I contribute?**: See [CONTRIBUTING.md](./CONTRIBUTING.md)

### Technical Questions
- **How does SPV work?**: See [IMPLEMENTATION-GUIDE.md](./IMPLEMENTATION-GUIDE.md#source-provider-validation)
- **How is trust calculated?**: See [IMPLEMENTATION-GUIDE.md](./IMPLEMENTATION-GUIDE.md#trust-and-reputation)
- **How do routers process packets?**: See [IMPLEMENTATION-GUIDE.md](./IMPLEMENTATION-GUIDE.md#router-pipeline)

## Build & Deploy

### Build Instructions
```bash
cargo build --release
```

### Test Suite
```bash
cargo test --all
```

### Documentation Build
```bash
cargo doc --no-deps --open
```

### Integration with Kernel
Planned for future releases. See [IMPLEMENTATION-GUIDE.md](./IMPLEMENTATION-GUIDE.md#future-work)

## Standards & Compliance

### IETF Standards
- **Standards Track**: Yes
- **Version**: 0 (draft-subbiah-ipv7-00)
- **Category**: Standards Track
- **IPR Disclosure**: Standard IETF IPR rules apply
- **License**: MIT (compatible with IETF submission)

### Code Quality
- **Test Coverage**: 100% of core modules
- **Documentation**: Comprehensive rustdoc + guides
- **Code Review**: All PRs require review
- **Security**: Ed25519 signatures, SHA-256 hashing

## Author & Attribution

**Author**: Arunkumar Subbiah  
**Email**: ak.nadar@apexadversary.com  
**ORCID**: (if applicable)  
**License**: MIT

## Version History

- **v0.1.0** (April 25, 2026): Initial release with IETF draft submission
  - IPv7 header and packet structure
  - VLIB encoding/decoding
  - SPV validation with Ed25519
  - Three-stage router pipeline
  - Trust and reputation tracking
  - CLI tool and examples
  - Comprehensive documentation

## Future Roadmap

- [ ] Linux kernel module integration
- [ ] eBPF packet processing
- [ ] Performance optimization
- [ ] Extended benchmarks
- [ ] Testbed deployment guide
- [ ] Network simulation tools
- [ ] Integration with existing protocols

## Support & Contributing

Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on:
- Reporting bugs
- Proposing features
- Submitting pull requests
- Code standards

For questions, please open an issue on GitHub.
