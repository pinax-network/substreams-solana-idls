# Pool and market initialization decoders

These APIs decode instruction bytes and their ordered accounts. They do not prove
transaction success, successful parent invocations, account ownership, or that a
pool is new. Consumers must establish those facts before persisting metadata.
Account helpers validate the required positions and 32-byte keys; they permit
remaining accounts when the program contract does. Instruction parsers reject
unknown discriminators, incomplete fields and trailing bytes, except for the two
explicitly documented Pump `create_v2` EOF boundaries described below.

This page records the additive initializer coverage introduced after revision
`5c7a36d416a75cdd8d3affd51d292d2b88426776`. It is not a claim that every protocol
in the crate's swap catalog has a complete public initialization contract.
Existing swap APIs and previously serialized enum variant indices are preserved.

## Entry points

The usual `instructions::unpack` function returns the added typed variant.
`accounts::get_*_accounts` resolves the matching accounts from an `InstructionView`.
The protocol module names below are relative to `substreams_solana_idls`.

| Module | Instruction variants | Account helpers / required prefix |
| --- | --- | --- |
| `byreal::clmm` | `CreatePool` | `get_create_pool_accounts`, 16 |
| `pancakeswap` | `CreatePool` | `get_create_pool_accounts`, 13 |
| `raydium::launchpad` | `Initialize`, `InitializeV2`, `InitializeWithToken2022` | `get_initialize_accounts` / `get_initialize_v2_accounts`, 18; `get_initialize_with_token_2022_accounts`, 15 |
| `raydium::clmm::v3` | `CreateCustomizablePool`, `CreateCustomizablePoolV2`, `CreatePermissionedPool` | corresponding `get_create_*_accounts`, 13 / 14 / 15 |
| `phoenix` | `PhonenixInstruction::InitializeMarket` | `get_initialize_market_accounts`, 10; existing enum spelling retained |
| `darklake` | existing `InitializePool` | `get_initialize_pool_accounts`, 25 |
| `plasma` | existing `InitializePool` | `get_initialize_pool_accounts`, 10 |
| `moonshot` | existing `TokenMint` | `get_token_mint_accounts`, 11 |
| `boop` | existing `DeployBondingCurve`, `DeployBondingCurveFallback` | `get_deploy_bonding_curve_accounts` / `get_deploy_bonding_curve_fallback_accounts`, 10 |
| `dumpfun` | existing `Mint` | `get_mint_accounts`, 11 |
| `pumpfun::bonding_curve` | `CreateV2` | `get_create_v2_accounts`, 16; `get_create_v2_quote_accounts` for conditional token quote |
| `saros` | `Initialize` | `get_initialize_accounts`, 8 |
| `spl::token_swap` | additive `unpack_initialize` returns `InitializeInstruction` | `get_initialize_accounts`, 8 |
| `serum` | additive `unpack_initialize_market` returns `InitializeMarketInstruction` | existing `get_initialize_market_accounts` resolves the first 9; canonical execution also requires rent at index 9 |
| `stabble` | `Initialize` | `get_initialize_accounts`, 6; `get_initialize_mints` resolves ordered remaining asset mints |

The strict Serum entry point consumes the canonical version byte (0), little-endian
instruction tag (0) and exactly 34 payload bytes. The older generic unversioned
raw-payload decoder is unchanged. Likewise, the strict SPL Token Swap entry point
consumes tag 0 and exactly 97 payload bytes; the older generic unit `Initialize`
variant is unchanged. Use the new entry points when establishing initialization.

## Identity and metadata boundaries

- SPL Token Swap and Saros initializers contain two reserve token accounts and an
  LP mint, but no underlying mint accounts. Resolve underlying mint identities
  from trustworthy token-account context; never use the LP mint as an asset mint.
- Stabble is the weighted swap program. Its fixed `mint` account is the LP mint,
  and its fixed `vault` is shared vault state. Underlying mint accounts are the
  ordered remaining accounts. Match their count to both `weights` and `max_caps`.
  Per-asset vaults, decimals and token program owners need separate account evidence.
- Pump `create_v2` has 16 fixed accounts. No remaining accounts means native SOL;
  otherwise at least three are required: quote mint, associated quote reserve,
  quote token program. Further quote-control and ancestor-curve accounts are
  checked by the executing program. The helper does not reimplement those checks.
- The official Pump contract accepts EOF before `creator_fee_bps` and before
  `is_holder_reward`. `unpack` accepts exactly 0, 8 or 9 suffix bytes after the
  fixed prefix, defaulting omitted fields to zero/false. Other partial fields and
  trailing bytes fail. Serialization emits the complete current struct.
- Boop deployment and Dumpfun mint create token/native-SOL bonding pools. Boop
  exposes a separate SOL vault. Dumpfun does not expose a separate native vault
  in its mint accounts; do not invent one. Native SOL is not an SPL mint.
- Boop's separate Meteora/Raydium migration instructions create destination
  program pools. Attribute their invoked initializers to those destination
  programs, not to a new Boop-owned pool.
- Pool decimals, deployment dates and historical availability are not inferred
  from source publication dates. A creation decoder cannot discover pools whose
  creation predates the indexed range.

## Source contracts

All references were inspected on 2026-10-26. Bundled IDLs below are existing
reference inputs at the pinned baseline revision, not newly inferred layouts.

