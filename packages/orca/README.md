# Orca

Solana's Whirlpool concentrated liquidity market maker (CLMM).

## Program

| | |
|---|---|
| **Program ID** | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| **Framework** | Anchor |
| **Docs** | [docs.orca.so](https://docs.orca.so) |

## Instructions Decoded

### Pool & Config
- `InitializeConfig` — Initialize Whirlpool config
- `InitializePool` / `InitializePoolV2` / `InitializePoolWithAdaptiveFee` — Create pool
- `InitializeFeeTier` — Create fee tier
- `InitializeAdaptiveFeeTier` — Create adaptive fee tier
- `InitializeTickArray` — Initialize tick array
- `InitializeConfigExtension` — Initialize config extension

### Positions
- `OpenPosition` / `OpenPositionWithMetadata` / `OpenPositionWithTokenExtensions` — Open position
- `ClosePosition` / `ClosePositionWithTokenExtensions` — Close position
- `IncreaseLiquidity` / `IncreaseLiquidityV2` — Add liquidity
- `DecreaseLiquidity` / `DecreaseLiquidityV2` — Remove liquidity
- `UpdateFeesAndRewards` — Update fees and rewards
- `ResetPositionRange` — Reset position range
- `LockPosition` / `TransferLockedPosition` — Lock management

### Bundles
- `InitializePositionBundle` / `InitializePositionBundleWithMetadata` — Create position bundle
- `OpenBundledPosition` / `CloseBundledPosition` — Manage bundled positions
- `DeletePositionBundle` — Delete bundle

### Swaps
- `Swap` / `SwapV2` — Single-hop swap
- `TwoHopSwap` / `TwoHopSwapV2` — Two-hop swap

### Fee Collection
- `CollectFees` / `CollectFeesV2` — Collect position fees
- `CollectProtocolFees` / `CollectProtocolFeesV2` — Collect protocol fees
- `CollectReward` / `CollectRewardV2` — Collect rewards

### Admin
- `SetDefaultFeeRate` / `SetDefaultBaseFeeRate` / `SetFeeRate` — Fee rates
- `SetDefaultProtocolFeeRate` / `SetProtocolFeeRate` — Protocol fee rates
- `SetFeeAuthority` / `SetCollectProtocolFeesAuthority` / `SetDelegatedFeeAuthority` — Authorities
- `SetRewardAuthority` / `SetRewardAuthorityBySuperAuthority` / `SetRewardEmissionsSuperAuthority`
- `SetRewardEmissions` / `SetRewardEmissionsV2` — Reward emissions
- `InitializeReward` / `InitializeRewardV2` — Initialize rewards
- `SetInitializePoolAuthority` — Pool authority
- `SetPresetAdaptiveFeeConstants` — Adaptive fee presets
- `SetFeeRateByDelegatedFeeAuthority` — Delegated fee rate
- `SetConfigExtensionAuthority` / `SetTokenBadgeAuthority` — Extension authorities
- `InitializeTokenBadge` / `DeleteTokenBadge` — Token badges

## Events Decoded

- `PoolInitialized` — Pool created
- `Traded` — Swap executed
- `LiquidityIncreased` — Liquidity added
- `LiquidityDecreased` — Liquidity removed
