# Substreams Solana IDLs

> **Type-safe instruction & event decoders for any Solana program you want to index with Substreams.**

This crate bundles thin, `no_std`-friendly byte-codecs that turn on-chain **instruction** and **event** byte arrays into well-typed Rust enums & structs, ready for use inside Substreams modules.

---

## Protocol Coverage

| Status | Ecosystem | Protocol / App | Program ID |
| ------ | --------- | -------------- | ---------- |
| ✅ | Pump.fun | Bonding-Curve | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| ✅ | Pump.fun | PumpSwap AMM | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| ✅ | Jupiter | Swap Aggregator V6 | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` |
| ✅ | Jupiter | Swap Aggregator V4 | `JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB` |
| ✅ | Raydium | AMM V4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| ✅ | Raydium | AMM CP | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| ✅ | Raydium | CLMM V3 | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| ✅ | Raydium | LaunchPad | `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj` |
| ✅ | Stabble | Stable Swap | `swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW` |
| ✅ | Orca | Whirlpool (CLMM) | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| ✅ | Meteora | Pools | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` |
| ✅ | Meteora | DAMM V2 | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG` |
| ✅ | Meteora | DLLM | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` |
| ❓ | BonkSwap | AMM | `BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p` |
| ✅ | Raydium | Stable AMM Anchor | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h` |
| ✅ | Jupiter | Limit Order | `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu` |
| ✅ | Jupiter | DCA | `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M` |
| ✅ | Phoenix | on-chain CLOB | `PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY` |
| 🚧 | PancakeSwap | AMM Swap | `HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq` |
| 🚧 | Drift | V2 Perpetual | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` |
| 💀 | SolFi | Swap | `SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe` |
| 💀 | SolFi | Swap V2 | `SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF` |
| ✅ | OKX DEX | Aggregation Router V2 | `6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma` |
| ✅ | DFlow | Swap Aggregator V4 | `DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH` |
| ✅ | Saros | AMM Swap | `SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr` |
| 💀 | HumidiFi | Swap | `9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp` |
| 💀 | Tessera V | Swap | `TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH` |
| 🚧 | Byreal CLMM | Swap | `REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2` |
| 💀 | Heaven DEX | Swap | `HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o` |
| ✅ | Marinade | Liquid Staking | `MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD` |
| 💀 | Orca | Token Swap ("Classic") | `9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP` |

**Legend**   ✅ done · 🚧 in progress · ❌ planned / help wanted · 💀 no IDL · ❓ partial support

> Want another program supported? Open an issue or PR with the program ID and IDL link.

---

## Import crate

Add to your `Cargo.toml`:

```toml
[dependencies]
substreams-solana-idls = { git = "https://github.com/pinax-network/substreams-solana-idls", tag = "v0.6.x" }
```

## Workspace Layout

This repository is a Rust workspace, with one crate per protocol under `packages/`.

```
substreams-solana-idls/
├── src/lib.rs            # Root crate re-exports selected protocol crates
├── packages/common       # Shared parsing helpers and ParseError
└── packages/<protocol>   # Protocol-specific decoders and tests
```

Most protocol crates follow this internal shape:

```
packages/<protocol>/src/
├── lib.rs                # Module declarations + PROGRAM_ID constants
├── instructions.rs       # Instruction enum + unpack(&[u8])
├── events.rs             # Event enum/structs + unpack(&[u8])
├── accounts.rs           # Account parsing helpers (when available)
└── <variant>/...         # Versioned modules (v4, v6, clmm, amm, etc.)
```

Examples:
- `packages/jupiter/src/v4/*` and `packages/jupiter/src/v6/*`
- `packages/raydium/src/amm/v4/*`, `clmm/v3/*`, `cpmm/*`, `stable/*`, `launchpad/*`
- `packages/pumpfun/src/bonding_curve/*` and `packages/pumpfun/src/amm/*`

---

## Quick Start (Pump.fun example)

```rust
use substreams_solana_idls::pumpfun;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, Error> {
    for tx in block.transactions() {
        for ix in tx.walk_instructions() {
            // Fast path: ignore anything not coming from Pump.fun
            if ix.program() != pumpfun::PROGRAM_ID { continue; }

            match pumpfun::events::unpack(ix.data()) {
                pumpfun::events::PumpFunEvent::Trade(tr) => {
                    // …handle trade…
                }
                _ => {}
            }
        }
    }
    Ok(Default::default())
}
```

Swap `pumpfun` for any protocol that is re-exported in `src/lib.rs`.

## Build & Test

```bash
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo check --workspace --target wasm32-unknown-unknown
```

---

## Contributing

* Add a new workspace crate under `packages/<protocol>`.
* Register it in root `Cargo.toml` under both `[workspace].members` and `[dependencies]`.
* Export it from `src/lib.rs` if it should be reachable via `substreams_solana_idls::<protocol>`.
* Follow existing package structure (`lib.rs`, `instructions.rs`, `events.rs`, `accounts.rs` and/or variant submodules).

PRs and issues welcome!