| Coverage | Pinned reference |
| --- | --- |
| Byreal | [bundled CLMM IDL](https://github.com/pinax-network/substreams-idls/blob/5c7a36d416a75cdd8d3affd51d292d2b88426776/src/byreal/clmm/idl.json), `create_pool` |
| PancakeSwap | [bundled IDL](https://github.com/pinax-network/substreams-idls/blob/5c7a36d416a75cdd8d3affd51d292d2b88426776/src/pancakeswap/idl.json), `create_pool` |
| Raydium Launchpad | [bundled IDL](https://github.com/pinax-network/substreams-idls/blob/5c7a36d416a75cdd8d3affd51d292d2b88426776/src/raydium/launchpad/idl.json), three initialize variants |
| Raydium CLMM customizable | [official SDK](https://github.com/raydium-io/raydium-sdk-V2/blob/73e95fcb3cf5df55637286a573655e5f64182b62/src/raydium/clmm/instrument.ts), V1/V2 layouts and discriminators; [program fee enum](https://github.com/raydium-io/raydium-clmm/blob/edfc5c0a0cd8084ed694668391174a35b5246cf5/programs/amm/src/states/pool_fee.rs) |
| Raydium CLMM permissioned | [program source](https://github.com/raydium-io/raydium-clmm/blob/edfc5c0a0cd8084ed694668391174a35b5246cf5/programs/amm/src/instructions/create_permissioned_pool.rs); [October 22 rename/removal notes](https://github.com/raydium-io/raydium-docs-v1/blob/36823847c22064f9ba0aa4d931f2a181fbbcd5a5/products/clmm/instructions.mdx) |
| Phoenix | [bundled Shank IDL](https://github.com/pinax-network/substreams-idls/blob/5c7a36d416a75cdd8d3affd51d292d2b88426776/src/phoenix/idl.json), tag 100 |
| Moonshot | [bundled IDL](https://github.com/pinax-network/substreams-idls/blob/5c7a36d416a75cdd8d3affd51d292d2b88426776/src/moonshot/idl.json), `tokenMint` |
| Darklake | [official SDK account metas](https://github.com/darklakefi/sdk-on-chain/blob/dab51a2a5180127228ce760432f61001fb26d3f7/src/account_metas.rs), `DarklakeAmmInitializePool`; matching construction in the SDK's AMM implementation |
| Plasma | [official IDL](https://github.com/Ellipsis-Labs/plasma/blob/86fb3503e3a02c1dea2e8b6c2ed156d61a0b4a1b/idl/plasma.json), `InitializePool` tag 6 |
| Pump bonding | [official IDL](https://github.com/pump-fun/pump-public-docs/blob/8cda1fa30ea658b20909d8aedf002047119388d2/idl/pump.json), `create_v2`, including remaining-account and EOF documentation |
| SPL Token Swap | [official instruction source](https://github.com/solana-labs/solana-program-library/blob/264ca72de06b0c2b45c0b15d298000fe3f82db2e/token-swap/program/src/instruction.rs) |
| Saros | [official SDK instruction builder](https://github.com/saros-xyz/saros-sdk/blob/60a36e36f72a7a29f68dbd83c31b2565482b345c/src/swap/sarosSwapIntructions.js) |
| Serum | [official instruction source](https://github.com/project-serum/serum-dex/blob/92992b308885f5323b3f51eb1a0c899e35c62cb3/dex/src/instruction.rs) |
| Stabble | [official initializer](https://github.com/stabbleorg/amm_v1/blob/ecae315d90d9b056545d1ec86679e6cb40090217/programs/weighted-swap/src/instructions/initialize.rs); [official SDK remaining accounts](https://github.com/stabbleorg/amm-sdk/blob/b3ce8a43aa4601b5bfa6c8daeba9e108ab364d2b/packages/sdk/src/programs/weighted-swap.ts) |

Boop and Dumpfun account contracts come from the public on-chain Anchor IDL
responses for the exact program constants in `src/boop/mod.rs` and
`src/dumpfun/mod.rs`. Relevant instruction extracts, retrieval URLs, response
program IDs, retrieval date and complete-response SHA-256 hashes are retained in
[`tests/fixtures/pool-initialization`](../tests/fixtures/pool-initialization/).
The service reports `type: anchor` and `valid: true`. Independently, the canonical
Anchor IDL address was derived from each program ID (empty-seed program PDA,
then `create_with_seed` using `anchor:idl`). Public finalized `getAccountInfo`
responses confirm that each IDL account is owned by that exact program. Their
zlib-decoded instruction definitions equal the extracts byte-for-byte as JSON
values. The compressed RPC responses are retained beside the extracts, which
record IDL account addresses, owners, slots, and response/data/IDL hashes.
This is program-published IDL provenance, not a successful creation fixture.
See the [service documentation](https://idl.solana.com/docs).

## Explicit public-contract gaps

This change does not invent initializers for swap-only catalog entries. The
Lifinity V2 on-chain IDL returned for its exact program ID does not include a pool
initializer. Public Obric integrations expose swaps but have not established
creation account contracts for the catalog's V2/V3 program IDs. SolFi V1/V2 and
GoonFi likewise have no established public creation layout in this change.
Heaven explicitly describes integration as private in its
[published integration repository](https://github.com/heavenxyz/heaven-integration/blob/b963f63cab94ba6e28c98b5fb3975e9edd4af9cd/README.md).
Missing IDL retrieval alone is not proof that a program is private or has no
initialization instruction. These gaps require an authoritative contract or
independently validated creation evidence before admission.

## Validation

`tests/pool_initialization.rs` constructs independent wire fixtures and distinct
account keys. It checks exact scalar/string/vector/option fields, every required
account position, missing and malformed keys, conditional quote accounts,
truncation, trailing bytes, and documented Pump EOF compatibility. Existing
Moonshot, Darklake, Plasma, Boop and Dumpfun instruction parsers are checked with
the newly added account layouts. `tests/raydium_pool_creation.rs` covers the three
additional CLMM families, their distinct account layouts and invalid wire fields.
These are parser tests, not captured successful creation transactions. Existing
Pump AMM captured tests and their separate coverage remain documented in
[`pump-amm-creation.md`](pump-amm-creation.md).
