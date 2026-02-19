# Tensor

Leading Solana NFT marketplace. Supports cNFT, pNFT (legacy), Token-2022, WNS, and Metaplex Core.

## Program

| | |
|---|---|
| **Program ID** | `TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp` |
| **Framework** | Anchor |
| **Source** | [GitHub](https://github.com/tensor-foundation/marketplace) |
| **Docs** | [docs.tensor.trade](https://docs.tensor.trade) |

## Instructions Decoded (36)

### Listings & Bids (core)
- `List`, `Delist`, `Edit`, `Buy`, `BuySpl`, `CloseExpiredListing`
- `Bid`, `CancelBid`, `CloseExpiredBid`
- `TakeBidMetaHash`, `TakeBidFullMeta`

### Legacy (pNFT)
- `ListLegacy`, `DelistLegacy`, `BuyLegacy`, `BuyLegacySpl`
- `CloseExpiredListingLegacy`, `TakeBidLegacy`

### Token-2022
- `ListT22`, `DelistT22`, `BuyT22`, `BuyT22Spl`
- `CloseExpiredListingT22`, `TakeBidT22`

### WNS
- `ListWns`, `DelistWns`, `BuyWns`, `BuyWnsSpl`
- `CloseExpiredListingWns`, `TakeBidWns`

### Metaplex Core
- `ListCore`, `DelistCore`, `BuyCore`, `BuyCoreSpl`
- `CloseExpiredListingCore`, `TakeBidCore`

### Admin
- `TcompNoop` — Event relay

## Events Decoded
- `MakeEvent` — Listing or bid created/edited
- `TakeEvent` — Listing bought or bid accepted (includes fees breakdown)
