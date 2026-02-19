# Metaplex

Solana IDL support for Metaplex programs.

## Programs

| Submodule | Program | Program ID |
|---|---|---|
| `token_metadata` | Token Metadata | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s` |
| `bubblegum` | Bubblegum (Compressed NFTs) | `BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY` |

## Token Metadata

Non-Anchor program with 56 instructions using sequential u8 index discriminators.
The first byte of instruction data identifies the instruction.

Key instructions with parsed payloads:
- `CreateMetadataAccountV3`
- `CreateMasterEditionV3`
- `UpdateMetadataAccountV2`
- `Verify`

## Bubblegum

Anchor program with 17 instructions using `sha256("global:<snake_case_name>")[..8]` discriminators.
