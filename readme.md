# Reading App Web

Reading App Web is a browser-first reading and writing application built with `egui` and compiled to WebAssembly.
All application data is stored locally in IndexedDB, so no backend service is required.

## Features

- Local-first persistence with IndexedDB (`articles`, `settings`)
- Client-only architecture with no required server-side API
- Configurable development server settings from `config/development.toml`
- Development and build scripts with emoji-free terminal output
- Article list rendered as card-based UI in the application
- Optional PWA service worker support for offline caching

## Tech Stack

- Rust 2024 edition
- `eframe` / `egui` for UI
- `wasm-bindgen` and `web-sys` for browser interop
- `indexed_db_futures` for IndexedDB access
- Trunk for build + serve workflow

## Prerequisites

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target
- Trunk (`cargo install --locked trunk`)
- Optional: `just` task runner

## Quick Start

### 1) Initial setup

```bash
./scripts/setup.sh
```

### 2) Configure dev server

Edit `config/development.toml`:

```toml
[serve]
address = "127.0.0.1"
port = 8080
open = true
```

### 3) Start development server

```bash
./scripts/dev.sh
```

Or, with `just`:

```bash
just dev
```

### 4) Build production assets

```bash
./scripts/build.sh
```

Or:

```bash
just build
```

## Common Commands

### Script-based

```bash
./scripts/setup.sh
./scripts/dev.sh
./scripts/build.sh
./scripts/deploy.sh
./scripts/clean.sh all
```

### Justfile-based

```bash
just setup
just dev
just build
just fmt
just test
just clean
```

## Project Layout

```text
reading_app_web/
|- assets/
|  `- sw.js
|- config/
|  `- development.toml
|- scripts/
|  |- setup.sh
|  |- dev.sh
|  |- build.sh
|  |- deploy.sh
|  `- clean.sh
|- src/
|  |- bin/
|  |  |- server_config.rs
|  |  `- trunk_log_filter.rs
|  |- database/
|  |  |- index_db.rs
|  |  `- mod.rs
|  |- ui/
|  |  |- app.rs
|  |  `- mod.rs
|  `- lib.rs
|- Cargo.toml
|- Trunk.toml
|- index.html
`- justfile
```

## Architecture Notes

- `src/lib.rs` is the wasm entrypoint and launches the egui web runner.
- `src/database/index_db.rs` owns IndexedDB schema and CRUD operations.
- `src/ui/app.rs` contains app state and panels (editor, settings, article cards).
- `src/bin/server_config.rs` resolves dev server config for scripts.
- `src/bin/trunk_log_filter.rs` removes emoji characters from Trunk log streams.

## Data Model

Primary IndexedDB stores:

- `articles`
  - `id`, `title`, `content`, `created_at`, `word_count`, `reading_time_minutes`
- `settings`
  - `id`, `font_size`, `font_family`, `theme`

## Development Workflow

- Keep `./scripts/dev.sh` running while editing files.
- Trunk rebuilds automatically and refreshes the browser.
- Append `#dev` to bypass service worker caching during development.

Example URL pattern:

`http://<address>:<port>/index.html#dev`

## Testing and Formatting

```bash
dprint fmt
cargo test --lib
```

## API Documentation

Generate project docs with rustdoc:

```bash
cargo doc --no-deps --open
```

## Deployment

### GitHub Actions

Push to your main branch and let the workflow in `.github/workflows/pages.yml` deploy artifacts.

### Manual

```bash
./scripts/build.sh
./scripts/deploy.sh
```

## Troubleshooting

### IndexedDB appears empty or stale

- Open browser devtools and inspect Application > IndexedDB.
- Ensure you are using the same origin (address + port).

### Dev config changes are not reflected

- Verify `config/development.toml` syntax.
- Restart `./scripts/dev.sh` after changing config.

### Build failures in wasm optimization

- Run a clean rebuild:

```bash
./scripts/clean.sh all
./scripts/build.sh
```

If failures persist, check your local `wasm-opt` version and Trunk output for feature-compatibility diagnostics.

## License

MIT OR Apache-2.0
