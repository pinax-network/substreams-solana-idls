use borsh::{to_vec, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana_idls::common::ParseError;
use substreams_solana_idls::metaplex::token_metadata::accounts as tm_accounts;

use crate::metaplex_fixtures;

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum Key {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
}

#[derive(BorshSerialize)]
struct Creator {
    address: Pubkey,
    verified: bool,
    share: u8,
}

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum TokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
    ProgrammableNonFungible,
    ProgrammableNonFungibleEdition,
}

#[derive(BorshSerialize)]
struct Collection;

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(BorshSerialize)]
struct Uses {
    use_method: UseMethod,
    remaining: u64,
    total: u64,
}

#[derive(BorshSerialize)]
struct CollectionDetails;

#[derive(BorshSerialize)]
struct ProgrammableConfig;

#[derive(BorshSerialize)]
struct MetadataFixture {
    key: Key,
    update_authority: Pubkey,
    mint: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    creators: Option<Vec<Creator>>,
    primary_sale_happened: bool,
    is_mutable: bool,
    edition_nonce: Option<u8>,
    token_standard: Option<TokenStandard>,
    collection: Option<Collection>,
    uses: Option<Uses>,
    collection_details: Option<CollectionDetails>,
    programmable_config: Option<ProgrammableConfig>,
}

#[derive(BorshSerialize)]
struct MasterEditionFixture {
    key: Key,
    supply: u64,
    max_supply: Option<u64>,
}

#[test]
fn token_metadata_unknown_account_key() {
    assert!(tm_accounts::unpack(&[255]).is_err());
}

#[test]
fn token_metadata_empty_account() {
    assert!(matches!(tm_accounts::unpack(&[]), Err(ParseError::TooShort(0))));
}

#[test]
fn token_metadata_metadata_account() {
    let fixture = MetadataFixture {
        key: Key::MetadataV1,
        update_authority: "11111111111111111111111111111112".parse().expect("parse update authority"),
        mint: "So11111111111111111111111111111111111111112".parse().expect("parse mint"),
        name: "Wrapped SOL".to_string(),
        symbol: "WSOL".to_string(),
        uri: "https://example.com/wsol.json".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        primary_sale_happened: false,
        is_mutable: true,
        edition_nonce: Some(254),
        token_standard: Some(TokenStandard::Fungible),
        collection: None,
        uses: None,
        collection_details: None,
        programmable_config: None,
    };

    let data = to_vec(&fixture).expect("serialize metadata account");
    match tm_accounts::unpack(&data).expect("decode metadata account") {
        tm_accounts::TokenMetadataAccount::Metadata(parsed) => {
            assert_eq!(parsed.update_authority, fixture.update_authority);
            assert_eq!(parsed.mint, fixture.mint);
            assert_eq!(parsed.name, "Wrapped SOL");
            assert_eq!(parsed.symbol, "WSOL");
            assert_eq!(parsed.uri, "https://example.com/wsol.json");
            assert_eq!(parsed.seller_fee_basis_points, 0);
            assert_eq!(parsed.primary_sale_happened, false);
            assert!(parsed.is_mutable);
            assert_eq!(parsed.edition_nonce, Some(254));
            assert_eq!(parsed.token_standard, Some(tm_accounts::TokenStandard::Fungible));
        }
        _ => panic!("expected Metadata account"),
    }
}

#[test]
fn token_metadata_master_edition_v2_account() {
    let fixture = MasterEditionFixture {
        key: Key::MasterEditionV2,
        supply: 42,
        max_supply: Some(100),
    };

    let data = to_vec(&fixture).expect("serialize master edition account");
    match tm_accounts::unpack(&data).expect("decode master edition account") {
        tm_accounts::TokenMetadataAccount::MasterEditionV2(parsed) => {
            assert_eq!(parsed.supply, 42);
            assert_eq!(parsed.max_supply, Some(100));
        }
        _ => panic!("expected MasterEditionV2 account"),
    }
}

#[test]
fn token_metadata_real_metadata_account() {
    let data = metaplex_fixtures::real_metadata_account();
    match tm_accounts::unpack(&data).expect("decode metadata account") {
        tm_accounts::TokenMetadataAccount::Metadata(parsed) => {
            assert_eq!(parsed.update_authority.to_string(), "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM");
            assert_eq!(parsed.mint.to_string(), "5dNYcCZXEGfGgbdUdq7MMR7KLsNJLLLgL83wLH8Fpump");
            assert_eq!(parsed.name, "MOO DOG");
            assert_eq!(parsed.symbol, "MOODOG");
            assert_eq!(parsed.uri, "https://ipfs.io/ipfs/QmbeFeWTrm1u1ev5VreMoqNK4aVuxtBXKpMdTrjdnHj7P3");
            assert_eq!(parsed.seller_fee_basis_points, 0);
            assert!(!parsed.primary_sale_happened);
            assert!(!parsed.is_mutable);
            assert_eq!(parsed.edition_nonce, None);
            assert_eq!(parsed.token_standard, None);
        }
        _ => panic!("expected Metadata account"),
    }
}

#[test]
fn token_metadata_real_master_edition_v2_account() {
    match tm_accounts::unpack(metaplex_fixtures::REAL_MASTER_EDITION_ACCOUNT).expect("decode master edition account") {
        tm_accounts::TokenMetadataAccount::MasterEditionV2(parsed) => {
            assert_eq!(parsed.supply, 0);
            assert_eq!(parsed.max_supply, Some(0));
        }
        _ => panic!("expected MasterEditionV2 account"),
    }
}

#[test]
fn token_metadata_real_edition_account() {
    match tm_accounts::unpack(metaplex_fixtures::REAL_EDITION_ACCOUNT).expect("decode edition account") {
        tm_accounts::TokenMetadataAccount::Edition(parsed) => {
            assert_eq!(parsed.parent.to_string(), "DDvbjs9KUjUKXWfAdheK6fFfdvHkgaERghaWTUpXHzFe");
            assert_eq!(parsed.edition, 430);
        }
        _ => panic!("expected Edition account"),
    }
}
