# CLAUDE.md - Claude Code Project Guide

> This document provides Claude Code with comprehensive context for working on the lbxd project.

## Project Identity

**lbxd** is a beautiful command-line tool written in pure Rust that brings Letterboxd (the social network for film lovers) to your terminal. It enables users to view movie activity, browse collections interactively, search for movies, and export data—all from the command line.

**Repository**: https://github.com/Pranav-Karra-3301/lbxd
**License**: MIT
**Current Version**: 3.0.0

## Project Philosophy

### Core Principles

1. **Pure Rust, No Compromises**: The project is 100% Rust with zero Python dependencies. This ensures maximum performance, type safety, and cross-platform reliability.

2. **Terminal-First Experience**: Every feature is designed for the terminal. Rich TUI interfaces, ASCII art, colorful output, and inline poster display create a delightful experience.

3. **Works Out of the Box**: No API key configuration required. Built-in defaults for TMDB and OMDB APIs mean users can install and immediately use the tool.

4. **Safety Over Speed**: Code correctness and user data safety take precedence over raw performance. Every async operation handles errors gracefully.

5. **Minimal Dependencies**: Only include dependencies that are absolutely necessary. Each crate added must justify its inclusion.

## Architecture Overview

```
src/
├── main.rs                          # CLI entry point, command dispatcher
├── lib.rs                           # Library exports, module declarations
├── cli/mod.rs                       # clap-based CLI definitions
├── tui/                             # Terminal UI (ratatui-based)
│   ├── mod.rs                       # Event loop, keyboard handling
│   ├── app.rs                       # Application state
│   ├── grid.rs                      # Grid rendering for collections
│   ├── styles.rs                    # Color schemes and styling
│   └── progress.rs                  # Progress indicators
├── display/mod.rs                   # Terminal output formatting
├── letterboxd_client_rust/mod.rs    # rustboxd wrapper
├── tmdb/mod.rs                      # TMDB API client
├── omdb/mod.rs                      # OMDB API client
├── feed/mod.rs                      # RSS feed parser
├── models/mod.rs                    # Data structures (Movie, UserEntry, etc.)
├── profile/mod.rs                   # User profile and statistics
├── cache/mod.rs                     # 6-hour TTL caching system
├── config/mod.rs                    # Configuration management
├── export/mod.rs                    # JSON/Markdown export
├── onboarding/mod.rs                # Interactive setup wizard
├── batch_loader.rs                  # Concurrent data loading
└── viu/mod.rs                       # Terminal image display
```

## Tech Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| Language | Rust 1.88+ | Core implementation |
| CLI Framework | clap 4.5 | Command-line parsing with derive macros |
| Async Runtime | tokio | Async I/O and concurrency |
| HTTP Client | reqwest | API requests |
| TUI Framework | ratatui + crossterm | Interactive terminal UI |
| Serialization | serde + serde_json | Data serialization |
| Error Handling | anyhow | Ergonomic error handling |
| Letterboxd | rustboxd | Native Rust Letterboxd library |
| Date/Time | chrono | Date handling |
| HTML Parsing | scraper | Web scraping |

### External Dependencies

- **viu**: Required for terminal image display (posters)
- **TMDB API**: Movie search and metadata (built-in key)
- **OMDB API**: IMDb ratings (built-in key)

## Development Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Production build

# Run
cargo run -- [args]            # Run with arguments
cargo run -- recent username   # Example: show recent activity

# Test
cargo test                     # Run all tests
cargo test --verbose           # Verbose test output

# Lint & Format
cargo clippy                   # Run linter
cargo clippy -- -D warnings    # Strict linting (CI mode)
cargo fmt                      # Format code
cargo fmt -- --check           # Check formatting (CI mode)

# Install
cargo install --path .         # Install locally
```

## Code Style Guidelines

### Rust Conventions

1. **Use `anyhow::Result<T>`** for error handling in application code
2. **Use `#[derive(...)]`** generously for common traits
3. **Prefer `async/await`** for all I/O operations
4. **Use `tokio::spawn`** for concurrent operations
5. **Keep functions small** - single responsibility principle

### Naming Conventions

- **Modules**: `snake_case` (e.g., `letterboxd_client`)
- **Types**: `PascalCase` (e.g., `ConfigManager`)
- **Functions**: `snake_case` (e.g., `fetch_recent_activity`)
- **Constants**: `SCREAMING_SNAKE_CASE`
- **File names**: Match module names

### Code Organization

