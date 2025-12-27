# AGENTS.md - AI Coding Assistant Guide

> This document provides context for GitHub Copilot, Cursor, and other AI coding assistants working on the lbxd project.

---

## Project Overview

**lbxd** is a pure Rust CLI tool that brings Letterboxd to the terminal. Users can view movie activity, browse collections with an interactive TUI, search movies, and export data.

| Attribute | Value |
|-----------|-------|
| Language | Rust (100%) |
| Version | 3.0.0 |
| License | MIT |
| Platforms | Linux, macOS, Windows |

---

## Philosophy & Guidelines

### Core Values

1. **Safety First**: Every code suggestion must prioritize correctness over cleverness. Rust's type system is our ally—use it.

2. **Explain Your Work**: When generating code, include brief comments explaining *why* something is done, not just *what*. Future maintainers need context.

3. **Pure Rust**: No Python, no shell scripts for core functionality. All features must be implemented in Rust.

4. **Terminal Excellence**: This is a CLI tool. Every feature should be keyboard-accessible and work beautifully in the terminal.

5. **Zero Configuration**: The tool should work immediately after installation. No mandatory API keys or setup.

### Code Generation Guidelines

When generating code for this project:

```rust
// DO: Use Result types with proper error handling
pub async fn fetch_user(username: &str) -> anyhow::Result<User> {
    let response = client.get(&url).send().await?;
    let user: User = response.json().await?;
    Ok(user)
}

// DON'T: Use unwrap() in production code
pub async fn fetch_user(username: &str) -> User {
    let response = client.get(&url).send().await.unwrap(); // BAD!
    response.json().await.unwrap() // BAD!
}
```

```rust
// DO: Handle all error cases gracefully
match result {
    Ok(data) => process(data),
    Err(e) => eprintln!("Error: {}", e),
}

// DON'T: Panic on errors
result.expect("this should never fail"); // BAD in prod!
```

---

## Architecture

```
src/
├── main.rs              # Entry point, command dispatch
├── lib.rs               # Library root
├── cli/mod.rs           # clap command definitions
├── tui/                 # Interactive terminal UI
├── display/mod.rs       # Output formatting
├── letterboxd_client_rust/  # Letterboxd API
├── tmdb/mod.rs          # TMDB API client
├── omdb/mod.rs          # OMDB API client
├── feed/mod.rs          # RSS parsing
├── models/mod.rs        # Data structures
├── cache/mod.rs         # Caching system
├── config/mod.rs        # User configuration
└── export/mod.rs        # Data export
```

### Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI parsing with derive macros |
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `ratatui` | TUI framework |
| `serde` | Serialization |
| `anyhow` | Error handling |
| `rustboxd` | Letterboxd data access |

---

## Coding Standards

### Rust Style

```rust
// Module structure
mod feature_name {
    use crate::models::Movie;
    use anyhow::Result;

    pub struct FeatureName {
        // Fields
    }

    impl FeatureName {
        pub fn new() -> Self { /* ... */ }
        pub async fn execute(&self) -> Result<()> { /* ... */ }
    }
}
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Modules | snake_case | `letterboxd_client` |
| Structs | PascalCase | `ConfigManager` |
| Functions | snake_case | `fetch_recent_activity` |
| Constants | SCREAMING_SNAKE | `DEFAULT_CACHE_TTL` |

### Async Patterns

```rust
// Concurrent operations
let (user, movies) = tokio::join!(
    fetch_user(username),
    fetch_movies(username)
);

// Sequential with error handling
let user = fetch_user(username).await?;
let movies = fetch_movies(&user.id).await?;
```

---

## Safety Rules

### NEVER Generate Code That:

1. **Uses `unwrap()` or `expect()` without justification**
   - Always use `?` operator or proper match handling

2. **Hardcodes secrets or API keys**
   - Use environment variables: `std::env::var("API_KEY")`

3. **Uses `unsafe` blocks**
   - Unless absolutely necessary with detailed justification

4. **Ignores errors silently**
   - Every error must be propagated or logged

5. **Uses blocking I/O in async context**
   - Use `tokio::fs` instead of `std::fs` in async functions

6. **Stores sensitive data in plain text**
   - No passwords, tokens, or personal data in files

7. **Makes unbounded network requests**
   - Always implement timeouts and rate limiting

### ALWAYS Generate Code That:

1. **Validates input** - Check usernames, paths, numeric ranges
2. **Handles network failures** - Networks are unreliable
3. **Uses HTTPS** - Never allow HTTP for API calls
4. **Implements timeouts** - Prevent hanging operations
5. **Is cross-platform** - Test paths, file operations
6. **Is documented** - At minimum, doc comments for public API

---

## Development Workflow

### Building

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo run -- [args]      # Run with args
```

