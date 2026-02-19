# Pump.fun

Solana's bonding curve token launcher and PumpSwap AMM.

## Programs

### Bonding Curve

| | |
|---|---|
| **Program ID** | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| **Framework** | Custom (non-Anchor) |

### PumpSwap AMM

| | |
|---|---|
| **Program ID** | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| **Framework** | Anchor |

## Instructions Decoded

### Bonding Curve

- `Initialize`
- `SetParams` — Configure bonding curve parameters
- `Create` — Create a new token with bonding curve
- `Buy` — Buy tokens on the bonding curve
- `Sell` — Sell tokens on the bonding curve
- `Withdraw` — Withdraw liquidity after migration

### PumpSwap AMM

- `Buy` — Buy tokens from the AMM pool
- `Sell` — Sell tokens to the AMM pool
- `CreateConfig` — Create AMM configuration
- `CreatePoolV1` / `CreatePoolV2` — Create a liquidity pool
- `Deposit` — Deposit liquidity
- `Withdraw` — Withdraw liquidity
- `Disable` — Disable a pool
- `ExtendAccount` — Extend account data
- `UpdateAdmin` — Update admin authority
- `UpdateFeeConfig` — Update fee configuration

## Events Decoded

### Bonding Curve

- `CreateEvent` — Token created
- `TradeEventV0` / `TradeEventV1` / `TradeEventV2` / `TradeEventV3` — Trade executed (versioned)
- `CompleteEvent` — Bonding curve completed (migration)
- `SetParamsEvent` — Parameters updated

### PumpSwap AMM

- `BuyEventV1` / `BuyEventV2` — Buy executed
- `SellEventV1` / `SellEventV2` — Sell executed
- `CreateConfigEvent` — Config created
- `CreatePoolEventV1` / `CreatePoolEventV2` — Pool created
- `DepositEvent` — Liquidity deposited
- `DisableEvent` — Pool disabled
- `ExtendAccountEvent` — Account extended
- `UpdateAdminEvent` — Admin updated
- `UpdateFeeConfigEvent` — Fee config updated
- `WithdrawEvent` — Liquidity withdrawn
