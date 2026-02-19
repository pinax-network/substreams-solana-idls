# Skill: Build and Test

## Architecture

This is a Rust **workspace** crate. Each protocol has its own package under `packages/`, with the root crate re-exporting everything.

Decoders are hand-written (not auto-generated) using `borsh` deserialization from Solana IDLs or program source code.

## Build

```bash
cargo build --workspace
```

## Test

```bash
cargo test --workspace
```

Tests use real base64-encoded transaction data to verify decoding.

## Lint

```bash
cargo clippy --workspace -- -D warnings
```

Clippy is enforced in CI with `-D warnings` (all warnings are errors).

## Wasm Check

```bash
cargo check --workspace --target wasm32-unknown-unknown
```

Substreams runs on WASM — all code must compile for `wasm32-unknown-unknown`.

## Toolchain

- **Rust version:** 1.80 (specified in `rust-toolchain.toml`)
- **Target:** `wasm32-unknown-unknown` (for Substreams)
- **CI:** Single job — clippy + test + wasm32 check

## Key Dependencies

- `substreams` — Core Substreams SDK
- `substreams-solana` — Solana-specific Substreams types
- `solana-program` — Solana program types (Pubkey, etc.)
- `borsh` — Binary serialization/deserialization (Solana standard)
- `common` — Shared crate with discriminator helpers

## Before Pushing

Always run:
1. `cargo clippy --workspace -- -D warnings` — lint check
2. `cargo test --workspace` — run all tests
3. `cargo check --workspace --target wasm32-unknown-unknown` — WASM compatibility
