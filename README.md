# lbxd 🎬

[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**lbxd** is a beautiful command-line tool written in Rust that lets you view Letterboxd activity directly in your terminal. It fetches any user's public RSS feed and displays their recent movies, ratings, and reviews in clean, expressive ASCII art and stylized text.

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

- 📽️ **Recent Activity**: View a user's recent movies, ratings, and reviews
- 🔍 **Search**: Find specific titles in a user's activity
- 📊 **Comparisons**: Compare multiple users' movie preferences *(coming soon)*
- 📤 **Export**: Export data to JSON or Markdown formats
- 📈 **Summaries**: Generate viewing summaries and statistics *(coming soon)*
- 💾 **Caching**: Offline access with intelligent caching
- 🎨 **Beautiful Display**: Clean ASCII art and colorized terminal output
- ⚡ **Fast**: Built in Rust for maximum performance

## 🚀 Installation

### Prerequisites

- Rust 1.88.0 or later
- A terminal that supports UTF-8 and ANSI colors

### From Source

```bash
# Clone the repository
git clone https://github.com/pranavkarra/lbxd.git
cd lbxd

# Build and install
cargo install --path .
```

### Package Managers *(coming soon)*

```bash
# Homebrew (macOS/Linux)
brew install lbxd

# Chocolatey (Windows)
choco install lbxd

# Winget (Windows)
winget install lbxd

# APT (Ubuntu/Debian)
sudo apt install lbxd
```

## 📖 Usage

### Show Recent Activity

Display a user's recent Letterboxd activity:

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

### Search for Movies

Find specific titles in a user's activity:

```bash
# Search for movies containing "blade runner"
lbxd search username "blade runner"
```

### Export Data

Export user data to various formats:

```bash
# Export to JSON
lbxd export username --format json --output data.json

# Export to Markdown
lbxd export username --format markdown --output report.md
```

### Compare Users *(coming soon)*

Compare multiple users' movie preferences:

```bash
# Compare two or more users
lbxd compare user1 user2 user3
```

### Generate Summaries *(coming soon)*

Create viewing summaries and statistics:

```bash
# Generate summary for 2024
lbxd summary username --year 2024
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

## 🛠️ Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/pranavkarra/lbxd.git
cd lbxd

# Run tests
cargo test

# Build in release mode
cargo build --release

# Run with debug output
RUST_LOG=debug cargo run -- recent username
```

### Project Structure

```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Library root
├── cli/              # Command-line interface
├── display/          # Terminal output and styling
├── feed/             # RSS feed parsing
├── models/           # Data structures
├── cache/            # Caching system
└── export/           # Data export functionality
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

- [Letterboxd](https://letterboxd.com/) for providing RSS feeds
- The Rust community for excellent crates and tools
- All contributors who help improve this project

## 📞 Support

If you encounter any issues or have questions:

- 🐛 [Report bugs](https://github.com/pranavkarra/lbxd/issues)
- 💡 [Request features](https://github.com/pranavkarra/lbxd/issues)
- 📖 [Read the documentation](https://github.com/pranavkarra/lbxd/wiki)