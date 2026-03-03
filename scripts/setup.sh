#!/usr/bin/env bash

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_header "Setting Up Development Environment"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed"
    print_info "Install from: https://rustup.rs/"
    exit 1
fi

print_info "Rust version: $(rustc --version)"

# Install wasm32-unknown-unknown target
print_info "Installing wasm32-unknown-unknown target..."
if rustup target add wasm32-unknown-unknown; then
    print_info "Target installed successfully"
else
    print_error "Failed to install wasm32-unknown-unknown target"
    exit 1
fi

# Install Trunk
if command -v trunk &> /dev/null; then
    print_info "Trunk already installed: $(trunk --version)"
else
    print_info "Installing Trunk..."
    if cargo install --locked trunk; then
        print_info "Trunk installed successfully"
    else
        print_error "Failed to install Trunk"
        exit 1
    fi
fi

# Make scripts executable
print_info "Making scripts executable..."
chmod +x scripts/*.sh

print_header "Setup Complete"
print_info "Available commands:"
print_info "  ./scripts/dev.sh      - Start development server"
print_info "  ./scripts/build.sh    - Build for production"
print_info "  ./scripts/deploy.sh   - Deploy to GitHub Pages"
print_info "  ./scripts/clean.sh    - Clean build artifacts"
print_info "  config/development.toml - Dev server address/port/open settings"
print_info ""
print_info "Next steps:"
print_info "  1. Run: ./scripts/dev.sh"
print_info "  2. Open the URL shown by the dev script"
print_info "  3. Start coding!"
