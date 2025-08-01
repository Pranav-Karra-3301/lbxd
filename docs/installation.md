# Installation

## Prerequisites

Before installing lbxd, ensure you have:

- **Rust**: Version 1.88.0 or later
- **Python 3**: For Letterboxd data integration  
- **Terminal**: UTF-8 and ANSI color support recommended

## Installation Methods

### Homebrew (Recommended)

The easiest way to install lbxd on macOS and Linux:

```bash
# Add the tap
brew tap pranav-karra-3301/lbxd

# Install lbxd
brew install lbxd
```

### From Source

If you prefer to build from source or want the latest development version:

```bash
# Clone the repository
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd

# Build and install
cargo build --release
cargo install --path .
```

### Package Managers

#### Windows (Chocolatey)

```powershell
choco install lbxd
```

#### Windows (Winget)

```powershell
winget install lbxd
```

#### Debian/Ubuntu

Download the `.deb` package from the [releases page](https://github.com/Pranav-Karra-3301/lbxd/releases) and install:

```bash
sudo dpkg -i lbxd_*.deb
```

## Verification

After installation, verify that lbxd is working correctly:

```bash
lbxd --version
```

You should see the version number displayed, confirming the installation was successful.
