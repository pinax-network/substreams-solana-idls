# Repository Navigation Guide

This document captures durable navigation and workflow knowledge for `substreams-solana-idls`.

## 1) Workspace Layout

- `Cargo.toml`: root package metadata plus workspace membership.
- `src/lib.rs`: root re-exports of selected protocol crates.
- `packages/common/`: shared `ParseError` and account parsing helpers used by protocol crates.
- `packages/<protocol>/`: one crate per protocol (for example `raydium`, `jupiter`, `pumpfun`).
- `.github/workflows/`: CI/publish pipelines.
- `target/`: Rust build artifacts (generated; not source).

Each protocol package typically contains:

- `Cargo.toml`: crate dependencies and metadata.
- `src/lib.rs`: protocol module entrypoint and `PROGRAM_ID` constants.
- `src/instructions.rs`: instruction enums + `unpack`.
- `src/events.rs`: event enums/structs + `unpack` (if supported).
- `src/accounts.rs`: account helper structs/functions (if needed).
- `src/<variant>/`: versioned or product-specific submodules (`v4`, `v6`, `amm`, `clmm`, etc.).
- `tests/*.rs`: decode fixtures and behavior checks.

## 2) Architecture / Data-Flow Mental Model

- Input: raw Solana instruction/log bytes.
- Decoder entrypoint: `<module>::instructions::unpack(data)` and/or `<module>::events::unpack(data)`.
- Dispatch mechanism: match on discriminators, then Borsh-deserialize into typed structs.
- Shared errors: `common::ParseError` standardizes decode failures across crates.
- Substreams usage: downstream mappings filter by `PROGRAM_ID`, decode, then map to output entities.

Think of each protocol crate as a codec library with a stable decode API: `PROGRAM_ID` + `unpack` + typed structs.

## 3) Where To Edit For X

- Add a new protocol crate:
  - Add `packages/<protocol>/`.
  - Register it in `Cargo.toml` `[workspace].members` and `[dependencies]`.
  - Optionally re-export in `src/lib.rs`.

- Add support for a new program/version inside an existing protocol:
  - Add a new module under `packages/<protocol>/src/<variant>/`.
  - Wire it in `packages/<protocol>/src/lib.rs` or parent `mod.rs`.
  - Add/extend tests in `packages/<protocol>/tests/`.

- Update instruction decoding logic:
  - Edit `instructions.rs` in the relevant protocol/variant module.

- Update event decoding logic:
  - Edit `events.rs` (or `anchor_cpi_event.rs`/`logs.rs` where present).

- Update account parsing helpers:
  - Edit `accounts.rs` in the relevant module.
  - Shared account helper macros live in `packages/common/src/accounts.rs`.

- Change shared parse error behavior:
  - Edit `packages/common/src/lib.rs` (`ParseError`).

- Expose protocol from top-level crate path:
  - Add `pub use <protocol>;` in `src/lib.rs`.

## 4) Key Build/Test Anchors

- Format check: `cargo fmt --all -- --check`
- Lint check: `cargo clippy --workspace -- -D warnings`
- Tests: `cargo test --workspace`
- WASM compatibility: `cargo check --workspace --target wasm32-unknown-unknown`

CI equivalents are defined in `.github/workflows/ci.yml`.

## 5) Source vs Generated Artifacts

Source of truth:

- `packages/**/src/**/*.rs`
- `packages/**/tests/**/*.rs`
- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`
- `.github/workflows/*.yml`

Generated/build artifacts:

- `target/**`

Committed data artifacts (treated as inputs, not build outputs):

- `packages/**/src/**/idl*.json`
- `packages/**/src/idl.json`

Those JSON IDLs are checked in and used by maintainers as decoder references.
