# Obric

Obric V2 & V3 — oracle-based swap protocol on Solana.

## Programs

| | |
|---|---|
| **V2 Program ID** | `coUnmi3oBUtwtd9fU42p6jg75UmdnNMgBQMedGNYEAs` |
| **V3 Program ID** | `ob2wXVFGiWXkPwYv8CSpPMm5m3NR7DjSrVJAHGSb9Gu` |
| **Framework** | Anchor |

## V2 Instructions Decoded

- `CreatePair` / `CreatePairV2` — Create a trading pair
- `UpdateConcentration` — Update pair concentration
- `UpdateVersion` — Update pair version
- `UpdateFeeParams` — Update fee parameters
- `UpdateOracles` — Update oracle feeds
- `WithdrawFees` — Withdraw protocol fees
- `Deposit` — Deposit liquidity
- `Withdraw` — Withdraw liquidity
- `SwapXToY` / `SwapYToX` / `Swap` — Swap tokens

## V3 Instructions Decoded

- `CreatePair` — Create a trading pair
- `RefreshLendingStatus` — Refresh marginfi lending status
- `UpdateConcentration` — Update pair concentration
- `UpdateFeeParams` — Update fee parameters
- `Deposit` — Deposit liquidity
- `Withdraw` — Withdraw liquidity
- `SwapXToY` / `SwapYToX` — Swap tokens

## Accounts

- V2: `SwapXToYAccounts`, `SwapYToXAccounts`, `SwapAccounts`
- V3: `SwapXToYAccounts`, `SwapYToXAccounts`
