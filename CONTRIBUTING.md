# Contributing to lbxd

Thanks for your interest in contributing!

## Quick Start

```bash
# Clone and setup
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd
cargo install viu  # Required for poster display

# Build and test
cargo build
cargo test
cargo clippy
cargo fmt --check
```

## Development Workflow

1. **Fork** the repository
2. **Create a branch** from `main`: `git checkout -b feature/your-feature`
3. **Make changes** and ensure tests pass
4. **Commit** with a clear message (see below)
5. **Push** and open a Pull Request

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: Add new command for X
fix: Resolve issue with Y
docs: Update usage examples
ci: Improve workflow caching
refactor: Simplify parsing logic
```

## Code Guidelines

- Run `cargo fmt` before committing
- Fix all `cargo clippy` warnings
- Add tests for new functionality
- Keep dependencies minimal
- Document public APIs

## Project Structure

```
src/
├── main.rs          # Entry point, command handling
├── cli/             # Clap command definitions
├── tui/             # Interactive terminal UI (ratatui)
├── display/         # Terminal output formatting
├── letterboxd_client/ # Letterboxd data fetching
├── tmdb/            # TMDB API client
├── models/          # Data structures
├── cache/           # Caching system
├── config/          # User configuration
└── export/          # JSON/Markdown/CSV export
```

## Testing

```bash
cargo test              # Run all tests
cargo test --verbose    # With output
cargo test <name>       # Specific test
```

## Questions?

Open an [issue](https://github.com/Pranav-Karra-3301/lbxd/issues) for questions or discussion.
