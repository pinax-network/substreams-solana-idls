# Substreams Solana IDLs

> **Type-safe instruction & event decoders for any Solana program you want to index with Substreams.**

This crate bundles thin, `no_std`-friendly byte-codecs that turn on-chain **instruction** and **event** byte arrays into well-typed Rust enums & structs, ready for use inside Substreams modules.

---

## Protocol Coverage

| Ecosystem | Protocol / App | Program ID |
| --------- | -------------- | ---------- |
| Pump.fun | Bonding-Curve | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| Pump.fun | PumpSwap AMM | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| Jupiter | Swap Aggregator V6 | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` |
| Jupiter | Swap Aggregator V4 | `JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB` |
| Jupiter | Limit Order | `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu` |
| Jupiter | DCA | `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M` |
| Raydium | AMM V4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| Raydium | AMM CP | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| Raydium | CLMM V3 | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| Raydium | LaunchPad | `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj` |
| Raydium | Stable AMM Anchor | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h` |
| Orca | Whirlpool (CLMM) | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| Meteora | Pools | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` |
| Meteora | DAMM V2 | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG` |
| Meteora | DLMM | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` |
| Openbook | V2 Order Book | `opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb` |
| Phoenix | on-chain CLOB | `PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY` |
| Darklake | Dark Pool DEX | `darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1` |
| Drift | V2 Perpetual | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` |
| Tensor | NFT Marketplace | `TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp` |
| Stabble | Stable Swap | `swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW` |
| PancakeSwap | AMM Swap | `HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq` |
| BonkSwap | AMM | `BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p` |
| Marinade | Liquid Staking | `MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD` |
| Sanctum | Liquid Staking | `5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx` |
| OKX DEX | Aggregation Router V2 | `6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma` |
| DFlow | Swap Aggregator V4 | `DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH` |
| GoonFi | DEX | `goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j` |
| Saros | AMM Swap | `SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr` |
| SolFi | Swap | `SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe` |
| SolFi | Swap V2 | `SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF` |
| HumidiFi | Swap | `9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp` |
| Tessera V | Swap | `TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH` |
| Byreal CLMM | Swap | `REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2` |
| Heaven DEX | Swap | `HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o` |
| Lifinity | AMM V2 | `2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c` |
| Moonshot | Token Launchpad | `MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG` |
| Obric | V2 | `coUnmi3oBUtwtd9fU42p6jg75UmdnNMgBQMedGNYEAs` |
| Obric | V3 | `ob2wXVFGiWXkPwYv8CSpPMm5m3NR7DjSrVJAHGSb9Gu` |
| Plasma | AMM | `srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi` |
| Boop | Token Launchpad | `boop8hVGQGqehUK2iVEMEnMrL5E7ZbEYKRBBRCkjGrf` |
| DumpFun | Token Launchpad | `BSfD6SHZigAfDWSjzD5Q41jw8LmKwtmjskCH9EXqjkAQ` |
| MagicEden | NFT Marketplace M2 | `M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K` |
| MagicEden | NFT Marketplace M3 | `M3mxk5W2tt27WGT7THox7PmgRDp4m6NEhL5xvxrBfS1` |
| Aldrin | AMM Swap | `AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6` |
| Serum | DEX V3 | `9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin` |
| Metaplex | Token Metadata | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s` |
| Metaplex | Bubblegum (cNFTs) | `BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY` |
| SPL | Token Program | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` |
| SPL | Token-2022 | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` |
| SPL | Token Swap | `SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8` |
| SPL | Token Lending | `LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi` |
| Native | System Program | `11111111111111111111111111111111` |
| Native | Stake Program | `Stake11111111111111111111111111111111111111` |
| Native | Vote Program | `Vote111111111111111111111111111111111111111` |

> Want another program supported? Open an issue or PR with the program ID and IDL link.

---

## Import crate

Add to your `Cargo.toml`:

```toml
[dependencies]
substreams-solana-idls = { git = "https://github.com/pinax-network/substreams-solana-idls", tag = "v1.0.0" }
```

## Workspace Layout

This repository is a single Rust crate with protocol modules under `src/`.

```
substreams-solana-idls/
├── src/lib.rs            # Top-level module exports
├── src/common            # Shared parsing helpers and ParseError
├── src/<protocol>        # Protocol-specific decoders
└── tests/                # Integration tests by protocol
```

Most protocol modules follow this internal shape:

```
src/<protocol>/
├── mod.rs                # Module declarations + PROGRAM_ID constants
├── instructions.rs       # Instruction enum + unpack(&[u8])
├── events.rs             # Event enum/structs + unpack(&[u8])
├── accounts.rs           # Account parsing helpers (when available)
└── <variant>/...         # Versioned modules (v4, v6, clmm, amm, etc.)
```

Examples:
- `src/jupiter/v4/*` and `src/jupiter/v6/*`
- `src/raydium/amm/v4/*`, `src/raydium/clmm/v3/*`, `src/raydium/cpmm/*`
- `src/pumpfun/bonding_curve/*` and `src/pumpfun/amm/*`

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

Swap `pumpfun` for any protocol module exported in `src/lib.rs`.

## Build & Test

```bash
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo check --workspace --target wasm32-unknown-unknown
```

---

## Contributing

* Add a new module under `src/<protocol>/` with `instructions.rs`, `events.rs`, and `mod.rs`.
* Register the module in `src/lib.rs`.
* Follow existing modules for structure & doc style.
* Keep dependencies minimal (`borsh`, `solana_program` only).

PRs and issues welcome!
