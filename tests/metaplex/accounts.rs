use borsh::{to_vec, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana_idls::common::ParseError;
use substreams_solana_idls::metaplex::bubblegum::accounts as bg_accounts;
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
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord,
    TokenOwnedEscrow,
    TokenRecord,
    MetadataDelegate,
    EditionMarkerV2,
    HolderDelegate,
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

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum TokenDelegateRole {
    Sale,
    Transfer,
    Utility,
    Staking,
    Standard,
    LockedTransfer,
    Migration,
}

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum TokenState {
    Unlocked,
    Locked,
    Listed,
}

#[allow(dead_code)]
#[derive(BorshSerialize)]
enum EscrowAuthority {
    TokenOwner,
    Creator(Pubkey),
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

#[derive(BorshSerialize)]
struct ReservationV1Fixture {
    address: Pubkey,
    spots_remaining: u8,
    total_spots: u8,
}

#[derive(BorshSerialize)]
struct ReservationFixture {
    address: Pubkey,
    spots_remaining: u64,
    total_spots: u64,
}

#[derive(BorshSerialize)]
struct ReservationListV1Fixture {
    key: Key,
    master_edition: Pubkey,
    supply_snapshot: Option<u64>,
    reservations: Vec<ReservationV1Fixture>,
}

#[derive(BorshSerialize)]
struct ReservationListV2Fixture {
    key: Key,
    master_edition: Pubkey,
    supply_snapshot: Option<u64>,
    reservations: Vec<ReservationFixture>,
    total_reservation_spots: u64,
    current_reservation_spots: u64,
}

#[derive(BorshSerialize)]
struct UseAuthorityRecordFixture {
    key: Key,
    allowed_uses: u64,
    bump: u8,
}

#[derive(BorshSerialize)]
struct CollectionAuthorityRecordFixture {
    key: Key,
    bump: u8,
    update_authority: Option<Pubkey>,
}

#[derive(BorshSerialize)]
struct TokenOwnedEscrowFixture {
    key: Key,
    base_token: Pubkey,
    authority: EscrowAuthority,
    bump: u8,
}

#[derive(BorshSerialize)]
struct TokenRecordFixture {
    key: Key,
    bump: u8,
    state: TokenState,
    rule_set_revision: Option<u64>,
    delegate: Option<Pubkey>,
    delegate_role: Option<TokenDelegateRole>,
    locked_transfer: Option<Pubkey>,
}

#[derive(BorshSerialize)]
struct DelegateRecordFixture {
    key: Key,
    bump: u8,
    mint: Pubkey,
    delegate: Pubkey,
    update_authority: Pubkey,
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
fn token_metadata_reservation_list_accounts() {
    let master_edition = "11111111111111111111111111111112".parse().expect("parse master edition");
    let reservation_address = "So11111111111111111111111111111111111111112".parse().expect("parse reservation address");

    let reservation_list_v1 = ReservationListV1Fixture {
        key: Key::ReservationListV1,
        master_edition,
        supply_snapshot: Some(7),
        reservations: vec![ReservationV1Fixture {
            address: reservation_address,
            spots_remaining: 2,
            total_spots: 3,
        }],
    };

    let data = to_vec(&reservation_list_v1).expect("serialize reservation list v1");
    match tm_accounts::unpack(&data).expect("decode reservation list v1") {
        tm_accounts::TokenMetadataAccount::ReservationListV1(parsed) => {
            assert_eq!(parsed.master_edition, master_edition);
            assert_eq!(parsed.supply_snapshot, Some(7));
            assert_eq!(parsed.reservations.len(), 1);
            assert_eq!(parsed.reservations[0].address, reservation_address);
            assert_eq!(parsed.reservations[0].spots_remaining, 2);
            assert_eq!(parsed.reservations[0].total_spots, 3);
        }
        _ => panic!("expected ReservationListV1 account"),
    }

    let reservation_list_v2 = ReservationListV2Fixture {
        key: Key::ReservationListV2,
        master_edition,
        supply_snapshot: Some(11),
        reservations: vec![ReservationFixture {
            address: reservation_address,
            spots_remaining: 5,
            total_spots: 8,
        }],
        total_reservation_spots: 8,
        current_reservation_spots: 5,
    };

    let data = to_vec(&reservation_list_v2).expect("serialize reservation list v2");
    match tm_accounts::unpack(&data).expect("decode reservation list v2") {
        tm_accounts::TokenMetadataAccount::ReservationListV2(parsed) => {
            assert_eq!(parsed.master_edition, master_edition);
            assert_eq!(parsed.supply_snapshot, Some(11));
            assert_eq!(parsed.reservations.len(), 1);
            assert_eq!(parsed.reservations[0].address, reservation_address);
            assert_eq!(parsed.reservations[0].spots_remaining, 5);
            assert_eq!(parsed.reservations[0].total_spots, 8);
            assert_eq!(parsed.total_reservation_spots, 8);
            assert_eq!(parsed.current_reservation_spots, 5);
        }
        _ => panic!("expected ReservationListV2 account"),
    }
}

#[test]
fn token_metadata_additional_record_accounts() {
    let mint = "So11111111111111111111111111111111111111112".parse().expect("parse mint");
    let delegate = "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM".parse().expect("parse delegate");
    let update_authority = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        .parse()
        .expect("parse update authority");
    let locked_transfer = "SysvarRent111111111111111111111111111111111".parse().expect("parse locked transfer");

    let use_authority = UseAuthorityRecordFixture {
        key: Key::UseAuthorityRecord,
        allowed_uses: 9,
        bump: 250,
    };
    let data = to_vec(&use_authority).expect("serialize use authority");
    match tm_accounts::unpack(&data).expect("decode use authority") {
        tm_accounts::TokenMetadataAccount::UseAuthorityRecord(parsed) => {
            assert_eq!(parsed.allowed_uses, 9);
            assert_eq!(parsed.bump, 250);
        }
        _ => panic!("expected UseAuthorityRecord account"),
    }

    let collection_authority = CollectionAuthorityRecordFixture {
        key: Key::CollectionAuthorityRecord,
        bump: 7,
        update_authority: Some(update_authority),
    };
    let data = to_vec(&collection_authority).expect("serialize collection authority");
    match tm_accounts::unpack(&data).expect("decode collection authority") {
        tm_accounts::TokenMetadataAccount::CollectionAuthorityRecord(parsed) => {
            assert_eq!(parsed.bump, 7);
            assert_eq!(parsed.update_authority, Some(update_authority));
        }
        _ => panic!("expected CollectionAuthorityRecord account"),
    }

    let token_owned_escrow = TokenOwnedEscrowFixture {
        key: Key::TokenOwnedEscrow,
        base_token: mint,
        authority: EscrowAuthority::Creator(update_authority),
        bump: 3,
    };
    let data = to_vec(&token_owned_escrow).expect("serialize token owned escrow");
    match tm_accounts::unpack(&data).expect("decode token owned escrow") {
        tm_accounts::TokenMetadataAccount::TokenOwnedEscrow(parsed) => {
            assert_eq!(parsed.base_token, mint);
            assert_eq!(parsed.authority, tm_accounts::EscrowAuthority::Creator(update_authority));
            assert_eq!(parsed.bump, 3);
        }
        _ => panic!("expected TokenOwnedEscrow account"),
    }

    let token_record = TokenRecordFixture {
        key: Key::TokenRecord,
        bump: 42,
        state: TokenState::Listed,
        rule_set_revision: Some(12),
        delegate: Some(delegate),
        delegate_role: Some(TokenDelegateRole::Transfer),
        locked_transfer: Some(locked_transfer),
    };
    let data = to_vec(&token_record).expect("serialize token record");
    match tm_accounts::unpack(&data).expect("decode token record") {
        tm_accounts::TokenMetadataAccount::TokenRecord(parsed) => {
            assert_eq!(parsed.bump, 42);
            assert_eq!(parsed.state, tm_accounts::TokenState::Listed);
            assert_eq!(parsed.rule_set_revision, Some(12));
            assert_eq!(parsed.delegate, Some(delegate));
            assert_eq!(parsed.delegate_role, Some(tm_accounts::TokenDelegateRole::Transfer));
            assert_eq!(parsed.locked_transfer, Some(locked_transfer));
        }
        _ => panic!("expected TokenRecord account"),
    }

    let metadata_delegate = DelegateRecordFixture {
        key: Key::MetadataDelegate,
        bump: 5,
        mint,
        delegate,
        update_authority,
    };
    let data = to_vec(&metadata_delegate).expect("serialize metadata delegate");
    match tm_accounts::unpack(&data).expect("decode metadata delegate") {
        tm_accounts::TokenMetadataAccount::MetadataDelegateRecord(parsed) => {
            assert_eq!(parsed.bump, 5);
            assert_eq!(parsed.mint, mint);
            assert_eq!(parsed.delegate, delegate);
            assert_eq!(parsed.update_authority, update_authority);
        }
        _ => panic!("expected MetadataDelegateRecord account"),
    }

    let holder_delegate = DelegateRecordFixture {
        key: Key::HolderDelegate,
        bump: 6,
        mint,
        delegate,
        update_authority,
    };
    let data = to_vec(&holder_delegate).expect("serialize holder delegate");
    match tm_accounts::unpack(&data).expect("decode holder delegate") {
        tm_accounts::TokenMetadataAccount::HolderDelegateRecord(parsed) => {
            assert_eq!(parsed.bump, 6);
            assert_eq!(parsed.mint, mint);
            assert_eq!(parsed.delegate, delegate);
            assert_eq!(parsed.update_authority, update_authority);
        }
        _ => panic!("expected HolderDelegateRecord account"),
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

#[test]
fn bubblegum_empty_account() {
    assert!(matches!(bg_accounts::unpack(&[]), Err(ParseError::TooShort(0))));
}

#[test]
fn bubblegum_unknown_account_discriminator() {
    assert!(matches!(
        bg_accounts::unpack(&[0; 8]),
        Err(ParseError::AnchorUnknown(discriminator)) if discriminator == [0; 8]
    ));
}

#[test]
fn bubblegum_tree_config_account() {
    let fixture = bg_accounts::TreeConfig {
        tree_creator: Pubkey::new_unique(),
        tree_delegate: Pubkey::new_unique(),
        total_mint_capacity: 1_000,
        num_minted: 25,
        is_public: true,
        is_decompressible: 1,
    };

    let mut data = bg_accounts::TREE_CONFIG_ACCOUNT.to_vec();
    data.extend_from_slice(&to_vec(&fixture).expect("serialize tree config account"));

    match bg_accounts::unpack(&data).expect("decode tree config account") {
        bg_accounts::BubblegumAccount::TreeConfig(parsed) => assert_eq!(parsed, fixture),
        _ => panic!("expected TreeConfig account"),
    }
}

#[test]
fn bubblegum_voucher_account() {
    let fixture = bg_accounts::Voucher {
        leaf_schema: Pubkey::new_unique(),
        index: 42,
        merkle_tree: Pubkey::new_unique(),
    };

    let mut data = bg_accounts::VOUCHER_ACCOUNT.to_vec();
    data.extend_from_slice(&to_vec(&fixture).expect("serialize voucher account"));

    match bg_accounts::unpack(&data).expect("decode voucher account") {
        bg_accounts::BubblegumAccount::Voucher(parsed) => assert_eq!(parsed, fixture),
        _ => panic!("expected Voucher account"),
    }
}
