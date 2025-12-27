# Add CLI Command

Create a new CLI command for lbxd.

## Required Information

The user should specify:
- Command name (e.g., "favorites")
- Description
- Required arguments
- Optional flags

## Implementation Steps

1. **Update cli/mod.rs**
   - Add the command variant to the `Commands` enum
   - Include doc comment and clap attributes

2. **Create handler module** (if complex)
   - New file in `src/` if substantial logic
   - Or inline in existing module if simple

3. **Update main.rs**
   - Add match arm to dispatch to handler

4. **Add tests**
   - Unit tests for the handler logic
   - Integration test for CLI parsing

## Template

```rust
// In cli/mod.rs
#[derive(Subcommand)]
pub enum Commands {
    /// [DESCRIPTION]
    CommandName {
        /// Username to query
        username: String,

        /// Optional limit
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

// In main.rs
Commands::CommandName { username, limit } => {
    command_name::execute(&username, limit).await?;
}
```
