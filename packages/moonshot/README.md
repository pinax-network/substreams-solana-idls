# Moonshot

Token launchpad on Solana.

## Program

| | |
|---|---|
| **Program ID** | `MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG` |
| **Framework** | Anchor (old IDL format) |

## Instructions Decoded

- `TokenMint` — Create a new token with bonding curve
- `Buy` — Buy tokens from the bonding curve
- `Sell` — Sell tokens back to the bonding curve
- `MigrateFunds` — Migrate funds after graduation
- `ConfigInit` — Initialize protocol configuration
- `ConfigUpdate` — Update protocol configuration

## Events

- `TradeEvent` — Emitted on buy/sell trades
- `MigrationEvent` — Emitted on fund migration

## Accounts

- `ConfigAccount` — Protocol configuration state
- `CurveAccount` — Bonding curve state per token
