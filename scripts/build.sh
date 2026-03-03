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

strip_emoji_stream() {
    CARGO_TARGET_DIR=target/trunk-tools cargo run --quiet --bin trunk_log_filter
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    print_error "Trunk is not installed"
    print_info "Install with: cargo install --locked trunk"
    exit 1
fi

# Check if wasm target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    print_warn "wasm32-unknown-unknown target not installed"
    print_info "Installing target..."
    rustup target add wasm32-unknown-unknown
fi

print_header "Building for Production"

# Get repository name for public URL
REPO_NAME=$(basename "$(git rev-parse --show-toplevel 2>/dev/null || echo 'reading_app')")

print_info "Repository: $REPO_NAME"
print_info "Building optimized WASM bundle..."

# Build with trunk for production
if trunk build --release --public-url "/$REPO_NAME/" --color never 2>&1 | strip_emoji_stream; then
    print_info "Build successful!"
    print_info "Output directory: ./dist"

    # Show build artifacts
    if [ -d "dist" ]; then
        print_info "Build artifacts:"
        ls -lh dist/ | tail -n +2 | awk '{printf "  %-20s %10s\n", $9, $5}'

        # Calculate total size
        TOTAL_SIZE=$(du -sh dist/ | cut -f1)
        print_info "Total size: $TOTAL_SIZE"
    fi
else
    print_error "Build failed"
    exit 1
fi

print_header "Build Complete"
print_info "To test locally: trunk serve --release --open"
print_info "To deploy: git push origin main (if GitHub Actions is configured)"
