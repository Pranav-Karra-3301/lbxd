# Installation

## Prerequisites

lbxd has minimal system requirements:

- **Python 3.8+** (automatically installed if missing)
- **Terminal** with Unicode support
- **Internet connection** for initial setup

**No additional requirements:**
- ❌ No Xcode installation needed
- ❌ No keychain setup required  
- ❌ No API key configuration needed (built-in defaults provided)
- ❌ No complex dependencies

lbxd works immediately after installation on all supported platforms.

## Quick Install

### One-Line Installation (Recommended)

```bash
# Unix/Linux/macOS
curl -sSL https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Pranav-Karra-3301/lbxd/main/install.ps1 | iex
```

This script will:
- ✅ Install Python 3 and pip (if needed)
- ✅ Install letterboxdpy dependency
- ✅ Install viu for enhanced image display
- ✅ Download and install the latest lbxd binary
- ✅ Set up PATH configuration

## Package Managers

### Homebrew (macOS/Linux)

```bash
# Add the tap
brew tap pranav-karra-3301/lbxd

# Install lbxd (includes all dependencies)
brew install lbxd
```

### Chocolatey (Windows)

```powershell
# Install lbxd (includes Python and dependencies)
choco install lbxd
```

### Winget (Windows)

```powershell
# Install via Windows Package Manager
winget install Pranav-Karra-3301.lbxd
```

### Cargo (All Platforms)

```bash
# Install from source (requires Rust)
cargo install lbxd

# Install dependencies manually
pip3 install letterboxdpy
cargo install viu  # Optional, for enhanced image display
```

## Manual Installation

### Download Prebuilt Binaries

1. Visit the [releases page](https://github.com/Pranav-Karra-3301/lbxd/releases)
2. Download the appropriate binary for your platform:
   - **Linux**: `lbxd-linux-x86_64.tar.gz`
   - **macOS Intel**: `lbxd-macos-x86_64.tar.gz`
   - **macOS Apple Silicon**: `lbxd-macos-aarch64.tar.gz`
   - **Windows**: `lbxd-windows-x86_64.exe.zip`

3. Extract and move to your PATH:

**Linux/macOS:**
```bash
# Extract the binary
tar -xzf lbxd-*.tar.gz

# Move to PATH
sudo mv lbxd /usr/local/bin/

# Make executable
chmod +x /usr/local/bin/lbxd
```

**Windows:**
```powershell
# Extract the zip file
# Move lbxd.exe to a folder in your PATH
# Or add the folder to your PATH environment variable
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd

# Run the installation script
./install.sh

# Or build manually
cargo build --release
cargo install --path .
```

## Dependencies

### Python Dependencies

lbxd requires `letterboxdpy` for Letterboxd integration:

```bash
# Install with pip
pip3 install letterboxdpy

# Or use the system package manager
# Ubuntu/Debian
sudo apt install python3-pip
pip3 install letterboxdpy

# macOS with Homebrew
brew install python@3.12
pip3 install letterboxdpy

# Windows
python -m pip install letterboxdpy
```

### Optional: Enhanced Image Display

For the best experience, install `viu` for terminal image display:

```bash
# Using Cargo
cargo install viu

# Using Homebrew (macOS/Linux)
brew install viu

# Using package managers
# Ubuntu/Debian
sudo apt install viu

# Arch Linux
sudo pacman -S viu

# Windows with Scoop
scoop install viu

# Windows with Chocolatey
choco install viu
```

**Note**: If `viu` is not available, lbxd will automatically use ASCII art mode.

## Platform-Specific Instructions

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

**Option 1: Chocolatey (Recommended)**
```powershell
# Install Chocolatey if not present
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install lbxd
choco install lbxd
```

**Option 2: Winget**
```powershell
winget install Pranav-Karra-3301.lbxd
```

**Option 3: Manual Installation**
1. Install [Python 3](https://python.org/downloads/)
2. Download the Windows binary from [releases](https://github.com/Pranav-Karra-3301/lbxd/releases)
3. Extract and add to PATH

## Docker

```bash
# Run lbxd in Docker
docker run --rm -it ghcr.io/pranav-karra-3301/lbxd:latest recent username

# Build Docker image locally
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd
docker build -t lbxd .
docker run --rm -it lbxd recent username
```

## Verification

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

# Check Python dependency
python3 -c "import letterboxdpy; print('letterboxdpy is available')"
```
