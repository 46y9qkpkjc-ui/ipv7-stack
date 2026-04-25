# GitHub Repository Configuration

## Repository Metadata

**Name**: ipv7-stack  
**Owner**: 46y9qkpkjc-ui  
**URL**: https://github.com/46y9qkpkjc-ui/ipv7-stack  
**Description**: IPv7 Identity-Centric Network Protocol - Rust Reference Implementation  
**Visibility**: Public  

## Repository Topics

Add the following topics to the GitHub repository for better discoverability:

- `ipv7`
- `network-protocol`
- `ietf`
- `rust`
- `cryptography`
- `identity-management`
- `trust-reputation`
- `source-provider-validation`
- `network-security`
- `reference-implementation`

## Branch Protection Rules

### Main Branch (`main`)
- Require pull request reviews before merging: **Yes** (minimum 1 review)
- Require status checks to pass before merging: **Yes**
- Require branches to be up to date: **Yes**
- Include administrators in restrictions: **No**
- Allow force pushes: **No** (except admins)
- Allow deletions: **No**

### Protected Checks
- Tests (ubuntu-latest, stable)
- Tests (ubuntu-latest, beta)
- Tests (macos-latest, stable)
- Tests (macos-latest, beta)
- Tests (windows-latest, stable)
- Tests (windows-latest, beta)
- Rustfmt
- Clippy
- Code Coverage

## Issue Labels

Standard labels with recommended colors:

| Label | Color | Description |
|-------|-------|-------------|
| `bug` | #d73a4a | Something isn't working |
| `enhancement` | #a2eeef | New feature or request |
| `documentation` | #0075ca | Improvements or additions to documentation |
| `good first issue` | #7057ff | Good for newcomers |
| `help wanted` | #008672 | Extra attention is needed |
| `security` | #ff6b6b | Security vulnerability |
| `performance` | #fbca04 | Performance improvement |
| `testing` | #cccccc | Testing and CI/CD |
| `wontfix` | #ffffff | This will not be worked on |
| `duplicate` | #cfd3d7 | This issue or PR already exists |

## Milestone Structure

### Version Milestones
- **v0.1.0**: Initial release with IETF draft (COMPLETED)
- **v0.2.0**: Enhanced documentation and testing
- **v0.3.0**: Kernel integration foundation
- **v1.0.0**: Stable release with eBPF module

### Release Process
1. Create milestone with version number
2. Tag issues/PRs with milestone
3. Create release branch from develop
4. Merge to main with version tag
5. Publish to crates.io
6. Create GitHub release with changelog

## Automated Workflows

### Continuous Integration
- **tests.yml**: Run on push and PR
  - Tests on Linux, macOS, Windows
  - Rust stable and beta
  - Formatting check (rustfmt)
  - Linting (clippy)
  - Code coverage (tarpaulin)

- **release.yml**: Run on version tags
  - Create GitHub release
  - Publish to crates.io

- **docs.yml**: Run on main branch push
  - Build rustdoc
  - Deploy to GitHub Pages

### Manual Workflows
(Can be added for specific tasks)

## Secrets & Variables

### Required Secrets
- `CARGO_TOKEN`: For publishing to crates.io
- `GITHUB_TOKEN`: Automatically provided (for releases)

### Environment Variables
Set in workflow files or repository settings as needed:
- `RUST_BACKTRACE=1` for detailed error logs
- `CARGO_TERM_COLOR=always` for colored output

## Pages & Website

### GitHub Pages
- **Source**: `gh-pages` branch (auto-generated from `target/doc`)
- **URL**: https://46y9qkpkjc-ui.github.io/ipv7-stack
- **Content**: Generated rustdoc from docs.yml workflow

### Custom Domain
(Optional) Can be configured in repository settings

## Deployment Configuration

### Production Deployment
- Crates.io: Automatic via release workflow
- Documentation: Automatic via docs workflow
- GitHub Releases: Automatic via release workflow

### Staging/Testing
- Develop branch for pre-release testing
- Create pull requests to main for review

## Security Configuration

### Branch Permissions
- Only maintainers can merge to main
- All code requires review
- Status checks must pass

### Secrets Scanning
- GitHub secret scanning enabled
- Alerts sent to maintainers

### Dependency Management
- Dependabot enabled for Cargo.toml updates
- Security advisories monitored

## Contributing Guidelines

See [CONTRIBUTING.md](./CONTRIBUTING.md) for:
- Code standards
- Pull request process
- Testing requirements
- Documentation expectations

## Repository Rules & Conventions

### Commit Messages
- Use conventional commits (feat:, fix:, docs:, etc.)
- Keep messages concise and descriptive
- Reference issues when applicable

### Pull Requests
- One feature per PR when possible
- Link to related issues
- Include tests for new features
- Update documentation as needed

### Release Tags
- Format: `v{major}.{minor}.{patch}`
- Example: `v0.1.0`, `v0.2.1`
- Sign commits with GPG when possible

## Integration Partners

### Bors-ng (Optional - for merge automation)
Can be configured to automate merging approved PRs

### Codecov (Optional - for coverage tracking)
Already integrated via workflows

### Dependabot (Built-in)
Automatically creates PRs for dependency updates

## Support & Maintenance

**Maintenance Status**: Active Development  
**Support Level**: Community-driven (IETF Standards Track)  
**Security Patches**: ASAP  
**Feature Requests**: Via Issues  
**Bug Reports**: Via Issues  

## License

**License Type**: MIT  
**License File**: [LICENSE](./LICENSE)  
**SPDX Identifier**: MIT  

See LICENSE file for full terms.

## Archive & Deprecation

(To be filled if/when relevant)

---

Last Updated: April 26, 2026  
Repository Manager: Arunkumar Subbiah