- One module per file/directory
- Keep related functionality together
- Public API at module root, private helpers in submodules
- Use `mod.rs` for directory modules

## Safety Guidelines

### NEVER Do These

1. **Never hardcode user credentials** - Use environment variables or config files
2. **Never commit API keys** - Even "test" keys can be exploited
3. **Never use `unwrap()` in production paths** - Use proper error handling
4. **Never ignore async errors** - Always propagate or log them
5. **Never use `unsafe` blocks** - Unless absolutely necessary with justification
6. **Never bypass rate limits** - Respect API provider limits
7. **Never store plaintext secrets** - Use proper secret management

### ALWAYS Do These

1. **Always validate user input** - Especially usernames and file paths
2. **Always handle HTTP errors gracefully** - Network can fail
3. **Always respect cache TTL** - Don't hammer APIs unnecessarily
4. **Always use HTTPS** - Never downgrade to HTTP
5. **Always test on all platforms** - Linux, macOS, Windows
6. **Always run clippy before commits** - Fix all warnings

## Git Workflow

### Branch Naming

- `main` - Stable release branch
- `feature/description` - New features
- `fix/description` - Bug fixes
- `refactor/description` - Code refactoring

### Commit Messages

Use conventional commits:
```
feat: add watchlist export command
fix: handle empty user profiles gracefully
refactor: extract cache logic into dedicated module
docs: update installation instructions
test: add integration tests for TMDB client
chore: update dependencies
```

### Before Committing

```bash
cargo fmt                      # Format code
cargo clippy -- -D warnings    # Check for issues
cargo test                     # Run tests
```

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Commit: `git commit -m "chore: bump version to X.Y.Z"`
4. Tag: `git tag vX.Y.Z`
5. Push: `git push && git push --tags`

The CI/CD pipeline will automatically:
- Build for all platforms (Linux, macOS, Windows)
- Create GitHub release with binaries
- Update package managers (Homebrew, Chocolatey, Winget)

## Common Patterns

### Adding a New CLI Command

1. Add command variant to `cli/mod.rs`
2. Add handler function in appropriate module
3. Add match arm in `main.rs`
4. Add tests
5. Update documentation

### Adding an API Client

1. Create new module in `src/`
2. Define response types in `models/mod.rs`
3. Implement async fetch methods
4. Add error handling for API failures
5. Integrate with caching system

### Modifying the TUI

1. Update state in `tui/app.rs`
2. Modify rendering in `tui/mod.rs` or `tui/grid.rs`
3. Update key bindings if needed
4. Test with various terminal sizes

## Testing

### Test Categories

1. **Unit Tests**: In-module tests for isolated logic
2. **Integration Tests**: Full command execution tests
3. **Smoke Tests**: Basic compilation and instantiation

### Running Specific Tests

```bash
cargo test test_name           # Run specific test
cargo test module::            # Run module tests
cargo test -- --nocapture      # Show println output
```

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -- recent username
RUST_LOG=lbxd=trace cargo run -- browse username
```

### Common Issues

1. **viu not found**: Install with `cargo install viu`
2. **Network errors**: Check internet connection, API availability
3. **Cache issues**: Clear `~/.cache/lbxd/`
4. **Terminal rendering**: Ensure terminal supports Unicode

## CI/CD

### GitHub Actions Workflows

1. **ci.yml**: Runs on push/PR
   - Tests on Ubuntu, Windows, macOS
   - Runs fmt, clippy, test, build
   - Tests stable and beta Rust

2. **release.yml**: Runs on version tags
   - Builds release binaries
   - Creates GitHub release
   - Uploads platform-specific archives

## Documentation

- **README.md**: User-facing documentation
- **INSTALLATION.md**: Detailed installation guide
- **CHANGELOG.md**: Version history
- **docs/**: Sphinx documentation (ReadTheDocs)

## Questions to Ask Before Making Changes

1. Does this change maintain backward compatibility?
2. Have I tested on all supported platforms?
3. Does this introduce new dependencies? Are they justified?
4. Is the error handling comprehensive?
5. Is the code readable without excessive comments?
6. Does this follow the existing patterns in the codebase?

## Project Goals

### Short Term
- Maintain stability and cross-platform compatibility
- Improve test coverage
- Enhance TUI responsiveness

### Long Term
- Add more Letterboxd features (lists, reviews, recommendations)
- Improve offline mode capabilities
- Add plugin system for extensibility

---

*This document should be updated whenever significant architectural changes are made.*
