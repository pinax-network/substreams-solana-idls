# BonkSwap

AMM with farming on Solana.

## Program

| | |
|---|---|
| **Program ID** | `BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p` |
| **Framework** | Custom |

## Instructions Decoded

- `CreatePool` — Create a new liquidity pool
- `CreateProvider` — Create a liquidity provider
- `CreateState` — Create protocol state
- `AddTokens` — Add tokens to a pool
- `AddSupply` — Add token supply
- `Swap` — Swap tokens
- `WithdrawShares` — Withdraw LP shares
- `WithdrawBuyback` / `WithdrawLpFee` / `WithdrawProjectFee` / `WithdrawMercantiFee` — Fee withdrawals
- `WithdrawRewards` — Withdraw farming rewards
- `CreateFarm` / `CreateDualFarm` / `CreateTripleFarm` — Create farming programs
- `ResetFarm` — Reset farming
- `UpdateRewardTokens` — Update reward tokens
- `UpdateFees` — Update fee parameters
- `ClosePool` — Close a pool

## Accounts

- `SwapAccounts`
