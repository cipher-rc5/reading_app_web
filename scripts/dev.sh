#!/usr/bin/env bash

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

load_server_config() {
    local config_output
    config_output=$(CARGO_TARGET_DIR=target/trunk-tools cargo run --quiet --bin server_config)

    while IFS='=' read -r key value; do
        case "$key" in
            SERVE_ADDRESS) SERVE_ADDRESS="$value" ;;
            SERVE_PORT) SERVE_PORT="$value" ;;
            SERVE_OPEN) SERVE_OPEN="$value" ;;
        esac
    done <<< "$config_output"
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
    print_warn "Trunk is not installed"
    print_info "Install with: cargo install --locked trunk"
    exit 1
fi

# Check if wasm target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    print_warn "wasm32-unknown-unknown target not installed"
    print_info "Installing target..."
    rustup target add wasm32-unknown-unknown
fi

SERVE_ADDRESS="127.0.0.1"
SERVE_PORT="8080"
SERVE_OPEN="true"
load_server_config

print_header "Starting Development Server"

print_info "Building and serving your egui app..."
print_info "Server will be available at: http://${SERVE_ADDRESS}:${SERVE_PORT}"
print_info "Note: Append #dev to skip PWA caching during development"
print_info "Example: http://${SERVE_ADDRESS}:${SERVE_PORT}/index.html#dev"
print_warn "Press Ctrl+C to stop the server"

# Start trunk dev server
trunk serve --address "$SERVE_ADDRESS" --port "$SERVE_PORT" --open "$SERVE_OPEN" --color never 2>&1 | strip_emoji_stream
