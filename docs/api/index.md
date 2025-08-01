# API Documentation

This section contains information about lbxd's Rust modules and API.

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

## Local API Documentation

To generate and view the complete API documentation locally, run:

```bash
# Generate Rust documentation
cargo doc --no-deps --document-private-items

# Open in browser
cargo doc --open
```

This will generate detailed documentation for all public and private APIs, including examples and cross-references.
