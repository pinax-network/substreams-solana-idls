use borsh::to_vec;
use borsh::BorshSerialize;
use hex_literal::hex;
use solana_program::pubkey::Pubkey;

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum MetadataKey {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
}

#[derive(BorshSerialize)]
struct MetadataAccountFixture {
    key: MetadataKey,
    update_authority: Pubkey,
    mint: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    creators: Option<Vec<MetadataCreator>>,
    primary_sale_happened: bool,
    is_mutable: bool,
    edition_nonce: Option<u8>,
    token_standard: Option<MetadataTokenStandard>,
    collection: Option<MetadataCollection>,
    uses: Option<MetadataUses>,
    collection_details: Option<MetadataCollectionDetails>,
    programmable_config: Option<MetadataProgrammableConfig>,
}

#[derive(BorshSerialize)]
struct MetadataCreator {
    address: Pubkey,
    verified: bool,
    share: u8,
}

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum MetadataTokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
    ProgrammableNonFungible,
    ProgrammableNonFungibleEdition,
}

#[derive(BorshSerialize)]
struct MetadataCollection {
    verified: bool,
    key: Pubkey,
}

#[derive(BorshSerialize)]
struct MetadataUses {
    use_method: u8,
    remaining: u64,
    total: u64,
}

#[derive(BorshSerialize)]
struct MetadataCollectionDetails;

#[derive(BorshSerialize)]
struct MetadataProgrammableConfig;

// Real Metaplex Token Metadata instruction payloads captured from public mainnet
// transactions.

// Source transaction:
// https://github.com/0xjeffro/tx-parser/blob/475b1ebff79a2f41ec966919fdefa01f11f6c5d7/solana/data/pumpfun_create_0.json
// Signature:
// 2s393PSYYxJJJfGiwHf18HZeC68nZs44ssbeB4aAkeYMyd1dyiiu3yVmGyRWZuArk5HzYDgVxYfhKLYd2CJ8kCBj
// Inner instruction: Metaplex `CreateMetadataAccountV3`
pub const REAL_CREATE_METADATA_ACCOUNT_V3_IX: &[u8] = &hex!(
    "
    21 07 00 00 00 4d 4f 4f 20 44 4f 47
    06 00 00 00 4d 4f 4f 44 4f 47
    43 00 00 00
    68 74 74 70 73 3a 2f 2f 69 70 66 73 2e 69 6f 2f 69 70 66 73 2f 51 6d 62 65 46 65 57 54 72 6d
    31 75 31 65 76 35 56 72 65 4d 6f 71 4e 4b 34 61 56 75 78 74 42 58 4b 70 4d 64 54 72 6a 64 6e
    48 6a 37 50 33
    00 00
    00
    00
    00
    00
    00
    "
);

// Source transaction:
// https://github.com/ilmoi/nft-armory-node/blob/e964544081baa71afcbbaf2066c9175b33e54104/txs/Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL.json
// Signature:
// Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL
// Inner instruction: Metaplex `Create Metadata Accounts`
pub const REAL_CREATE_METADATA_ACCOUNT_IX: &[u8] = &hex!(
    "
    00
    20 00 00 00 53 61 69 62 61 47 61 6e 67 20 23 31 30 39 32 00 00 00 00 00 00 00 00 00 00 00 00
    00 00 00 00
    0a 00 00 00 53 42 41 47 4e 47 00 00 00 00
    c8 00 00 00 68 74 74 70 73 3a 2f 2f 61 72 77 65 61 76 65 2e 6e 65 74 2f 38 77 68 5a 44 66 4d
    48 73 75 50 64 5a 70 62 72 5f 71 4b 6f 6d 72 78 62 7a 6a 6b 39 4f 51 49 6a 59 6d 6a 72 57 4c
    5a 72 39 2d 55 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    f4 01
    01
    05 00 00 00
    98 ce 42 98 bf d0 0c bb b7 ff 36 dc e7 1f 77 9b 96 ed 5e 94 59 83 8a 8d ec 95 20 b3 dc 02 dd
    0b 01 00
    4a 76 2c ec ff 2b d0 94 85 e0 b0 37 2e d3 b9 f4 0d 53 a4 9c 53 8a cb 6b bc 4f 0d ff 09 45 d3
    36 00 46
    59 61 54 32 76 e3 22 0c ca 49 c1 98 b8 7f 48 a0 3c d9 20 36 63 49 67 eb 39 68 c8 81 14 b3 26
    81 00 14
    80 27 4f 62 ac 19 73 e6 ba 31 78 b3 71 66 36 79 d8 c0 e6 24 7e d5 5f 1b ea c5 bf 49 c6 68 2d
    84 00 05
    21 2e b5 42 9f 6f de d9 a8 56 4f e6 22 63 ab 68 65 0d 5e ea 0d 30 83 ae b0 3c 93 b2 3d 3e 8e
    f8 00 05
    01
    "
);

