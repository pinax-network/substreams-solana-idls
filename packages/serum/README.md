# Serum

Serum DEX V3 — Solana's on-chain central limit order book (CLOB).

## Program

| | |
|---|---|
| **Program ID** | `9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin` |
| **Framework** | Custom (non-Anchor, u32 LE discriminators) |
| **Docs** | [docs.projectserum.com](https://docs.projectserum.com) |

## Instructions Decoded

- `InitializeMarket` — Initialize a new market
- `NewOrder` — Place a new order (V1)
- `MatchOrders` — Match orders in the orderbook
- `ConsumeEvents` — Consume events from the event queue
- `CancelOrder` — Cancel an order (V1)
- `SettleFunds` — Settle funds for an OpenOrders account
- `CancelOrderByClientId` — Cancel order by client ID
- `DisableMarket` — Disable a market
- `SweepFees` — Sweep accumulated fees
- `NewOrderV2` — Place a new order (V2)
- `NewOrderV3` — Place a new order (V3)
- `CancelOrderV2` — Cancel an order (V2)
- `CancelOrderByClientIdV2` — Cancel order by client ID (V2)
- `SendTake` — Send a take order
- `CloseOpenOrders` — Close an OpenOrders account
- `InitOpenOrders` — Initialize an OpenOrders account
- `Prune` — Prune orders for an OpenOrders account
- `ConsumeEventsPermissioned` — Consume events (permissioned)

## Events Decoded

None.
