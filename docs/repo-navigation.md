# Repository Navigation Guide

This repository is one Rust crate, `substreams-solana-idls`.

## Layout and edit routing

- `Cargo.toml`, `Cargo.lock`: crate metadata and dependencies.
- `src/lib.rs`: protocol module exports.
- `src/common/`: shared `ParseError`, account errors and parsing macros.
- `src/<protocol>/`: protocol decoders, with variant submodules where needed.
- `tests/<protocol>/`: integration fixtures, registered by `tests/<protocol>.rs`.
- `.github/workflows/`: native tests, WASM checks and release publishing.

Within a protocol module, `mod.rs` declares modules and program-address constants;
`instructions.rs` and `events.rs` decode raw bytes; `accounts.rs` resolves instruction
accounts. JSON IDLs in these directories are reference inputs, not generated build output.
`target/` is generated and ignored.

Add protocols under `src/`, export them from `src/lib.rs`, and register their tests under
`tests/`. Change shared parsing behavior in `src/common/`; keep protocol-specific changes
in the relevant module. There is no `packages/` workspace.

## Model and dependency boundary

Version 2 uses the Buffa-based `substreams-solana-core` 0.16 model, aliased as
`substreams_solana` inside the crate. `substreams-solana-macro` supplies `b58!` directly.
Neither dependency introduces the Substreams runtime. Borsh instruction/event decoding
is independent of the protobuf runtime; account helpers take the owned model's
`InstructionView` and therefore require downstream consumers to use the same model version.

The upstream Solana repository owns the protobuf schemas and Buf generation configuration.
This repository does not contain `.proto` files or generated block models. A model upgrade
requires checking public account-helper compatibility and migrating protobuf fixtures;
do not add a duplicate private block model or copy generated upstream types here.

Buffa message presence must be checked explicitly when required: `.as_option()` and
`.is_set()` distinguish an absent message from the default returned by dereferencing it.
Lazy views are a separate upstream API; current account helpers use owned `InstructionView`.

## Validation

Rust 1.85 is the declared minimum and the CI/publishing toolchain. Run:

```bash
cargo test --locked
cargo check --locked --target wasm32-unknown-unknown
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
git diff --check
```

Keep fixture bytes and expected decoder outputs stable during dependency-only migrations.
Pump AMM creation layouts and their captured-chain evidence are documented in
[`pump-amm-creation.md`](pump-amm-creation.md).
For normal/build/development dependency inspection, use:

```bash
cargo tree --locked --edges normal,build,dev
```

The crate should not depend on the `substreams` SDK or its WASM allocator symbols.
