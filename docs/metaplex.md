# Metaplex Program Decoding Reference

Comprehensive guide to decoding Metaplex Token Metadata and Bubblegum programs on Solana.

## Program IDs

| Program | Address |
|---------|---------|
| Token Metadata | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s` |
| Bubblegum (compressed NFTs) | `BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY` |
| Candy Machine v3 (core) | `CndyV3LdqHUfDLRKUJPJWkN3cE8CZ7rpfPyo3YjAGFoN` |
| Candy Machine v3 (guard) | `Guard1JwRhJkVH6XZhzoYxeBVQe872VH6QggF4BWmS9g` |
| Token Auth Rules | `auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg` |

## Discriminator Systems

### Token Metadata -- Single-byte u8 index (NOT Anchor)

Token Metadata is **not** an Anchor program. It uses a custom Borsh-based instruction processor where the **first byte** of instruction data is a sequential `u8` index (0-55).

```
Instruction data: [discriminator_u8, ...borsh_serialized_args]
```

Some v1.13+ instructions (Create=42, Update=50) use a **sub-discriminator** (second byte) for version/variant selection:

```
Instruction data: [42, 0, ...args]  // Create V1 (disc=42, sub-disc=0)
Instruction data: [50, 0, ...args]  // Update V1 (disc=50, sub-disc=0)
```

### Bubblegum -- Anchor 8-byte SHA256 discriminators

Bubblegum is an Anchor program using standard discriminators computed as:

```
sha256("global:<snake_case_instruction_name>")[..8]
```

Account discriminators use:

```
sha256("account:<AccountName>")[..8]
```

## Token Metadata Instructions (All 56)

### Legacy/Deprecated (0-9)

| Disc | Instruction | Notes |
|------|-------------|-------|
| 0 | CreateMetadataAccount | v1, deprecated |
| 1 | UpdateMetadataAccount | v1, deprecated |
| 2 | DeprecatedCreateMasterEdition | |
| 3 | DeprecatedMintNewEditionFromMasterEditionViaPrintingToken | |
| 4 | UpdatePrimarySaleHappenedViaToken | |
| 5 | DeprecatedSetReservationList | |
| 6 | DeprecatedCreateReservationList | |
| 7 | SignMetadata | |
| 8 | DeprecatedMintPrintingTokensViaToken | |
| 9 | DeprecatedMintPrintingTokens | |

### V2/V3 Instructions (10-40)

| Disc | Instruction | Notes |
|------|-------------|-------|
| 10 | CreateMasterEdition | v2 |
| 11 | MintNewEditionFromMasterEditionViaToken | |
| 12 | ConvertMasterEditionV1ToV2 | |
| 13 | MintNewEditionFromMasterEditionViaVaultProxy | |
| 14 | PuffMetadata | |
| 15 | UpdateMetadataAccountV2 | Most common update instruction |
| 16 | CreateMetadataAccountV2 | |
| 17 | CreateMasterEditionV3 | |
| 18 | VerifyCollection | |
| 19 | Utilize | |
| 20 | ApproveUseAuthority | |
| 21 | RevokeUseAuthority | |
| 22 | UnverifyCollection | |
| 23 | ApproveCollectionAuthority | |
| 24 | RevokeCollectionAuthority | |
| 25 | SetAndVerifyCollection | |
| 26 | FreezeDelegatedAccount | |
| 27 | ThawDelegatedAccount | |
| 28 | RemoveCreatorVerification | |
| 29 | BurnNft | |
| 30 | VerifySizedCollectionItem | |
| 31 | UnverifySizedCollectionItem | |
| 32 | SetAndVerifySizedCollectionItem | |
| 33 | CreateMetadataAccountV3 | Current recommended for legacy flow |
| 34 | SetCollectionSize | |
| 35 | SetTokenStandard | |
| 36 | BubblegumSetCollectionSize | |
| 37 | BurnEditionNft | |
| 38 | CreateEscrowAccount | |
| 39 | CloseEscrowAccount | |
| 40 | TransferOutOfEscrow | |

### Unified v1.13+ Instructions (41-55)

These replaced the older versioned instructions. They support all token standards including Programmable NFTs.

| Disc | Instruction | Notes |
|------|-------------|-------|
| 41 | Burn | Unified burn across all token standards |
| 42 | Create | Replaces CreateMetadataAccountV3 + CreateMasterEditionV3. Has sub-discriminator. |
| 43 | Mint | Separate minting step for editions/tokens |
| 44 | Delegate | Granular delegation with many role types |
| 45 | Revoke | |
| 46 | Lock | Freeze pNFT tokens |
| 47 | Unlock | Thaw pNFT tokens |
| 48 | Migrate | Migrate NFTs to new token standards (e.g. NFT to pNFT) |
| 49 | Transfer | Handles all token standards including pNFT royalty-enforced transfers |
| 50 | Update | Has sub-discriminator. 9 variants for different delegate types. |
| 51 | Use | |
| 52 | Verify | Unified creator and collection verification |
| 53 | Unverify | |
| 54 | Collect | Collect protocol fees |
| 55 | Print | Print editions from master editions |

### Update (50) Sub-variants

The Update instruction has 9 sub-discriminator variants, each limiting which fields a specific delegate type can modify:

| Sub-disc | Variant |
|----------|---------|
| 0 | V1 (update authority) |
| 1 | AsUpdateAuthorityV2 |
| 2 | AsAuthorityItemDelegateV2 |
| 3 | AsCollectionDelegateV2 |
| 4 | AsDataDelegateV2 |
| 5 | AsProgrammableConfigDelegateV2 |
| 6 | AsDataItemDelegateV2 |
| 7 | AsCollectionItemDelegateV2 |
| 8 | AsProgrammableConfigItemDelegateV2 |

## Account Types and Discriminators

Token Metadata account types are identified by the **first byte** (a `Key` enum):

| Key | Account Type | Notes |
|-----|-------------|-------|
| 0 | Uninitialized | |
| 1 | EditionV1 | Print edition (41 bytes) |
| 2 | MasterEditionV1 | Deprecated |
| 3 | ReservationListV1 | |
| 4 | MetadataV1 | Primary metadata account |
| 5 | ReservationListV2 | |
| 6 | MasterEditionV2 | Current master edition |
| 7 | EditionMarker | |
| 8 | UseAuthorityRecord | 10 bytes |
| 9 | CollectionAuthorityRecord | |
| 10 | TokenOwnedEscrow | |
| 11 | TokenRecord | For pNFTs, 80 bytes |
| 12 | MetadataDelegate | |
| 13 | EditionMarkerV2 | |
| 14 | HolderDelegate | |

### Metadata Account Layout (Key=4)

```
key: u8                                    // Always 4
update_authority: Pubkey                   // 32 bytes
mint: Pubkey                               // 32 bytes
name: String                               // Borsh: 4-byte length + UTF-8
symbol: String
uri: String
seller_fee_basis_points: u16
creators: Option<Vec<Creator>>             // Creator = { address: Pubkey, verified: bool, share: u8 }
primary_sale_happened: bool
is_mutable: bool
edition_nonce: Option<u8>
token_standard: Option<TokenStandard>
collection: Option<Collection>             // { verified: bool, key: Pubkey }
uses: Option<Uses>                         // { use_method: u8, remaining: u64, total: u64 }
collection_details: Option<CollectionDetails>
programmable_config: Option<ProgrammableConfig>  // V1 { rule_set: Option<Pubkey> }
```

### MasterEdition Account (Key=6)

```
key: u8                  // Always 6
supply: u64
max_supply: Option<u64>
```

### Edition Account (Key=1)

```
key: u8                  // Always 1
parent: Pubkey           // Points to MasterEdition
edition: u64             // Edition number
```

### TokenRecord Account (Key=11, pNFTs only)

```
key: u8                              // Always 11
bump: u8
state: TokenState                    // Unlocked=0, Locked=1, Listed=2
rule_set_revision: Option<u64>
delegate: Option<Pubkey>
delegate_role: Option<TokenDelegateRole>
locked_transfer: Option<Pubkey>
```

## Token Standards

```rust
enum TokenStandard {
    NonFungible = 0,                    // Standard 1/1 NFT with MasterEdition
    FungibleAsset = 1,                  // Semi-fungible (e.g. game items, supply > 1)
    Fungible = 2,                       // Fungible SPL token with metadata
    NonFungibleEdition = 3,             // Print edition of a NonFungible
    ProgrammableNonFungible = 4,        // pNFT with programmable royalty enforcement
    ProgrammableNonFungibleEdition = 5, // Print edition of a pNFT
}
```

## PDA Derivation

All PDAs use Token Metadata program ID as the program owner.

| PDA Type | Seeds |
|----------|-------|
| Metadata | `["metadata", program_id, mint]` |
| Master Edition | `["metadata", program_id, mint, "edition"]` |
| Edition (print) | `["metadata", program_id, mint, "edition"]` (derived from print's mint) |
| Edition Marker | `["metadata", program_id, mint, "edition", edition_number_str]` |
| Collection Authority Record | `["metadata", program_id, mint, "collection_authority", authority]` |
| Use Authority Record | `["metadata", program_id, mint, "user", use_authority]` |
| Token Record (pNFTs) | `["metadata", program_id, mint, "token_record", token_account]` |

## Bubblegum Instructions (Compressed NFTs)

| Discriminator | Instruction |
|---------------|------------|
| `[116,110,29,56,107,219,42,93]` | Burn |
| `[111,76,232,50,39,175,48,242]` | CancelRedeem |
| `[165,83,136,142,89,202,47,220]` | CreateTree |
| `[54,85,76,70,228,250,164,81]` | DecompressV1 |
| `[90,147,75,178,85,88,4,137]` | Delegate |
| `[153,18,178,47,197,158,86,15]` | MintToCollectionV1 |
| `[145,98,192,118,184,147,118,104]` | MintV1 |
| `[184,12,86,149,70,196,97,225]` | Redeem |
| `[235,242,121,216,158,234,180,234]` | SetAndVerifyCollection |
| `[82,104,152,6,149,111,100,13]` | SetDecompressibleState |
| `[253,118,66,37,190,49,154,102]` | SetTreeDelegate |
| `[163,52,200,231,140,3,69,186]` | Transfer |
| `[250,251,42,106,41,137,186,168]` | UnverifyCollection |
| `[107,178,57,39,105,115,112,152]` | UnverifyCreator |
| `[56,113,101,253,79,55,122,169]` | VerifyCollection |
| `[52,17,96,132,71,4,85,194]` | VerifyCreator |
| `[170,182,43,239,97,78,225,186]` | UpdateMetadata |

### Bubblegum Account Discriminators

| Discriminator | Account |
|---------------|---------|
| `[122,245,175,248,171,34,0,207]` | TreeConfig |
| `[191,204,149,234,213,165,13,65]` | Voucher |

## Key Instruction Argument Structures

### CreateV1InstructionArgs (disc=42, sub-disc=0)

```rust
pub struct CreateV1InstructionArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub token_standard: TokenStandard,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub collection_details: Option<CollectionDetails>,
    pub rule_set: Option<Pubkey>,
    pub decimals: Option<u8>,
    pub print_supply: Option<PrintSupply>,
}
```

### UpdateV1InstructionArgs (disc=50, sub-disc=0)

```rust
pub struct UpdateV1InstructionArgs {
    pub new_update_authority: Option<Pubkey>,
    pub data: Option<Data>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
    pub collection: CollectionToggle,       // None | Clear | Set(Collection)
    pub collection_details: CollectionDetailsToggle,
    pub uses: UsesToggle,
    pub rule_set: RuleSetToggle,
    pub authorization_data: Option<AuthorizationData>,
}
```

### CreateMetadataAccountV3Args (disc=33)

```rust
pub struct CreateMetadataAccountV3Args {
    pub data: DataV2,
    pub is_mutable: bool,
    pub collection_details: Option<CollectionDetails>,
}
```

### CreateMasterEditionV3Args (disc=17)

```rust
pub struct CreateMasterEditionV3Args {
    pub max_supply: Option<u64>,
}
```

## Serialization

All data is **Borsh-serialized** (Binary Object Representation Serializer for Hashing):

- Strings: 4-byte little-endian length prefix + UTF-8 bytes
- Option: 1-byte tag (0=None, 1=Some) + value if Some
- Vec: 4-byte little-endian length prefix + elements
- Enums: 1-byte variant index + variant fields
- Structs: fields serialized in declaration order, no padding

## Events / Logs

Token Metadata does **not** emit structured Anchor events. It uses `msg!()` / `sol_log()` for program logs only.

Bubblegum emits Anchor events containing leaf/merkle tree change logs, used for indexing compressed NFT state.

## Codebase Implementation

This repo decodes Metaplex instructions in:

- `src/metaplex/token_metadata/instructions.rs` -- 56 instruction discriminators + parsed arg types
- `src/metaplex/token_metadata/accounts.rs` -- Account decoding via `mpl-token-metadata` v5.1.1
- `src/metaplex/bubblegum/instructions.rs` -- 17 Bubblegum instructions with Anchor discriminators
- `src/metaplex/bubblegum/accounts.rs` -- TreeConfig and Voucher accounts
- `tests/metaplex/` -- Test fixtures for instruction and account decoding

The `mpl-token-metadata` Rust crate v5.1.1 (auto-generated by Kinobi) provides the canonical account type definitions. Instruction decoding uses `TryFrom<&[u8]>` with Borsh deserialization.
