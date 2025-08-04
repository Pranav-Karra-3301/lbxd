#!/bin/bash

set -e

echo "🎬 LBXD Installation Script"
echo "=========================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Detect OS and architecture
detect_os() {
    case "$(uname -s)" in
        Darwin*)
            OS="macos"
            case "$(uname -m)" in
                arm64|aarch64) ARCH="aarch64" ;;
                x86_64) ARCH="x86_64" ;;
                *) ARCH="x86_64" ;;
            esac
            ;;
        Linux*)
            OS="linux"
            case "$(uname -m)" in
                x86_64) ARCH="x86_64" ;;
                aarch64|arm64) ARCH="aarch64" ;;
                armv7l) ARCH="armv7" ;;
                *) ARCH="x86_64" ;;
            esac
            ;;
        CYGWIN*|MINGW*|MSYS*)
            OS="windows"
            ARCH="x86_64"
            ;;
        *)
            OS="unknown"
            ARCH="unknown"
            ;;
    esac
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Python 3 and pip
install_python() {
    print_info "Checking Python 3 installation..."
    
    if command_exists python3; then
        PYTHON_VERSION=$(python3 --version 2>&1 | cut -d" " -f2)
        print_status "Python 3 found: $PYTHON_VERSION"
    else
        print_warning "Python 3 not found. Installing..."
        
        case $OS in
            "macos")
                if command_exists brew; then
                    brew install python3
                else
                    print_error "Homebrew not found. Please install Python 3 manually from https://python.org"
                    exit 1
                fi
                ;;
            "linux")
                if command_exists apt; then
                    sudo apt update
                    sudo apt install -y python3 python3-pip python3-venv
                elif command_exists yum; then
                    sudo yum install -y python3 python3-pip
                elif command_exists dnf; then
                    sudo dnf install -y python3 python3-pip
                elif command_exists pacman; then
                    sudo pacman -S python python-pip
                elif command_exists zypper; then
                    sudo zypper install python3 python3-pip
                else
                    print_error "Package manager not found. Please install Python 3 manually."
                    exit 1
                fi
                ;;
            "windows")
                print_error "Please install Python 3 from https://python.org and ensure it's in your PATH"
                exit 1
                ;;
            *)
                print_error "Unsupported OS. Please install Python 3 manually."
                exit 1
                ;;
        esac
        
        print_status "Python 3 installed successfully"
    fi
    
    # Verify pip installation
    if ! command_exists pip3 && ! command_exists pip; then
        print_warning "pip not found. Installing pip..."
        
        case $OS in
            "macos")
                python3 -m ensurepip --upgrade
                ;;
            "linux")
                if command_exists apt; then
                    sudo apt install -y python3-pip
                elif command_exists yum; then
                    sudo yum install -y python3-pip
                elif command_exists dnf; then
                    sudo dnf install -y python3-pip
                fi
                ;;
        esac
    fi
}

# Install Python dependencies
install_python_deps() {
    print_info "Installing Python dependencies..."
    
    # Use pip3 if available, otherwise pip
    PIP_CMD="pip3"
    if ! command_exists pip3; then
        PIP_CMD="pip"
    fi
    
    if ! command_exists $PIP_CMD; then
        print_error "pip not found. Please install pip manually."
        exit 1
    fi
    
    # Install letterboxdpy (required for Letterboxd integration)
    print_info "Installing letterboxdpy..."
    $PIP_CMD install --user letterboxdpy
    
    print_status "Python dependencies installed successfully"
}

# Install viu (for image display in terminal)
install_viu() {
    print_info "Checking viu installation..."
    
    if command_exists viu; then
        VIU_VERSION=$(viu --version 2>/dev/null | head -1 || echo "unknown")
        print_status "viu already installed: $VIU_VERSION"
        return 0
    fi
    
    print_warning "viu not found. Installing..."
    
    case $OS in
        "macos")
            if command_exists brew; then
                brew install viu
            else
                print_warning "Homebrew not found. Installing via cargo..."
                install_viu_via_cargo
            fi
            ;;
        "linux")
            # Try package manager first, then cargo
            if command_exists apt && apt list viu 2>/dev/null | grep -q viu; then
                sudo apt install -y viu
            elif command_exists pacman && pacman -Si viu >/dev/null 2>&1; then
                sudo pacman -S viu
            elif command_exists cargo; then
                cargo install viu
            else
                print_warning "Installing Rust to build viu..."
                install_rust
                cargo install viu
            fi
            ;;
        "windows")
            if command_exists cargo; then
                cargo install viu
            elif command_exists scoop; then
                scoop install viu
            elif command_exists choco; then
                choco install viu
            else
                print_warning "Installing Rust to build viu..."
                install_rust
                cargo install viu
            fi
            ;;
        *)
            print_warning "Unsupported OS for automatic viu installation"
            install_viu_via_cargo
            ;;
    esac
    
    if command_exists viu; then
        print_status "viu installed successfully"
    else
        print_warning "viu installation failed. LBXD will use ASCII art mode by default."
        print_info "You can manually install viu later from: https://github.com/atanunq/viu"
    fi
}

# Install viu via cargo
install_viu_via_cargo() {
    if ! command_exists cargo; then
        print_info "Installing Rust (required for viu)..."
        install_rust
    fi
    
    print_info "Building viu from source..."
    cargo install viu
}

# Install Rust and Cargo
install_rust() {
    if command_exists cargo; then
        print_status "Rust already installed"
        return 0
    fi
    
    print_info "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo environment
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi
    
    if command_exists cargo; then
        print_status "Rust installed successfully"
    else
        print_error "Rust installation failed"
        exit 1
    fi
}

