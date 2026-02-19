# Sanctum

Solana LST (Liquid Staking Token) swap router. Enables swaps between any LSTs via intermediate stake accounts.

## Program

| | |
|---|---|
| **Program ID** | `5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx` |
| **Framework** | Custom (non-Anchor) |
| **Source** | [GitHub](https://github.com/igneous-labs/sanctum-router-sdk) |
| **Docs** | [sanctum.so](https://www.sanctum.so) |

## Instructions Decoded

### User
- `StakeWrappedSol` — Wrap SOL → wSOL → stake → receive LST
- `SwapViaStake` — Swap one LST for another via stake account
- `DepositStake` — Deposit a stake account → receive LST
- `WithdrawWrappedSol` — Unwrap LST → wSOL
- `PrefundWithdrawStake` — Pre-fund a withdraw-stake operation
- `PrefundSwapViaStake` — Pre-fund a swap-via-stake operation

### Admin
- `CreateFeeTokenAccount` — Create fee token account for a mint
- `CloseFeeTokenAccount` — Close fee token account
- `WithdrawFees` — Withdraw accumulated protocol fees
