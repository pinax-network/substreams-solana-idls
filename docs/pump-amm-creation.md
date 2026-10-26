# Pump AMM creation layouts

`pumpfun::amm::instructions::unpack` and `events::unpack` support the historical
V2 and current creation layouts with the same on-chain discriminators. New V3
variants preserve the V2 fields in a public `base` member and expose the exact
new fields instead of discarding trailing bytes.

The current layout follows the official Pump IDL at
[pump-public-docs commit 8cda1fa](https://github.com/pump-fun/pump-public-docs/blob/8cda1fa30ea658b20909d8aedf002047119388d2/idl/pump_amm.json),
Git blob `d907b16a91f035aeb3c3182bfa0972a91bf23a86` (observed 2026-10-26).
`OptionBool` is a one-field `bool` tuple struct and `OptionU64` a one-field `u64`
tuple struct. Neither is a Borsh option, so neither carries an option tag.

| Decoder result | Total bytes including discriminator(s) | Added fields after V2 |
| --- | --- | --- |
| `CreatePoolV2` | 58 | Existing fields, unchanged |
| `CreatePoolV3` | 70 | `is_mayhem_mode`, `is_cashback_coin`, `creator_fee_bps`, `can_edit_creator_fee`, `is_holder_reward` |
| `CreatePoolEventV2` | 341 | Existing fields, unchanged |
| `CreatePoolEventV3` | 352 | `is_mayhem_mode`, `creator_fee_bps`, `can_edit_creator_fee`, `is_holder_reward` |

The exact supported payload lengths select variants. Truncated/extra bytes and
invalid Borsh booleans fail. An exact V2 prefix is valid V2; these shared
discriminators cannot distinguish historical V2 from a buffer truncated exactly
to the older complete layout. Previously rejected V1 creation remains rejected.
Instruction V3 is appended to the Rust enum so its existing derived Borsh enum
variant numbers remain unchanged. Exhaustive Rust matches need a new arm; swap
decoders, account layouts and existing creation structs are unchanged.

## Captured mainnet evidence

The four small raw payload files in `tests/pumpfun/fixtures/create-pool-*` were
extracted without modifying bytes from complete successful transactions fetched
from `https://api.mainnet-beta.solana.com` using `getBlock` on 2026-10-26,
with `encoding: json`, `transactionDetails: full`,
`maxSupportedTransactionVersion: 1`, and `commitment: finalized`.
The instruction is the `create_pool`
invocation and the event is its `CreatePoolEvent` self-CPI, with both Anchor
prefixes intact. No keys or private API credentials were used.

| Layout | Slot / full-block transaction index | Transaction signature |
| --- | --- | --- |
| V2 | 349999876 / 1546 | `5rg7xHWAG6jDohDMbHEboHw3z7tDpoXdiLWRMTQ9fcFEWKEMBUE9nuyJvssA6xA3ZfV8b1ppGPreC5UzXtBX5omX` |
| Current | 461766718 / 62 | `4kEbHbpER2yoxyjFz5Gf9UxQTckXEFMcbqZTcRbVEXCZhJssSYXCpZrqfBFCpeFoQgw77w36zox7iKf2YtzU6uBb` |

Tests require nonempty typed results, matching initialization/event fields and
an exact Borsh payload roundtrip. They pin current option-wrapper values, reject
invalid lengths/booleans/discriminators, and retain V2 output type and enum
serialization. Raw-byte decoding does not itself prove transaction commitment;
consumers must check the transaction and invocation evidence.

Fixture integrity (SHA-256):

| File | SHA-256 |
| --- | --- |
| `create-pool-current.event.bin` | `7c391f01f499a33600b5eacb8a937c1b58c794253eb81d062f70bb2abcbe8eb9` |
| `create-pool-current.instruction.bin` | `96142772feea27ae7c4badf50e64687d52887975469b264d5643f9b77ab63fe9` |
| `create-pool-v2.event.bin` | `eb930a38c53730dba3b84b97042a058fd8cc58f9715440f2c0338d93d1dfe450` |
| `create-pool-v2.instruction.bin` | `9e1479bd2c4d9e7297cc741fa28ce8a5ef92bd445ffdac0ee0b103178ce65b83` |
