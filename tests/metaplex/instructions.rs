use borsh::to_vec;
use substreams_solana_idls::common::ParseError;
use substreams_solana_idls::metaplex::bubblegum::instructions as bgum_ix;
use substreams_solana_idls::metaplex::token_metadata::instructions as tm_ix;

use crate::metaplex_fixtures;

#[test]
fn token_metadata_unknown_discriminator() {
    // discriminator 255 is not valid
    assert!(tm_ix::unpack(&[255]).is_err());
}

#[test]
fn token_metadata_empty() {
    assert!(matches!(tm_ix::unpack(&[]), Err(ParseError::TooShort(0))));
}

#[test]
fn token_metadata_create_requires_subdiscriminator() {
    assert!(matches!(
        tm_ix::unpack(&[tm_ix::CREATE]),
        Err(ParseError::InvalidLength { expected: 1, got: 0 })
    ));
}

#[test]
fn token_metadata_update_requires_subdiscriminator() {
    assert!(matches!(
        tm_ix::unpack(&[tm_ix::UPDATE]),
        Err(ParseError::InvalidLength { expected: 1, got: 0 })
    ));
}

#[test]
fn token_metadata_verify() {
    let data = [tm_ix::VERIFY];
    let ix = tm_ix::unpack(&data).unwrap();
    assert_eq!(ix, tm_ix::TokenMetadataInstruction::Verify);
}

#[test]
fn token_metadata_create_master_edition_v3() {
    // discriminator 17 + borsh-encoded Option<u64>::Some(100)
    let mut data = vec![tm_ix::CREATE_MASTER_EDITION_V3];
    data.push(1); // Some
    data.extend_from_slice(&100u64.to_le_bytes());
    let ix = tm_ix::unpack(&data).unwrap();
    match ix {
        tm_ix::TokenMetadataInstruction::CreateMasterEditionV3(args) => {
            assert_eq!(args.max_supply, Some(100));
        }
        _ => panic!("expected CreateMasterEditionV3"),
    }
}

#[test]
fn token_metadata_create_v1() {
    let args = tm_ix::CreateV1InstructionArgs {
        name: "Wrapped SOL".to_string(),
        symbol: "WSOL".to_string(),
        uri: "https://example.com/wsol.json".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        primary_sale_happened: false,
        is_mutable: true,
        token_standard: tm_ix::TokenStandard::Fungible,
        collection: None,
        uses: None,
        collection_details: None,
        rule_set: None,
        decimals: Some(9),
        print_supply: None,
    };

    let mut data = vec![tm_ix::CREATE];
    data.push(0);
    data.extend_from_slice(&to_vec(&args).unwrap());

    let ix = tm_ix::unpack(&data).unwrap();
    match ix {
        tm_ix::TokenMetadataInstruction::Create(parsed) => {
            assert_eq!(parsed.name, "Wrapped SOL");
            assert_eq!(parsed.symbol, "WSOL");
            assert_eq!(parsed.uri, "https://example.com/wsol.json");
            assert_eq!(parsed.decimals, Some(9));
        }
        _ => panic!("expected Create"),
    }
}

#[test]
fn token_metadata_create_v1_unknown_subdiscriminator() {
    let err = tm_ix::unpack(&[tm_ix::CREATE, 1]).unwrap_err();
    assert!(matches!(err, ParseError::TokenMetadataSubdiscriminatorUnknown(1)));
}

