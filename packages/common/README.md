# Common

Shared utilities for all protocol decoders.

## Contents

- **Account parsing** — Helpers for deserializing Solana account data
- **Discriminator helpers** — Utilities for matching instruction/event discriminators
- **Error types** — Shared `ParseError` enum

## Usage

```rust
use common::ParseError;
use common::accounts;
```

This crate is a workspace dependency — all protocol packages depend on it.
