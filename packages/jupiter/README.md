# Jupiter

Solana's leading DEX aggregator — V4, V6, DCA, and Limit Orders.

## Programs

### Swap Aggregator V6

| | |
|---|---|
| **Program ID** | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` |
| **Framework** | Anchor |
| **Docs** | [docs.jup.ag](https://docs.jup.ag) |

### Swap Aggregator V4

| | |
|---|---|
| **Program ID** | `JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB` |
| **Framework** | Custom |
| **Docs** | [docs.jup.ag](https://docs.jup.ag) |

### DCA (Dollar-Cost Averaging)

| | |
|---|---|
| **Program ID** | `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M` |
| **Framework** | Anchor |
| **Docs** | [docs.jup.ag](https://docs.jup.ag) |

### Limit Order

| | |
|---|---|
| **Program ID** | `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu` |
| **Framework** | Anchor |
| **Docs** | [docs.jup.ag](https://docs.jup.ag) |

## Instructions Decoded

### V6

- `Route` / `RouteWithTokenLedger` — Execute swap route
- `SharedAccountsRoute` / `SharedAccountsRouteWithTokenLedger` — Route with shared accounts
- `ExactOutRoute` / `SharedAccountsExactOutRoute` — Exact output swap
- `Claim` / `ClaimToken` / `CloseToken` — Token claims
- `CreateOpenOrders` / `CreateProgramOpenOrders` — Serum/OpenBook orders
- `CreateTokenLedger` / `SetTokenLedger` — Token ledger management
- `CreateTokenAccount` — Create token account

### V4

- `Route` — Execute swap route (supports Chain, Split legs)
- `Whirlpoolswapexactoutput` / `Raydiumswapexactoutput` / `Raydiumclmmswapexactoutput`
- `Createopenorders` / `Createopenorderv2`
- Individual DEX swap instructions: `Mercurialswap`, `Tokenswap`, `Senchaswap`, `Stepswap`, `Cropperswap`, `Raydiumswap`, `Cremaswap`, `Lifinityswap`, `Marinadedeposit`, `Marinadeunstake`, `Aldrinswap`, `Aldrinv2swap`, `Whirlpoolswap`, `Invariantswap`, `Meteoraswap`, `Goosefxswap`, `Deltafiswap`, `Balansolswap`, `Marcopoloswap`, `Dradexswap`, `Lifinityv2swap`, `Raydiumclmmswap`, `Phoenixswap`
- `Claimbonk`

### DCA

- `OpenDca` / `OpenDcaV2` — Open DCA position
- `CloseDca` — Close DCA position
- `Deposit` / `Withdraw` — Manage DCA funds
- `WithdrawFees` — Withdraw accumulated fees
- `InitiateFlashFill` / `FulfillFlashFill` — Flash fill execution
- `InitiateDlmmFill` / `FulfillDlmmFill` — DLMM fill execution
- `Transfer` / `EndAndClose` — Transfer and close

### Limit Order

- `InitializeOrder` — Create a limit order
- `FillOrder` — Fill an existing order
- `PreFlashFillOrder` / `FlashFillOrder` — Flash fill order
- `CancelOrder` / `CancelExpiredOrder` — Cancel orders
- `WithdrawFee` — Withdraw fees
- `InitFee` / `UpdateFee` — Fee configuration

## Events Decoded

### V6

- `SwapEvent` — Swap executed
- `FeeEvent` — Fee charged

### V4

- `SwapEvent` — Swap executed
- `FeeEvent` — Fee charged

### DCA

- `OpenedEvent` — DCA position opened
- `ClosedEvent` — DCA position closed
- `FilledEvent` — DCA order filled
- `CollectedFeeEvent` — Fee collected
- `WithdrawEvent` / `DepositEvent` — Funds withdrawn/deposited

### Limit Order

- `CreateOrderEvent` — Order created
- `CancelOrderEvent` — Order cancelled

## Accounts

### V6

- `ExactOutRouteAccounts`, `RouteAccounts`, `RouteWithTokenLedgerAccounts`
- `SharedAccountsExactOutRouteAccounts`, `SharedAccountsRouteAccounts`, `SharedAccountsRouteWithTokenLedgerAccounts`

### DCA

- `WithdrawAccounts`, `TransferAccounts`, `EndAndCloseAccounts`

### Limit Order

- `InitializeOrderAccounts`, `FillOrderAccounts`, `FlashFillOrderAccounts`
- `CancelOrderAccounts`, `CancelExpiredOrderAccounts`
