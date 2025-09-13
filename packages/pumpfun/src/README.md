# Pump.fun IDL helpers

> Program‑specific instruction & event codecs for the Pump.fun ecosystem.

---

## Supported Pump.fun programs

| Protocol | Description | Program ID |
| -------- | ----------- | ---------- |
| **Bonding‑Curve** | SPL‑Token ↔ SOL bonding pools | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| **PumpSwap AMM** | Experimental SPL‑Token ↔ SOL TWAMM | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |

---

## Crate layout

```text
pumpfun/
 ├── instructions.rs  # `PumpFunInstruction` enum + payload structs
 ├── events.rs        # `PumpFunEvent` enum + payload structs
 └── mod.rs           # re‑exports & conveniences
```

* **Discriminators** – first 8 bytes are Pump.fun‑specific, followed by Anchor’s 8 byte IDL discriminator (events only).
* **Payloads** – every instruction / event has its own struct with Borsh (de)serialization and inline field docs.

---

## Quick usage

```rust
use substreams_solana_idls::pumpfun;

let raw: &[u8] = /* tx instruction or log data */;

// Decode either instructions…
if let Ok(instr) = pumpfun::instructions::PumpFunInstruction::try_from(raw) {
    match instr {
        pumpfun::instructions::PumpFunInstruction::Buy(buy) => {
            // handle buy
        }
        _ => {}
    }
}

// …or events
if let Ok(event) = pumpfun::events::PumpFunEvent::try_from(raw) {
    if let pumpfun::events::PumpFunEvent::Trade(trade) = event {
        // handle trade event
    }
}
```
