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

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

usage() {
    cat <<'USAGE'
Usage: ./clean.sh <mode>

Available modes:
  cargo    Remove Cargo build artifacts (runs `cargo clean`)
  trunk    Delete Trunk build outputs in ./dist
  all      Perform every clean action above
  help     Show this message
USAGE
}

ensure_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "Error: required command '$1' not found." >&2
        exit 1
    fi
}

clean_cargo() {
    ensure_cmd cargo
    print_info "Running cargo clean"
    cargo clean
}

clean_trunk() {
    if [[ -d "dist" ]]; then
        print_info "Removing dist directory"
        rm -rf dist
    else
        print_info "dist directory not found, skipping"
    fi
}

MODE="${1:-help}"

print_header "Cleaning Build Artifacts"

case "$MODE" in
    cargo)
        clean_cargo
        ;;
    trunk)
        clean_trunk
        ;;
    all)
        clean_cargo
        clean_trunk
        ;;
    help|-h|--help)
        usage
        exit 0
        ;;
    *)
        echo "Error: unknown mode '$MODE'" >&2
        usage
        exit 1
        ;;
esac

print_info "Done"
