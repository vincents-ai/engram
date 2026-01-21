# Release Process

This document describes the release process for Engram.

## Version Numbering

Engram uses [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

Current version: See [Cargo.toml](./Cargo.toml)

## Release Checklist

### Pre-Release

1. **Update CHANGELOG.md**
   - Add new version header with date
   - Document all changes since last release
   - Categorize: Added, Changed, Deprecated, Removed, Fixed, Security

2. **Update Version in Cargo.toml**
   ```bash
   # Edit version in Cargo.toml
   version = "X.Y.Z"
   ```

3. **Run Full Test Suite**
   ```bash
   cargo test --all-features
   cargo clippy --all-features
   cargo fmt --all -- --check
   ```

4. **Update Nix Flake** (if applicable)
   ```bash
   # Ensure flake.nix has correct version
   ```

### Creating a Release

1. **Create and Push Tag**
   ```bash
   # Create annotated tag
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   
   # Push tag
   git push origin vX.Y.Z
   ```

2. **GitHub Actions Will Automatically**:
   - Build release binaries
   - Create GitHub release
   - Upload artifacts (Linux binary, Nix package, checksums)

### Post-Release

1. **Verify Release**
   - Check GitHub release was created
   - Verify downloads work
   - Confirm checksums match

2. **Announce Release**
   - Update documentation if needed
   - Notify users (social media, newsletters)

## Distribution Channels

### GitHub Releases
- Binary: `engram-linux-amd64`
- Nix: `engram-nix`
- Checksums: SHA256 files

### Nix Package Manager
```bash
# Install latest release
nix run github:vincents-ai/engram

# Or install specific version
nix run github:vincents-ai/engram/vX.Y.Z
```

### Cargo
```bash
cargo install engram
```

### Homebrew (macOS)
```bash
brew install vincents-ai/engram/engram
```

## CI/CD Pipeline

### Workflows

#### CI (ci.yml)
- Runs on: push to main/develop, pull requests
- Jobs:
  - `test`: Unit tests, clippy, fmt
  - `build`: Release binary build
  - `nix-build`: Nix package build
  - `examples`: Verify all examples compile
  - `coverage`: Code coverage with tarpaulin

#### Release (release.yml)
- Triggered by: git tag push (`v*`)
- Jobs:
  - `release`: Build binaries, create GitHub release
  - `release-brew`: Update Homebrew formula
  - `notify`: Send notifications

#### Security Audit (security.yml)
- Triggered by: schedule (weekly), dependency changes
- Jobs:
  - `security-audit`: Run cargo audit
  - `report`: Create issues for vulnerabilities

### Required Secrets

For CI/CD to work, set these GitHub secrets:
- `HOMEBREW_TOKEN`: GitHub PAT for Homebrew updates
- `SLACK_WEBHOOK_URL`: Slack webhook for notifications (optional)

## Artifacts

Release artifacts include:
- `engram-linux-amd64`: Linux x86_64 binary
- `engram-nix`: Nix package
- `*-sha256`: Checksum files for verification

## Verification

Verify downloaded artifacts:
```bash
# Download checksums
curl -L https://github.com/vincents-ai/engram/releases/download/vX.Y.Z/engram-linux-amd64.sha256

# Verify
sha256sum -c engram-linux-amd64.sha256
sha256sum -c engram-nix.sha256
```

## Rollback Procedure

If a release has critical issues:

1. **Patch Release**
   ```bash
   # Create patch version
   git checkout vX.Y.Z
   # Fix the issue
   git commit -m "fix: critical issue in vX.Y.Z"
   git tag -a vX.Y.Z -m "Hotfix vX.Y.Z"
   git push origin vX.Y.Z
   ```

2. **GitHub Release Notes**
   - Mark broken release as "Latest" on GitHub releases page
   - Create new release as "Latest"

## Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Security**: Report vulnerabilities via GitHub Security Advisories
