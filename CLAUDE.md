# substreams-solana-idls

Type-safe Solana instruction & event decoders for Substreams.

## Agent Instructions

See **[.agents/skills/](.agents/skills/)** for all workflows and conventions.

## Domain Knowledge

- **[docs/metaplex.md](docs/metaplex.md)** — Metaplex Token Metadata & Bubblegum decoding reference (discriminators, account layouts, instructions, PDA derivation, serialization)

## Quick Start

```bash
cargo clippy --workspace -- -D warnings    # Lint
cargo test --workspace                      # Test
cargo check --workspace --target wasm32-unknown-unknown  # WASM check
```
