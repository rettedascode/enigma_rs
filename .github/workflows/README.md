# GitHub Actions Workflows

This directory contains GitHub Actions workflows for the Enigma Simulator project.

## Workflows

### CI (`ci.yml`)
Main continuous integration workflow that runs on every push and pull request:

- **Test**: Runs tests on stable, beta, and nightly Rust versions
- **Clippy**: Performs linting with Clippy
- **Rustfmt**: Checks code formatting
- **Build**: Builds the project on Ubuntu, Windows, and macOS
- **Security Audit**: Runs `cargo audit` for security vulnerabilities
- **Documentation**: Generates and uploads documentation

### Release (`release.yml`)
Automated release workflow that triggers on version tags:

- Builds release binaries for all platforms
- Creates compressed archives
- Automatically creates GitHub releases with assets

### Quality (`quality.yml`)
Code quality checks that run weekly and on push/PR:

- **Complexity Analysis**: Uses `cargo-geiger` and `cargo-deny`
- **Test Coverage**: Generates coverage reports with `cargo-tarpaulin`
- **Benchmarks**: Runs performance benchmarks
- **Dead Code Detection**: Finds unused dependencies
- **License Check**: Verifies license compliance

### Auto Label (`label.yml`)
Automatically labels issues and pull requests based on content:

- Detects bug reports, feature requests, documentation updates
- Adds appropriate labels for GUI, CLI, performance, testing
- Sets priority levels based on content

## Usage

### Running Tests Locally
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run clippy
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check
```

### Creating a Release
1. Update version in `Cargo.toml`
2. Create and push a version tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. The release workflow will automatically create a GitHub release

### Dependencies
The project uses Dependabot to automatically update dependencies weekly.

## Configuration

### Secrets
No secrets are required for the current workflows.

### Environment Variables
- `CARGO_TERM_COLOR=always`: Enables colored output in Cargo commands

### Caching
All workflows use GitHub Actions caching to speed up builds:
- Cargo registry cache
- Cargo build cache
- Target directory cache

## Monitoring

You can monitor the status of workflows:
- In the GitHub repository's "Actions" tab
- Via status badges (can be added to README)
- Through notifications (if enabled)

## Troubleshooting

### Common Issues
1. **Build failures**: Check Rust version compatibility
2. **Test failures**: Ensure all tests pass locally first
3. **Formatting issues**: Run `cargo fmt` locally
4. **Clippy warnings**: Address all clippy suggestions

### Debugging
- Enable debug logging by setting `RUST_LOG=debug`
- Check workflow logs in the GitHub Actions tab
- Use `cargo test --verbose` for detailed test output
