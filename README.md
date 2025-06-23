# Substreams Solana IDLs

> Type-safe Solana IDLs & instruction/event decoders for Substreams modules.

## Includes

| Support | Ecosystem | Protocol | Program ID |
|---------|----------|------------|-----------------|
| 🚧       | Pump.fun | Pump Bonding-Curve | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| 🚧       | Pump.fun | PumpSwap AMM | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| 🚧       | Raydium | AMM V4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| ❌       | Raydium | AMM CP | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| ❌       | Raydium | CLMM V3 | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| ❌       | Raydium | Stable Swap AMM | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h` |
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

## Quickstart

```rust
...
use substreams_solana_idls::pumpfun;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, Error> {
    for tx in block.transactions() {
        for instruction in tx.walk_instructions() {
            match pumpfun::events::unpack(instruction.data()) {
                pumpfun::events::PumpFunEvent::Trade(trade) => {
                    /* Trade event handling logic */
                }
                _ => {},
            }
        }
    }
}
