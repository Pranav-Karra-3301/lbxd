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

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        OS="windows"
    else
        OS="unknown"
    fi
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Python 3
install_python() {
    print_info "Checking Python 3 installation..."
    
    if command_exists python3; then
        PYTHON_VERSION=$(python3 --version 2>&1 | cut -d" " -f2)
        print_status "Python 3 found: $PYTHON_VERSION"
        return 0
    fi
    
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
                sudo apt install -y python3 python3-pip
            elif command_exists yum; then
                sudo yum install -y python3 python3-pip
            elif command_exists pacman; then
                sudo pacman -S python python-pip
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
}

# Install Python dependencies
install_python_deps() {
    print_info "Installing Python dependencies..."
    
    # Check if pip exists
    if ! command_exists pip3 && ! command_exists pip; then
        print_error "pip not found. Please install pip manually."
        exit 1
    fi
    
    # Use pip3 if available, otherwise pip
    PIP_CMD="pip3"
    if ! command_exists pip3; then
        PIP_CMD="pip"
    fi
    
    # Install required packages
    $PIP_CMD install --user opencv-python pillow numpy
    
    print_status "Python dependencies installed successfully"
}

# Install viu
install_viu() {
    print_info "Checking viu installation..."
    
    if command_exists viu; then
        print_status "viu already installed"
        return 0
    fi
    
    print_warning "viu not found. Installing..."
    
    case $OS in
        "macos")
            if command_exists brew; then
                brew install viu
            else
                print_error "Homebrew not found. Please install viu manually: https://github.com/atanunq/viu"
                print_info "You can still use LBXD with ASCII art mode using --ascii flag"
                return 1
            fi
            ;;
        "linux")
            if command_exists cargo; then
                cargo install viu
            elif command_exists apt && grep -q "Ubuntu\|Debian" /etc/os-release; then
                # Try to install from package manager first
                sudo apt update
                if apt list viu 2>/dev/null | grep -q viu; then
                    sudo apt install -y viu
                else
                    # Install via cargo if available
                    if command_exists cargo; then
                        cargo install viu
                    else
                        print_warning "viu not available in package manager. Installing Rust to build viu..."
                        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                        source ~/.cargo/env
                        cargo install viu
                    fi
                fi
            else
                print_warning "Installing viu via cargo..."
                if ! command_exists cargo; then
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source ~/.cargo/env
                fi
                cargo install viu
            fi
            ;;
        "windows")
            if command_exists cargo; then
                cargo install viu
            else
                print_error "Please install Rust and Cargo, then run: cargo install viu"
                print_info "You can still use LBXD with ASCII art mode using --ascii flag"
                return 1
            fi
            ;;
        *)
            print_warning "Unsupported OS for automatic viu installation"
            print_info "Please install viu manually: https://github.com/atanunq/viu"
            print_info "You can still use LBXD with ASCII art mode using --ascii flag"
            return 1
            ;;
    esac
    
    print_status "viu installed successfully"
}

# Main installation process
main() {
    echo
    print_info "Starting LBXD dependency installation..."
    echo
    
    # Detect operating system
    detect_os
    print_info "Detected OS: $OS"
    echo
    
    # Install Python 3
    install_python
    echo
    
    # Install Python dependencies
    install_python_deps
    echo
    
    # Install viu
    install_viu
    echo
    
    print_status "Installation complete!"
    echo
    print_info "Next steps:"
    echo "  1. Build and install lbxd: cargo install --path ."
    echo "  2. Run: lbxd"
    echo "  3. Follow the interactive setup to configure your preferences"
    echo
    print_info "If you encounter any issues, please check the README or file an issue on GitHub."
}

# Run main function
main "$@"