# Download and install prebuilt binary
install_prebuilt_binary() {
    print_info "Attempting to install prebuilt lbxd binary..."
    
    # Get latest release version
    LATEST_VERSION=$(curl -s https://api.github.com/repos/Pranav-Karra-3301/lbxd/releases/latest | grep '"tag_name"' | cut -d'"' -f4 2>/dev/null || echo "")
    
    if [ -z "$LATEST_VERSION" ]; then
        print_warning "Could not fetch latest version. Will build from source."
        return 1
    fi
    
    print_info "Latest version: $LATEST_VERSION"
    
    # Determine download URL based on OS and architecture
    case $OS in
        "macos")
            if [ "$ARCH" = "aarch64" ]; then
                BINARY_URL="https://github.com/Pranav-Karra-3301/lbxd/releases/download/$LATEST_VERSION/lbxd-macos-aarch64.tar.gz"
            else
                BINARY_URL="https://github.com/Pranav-Karra-3301/lbxd/releases/download/$LATEST_VERSION/lbxd-macos-x86_64.tar.gz"
            fi
            ;;
        "linux")
            if [ "$ARCH" = "x86_64" ]; then
                BINARY_URL="https://github.com/Pranav-Karra-3301/lbxd/releases/download/$LATEST_VERSION/lbxd-linux-x86_64.tar.gz"
            else
                print_warning "No prebuilt binary for $OS-$ARCH. Will build from source."
                return 1
            fi
            ;;
        "windows")
            BINARY_URL="https://github.com/Pranav-Karra-3301/lbxd/releases/download/$LATEST_VERSION/lbxd-windows-x86_64.exe.zip"
            ;;
        *)
            print_warning "No prebuilt binary for $OS. Will build from source."
            return 1
            ;;
    esac
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    print_info "Downloading $BINARY_URL..."
    if ! curl -L -o "lbxd-binary.archive" "$BINARY_URL"; then
        print_warning "Download failed. Will build from source."
        cd - > /dev/null
        rm -rf "$TEMP_DIR"
        return 1
    fi
    
    # Extract binary
    case $BINARY_URL in
        *.tar.gz)
            tar -xzf lbxd-binary.archive
            ;;
        *.zip)
            unzip -q lbxd-binary.archive
            ;;
    esac
    
    # Find and install binary
    BINARY_NAME="lbxd"
    if [ "$OS" = "windows" ]; then
        BINARY_NAME="lbxd.exe"
    fi
    
    if [ -f "$BINARY_NAME" ]; then
        # Install to appropriate location
        case $OS in
            "macos"|"linux")
                INSTALL_DIR="$HOME/.local/bin"
                mkdir -p "$INSTALL_DIR"
                cp "$BINARY_NAME" "$INSTALL_DIR/"
                chmod +x "$INSTALL_DIR/$BINARY_NAME"
                
                # Add to PATH if not already there
                if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
                    echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$HOME/.bashrc"
                    echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$HOME/.zshrc" 2>/dev/null || true
                    print_info "Added $INSTALL_DIR to PATH. Please restart your terminal or run: source ~/.bashrc"
                fi
                ;;
            "windows")
                INSTALL_DIR="$HOME/bin"
                mkdir -p "$INSTALL_DIR"
                cp "$BINARY_NAME" "$INSTALL_DIR/"
                print_info "Binary installed to $INSTALL_DIR. Please add this directory to your PATH."
                ;;
        esac
        
        cd - > /dev/null
        rm -rf "$TEMP_DIR"
        print_status "lbxd binary installed successfully"
        return 0
    else
        print_warning "Binary not found in archive. Will build from source."
        cd - > /dev/null
        rm -rf "$TEMP_DIR"
        return 1
    fi
}

# Build and install from source
install_from_source() {
    print_info "Installing lbxd from source..."
    
    # Ensure Rust is installed
    if ! command_exists cargo; then
        print_info "Installing Rust (required for building)..."
        install_rust
    fi
    
    # Build and install
    print_info "Building lbxd (this may take a few minutes)..."
    cargo install --path . --force
    
    if command_exists lbxd; then
        print_status "lbxd built and installed successfully"
    else
        print_error "Failed to build lbxd from source"
        exit 1
    fi
}

# Main installation process
main() {
    echo
    print_info "Starting LBXD installation..."
    echo
    
    # Detect operating system and architecture
    detect_os
    print_info "Detected OS: $OS ($ARCH)"
    echo
    
    # Install Python 3
    install_python
    echo
    
    # Install Python dependencies
    install_python_deps
    echo
    
    # Install viu (optional but recommended)
    install_viu
    echo
    
    # Try to install prebuilt binary first, fall back to source build
    if ! install_prebuilt_binary; then
        print_info "Prebuilt binary not available or failed. Building from source..."
        install_from_source
    fi
    echo
    
    print_status "Installation complete!"
    echo
    print_info "🎬 lbxd is now ready to use!"
    echo
    print_info "Quick start:"
    echo "  • Run 'lbxd --help' to see all available commands"
    echo "  • Try 'lbxd recent username' to see a user's recent activity"
    echo "  • Use 'lbxd movie \"movie title\"' to search for movies"
    echo "  • Run 'lbxd --reconfig' to change settings anytime"
    echo
    
    # Check if lbxd is in PATH
    if ! command_exists lbxd; then
        print_warning "lbxd may not be in your PATH. You might need to:"
        echo "  • Restart your terminal"
        echo "  • Run: source ~/.bashrc (or ~/.zshrc)"
        echo "  • Or add ~/.local/bin to your PATH manually"
    fi
    
    echo
    print_info "📖 Documentation: https://github.com/Pranav-Karra-3301/lbxd"
    print_info "🐛 Report issues: https://github.com/Pranav-Karra-3301/lbxd/issues"
}

# Run main function
main "$@"