# Changelog

All notable changes to lbxd will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2025-08-09

### 🚨 BREAKING CHANGES

This is a major release that completely removes Python dependencies and transitions to a pure Rust implementation.

### Changed
- **Replaced letterboxdpy with rustboxd**: Complete migration from Python-based letterboxdpy to native Rust implementation (rustboxd)
- **viu is now required**: Terminal image display via viu is no longer optional - it's a required dependency
- **Improved performance**: Native Rust implementation provides better performance and reliability
- **Simplified installation**: No more Python or pip requirements, reducing installation complexity

### Removed
- All Python dependencies (letterboxdpy, Python 3.8+ requirement)
- ASCII art implementation and all related code
- `--ascii` CLI flags from all commands
- Python installation steps from install scripts

### Added
- rustboxd as the native Rust library for Letterboxd data access
- Enhanced error handling and async operations
- Better integration with the Rust ecosystem

### Migration Guide

If you're upgrading from v2.x to v3.0.0:

1. **Remove Python dependencies** (no longer needed):
   ```bash
   # You can uninstall letterboxdpy if you don't need it for other projects
   pip3 uninstall letterboxdpy
   ```

2. **Install viu** (now required):
   ```bash
   # macOS
   brew install viu
   
   # Or using cargo
   cargo install viu
   ```

3. **Update lbxd**:
   ```bash
   # If using Homebrew
   brew upgrade lbxd
   
   # Or build from source
   git pull
   cargo install --path . --force
   ```

### Technical Details

- Uses rustboxd library for all Letterboxd data scraping
- Async/await patterns throughout for better performance
- Native HTML parsing with the scraper crate
- Improved error handling and recovery

## [2.2.3] - Previous Release

Last version before the major rustboxd migration.

---

For more details, see the [GitHub releases page](https://github.com/Pranav-Karra-3301/lbxd/releases).