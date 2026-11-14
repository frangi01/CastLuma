# CastLuma Rust Core

Platform-independent domain models and session transition rules for CastLuma.

## Current scope

The core models devices, media sources, and logical session transitions.
It does not yet perform device discovery, network communication, media serving or platform integration.

Session transitions calculate the next state without performing I/O or managing resources.

## Development

Run from the `core` directory:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## API documentation

Public types, methods and usage examples are documented in the source.

Generate the API documentation with:

```bash
cargo doc --workspace --no-deps
```

Open `target/doc/castluma_core/index.html` to browse it.