### Testing

```bash
cargo test               # Run tests
cargo test --verbose     # Verbose output
cargo test test_name     # Specific test
```

### Quality Checks

```bash
cargo fmt                # Format code
cargo clippy             # Lint code
cargo clippy -- -D warnings  # Strict mode
```

---

## Common Tasks

### Adding a CLI Command

1. Add to `cli/mod.rs`:
```rust
#[derive(Subcommand)]
pub enum Commands {
    // ...existing commands...

    /// New command description
    NewCommand {
        /// Argument description
        #[arg(short, long)]
        arg_name: String,
    },
}
```

2. Handle in `main.rs`:
```rust
Commands::NewCommand { arg_name } => {
    new_command::execute(&arg_name).await?;
}
```

### Adding an API Client

```rust
// src/new_api/mod.rs
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

pub struct NewApiClient {
    client: Client,
    base_url: String,
}

impl NewApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.example.com".to_string(),
        }
    }

    pub async fn fetch_data(&self, id: &str) -> Result<Data> {
        let url = format!("{}/data/{}", self.base_url, id);
        let response = self.client.get(&url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API error: {}", response.status());
        }

        let data: Data = response.json().await?;
        Ok(data)
    }
}
```

---

## Commit Message Format

Use conventional commits:

```
feat: add new feature description
fix: resolve issue with X
refactor: restructure Y module
docs: update Z documentation
test: add tests for W
chore: update dependencies
```

---

## Forbidden Patterns

```rust
// BAD: Panicking on errors
fn process(data: Option<Data>) -> Data {
    data.unwrap()  // NEVER in production
}

// BAD: Ignoring results
fn save(data: &Data) {
    let _ = file.write_all(bytes);  // Error ignored!
}

// BAD: Unbounded recursion
fn deep_process(items: Vec<Item>) {
    for item in items {
        deep_process(item.children);  // No depth limit!
    }
}

// BAD: SQL/Command injection
fn query(user_input: &str) {
    let cmd = format!("SELECT * FROM users WHERE name = '{}'", user_input);
    // Direct string interpolation = injection risk
}
```

---

## Preferred Patterns

```rust
// GOOD: Proper error handling
fn process(data: Option<Data>) -> Result<Data> {
    data.ok_or_else(|| anyhow::anyhow!("Data not found"))
}

// GOOD: Handling write results
fn save(data: &Data) -> Result<()> {
    file.write_all(bytes)?;
    file.flush()?;
    Ok(())
}

// GOOD: Bounded recursion
fn deep_process(items: Vec<Item>, depth: usize) -> Result<()> {
    if depth > MAX_DEPTH {
        anyhow::bail!("Max depth exceeded");
    }
    for item in items {
        deep_process(item.children, depth + 1)?;
    }
    Ok(())
}

// GOOD: Parameterized queries (when using SQL)
fn query(user_input: &str) -> Result<Vec<User>> {
    sqlx::query_as("SELECT * FROM users WHERE name = ?")
        .bind(user_input)
        .fetch_all(&pool)
        .await
}
```

---

## Project Goals

### What lbxd IS:
- A beautiful terminal tool for Letterboxd
- A pure Rust implementation
- Fast, reliable, cross-platform
- Works offline with caching

### What lbxd is NOT:
- A web application
- A mobile app
- A Letterboxd replacement
- A data mining tool

---

## Quick Reference

| Need | Solution |
|------|----------|
| Error handling | `anyhow::Result<T>` |
| HTTP requests | `reqwest` with async/await |
| CLI parsing | `clap` with derive |
| Terminal UI | `ratatui` |
| JSON handling | `serde_json` |
| Async runtime | `tokio` |
| Date/Time | `chrono` |

---

*When in doubt, prioritize code safety and readability over cleverness.*
