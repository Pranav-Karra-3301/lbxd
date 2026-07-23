<p align="center">
  <img src="lbxd.png" alt="lbxd" width="200"/>
</p>


[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/v/release/Pranav-Karra-3301/lbxd)](https://github.com/Pranav-Karra-3301/lbxd/releases)
[![Crates.io](https://img.shields.io/crates/v/lbxd.svg)](https://crates.io/crates/lbxd)

**lbxd** is a beautiful command-line tool written in Rust that brings Letterboxd to your terminal. View any user's activity, browse collections interactively, and explore movie data with rich terminal displays.

[Website](https://lbxd.pranavkarra.me) · [crates.io](https://crates.io/crates/lbxd) · [Docs](https://docs.rs/lbxd) · [Installation](INSTALLATION.md)

```
    ██╗     ██████╗ ██╗  ██╗██████╗ 
    ██║     ██╔══██╗╚██╗██╔╝██╔══██╗
    ██║     ██████╔╝ ╚███╔╝ ██║  ██║
    ██║     ██╔══██╗ ██╔██╗ ██║  ██║
    ███████╗██████╔╝██╔╝ ██╗██████╔╝
    ╚══════╝╚═════╝ ╚═╝  ╚═╝╚═════╝ 
        Letterboxd in your terminal
```

## ✨ Features

- 🎭 **Interactive TUI**: Browse complete movie collections with a beautiful terminal interface
- 📽️ **Recent Activity**: View any user's recent movies, ratings, and reviews
- 🔍 **Search**: Find specific titles in user activity history
- 🎬 **Movie Database**: Search TMDB for detailed movie information
- 📤 **Export**: Export user data to JSON or Markdown formats
- ⚙️ **Configuration**: Persistent settings and user preferences
- 💾 **Caching**: Offline access with intelligent data caching
- 🎨 **Beautiful Display**: Rich terminal output with ASCII art and colors
- ⚡ **Fast**: Built in Rust for maximum performance and reliability

## 🚀 Installation

### Quick Install (Recommended)

**One-line installation:**
```bash
# Unix/Linux/macOS
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.ps1 | iex
```

> 📖 **For detailed installation instructions across all platforms and package managers, see [INSTALLATION.md](INSTALLATION.md)**

### Package Managers

**Cargo (crates.io):**
```bash
cargo install lbxd
```

**Homebrew (macOS/Linux):**
```bash
brew tap pranav-karra-3301/lbxd
brew install lbxd
```
*Formula maintained at: [homebrew-lbxd](https://github.com/Pranav-Karra-3301/homebrew-lbxd)*

**Chocolatey (Windows):**
```powershell
choco install lbxd
```

**Winget (Windows):**
```powershell
winget install Pranav-Karra-3301.lbxd
```

### Building from Source

**Prerequisites:**
- Rust 1.88.0 or later
- Git
- viu (required for poster display)

**Build steps:**
```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd

# Install viu (required)
# macOS: brew install viu
# Cargo: cargo install viu

# Install viu if not already installed
cargo install viu

# Build and install lbxd
cargo build --release
cargo install --path .

# Verify installation
lbxd --version
```

**Development build:**
```bash
# Clone and enter directory
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd

# Install dependencies
pip3 install letterboxdpy

# Run in development mode
cargo run -- --help

# Run tests
cargo test

# Build optimized release
cargo build --release
```

### Prebuilt Binaries

Download prebuilt binaries from the [releases page](https://github.com/Pranav-Karra-3301/lbxd/releases):

- **Linux**: `lbxd-linux-x86_64.tar.gz`
- **macOS Intel**: `lbxd-macos-x86_64.tar.gz`
- **macOS Apple Silicon**: `lbxd-macos-aarch64.tar.gz`
- **Windows**: `lbxd-windows-x86_64.exe.zip`

### System Dependencies

**Required:**
- `viu` for terminal image display

**Built-in:**
- rustboxd for Letterboxd data access
- Modern terminal with Unicode support

**No setup required:**
- ❌ No API keys needed (built-in defaults provided)
- ❌ No Xcode or complex dependencies
- ✅ Works immediately after installation

## 📖 Usage

### Quick Start

```bash
# Show version and help
lbxd

# Browse a user's collection interactively
lbxd browse username

# Show profile stats  
lbxd username
```

### Commands

#### Interactive Browsing
```bash
# Launch interactive TUI for browsing complete collections
lbxd browse username
```

#### Recent Activity
```bash
# Show recent activity for a user
lbxd recent username

# Limit to 10 most recent entries
lbxd recent username --limit 10

# Show only rated films
lbxd recent username --rated

# Show only reviewed films  
lbxd recent username --reviewed

# Filter by specific date
lbxd recent username --date 2024-01-15
```

#### Search
```bash
# Search for movies in user's activity
lbxd search username "blade runner"

# Search TMDB movie database
lbxd movie "dune 2021"
```

#### Data Export
```bash
# Export to JSON
lbxd export username --format json --output data.json

# Export to Markdown
lbxd export username --format markdown --output report.md
```

#### Configuration
```bash
# Show current settings
lbxd config show

# Set default username
lbxd config set-user myusername

# Check saved username
lbxd config whoami
```

## 🎨 Output Examples

### Recent Activity
```
════════════════════════════════════════════════════════════
📽️  username Activity
════════════════════════════════════════════════════════════

  🎬 Blade Runner 2049 (2017)
  ⭐ ★★★★★ (5.0/5)
  ♥ Liked
  💭 A stunning sequel that honors the original while expanding the universe...
  📅 January 15, 2024

──────────────────────────────────────────────────────────

  🎬 Dune (2021)
  ⭐ ★★★★☆ (4.5/5)
  💭 Villeneuve's adaptation is visually breathtaking...
  📅 January 14, 2024
```

## ⚙️ Configuration

lbxd automatically creates a cache directory at `~/.cache/lbxd/` to store user data for offline access. Cache entries expire after 6 hours to ensure fresh data.

### API Keys

lbxd comes with built-in API keys for TMDB and OMDB, so it works out of the box without any configuration. If you want to use your own API keys, you can set environment variables:

```bash
# Optional: Use your own TMDB API key
export TMDB_API_KEY="your_tmdb_api_key_here"

# Optional: Use your own OMDB API key  
export OMDB_API_KEY="your_omdb_api_key_here"
```

For detailed API key configuration, see the [API Keys Guide](docs/api-keys.md).

### Quick Examples

```bash
# Works immediately with default keys
lbxd recent username

# Optional: Use your own TMDB API key
export TMDB_API_KEY="your_key_here"
lbxd movie "Inception"

# See examples/api-key-demo.sh for more configuration options
```

## 🛠️ Development

### Building from Source

**Prerequisites:**
- Rust 1.88.0 or later
- Git
- viu (required for poster display)

**Development setup:**
```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd

# Install viu (required)
# macOS: brew install viu
# Cargo: cargo install viu

# Install viu if not already installed
cargo install viu

# Run in development mode
cargo run -- --help

# Run tests
cargo test

# Run linting
cargo clippy

# Format code
cargo fmt

# Build optimized release
cargo build --release
```

**Project Structure:**
```
src/
├── main.rs              # CLI entry point and version display
├── lib.rs               # Library root
├── cli/                 # Command-line interface definitions
├── tui/                 # Interactive terminal user interface
├── display/             # Terminal output and styling
├── letterboxd_client/   # Letterboxd data integration
├── tmdb/                # The Movie Database API client
├── omdb/                # Open Movie Database integration
├── feed/                # RSS feed parsing
├── models/              # Data structures and types
├── cache/               # Intelligent caching system
├── config/              # Configuration management
└── export/              # Data export functionality
```

**Environment Variables:**
```bash
# Optional: Use your own API keys
export TMDB_API_KEY="your_tmdb_api_key_here"
export OMDB_API_KEY="your_omdb_api_key_here"

# Development logging
export RUST_LOG=debug
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Letterboxd](https://letterboxd.com/) for providing RSS feeds and public data
- [The Movie Database (TMDB)](https://www.themoviedb.org/) for comprehensive movie information
- [Open Movie Database (OMDB)](http://www.omdbapi.com/) for additional movie metadata
- The Rust community for excellent crates and development tools
- rustboxd - Native Rust library for Letterboxd data (built-in)

## 📞 Support

If you encounter any issues or have questions:

- 🐛 [Report bugs](https://github.com/Pranav-Karra-3301/lbxd/issues)
- 💡 [Request features](https://github.com/Pranav-Karra-3301/lbxd/issues)

---

Built by [Pranav Karra](https://pranavkarra.me)
