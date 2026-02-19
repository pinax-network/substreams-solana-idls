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
| ✅ | Jupiter | Limit Order | `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu` |
| ✅ | Jupiter | DCA | `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M` |
| ✅ | Raydium | AMM V4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| ✅ | Raydium | AMM CP | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| ✅ | Raydium | CLMM V3 | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| ✅ | Raydium | LaunchPad | `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj` |
| ✅ | Raydium | Stable AMM Anchor | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h` |
| ✅ | Orca | Whirlpool (CLMM) | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| ✅ | Meteora | Pools | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` |
| ✅ | Meteora | DAMM V2 | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG` |
| ✅ | Meteora | DLLM | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` |
| ✅ | Openbook | V2 Order Book | `opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb` |
| ✅ | Phoenix | on-chain CLOB | `PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY` |
| ✅ | Drift | V2 Perpetual | `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH` |
| ✅ | Tensor | NFT Marketplace | `TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp` |
| ✅ | Stabble | Stable Swap | `swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW` |
| ✅ | PancakeSwap | AMM Swap | `HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq` |
| ✅ | BonkSwap | AMM | `BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p` |
| ✅ | Marinade | Liquid Staking | `MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD` |
| ✅ | Sanctum | Liquid Staking | `5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx` |
| ✅ | OKX DEX | Aggregation Router V2 | `6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma` |
| ✅ | DFlow | Swap Aggregator V4 | `DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH` |
| ✅ | Saros | AMM Swap | `SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr` |
| ✅ | SolFi | Swap | `SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe` |
| ✅ | SolFi | Swap V2 | `SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF` |
| ✅ | HumidiFi | Swap | `9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp` |
| ✅ | Tessera V | Swap | `TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH` |
| ✅ | Byreal CLMM | Swap | `REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2` |
| ✅ | Heaven DEX | Swap | `HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o` |

**Legend**   ✅ done · ❌ planned / help wanted

> Want another program supported? Open an issue or PR with the program ID and IDL link.

---

## Import crate

Add to your `Cargo.toml`:

```toml
[dependencies]
substreams-solana-idls = { git = "https://github.com/pinax-network/substreams-solana-idls", tag = "v0.6.x" }
```

## Module Layout

```
substreams_solana_idls::
├── <protocol>::PROGRAM_ID        // Program ID constant
├── <protocol>::instructions      // Instruction enum + `unpack(&[u8])`
├── <protocol>::accounts          // Account structs + `get_*_accounts`
└── <protocol>::events            // Event enum + `unpack(&[u8])`
```

Every protocol lives in its own module so you only pull in what you need.

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

Swap `pumpfun` for any supported protocol and you get the same ergonomic API.

---

## Contributing

* Add a new folder under `src/<protocol>` with `instructions.rs`, `events.rs`, and `mod.rs`.
* Follow existing modules for structure & doc style.
* Keep dependencies minimal (`borsh`, `solana_program` only).

PRs and issues welcome!