#[test]
fn token_metadata_update_v1() {
    let args = tm_ix::UpdateV1InstructionArgs {
        new_update_authority: None,
        data: Some(tm_ix::Data {
            name: "Wrapped SOL".to_string(),
            symbol: "WSOL".to_string(),
            uri: "https://example.com/wsol.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
        }),
        primary_sale_happened: None,
        is_mutable: Some(true),
        collection: tm_ix::CollectionToggle::None,
        collection_details: tm_ix::CollectionDetailsToggle::None,
        uses: tm_ix::UsesToggle::None,
        rule_set: tm_ix::RuleSetToggle::None,
        authorization_data: None,
    };

    let mut data = vec![tm_ix::UPDATE];
    data.push(0);
    data.extend_from_slice(&to_vec(&args).unwrap());

    let ix = tm_ix::unpack(&data).unwrap();
    match ix {
        tm_ix::TokenMetadataInstruction::Update(parsed) => {
            let parsed_data = parsed.data.expect("expected data");
            assert_eq!(parsed_data.name, "Wrapped SOL");
            assert_eq!(parsed_data.symbol, "WSOL");
            assert_eq!(parsed_data.uri, "https://example.com/wsol.json");
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn token_metadata_real_create_metadata_account_v3() {
    let ix = tm_ix::unpack(metaplex_fixtures::REAL_CREATE_METADATA_ACCOUNT_V3_IX).unwrap();
    match ix {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccountV3(args) => {
            assert_eq!(args.data.name, "MOO DOG");
            assert_eq!(args.data.symbol, "MOODOG");
            assert_eq!(args.data.uri, "https://ipfs.io/ipfs/QmbeFeWTrm1u1ev5VreMoqNK4aVuxtBXKpMdTrjdnHj7P3");
            assert_eq!(args.data.seller_fee_basis_points, 0);
            assert!(!args.is_mutable);
            assert!(args.collection_details.is_none());
        }
        _ => panic!("expected CreateMetadataAccountV3"),
    }
}

#[test]
fn token_metadata_real_legacy_instruction_payloads() {
    assert_eq!(
        tm_ix::unpack(metaplex_fixtures::REAL_CREATE_METADATA_ACCOUNT_IX).unwrap(),
        tm_ix::TokenMetadataInstruction::CreateMetadataAccount
    );
    assert_eq!(
        tm_ix::unpack(metaplex_fixtures::REAL_CREATE_MASTER_EDITION_IX).unwrap(),
        tm_ix::TokenMetadataInstruction::CreateMasterEdition
    );
    assert_eq!(
        tm_ix::unpack(metaplex_fixtures::REAL_MINT_NEW_EDITION_VIA_TOKEN_IX).unwrap(),
        tm_ix::TokenMetadataInstruction::MintNewEditionFromMasterEditionViaToken
    );
    assert_eq!(
        tm_ix::unpack(metaplex_fixtures::REAL_UPDATE_METADATA_ACCOUNT_IX).unwrap(),
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccount
    );
}

#[test]
fn bubblegum_unknown_discriminator() {
    assert!(bgum_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn bubblegum_too_short() {
    assert!(bgum_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn bubblegum_known_discriminators() {
    let cases = [
        (bgum_ix::BURN, bgum_ix::BubblegumInstruction::Burn),
        (bgum_ix::CANCEL_REDEEM, bgum_ix::BubblegumInstruction::CancelRedeem),
        (bgum_ix::CREATE_TREE, bgum_ix::BubblegumInstruction::CreateTree),
        (bgum_ix::DECOMPRESS_V1, bgum_ix::BubblegumInstruction::DecompressV1),
        (bgum_ix::DELEGATE, bgum_ix::BubblegumInstruction::Delegate),
        (bgum_ix::MINT_TO_COLLECTION_V1, bgum_ix::BubblegumInstruction::MintToCollectionV1),
        (bgum_ix::MINT_V1, bgum_ix::BubblegumInstruction::MintV1),
        (bgum_ix::REDEEM, bgum_ix::BubblegumInstruction::Redeem),
        (bgum_ix::SET_AND_VERIFY_COLLECTION, bgum_ix::BubblegumInstruction::SetAndVerifyCollection),
        (bgum_ix::SET_DECOMPRESSIBLE_STATE, bgum_ix::BubblegumInstruction::SetDecompressibleState),
        (bgum_ix::SET_TREE_DELEGATE, bgum_ix::BubblegumInstruction::SetTreeDelegate),
        (bgum_ix::TRANSFER, bgum_ix::BubblegumInstruction::Transfer),
        (bgum_ix::UNVERIFY_COLLECTION, bgum_ix::BubblegumInstruction::UnverifyCollection),
        (bgum_ix::UNVERIFY_CREATOR, bgum_ix::BubblegumInstruction::UnverifyCreator),
        (bgum_ix::VERIFY_COLLECTION, bgum_ix::BubblegumInstruction::VerifyCollection),
        (bgum_ix::VERIFY_CREATOR, bgum_ix::BubblegumInstruction::VerifyCreator),
        (bgum_ix::UPDATE_METADATA, bgum_ix::BubblegumInstruction::UpdateMetadata),
    ];

    for (discriminator, expected) in cases {
        // Append trailing payload bytes to verify unpack tolerates extra data
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&[0u8; 32]);
        let ix = bgum_ix::unpack(&data).unwrap();
        assert_eq!(ix, expected);
    }
}
