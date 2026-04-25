# Contributing to IPv7 Stack

Thank you for your interest in contributing to the IPv7 Stack implementation! This document provides guidelines for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/ipv7-stack.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Build the project: `cargo build`
5. Run tests: `cargo test`

## Development Workflow

### Making Changes

- Follow Rust naming conventions (snake_case for functions, CamelCase for types)
- Add tests for new functionality
- Update documentation in docstrings
- Keep commits atomic and well-described

### Code Style

This project follows standard Rust conventions. Run `cargo fmt` to format code:

```bash
cargo fmt
```

Check code quality with clippy:

```bash
cargo clippy -- -D warnings
```

### Testing

Write tests for all new functionality:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Add integration tests in the `tests/` directory.

## Areas for Contribution

### Core Implementation
- Kernel module integration (eBPF, netfilter)
- Performance optimizations
- Memory efficiency improvements

### Documentation
- API documentation
- Integration guides
- Security best practices

### Examples
- Real-world usage examples
- Integration scenarios
- Performance benchmarks

### Testing
- Additional test cases
- Fuzzing support
- Benchmarking suite

## Protocol Compliance

All changes should maintain compliance with [draft-subbiah-ipv7-00](https://datatracker.ietf.org/doc/draft-subbiah-ipv7-00/).

Key areas to maintain:
- Fixed 40-byte header format
- Variable-Length Identity Block (VLIB) structure
- Source-Provider Validation (SPV) mechanism
- Ephemeral Identity Token (EIT) generation
- Three-stage processing pipeline

## Commit Message Guidelines

Use clear, descriptive commit messages:

```
feat: Add EIT expiration validation

- Validate EIT timestamps on packet processing
- Drop packets with expired tokens
- Add tests for EIT validation

Refs: draft-subbiah-ipv7-00 Section 6.4
```

Format:
- Type: feat, fix, docs, test, perf, refactor
- Subject: 50 characters or less
- Body: Explain what and why, not how
- References: Link to related issues or spec sections

## Pull Request Process

1. Update documentation and tests
2. Ensure all tests pass: `cargo test`
3. Ensure code is formatted: `cargo fmt`
4. Ensure no clippy warnings: `cargo clippy`
5. Create pull request with clear description
6. Link related issues
7. Respond to review feedback

## Security Considerations

- Never disable signature verification in tests
- Document cryptographic assumptions
- Consider side-channel attacks in crypto code
- Report security issues privately to maintainer

## Questions?

- Open an issue for questions
- Check existing issues for similar topics
- Review protocol specification for clarification

## Code of Conduct

- Be respectful and inclusive
- Focus on technical merit of ideas
- Help others learn and grow

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to IPv7 Stack! 🚀
