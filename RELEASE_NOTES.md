# lbxd v3.0.0 - Native Rust Implementation

## 🚨 Breaking Changes

This is a major release that completely removes Python dependencies and transitions to a pure Rust implementation.

## What's Changed

### ✨ Major Changes
- **Replaced letterboxdpy with rustboxd** - Complete migration to native Rust implementation
- **No Python required** - Removed all Python dependencies 
- **viu is now required** - Terminal image display via viu is mandatory
- **Improved performance** - Native Rust provides better speed and reliability
- **Simplified installation** - No more pip or Python setup needed

### 🗑️ Removed
- All Python dependencies (letterboxdpy, Python 3.8+ requirement)
- ASCII art implementation and related code
- `--ascii` CLI flags from all commands
- Python installation steps from scripts

### 📦 Migration Guide

**Upgrading from v2.x:**

1. Install viu (now required):
   ```bash
   # macOS
   brew install viu
   
   # Or using cargo
   cargo install viu
   ```

2. Update lbxd:
   ```bash
   # If using Homebrew
   brew upgrade lbxd
   
   # Or build from source
   git pull
   cargo install --path . --force
   ```

3. Python dependencies can be removed (no longer needed)

### 🔧 Technical Details
- Uses rustboxd library for Letterboxd data scraping
- Async/await patterns throughout
- Native HTML parsing with scraper crate
- Better error handling and recovery

## Installation

### Homebrew (macOS/Linux)
```bash
brew tap pranav-karra-3301/lbxd
brew install lbxd
```

### From Source
```bash
git clone https://github.com/Pranav-Karra-3301/lbxd
cd lbxd
cargo install --path .
```

### Pre-built Binaries
Download from the [releases page](https://github.com/Pranav-Karra-3301/lbxd/releases/tag/v3.0.0).

## Verification

After installation, verify everything is working:
```bash
# Check version
lbxd --version

# Verify viu is available
viu --version

# Test basic functionality
lbxd recent <username>
```

**Full Changelog**: https://github.com/Pranav-Karra-3301/lbxd/compare/v2.2.3...v3.0.0