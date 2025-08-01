# API Documentation

This section contains the API documentation for lbxd's Rust modules.

## Overview

lbxd is built with a modular architecture in Rust. The main modules include:

- **CLI**: Command-line interface and argument parsing
- **Config**: Configuration management
- **Cache**: Data caching system
- **Display**: Terminal output formatting
- **Feed**: RSS feed processing
- **Letterboxd Client**: Letterboxd data integration
- **Models**: Data structures and types
- **OMDB/TMDB**: Movie database integrations
- **TUI**: Terminal user interface
- **Export**: Data export functionality

## Generated Documentation

The complete API documentation is generated from the Rust source code using `cargo doc`. 

You can find the detailed API documentation at: [API Reference](./rust_api/index.html)

## Key Modules

### CLI Module

Handles command-line argument parsing and command dispatch.

### Config Module

Manages application configuration, including:
- Loading and saving configuration files
- Environment variable handling
- Default value management

### Cache Module

Provides efficient data caching with:
- Configurable TTL (time-to-live)
- Size-based eviction
- Disk persistence

### Display Module

Handles terminal output formatting:
- Colored output
- ASCII art rendering
- Progress indicators
- Table formatting

### TUI Module

Interactive terminal user interface featuring:
- Grid-based movie browsing
- Keyboard navigation
- Real-time updates
- Responsive design

### Models Module

Core data structures for:
- Movie information
- User profiles
- Activity feeds
- Export formats

## Building Documentation

To build the complete API documentation locally:

```bash
# Generate Rust documentation
cargo doc --no-deps --document-private-items

# Open in browser
cargo doc --open
```

This will generate detailed documentation for all public and private APIs, including examples and cross-references.
