# Substreams IDLs

This directory contains Anchor IDLs for Solana blockchain that Substreams supports.

## Includes

| Support | Ecosystem | Protocol | Program ID |
|---------|----------|------------|-----------------|
| ✅       | Raydium | AMM V4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| ❌       | Raydium | AMM CP | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| ❌       | Raydium | CLMM V3 | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| ❌       | Raydium | Stable Swap AMM | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h` |
| 🚧       | Pump.fun | Pump Bonding-Curve | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| ❌       | Pump.fun | PumpSwap AMM | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| ❌       | Jupiter | Swap Aggregator V6 | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` |
| ❌       | Jupiter | Limit Order | `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu` |
| ❌       | Jupiter | DCA | `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M` |
| ❌       | Jupiter | Legacy Swap Routers V4/V5 | `JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB` |
| ❌       | Jupiter | Legacy Swap Routers ≤V3 | `JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph` |
| ❌       | Orca | Whirlpool AMM (CLMM v3-style) |  `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| ❌       | Orca | Token Swap V2 ("Classic" pools) - AMM V2 | `9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP` |

## Status

- ✅ Completed
- 🚧 In progress
- ❌ Not supported

## References

- <https://docs.vybenetwork.com/docs/real-time-trades>

## How to contribute?

- Compile the ABI from the source code.
- Add the ABI to the corresponding directory.
  - Only include `abi` section in the JSON file.
  - Name the file as the contract name (ex: `evm/token/ERC20.json`).
- Build `cargo build`
- Update/add new contract to `mod.rs` file(s).
- Run `cargo test` to ensure everything is working.
- Create a PR.

## Quickstart

```rust
...
use substreams_idl::raydium::Instructions;

// Iterates over successful transactions
for trx in block.transactions() {
  // Iterates over all logs in the transaction, excluding those from calls that were not recorded to the chain's state.
  // The logs are sorted by their ordinal and returned as pairs of (log, call) where call is the call that produced the log.
  for (log, call_view) in trx.logs_with_calls() {
    // -- Transfer --
    let transfer = match Transfer::decode(&log) {
        Some(transfer) => transfer,
        None => continue,
    };
    // transfer.from => 6D1D1ebe7dA598194293784252659e862d55b52c
    // transfer.to => c7bBeC68d12a0d1830360F8Ec58fA599bA1b0e9b
    // transfer.value => 3400000000
  }
}
```
