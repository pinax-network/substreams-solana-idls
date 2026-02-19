# Raydium

Solana's leading AMM and liquidity protocol — AMM V4, CPMM, CLMM V3, Stable, and LaunchPad.

## Programs

### AMM V4

| | |
|---|---|
| **Program ID** | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| **Framework** | Custom (non-Anchor) |
| **Docs** | [docs.raydium.io](https://docs.raydium.io) |

### CPMM (CP-Swap)

| | |
|---|---|
| **Program ID** | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| **Framework** | Anchor |
| **Docs** | [docs.raydium.io](https://docs.raydium.io) |

### CLMM V3

| | |
|---|---|
| **Program ID** | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| **Framework** | Anchor |
| **Docs** | [docs.raydium.io](https://docs.raydium.io) |

### Stable AMM

| | |
|---|---|
| **Program ID** | `5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h` |
| **Framework** | Anchor |
| **Docs** | [docs.raydium.io](https://docs.raydium.io) |

### LaunchPad

| | |
|---|---|
| **Program ID** | `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj` |
| **Framework** | Anchor |
| **Docs** | [docs.raydium.io](https://docs.raydium.io) |

## Instructions Decoded

### AMM V4

- `Initialize` / `Initialize2` — Initialize AMM pool
- `Deposit` / `Withdraw` — Add/remove liquidity
- `SwapBaseIn` / `SwapBaseOut` — Swap tokens
- `MonitorStep` — Monitor step for AMM
- `MigrateToOpenBook` — Migrate to OpenBook
- `SetParams` — Set pool parameters
- `WithdrawPnl` / `WithdrawSrm` — Withdraw PnL / SRM
- `SimulateInfo` — Simulate swap info
- `AdminCancelOrders` — Admin cancel orders
- `CreateConfigAccount` / `UpdateConfigAccount` — Config management
- `PreInitialize` — Pre-initialize pool

### CPMM

- `CreateAmmConfig` / `UpdateAmmConfig` — AMM config management
- `Initialize` / `InitializeWithPermission` — Initialize pool
- `Deposit` / `Withdraw` — Add/remove liquidity
- `SwapBaseInput` / `SwapBaseOutput` — Swap tokens
- `CollectFundFee` / `CollectProtocolFee` / `CollectCreatorFee` — Fee collection
- `CreatePermissionPda` / `ClosePermissionPda` — Permission management
- `UpdatePoolStatus` — Update pool status

### CLMM V3

- `CreateAmmConfig` / `UpdateAmmConfig` — Config management
- `CreatePool` — Create concentrated liquidity pool
- `OpenPosition` / `OpenPositionV2` / `OpenPositionWithToken22Nft` — Open position
- `ClosePosition` — Close position
- `IncreaseLiquidity` / `IncreaseLiquidityV2` — Add liquidity
- `DecreaseLiquidity` / `DecreaseLiquidityV2` — Remove liquidity
- `Swap` / `SwapV2` / `SwapRouterBaseIn` — Swap tokens
- `InitializeReward` / `SetRewardParams` / `UpdateRewardInfos` — Reward management
- `CollectFundFee` / `CollectProtocolFee` / `CollectRemainingRewards` — Fee collection
- `CreateOperationAccount` / `UpdateOperationAccount` / `UpdatePoolStatus` — Operations
- `CreateSupportMintAssociated` / `TransferRewardOwner`

### Stable AMM

- `Initialize` — Initialize stable pool
- `Deposit` / `Withdraw` — Add/remove liquidity
- `SwapBaseIn` / `SwapBaseOut` — Swap tokens
- `PreInitialize` — Pre-initialize pool

### LaunchPad

- `BuyExactIn` / `BuyExactOut` — Buy tokens
- `SellExactIn` / `SellExactOut` — Sell tokens

## Events Decoded

### AMM V4 (Logs)

- `InitLog` — Pool initialized
- `DepositLog` / `WithdrawLog` — Liquidity added/removed
- `SwapBaseInLog` / `SwapBaseOutLog` — Swap executed

### CPMM

- `LpChangeEvent` — Liquidity changed
- `SwapEventV1` / `SwapEventV2` — Swap executed

### CLMM V3

- `PoolCreatedEvent` — Pool created
- `SwapEvent` — Swap executed
- `CreatePersonalPositionEvent` — Position created
- `IncreaseLiquidityEvent` / `DecreaseLiquidityEvent` — Liquidity changed
- `LiquidityCalculateEvent` / `LiquidityChangeEvent` — Liquidity calculations
- `CollectPersonalFeeEvent` / `CollectProtocolFeeEvent` — Fees collected
- `ConfigChangeEvent` / `UpdateRewardInfosEvent` — Config/reward updates

### Stable AMM

- `SwapEvent` — Swap executed

## Accounts

### AMM V4

- `SwapBaseAccounts`

### CPMM

- `ClosePermissionPdaAccounts`, `CollectCreatorFeeAccounts`, `CollectFundFeeAccounts`, `CollectProtocolFeeAccounts`
- `CreateAmmConfigAccounts`, `CreatePermissionPdaAccounts`
- `DepositAccounts`, `InitializeAccounts`, `InitializeWithPermissionAccounts`
- `SwapBaseInputAccounts`, `SwapBaseOutputAccounts`
- `UpdateAmmConfigAccounts`, `UpdatePoolStatusAccounts`, `WithdrawAccounts`

### CLMM V3

- `ClosePositionAccounts`, `CollectFundFeeAccounts`, `CollectProtocolFeeAccounts`, `CollectRemainingRewardsAccounts`
- `CreateAmmConfigAccounts`, `CreateOperationAccountAccounts`, `CreatePoolAccounts`, `CreateSupportMintAssociatedAccounts`
- `DecreaseLiquidityAccounts`, `DecreaseLiquidityV2Accounts`
- `IncreaseLiquidityAccounts`, `IncreaseLiquidityV2Accounts`
- `InitializeRewardAccounts`
- `OpenPositionAccounts`, `OpenPositionV2Accounts`, `OpenPositionWithToken22NftAccounts`
- `SetRewardParamsAccounts`, `SwapAccounts`, `SwapRouterBaseInAccounts`, `SwapV2Accounts`
- `TransferRewardOwnerAccounts`, `UpdateAmmConfigAccounts`, `UpdateOperationAccountAccounts`, `UpdatePoolStatusAccounts`
