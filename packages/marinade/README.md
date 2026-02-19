# Marinade Finance

Solana's leading liquid staking protocol. Stake SOL, receive mSOL.

## Program

| | |
|---|---|
| **Program ID** | `MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD` |
| **Framework** | Anchor 0.27 |
| **Source** | [GitHub](https://github.com/marinade-finance/liquid-staking-program) |
| **Docs** | [docs.marinade.finance](https://docs.marinade.finance) |

## Instructions Decoded

### User
- `Deposit` — Stake SOL, receive mSOL
- `DepositStakeAccount` — Deposit an existing stake account, receive mSOL
- `LiquidUnstake` — Instantly unstake mSOL → SOL via liquidity pool
- `AddLiquidity` — Add SOL to the SOL/mSOL liquidity pool
- `RemoveLiquidity` — Burn LP tokens to withdraw SOL + mSOL
- `OrderUnstake` — Start delayed unstake (receive ticket)
- `Claim` — Claim SOL from completed unstake ticket
- `WithdrawStakeAccount` — Withdraw as stake account (burns mSOL)

### Admin / Crank
- `Initialize`, `ChangeAuthority`, `ConfigMarinade`, `ConfigLp`
- `AddValidator`, `RemoveValidator`, `SetValidatorScore`, `ConfigValidatorSystem`
- `StakeReserve`, `UpdateActive`, `UpdateDeactivated`, `DeactivateStake`
- `EmergencyUnstake`, `PartialUnstake`, `MergeStakes`, `Redelegate`
- `Pause`, `Resume`, `ReallocateValidatorScore`, `RebalanceStake`

## Events Decoded
- `DepositEvent` — SOL deposited, mSOL minted
- `DepositStakeAccountEvent` — Stake account deposited
- `LiquidUnstakeEvent` — mSOL liquid unstaked to SOL
- `AddLiquidityEvent` — SOL added to liquidity pool
- `RemoveLiquidityEvent` — LP tokens burned
- `WithdrawStakeAccountEvent` — Stake account withdrawn
