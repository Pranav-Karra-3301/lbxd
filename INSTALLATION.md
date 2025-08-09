# 📦 lbxd Installation Guide

Complete guide for installing lbxd across all platforms and package managers.

## 🚀 Quick Installation

### One-Line Install (Recommended)

**Unix/Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.ps1 | iex
```

This automatically installs all dependencies and the latest version of lbxd.

---

## 📋 Prerequisites

- **viu** for terminal image display
- **Modern terminal** with Unicode support
- **Internet connection** for initial setup

**Optional but recommended:**
- `viu` for enhanced terminal image display
- `curl` for network requests

---

## 🍺 Package Managers

### Homebrew (macOS/Linux)

```bash
# Add the tap
brew tap pranav-karra-3301/lbxd

# Install lbxd (includes all dependencies)
brew install lbxd

# Verify installation
lbxd --version
```

**Formula maintained at:** [homebrew-lbxd](https://github.com/Pranav-Karra-3301/homebrew-lbxd)

### Chocolatey (Windows)

```powershell
# Install lbxd (includes all dependencies)
choco install lbxd

# Verify installation
lbxd --version
```

### Winget (Windows)

```powershell
# Install via Windows Package Manager
winget install Pranav-Karra-3301.lbxd

# Verify installation
lbxd --version
```

### Cargo (All Platforms)

```bash
# Install from source (requires Rust)
cargo install lbxd

# Install dependencies manually
cargo install viu  # Required for image display

# Verify installation
lbxd --version
```

---

## 📥 Prebuilt Binaries

Download prebuilt binaries from the [releases page](https://github.com/Pranav-Karra-3301/lbxd/releases):

### Linux (x86_64)
```bash
# Download and extract
curl -L -o lbxd-linux-x86_64.tar.gz https://github.com/Pranav-Karra-3301/lbxd/releases/latest/download/lbxd-linux-x86_64.tar.gz
tar -xzf lbxd-linux-x86_64.tar.gz

# Move to PATH
sudo mv lbxd /usr/local/bin/
chmod +x /usr/local/bin/lbxd

# Install viu (required)
brew install viu
```

### macOS (Intel)
```bash
# Download and extract
curl -L -o lbxd-macos-x86_64.tar.gz https://github.com/Pranav-Karra-3301/lbxd/releases/latest/download/lbxd-macos-x86_64.tar.gz
tar -xzf lbxd-macos-x86_64.tar.gz

# Move to PATH
sudo mv lbxd /usr/local/bin/
chmod +x /usr/local/bin/lbxd

# Install viu (required)
brew install viu
```

### macOS (Apple Silicon)
```bash
# Download and extract
curl -L -o lbxd-macos-aarch64.tar.gz https://github.com/Pranav-Karra-3301/lbxd/releases/latest/download/lbxd-macos-aarch64.tar.gz
tar -xzf lbxd-macos-aarch64.tar.gz

# Move to PATH
sudo mv lbxd /usr/local/bin/
chmod +x /usr/local/bin/lbxd

# Install viu (required)
brew install viu
```

### Windows (x86_64)
```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/Pranav-Karra-3301/lbxd/releases/latest/download/lbxd-windows-x86_64.exe.zip" -OutFile "lbxd-windows.zip"
Expand-Archive -Path "lbxd-windows.zip" -DestinationPath "C:\Program Files\lbxd\"

# Add to PATH (restart terminal after)
$env:PATH += ";C:\Program Files\lbxd"

# Install viu (required)
cargo install viu
```

---

## 🔨 Building from Source

### Prerequisites
- **Rust 1.88.0+** ([install here](https://rustup.rs/))
- **Git**
- **viu** for image display

### Basic Build
```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd

# Install viu (required)
brew install viu

# Install viu for enhanced image display (optional)
cargo install viu

# Build and install lbxd
cargo build --release
cargo install --path .

# Verify installation
lbxd --version
```

### Development Build
```bash
# Clone and enter directory
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd

# Install viu if not already installed
cargo install viu

# Run in development mode
cargo run -- --help

# Run tests
cargo test

# Build optimized release
cargo build --release
```

### Cross-compilation
```bash
# Install target for cross-compilation
rustup target add x86_64-unknown-linux-gnu

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu

# Available targets:
# - x86_64-unknown-linux-gnu (Linux x64)
# - x86_64-apple-darwin (macOS Intel)
# - aarch64-apple-darwin (macOS Apple Silicon)
# - x86_64-pc-windows-msvc (Windows x64)
```

---

## 🐳 Docker

```bash
# Run lbxd in Docker
docker run --rm -it ghcr.io/pranav-karra-3301/lbxd:latest recent username

# Build Docker image locally
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd
docker build -t lbxd .
docker run --rm -it lbxd recent username
```

---

## 🖥️ Platform-Specific Instructions

### Ubuntu/Debian
```bash
# Update package list
sudo apt update

# Install dependencies
sudo apt install curl python3 python3-pip

# Install lbxd
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash
```

### Arch Linux
```bash
# Install dependencies
sudo pacman -S curl python python-pip

# Install lbxd
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash
```

### CentOS/RHEL/Fedora
```bash
# CentOS/RHEL
sudo yum install curl python3 python3-pip

# Fedora
sudo dnf install curl python3 python3-pip

# Install lbxd
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash
```

### macOS
```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install lbxd via Homebrew (recommended)
brew tap pranav-karra-3301/lbxd
brew install lbxd

# Or use the install script
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash
```

### Windows
```powershell
# Install via Chocolatey (recommended)
choco install lbxd

# Or via Winget
winget install Pranav-Karra-3301.lbxd

# Or use PowerShell install script
irm https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.ps1 | iex
```

---

## ✅ Verification

After installation, verify that lbxd is working:

```bash
# Check version
lbxd --version

# Show help
lbxd --help

# Test with a quick command
lbxd recent letterboxd

# Check if viu is available (optional)
viu --version

# Check viu installation
viu --version
```

---

## 🔧 Dependencies

### Required
- **viu** for terminal image display
- **curl** for network requests

### Optional but Recommended
- **viu** for enhanced terminal image display
- **Modern terminal** with Unicode and ANSI color support

### No Setup Required
- ❌ No API keys needed (built-in defaults provided)
- ❌ No Xcode or complex dependencies
- ✅ Works immediately after installation

---

## 🛠️ Troubleshooting

### Common Issues

**viu not found:**
```bash
cargo install viu
# or on macOS: brew install viu
```

**Binary not in PATH:**
```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="$HOME/.local/bin:$PATH"
source ~/.bashrc  # or ~/.zshrc
```

**Permission denied (Linux/macOS):**
```bash
sudo chmod +x /usr/local/bin/lbxd
```

**Windows PATH issues:**
```powershell
# Add to PATH permanently
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\lbxd", "User")
```

### Getting Help

- 📖 **Documentation**: [GitHub Repository](https://github.com/Pranav-Karra-3301/lbxd)
- 🐛 **Report Issues**: [GitHub Issues](https://github.com/Pranav-Karra-3301/lbxd/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/Pranav-Karra-3301/lbxd/discussions)

---

## 🔄 Updating

### Via Package Managers
```bash
# Homebrew
brew update && brew upgrade lbxd

# Chocolatey
choco upgrade lbxd

# Winget
winget upgrade Pranav-Karra-3301.lbxd

# Cargo
cargo install lbxd --force
```

### Manual Update
```bash
# Download latest install script
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash
```

---

**Happy movie browsing! 🎬**
