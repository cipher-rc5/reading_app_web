# Agent Guidelines

## Build & Development Commands

- **Development server**: `./scripts/dev.sh` (auto-rebuilds on file changes)
- **Production build**: `./scripts/build.sh` (outputs to `./dist`)
- **Code format & lint**: `dprint fmt` (runs rustfmt on .rs files, formats other files)
- **Testing**: `cargo test --lib` (unit tests only; no test harness for WASM)
- **Single test**: `cargo test --lib test_name -- --nocapture`

## Code Style Guidelines

### Rust Code

- **Edition**: 2024 (Cargo.toml)
- **Formatting**: Use `dprint fmt` for formatting (configured via dprint.json)
- **Naming**: snake_case for functions/variables, CamelCase for types/structs
- **Imports**: Group by: standard library, external crates, local modules. Use `pub use` in mod.rs files
- **Error handling**: Use `anyhow::Result<T>` for fallible operations; propagate errors with `?` operator
- **Async/await**: Use `spawn_local()` for async tasks on WASM; always handle errors in async blocks
- **Comments**: Minimal; code should be self-documenting. Use `tracing::info!()` for debug logs

### Architecture

- **Modules**: Organize by domain (`database`, `ui`, etc.)
- **Types**: Derive `Clone, Debug, Serialize, Deserialize` where needed
- **Database**: IndexedDB via `indexed_db_futures` crate; implement methods that return `Result<T>`
- **UI**: Use egui framework; separate UI logic from business logic

### No Cursor/Copilot Rules Found

This repo has no `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` files.
