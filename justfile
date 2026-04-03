set shell := ["bash", "-cu"]

default:
    just dev

dev:
    set -euo pipefail; \
    if ! command -v trunk >/dev/null 2>&1; then echo "[ERROR] Trunk is not installed"; echo "[INFO] Install with: cargo install --locked trunk"; exit 1; fi; \
    if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then echo "[WARN] wasm32-unknown-unknown target not installed"; echo "[INFO] Installing target..."; rustup target add wasm32-unknown-unknown; fi; \
    SERVE_ADDRESS="127.0.0.1"; SERVE_PORT="8080"; SERVE_OPEN="true"; \
    while IFS='=' read -r key value; do case "$key" in SERVE_ADDRESS) SERVE_ADDRESS="$value" ;; SERVE_PORT) SERVE_PORT="$value" ;; SERVE_OPEN) SERVE_OPEN="$value" ;; esac; done <<< "$(CARGO_TARGET_DIR=target/trunk-tools cargo run --quiet --bin server_config)"; \
    echo "[INFO] Building and serving your egui app..."; \
    echo "[INFO] Server will be available at: http://${SERVE_ADDRESS}:${SERVE_PORT}"; \
    echo "[INFO] Note: Append #dev to skip PWA caching during development"; \
    echo "[INFO] Example: http://${SERVE_ADDRESS}:${SERVE_PORT}/index.html#dev"; \
    echo "[WARN] Press Ctrl+C to stop the server"; \
    trunk serve --address "$SERVE_ADDRESS" --port "$SERVE_PORT" --open "$SERVE_OPEN" --color never 2>&1 | CARGO_TARGET_DIR=target/trunk-tools cargo run --quiet --bin trunk_log_filter

build:
    set -euo pipefail; \
    if ! command -v trunk >/dev/null 2>&1; then echo "[ERROR] Trunk is not installed"; echo "[INFO] Install with: cargo install --locked trunk"; exit 1; fi; \
    if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then echo "[WARN] wasm32-unknown-unknown target not installed"; echo "[INFO] Installing target..."; rustup target add wasm32-unknown-unknown; fi; \
    REPO_NAME="$(basename "$(git rev-parse --show-toplevel 2>/dev/null || echo 'reading_app')")"; \
    echo "[INFO] Repository: $REPO_NAME"; \
    echo "[INFO] Building optimized WASM bundle..."; \
    if trunk build --release --public-url "/$REPO_NAME/" --color never 2>&1 | CARGO_TARGET_DIR=target/trunk-tools cargo run --quiet --bin trunk_log_filter; then \
    echo "[INFO] Build successful!"; \
    echo "[INFO] Output directory: ./dist"; \
    else \
    echo "[ERROR] Build failed"; \
    exit 1; \
    fi

setup:
    set -euo pipefail; \
    if ! command -v rustc >/dev/null 2>&1; then echo "[ERROR] Rust is not installed"; echo "[INFO] Install from: https://rustup.rs/"; exit 1; fi; \
    echo "[INFO] Rust version: $(rustc --version)"; \
    echo "[INFO] Installing wasm32-unknown-unknown target..."; \
    rustup target add wasm32-unknown-unknown; \
    if command -v trunk >/dev/null 2>&1; then echo "[INFO] Trunk already installed: $(trunk --version)"; else echo "[INFO] Installing Trunk..."; cargo install --locked trunk; fi; \
    chmod +x scripts/*.sh; \
    echo "[INFO] Setup complete"

clean mode="all":
    set -euo pipefail; \
    case "{{mode}}" in \
    cargo) echo "[INFO] Running cargo clean"; cargo clean ;; \
    trunk) if [[ -d "dist" ]]; then echo "[INFO] Removing dist directory"; rm -rf dist; else echo "[INFO] dist directory not found, skipping"; fi ;; \
    all) echo "[INFO] Running cargo clean"; cargo clean; if [[ -d "dist" ]]; then echo "[INFO] Removing dist directory"; rm -rf dist; else echo "[INFO] dist directory not found, skipping"; fi ;; \
    help|-h|--help) echo "Usage: just clean <mode>"; echo "Available modes: cargo, trunk, all, help" ;; \
    *) echo "[ERROR] unknown mode '{{mode}}'"; echo "Usage: just clean <mode>"; exit 1 ;; \
    esac

deploy:
    set -euo pipefail; \
    CURRENT_BRANCH="$(git branch --show-current)"; \
    echo "[INFO] Pushing branch: $CURRENT_BRANCH"; \
    git push origin "$CURRENT_BRANCH"

fmt:
    dprint fmt

test:
    cargo test --lib