// Source transaction:
// https://github.com/ilmoi/nft-armory-node/blob/e964544081baa71afcbbaf2066c9175b33e54104/txs/Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL.json
// Signature:
// Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL
// Inner instruction: Metaplex `Create Master Edition`
pub const REAL_CREATE_MASTER_EDITION_IX: &[u8] = &hex!("0a 01 00 00 00 00 00 00 00 00");

// Source transaction:
// https://github.com/ilmoi/nft-armory-node/blob/e964544081baa71afcbbaf2066c9175b33e54104/txs/21DoA9TmnP88N3zUwf1jYYRfv7UyoqpbPLJ4DT3BU7fov3zMzYGsyQjeSoUXERVZJrZEMMPLnR2mMRhQSeYQfJhD.json
// Signature:
// 21DoA9TmnP88N3zUwf1jYYRfv7UyoqpbPLJ4DT3BU7fov3zMzYGsyQjeSoUXERVZJrZEMMPLnR2mMRhQSeYQfJhD
// Inner instruction: Metaplex `Mint New Edition from Master Edition Via Token`
pub const REAL_MINT_NEW_EDITION_VIA_TOKEN_IX: &[u8] = &hex!("0b ae 01 00 00 00 00 00 00");

// Source transaction:
// https://github.com/ilmoi/nft-armory-node/blob/e964544081baa71afcbbaf2066c9175b33e54104/txs/Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL.json
// Signature:
// Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL
// Inner instruction: Metaplex `Update Metadata Accounts`
pub const REAL_UPDATE_METADATA_ACCOUNT_IX: &[u8] =
    &hex!("01 00 01 9a 24 91 9c 11 f7 cf 47 7b 45 75 f9 14 04 57 96 ec a0 e4 18 26 df 1d 8f 09 64 22 5e d9 92 67 18 01 01");

// Real account data reconstructed from production transaction/account values.

// Source transaction:
// https://github.com/0xjeffro/tx-parser/blob/475b1ebff79a2f41ec966919fdefa01f11f6c5d7/solana/data/pumpfun_create_0.json
// Signature:
// 2s393PSYYxJJJfGiwHf18HZeC68nZs44ssbeB4aAkeYMyd1dyiiu3yVmGyRWZuArk5HzYDgVxYfhKLYd2CJ8kCBj
// Metadata account:
// DTJKYfWFTCr1EZxehTPbXkU1V9LT5FdmGKJwEXsXuBht
pub fn real_metadata_account() -> Vec<u8> {
    to_vec(&MetadataAccountFixture {
        key: MetadataKey::MetadataV1,
        update_authority: "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM".parse::<Pubkey>().expect("parse update authority"),
        mint: "5dNYcCZXEGfGgbdUdq7MMR7KLsNJLLLgL83wLH8Fpump".parse::<Pubkey>().expect("parse mint"),
        name: "MOO DOG".to_string(),
        symbol: "MOODOG".to_string(),
        uri: "https://ipfs.io/ipfs/QmbeFeWTrm1u1ev5VreMoqNK4aVuxtBXKpMdTrjdnHj7P3".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        primary_sale_happened: false,
        is_mutable: false,
        edition_nonce: None,
        token_standard: None,
        collection: None,
        uses: None,
        collection_details: None,
        programmable_config: None,
    })
    .expect("serialize metadata fixture")
}

// Source transaction:
// https://github.com/ilmoi/nft-armory-node/blob/e964544081baa71afcbbaf2066c9175b33e54104/txs/Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL.json
// Signature:
// Jt6MQf7aaBQ26sftFC9vFxPL8gh3aQ9RkncHNm2N4sV66p6fee2UJyxJGJjLXfWzShVfMGDPgucMLJCu7kL2LWL
// Master edition account:
// 4Rkg1AWH2cvuJDxAkXd3wQZ9RzFMJTWYZiSritBj9nG6
pub const REAL_MASTER_EDITION_ACCOUNT: &[u8] = &hex!("06 00 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00 00");

// Source transaction:
// https://github.com/ilmoi/nft-armory-node/blob/e964544081baa71afcbbaf2066c9175b33e54104/txs/21DoA9TmnP88N3zUwf1jYYRfv7UyoqpbPLJ4DT3BU7fov3zMzYGsyQjeSoUXERVZJrZEMMPLnR2mMRhQSeYQfJhD.json
// Signature:
// 21DoA9TmnP88N3zUwf1jYYRfv7UyoqpbPLJ4DT3BU7fov3zMzYGsyQjeSoUXERVZJrZEMMPLnR2mMRhQSeYQfJhD
// Edition account:
// FJkSH96NeqY7Du3nD5gsT4g7JCNKBow838iJqGFZCFhd
pub const REAL_EDITION_ACCOUNT: &[u8] = &hex!(
    "
    01
    b5 9b d1 3d d9 a7 1b cc 6f f8 ff f7 b3 35 42 ee a4 ff 4a 90 70 0c 98 0e 5d 31 59 13 b7 37 1b
    d5
    ae 01 00 00 00 00 00 00
    "
);
