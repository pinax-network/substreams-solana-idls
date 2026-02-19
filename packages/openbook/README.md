# Openbook V2

Central limit order book (CLOB) on Solana.

## Program

| | |
|---|---|
| **Program ID** | `opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb` |
| **Framework** | Anchor |

## Instructions Decoded

- `CreateMarket` — Create a new market for a token pair
- `CloseMarket` — Close a market
- `CreateOpenOrdersIndexer` / `CloseOpenOrdersIndexer` — Manage open orders indexers
- `CreateOpenOrdersAccount` / `CloseOpenOrdersAccount` — Manage open orders accounts
- `PlaceOrder` / `PlaceOrderPegged` / `PlaceOrders` / `PlaceTakeOrder` — Place orders
- `EditOrder` / `EditOrderPegged` — Edit existing orders
- `CancelOrder` / `CancelOrderByClientOrderId` / `CancelAllOrders` — Cancel orders
- `CancelAllAndPlaceOrders` — Atomic cancel-and-replace
- `ConsumeEvents` / `ConsumeGivenEvents` — Crank event processing
- `Deposit` / `Refill` — Deposit funds
- `SettleFunds` / `SettleFundsExpired` — Settle matched orders
- `SweepFees` — Collect protocol fees
- `SetDelegate` — Set account delegate
- `SetMarketExpired` — Mark market as expired
- `PruneOrders` — Prune expired orders
- `StubOracleCreate` / `StubOracleClose` / `StubOracleSet` — Test oracle management

## Events

- `DepositLog` — Deposit event
- `FillLog` — Order fill event
- `MarketMetaDataLog` — Market metadata
- `TotalOrderFillEvent` — Aggregate fill
- `SetDelegateLog` — Delegate change
- `SettleFundsLog` — Settlement event
- `SweepFeesLog` — Fee sweep event
- `OpenOrdersPositionLog` — Position snapshot

## Accounts

- `CreateMarketAccounts`, `PlaceOrderAccounts`, `SettleFundsAccounts`, etc.
