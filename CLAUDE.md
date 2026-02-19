# substreams-solana-idls

Type-safe Solana instruction & event decoders for Substreams.

## Agent Instructions

See **[.agents/skills/](.agents/skills/)** for all workflows and conventions.

## Quick Start

```bash
cargo clippy --workspace -- -D warnings    # Lint
cargo test --workspace                      # Test
cargo check --workspace --target wasm32-unknown-unknown  # WASM check
```
