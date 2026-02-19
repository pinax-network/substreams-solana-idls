# Meteora

Solana's dynamic liquidity protocol — Pools AMM, DAMM V2, and DLMM.

## Programs

### Pools (AMM)

| | |
|---|---|
| **Program ID** | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` |
| **Framework** | Anchor |
| **Docs** | [docs.meteora.ag](https://docs.meteora.ag) |

### DAMM V2

| | |
|---|---|
| **Program ID** | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG` |
| **Framework** | Anchor |
| **Docs** | [docs.meteora.ag](https://docs.meteora.ag) |

### DLMM

| | |
|---|---|
| **Program ID** | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` |
| **Framework** | Anchor |
| **Docs** | [docs.meteora.ag](https://docs.meteora.ag) |

## Instructions Decoded

### Pools (AMM)

- `InitializePermissionedPool` / `InitializePermissionlessPool` / `InitializePermissionlessPoolWithFeeTier` — Create pools
- `InitializePermissionlessConstantProductPoolWithConfig` / `InitializePermissionlessConstantProductPoolWithConfig2` — CP pool with config
- `InitializeCustomizablePermissionlessConstantProductPool` — Customizable CP pool
- `EnableOrDisablePool` — Toggle pool status
- `Swap` — Swap tokens
- `AddBalanceLiquidity` / `AddImbalanceLiquidity` — Add liquidity
- `RemoveBalanceLiquidity` / `RemoveLiquiditySingleSide` — Remove liquidity
- `BootstrapLiquidity` — Bootstrap initial liquidity
- `SetPoolFees` / `OverrideCurveParam` — Pool configuration
- `GetPoolInfo` — Query pool info
- `CreateMintMetadata` — Create LP mint metadata
- `CreateLockEscrow` / `Lock` / `ClaimFee` — Lock LP tokens & claim fees
- `CreateConfig` / `CloseConfig` — Config management
- `UpdateActivationPoint` — Update activation point
- `WithdrawProtocolFees` — Withdraw protocol fees
- `SetWhitelistedVault` — Set whitelisted vault
- `PartnerClaimFee` — Partner fee claims

### DAMM V2

- `InitializePool` / `InitializePoolWithDynamicConfig` / `InitializeCustomizablePool` — Create pools
- `Swap` — Swap tokens
- `AddLiquidity` / `RemoveLiquidity` / `RemoveAllLiquidity` — Manage liquidity
- `CreatePosition` / `ClosePosition` — Position management
- `LockPosition` / `PermanentLockPosition` — Lock positions
- `SplitPosition` — Split a position
- `RefreshVesting` — Refresh vesting
- `CreateConfig` / `CreateDynamicConfig` / `CloseConfig` — Config management
- `CreateTokenBadge` / `CloseTokenBadge` — Token badges
- `CreateClaimFeeOperator` / `CloseClaimFeeOperator` — Fee operators
- `ClaimPositionFee` / `ClaimPartnerFee` / `ClaimProtocolFee` — Fee claims
- `ClaimReward` / `FundReward` / `InitializeReward` — Rewards
- `UpdateRewardDuration` / `UpdateRewardFunder` / `WithdrawIneligibleReward`
- `SetPoolStatus` — Update pool status

### DLMM

- `InitializeLbPair` / `InitializeLbPair2` / `InitializePermissionLbPair` — Create LB pairs
- `InitializeCustomizablePermissionlessLbPair` / `InitializeCustomizablePermissionlessLbPair2` — Customizable pairs
- `Swap` / `Swap2` / `SwapExactOut` / `SwapExactOut2` / `SwapWithPriceImpact` / `SwapWithPriceImpact2` — Swap tokens
- `AddLiquidity` / `AddLiquidity2` / `AddLiquidityByStrategy` / `AddLiquidityByStrategy2` — Add liquidity
- `AddLiquidityByStrategyOneSide` / `AddLiquidityByWeight` / `AddLiquidityOneSide` — One-side liquidity
- `AddLiquidityOneSidePrecise` / `AddLiquidityOneSidePrecise2` — Precise one-side
- `RemoveLiquidity` / `RemoveLiquidity2` / `RemoveLiquidityByRange` / `RemoveLiquidityByRange2` / `RemoveAllLiquidity` — Remove liquidity
- `RebalanceLiquidity` — Rebalance positions
- `InitializePosition` / `InitializePositionPda` / `InitializePositionByOperator` — Open positions
- `ClosePosition` / `ClosePosition2` / `ClosePositionIfEmpty` — Close positions
- `InitializeBinArray` / `InitializeBinArrayBitmapExtension` — Bin array setup
- `InitializePresetParameter` / `InitializePresetParameter2` / `ClosePresetParameter` / `ClosePresetParameter2` — Presets
- `InitializeReward` / `FundReward` / `ClaimReward` / `ClaimReward2` — Rewards
- `ClaimFee` / `ClaimFee2` — Claim fees
- `GoToABin` / `IncreaseOracleLength` / `IncreasePositionLength` / `DecreasePositionLength`
- `SetActivationPoint` / `SetPairStatus` / `SetPairStatusPermissionless`
- `SetPreActivationDuration` / `SetPreActivationSwapAddress`
- `UpdateBaseFeeParameters` / `UpdateDynamicFeeParameters`
- `UpdateFeesAndRewards` / `UpdateFeesAndReward2` / `UpdatePositionOperator`
- `UpdateRewardDuration` / `UpdateRewardFunder` / `WithdrawIneligibleReward` / `WithdrawProtocolFee`
- `CreateClaimProtocolFeeOperator` / `CloseClaimProtocolFeeOperator`
- `InitializeTokenBadge` / `MigrateBinArray` / `MigratePosition`

## Events Decoded

### Pools (AMM)

- `AddLiquidity` / `RemoveLiquidity` / `BootstrapLiquidity` — Liquidity events
- `Swap` — Swap executed
- `SetPoolFees` / `OverrideCurveParam` — Config events
- `PoolInfo` / `PoolCreated` / `PoolEnabled` — Pool lifecycle
- `TransferAdmin` / `MigrateFeeAccount` — Admin events
- `CreateLockEscrow` / `Lock` / `ClaimFee` — Lock events
- `CreateConfig` / `CloseConfig` — Config events
- `WithdrawProtocolFees` / `PartnerClaimFees` — Fee events

## Accounts

### Pools (AMM)

- Per-instruction account structs for all major operations (swap, liquidity, config, lock, etc.)

### DAMM V2

- Per-instruction account structs for all operations (pool init, swap, liquidity, positions, rewards, fees, config)

### DLMM

- Per-instruction account structs for liquidity, swap, pair initialization, rewards, and rebalancing
