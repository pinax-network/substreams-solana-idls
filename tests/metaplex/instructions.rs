use borsh::to_vec;
use solana_program::pubkey::Pubkey;
use substreams_solana_idls::common::ParseError;
use substreams_solana_idls::metaplex::bubblegum::instructions as bgum_ix;
use substreams_solana_idls::metaplex::token_metadata::instructions as tm_ix;

use crate::metaplex_fixtures;

// ── Token Metadata: error handling ─────────────────────────────────────

#[test]
fn token_metadata_empty() {
    assert!(matches!(tm_ix::unpack(&[]), Err(ParseError::TooShort(0))));
}

#[test]
fn token_metadata_unknown_discriminator() {
    assert!(matches!(tm_ix::unpack(&[255]), Err(ParseError::TokenMetadataUnknown(255))));
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
fn token_metadata_create_v1_unknown_subdiscriminator() {
    let err = tm_ix::unpack(&[tm_ix::CREATE, 1]).unwrap_err();
    assert!(matches!(err, ParseError::TokenMetadataSubdiscriminatorUnknown(1)));
}

#[test]
fn token_metadata_update_v1_unknown_subdiscriminator() {
    let err = tm_ix::unpack(&[tm_ix::UPDATE, 99]).unwrap_err();
    assert!(matches!(err, ParseError::TokenMetadataSubdiscriminatorUnknown(99)));
}

// ── Token Metadata: all no-arg discriminators (table-driven) ───────────

#[test]
fn token_metadata_no_arg_discriminators() {
    let cases: Vec<(u8, tm_ix::TokenMetadataInstruction)> = vec![
        (tm_ix::DEPRECATED_CREATE_MASTER_EDITION, tm_ix::TokenMetadataInstruction::DeprecatedCreateMasterEdition),
        (
            tm_ix::DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN,
            tm_ix::TokenMetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
        ),
        (tm_ix::UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN, tm_ix::TokenMetadataInstruction::UpdatePrimarySaleHappenedViaToken),
        (tm_ix::DEPRECATED_SET_RESERVATION_LIST, tm_ix::TokenMetadataInstruction::DeprecatedSetReservationList),
        (tm_ix::DEPRECATED_CREATE_RESERVATION_LIST, tm_ix::TokenMetadataInstruction::DeprecatedCreateReservationList),
        (tm_ix::SIGN_METADATA, tm_ix::TokenMetadataInstruction::SignMetadata),
        (tm_ix::DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN, tm_ix::TokenMetadataInstruction::DeprecatedMintPrintingTokensViaToken),
        (tm_ix::DEPRECATED_MINT_PRINTING_TOKENS, tm_ix::TokenMetadataInstruction::DeprecatedMintPrintingTokens),
        (tm_ix::CONVERT_MASTER_EDITION_V1_TO_V2, tm_ix::TokenMetadataInstruction::ConvertMasterEditionV1ToV2),
        (
            tm_ix::MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY,
            tm_ix::TokenMetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy,
        ),
        (tm_ix::PUFF_METADATA, tm_ix::TokenMetadataInstruction::PuffMetadata),
        (tm_ix::VERIFY_COLLECTION, tm_ix::TokenMetadataInstruction::VerifyCollection),
        (tm_ix::UTILIZE, tm_ix::TokenMetadataInstruction::Utilize),
        (tm_ix::APPROVE_USE_AUTHORITY, tm_ix::TokenMetadataInstruction::ApproveUseAuthority),
        (tm_ix::REVOKE_USE_AUTHORITY, tm_ix::TokenMetadataInstruction::RevokeUseAuthority),
        (tm_ix::UNVERIFY_COLLECTION, tm_ix::TokenMetadataInstruction::UnverifyCollection),
        (tm_ix::APPROVE_COLLECTION_AUTHORITY, tm_ix::TokenMetadataInstruction::ApproveCollectionAuthority),
        (tm_ix::REVOKE_COLLECTION_AUTHORITY, tm_ix::TokenMetadataInstruction::RevokeCollectionAuthority),
        (tm_ix::SET_AND_VERIFY_COLLECTION, tm_ix::TokenMetadataInstruction::SetAndVerifyCollection),
        (tm_ix::FREEZE_DELEGATED_ACCOUNT, tm_ix::TokenMetadataInstruction::FreezeDelegatedAccount),
        (tm_ix::THAW_DELEGATED_ACCOUNT, tm_ix::TokenMetadataInstruction::ThawDelegatedAccount),
        (tm_ix::REMOVE_CREATOR_VERIFICATION, tm_ix::TokenMetadataInstruction::RemoveCreatorVerification),
        (tm_ix::BURN_NFT, tm_ix::TokenMetadataInstruction::BurnNft),
        (tm_ix::VERIFY_SIZED_COLLECTION_ITEM, tm_ix::TokenMetadataInstruction::VerifySizedCollectionItem),
        (tm_ix::UNVERIFY_SIZED_COLLECTION_ITEM, tm_ix::TokenMetadataInstruction::UnverifySizedCollectionItem),
        (tm_ix::SET_AND_VERIFY_SIZED_COLLECTION_ITEM, tm_ix::TokenMetadataInstruction::SetAndVerifySizedCollectionItem),
        (tm_ix::SET_TOKEN_STANDARD, tm_ix::TokenMetadataInstruction::SetTokenStandard),
        (tm_ix::BUBBLEGUM_SET_COLLECTION_SIZE, tm_ix::TokenMetadataInstruction::BubblegumSetCollectionSize),
        (tm_ix::BURN_EDITION_NFT, tm_ix::TokenMetadataInstruction::BurnEditionNft),
        (tm_ix::CREATE_ESCROW_ACCOUNT, tm_ix::TokenMetadataInstruction::CreateEscrowAccount),
        (tm_ix::CLOSE_ESCROW_ACCOUNT, tm_ix::TokenMetadataInstruction::CloseEscrowAccount),
        (tm_ix::TRANSFER_OUT_OF_ESCROW, tm_ix::TokenMetadataInstruction::TransferOutOfEscrow),
        (tm_ix::REVOKE, tm_ix::TokenMetadataInstruction::Revoke),
        (tm_ix::LOCK, tm_ix::TokenMetadataInstruction::Lock),
        (tm_ix::UNLOCK, tm_ix::TokenMetadataInstruction::Unlock),
        (tm_ix::MIGRATE, tm_ix::TokenMetadataInstruction::Migrate),
        (tm_ix::USE, tm_ix::TokenMetadataInstruction::Use),
        (tm_ix::VERIFY, tm_ix::TokenMetadataInstruction::Verify),
        (tm_ix::UNVERIFY, tm_ix::TokenMetadataInstruction::Unverify),
        (tm_ix::COLLECT, tm_ix::TokenMetadataInstruction::Collect),
        (tm_ix::PRINT, tm_ix::TokenMetadataInstruction::Print),
    ];

    for (disc, expected) in cases {
        // No-arg instructions should decode with just the discriminator byte
        let ix = tm_ix::unpack(&[disc]).unwrap_or_else(|e| panic!("disc {} failed: {}", disc, e));
        assert_eq!(ix, expected, "mismatch for discriminator {}", disc);

        // Should also tolerate trailing bytes
        let mut data = vec![disc];
        data.extend_from_slice(&[0u8; 16]);
        let ix2 = tm_ix::unpack(&data).unwrap_or_else(|e| panic!("disc {} with trailing failed: {}", disc, e));
        assert_eq!(ix2, expected, "trailing bytes mismatch for discriminator {}", disc);
    }
}

// ── Token Metadata: discriminator value correctness ────────────────────

#[test]
fn token_metadata_discriminator_values_are_sequential() {
    assert_eq!(tm_ix::CREATE_METADATA_ACCOUNT, 0);
    assert_eq!(tm_ix::UPDATE_METADATA_ACCOUNT, 1);
    assert_eq!(tm_ix::CREATE_MASTER_EDITION, 10);
    assert_eq!(tm_ix::MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN, 11);
    assert_eq!(tm_ix::UPDATE_METADATA_ACCOUNT_V2, 15);
    assert_eq!(tm_ix::CREATE_METADATA_ACCOUNT_V2, 16);
    assert_eq!(tm_ix::CREATE_MASTER_EDITION_V3, 17);
    assert_eq!(tm_ix::CREATE_METADATA_ACCOUNT_V3, 33);
    assert_eq!(tm_ix::SET_COLLECTION_SIZE, 34);
    assert_eq!(tm_ix::BURN, 41);
    assert_eq!(tm_ix::CREATE, 42);
    assert_eq!(tm_ix::MINT, 43);
    assert_eq!(tm_ix::DELEGATE, 44);
    assert_eq!(tm_ix::TRANSFER, 49);
    assert_eq!(tm_ix::UPDATE, 50);
    assert_eq!(tm_ix::VERIFY, 52);
    assert_eq!(tm_ix::PRINT, 55);
}

// ── Token Metadata: V1 instruction args (disc 0 & 1) ─────────────────

#[test]
fn token_metadata_create_metadata_account_v1() {
    let args = tm_ix::CreateMetadataAccountArgs {
        data: tm_ix::Data {
            name: "SaibaGang #1".to_string(),
            symbol: "SBAG".to_string(),
            uri: "https://arweave.net/abc123".to_string(),
            seller_fee_basis_points: 500,
            creators: Some(vec![tm_ix::Creator {
                address: Pubkey::new_from_array([1; 32]),
                verified: true,
                share: 100,
            }]),
        },
        is_mutable: true,
    };

    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT];
    data.extend_from_slice(&to_vec(&args).unwrap());

    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccount(parsed) => {
            assert_eq!(parsed.data.name, "SaibaGang #1");
            assert_eq!(parsed.data.symbol, "SBAG");
            assert_eq!(parsed.data.uri, "https://arweave.net/abc123");
            assert_eq!(parsed.data.seller_fee_basis_points, 500);
            assert_eq!(parsed.data.creators.as_ref().unwrap().len(), 1);
            assert!(parsed.is_mutable);
        }
        _ => panic!("expected CreateMetadataAccount"),
    }
}

#[test]
fn token_metadata_create_metadata_account_v1_minimal() {
    let args = tm_ix::CreateMetadataAccountArgs {
        data: tm_ix::Data {
            name: "".to_string(),
            symbol: "".to_string(),
            uri: "".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
        },
        is_mutable: false,
    };

    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT];
    data.extend_from_slice(&to_vec(&args).unwrap());

    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccount(parsed) => {
            assert_eq!(parsed.data.name, "");
            assert!(parsed.data.creators.is_none());
            assert!(!parsed.is_mutable);
        }
        _ => panic!("expected CreateMetadataAccount"),
    }
}

#[test]
fn token_metadata_update_metadata_account_v1() {
    let args = tm_ix::UpdateMetadataAccountArgs {
        data: Some(tm_ix::Data {
            name: "Updated NFT".to_string(),
            symbol: "UNFT".to_string(),
            uri: "https://arweave.net/updated".to_string(),
            seller_fee_basis_points: 250,
            creators: None,
        }),
        update_authority: Some(Pubkey::new_from_array([3; 32])),
        primary_sale_happened: Some(true),
    };

    let mut data = vec![tm_ix::UPDATE_METADATA_ACCOUNT];
    data.extend_from_slice(&to_vec(&args).unwrap());

    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccount(parsed) => {
            assert_eq!(parsed.data.as_ref().unwrap().name, "Updated NFT");
            assert_eq!(parsed.data.as_ref().unwrap().symbol, "UNFT");
            assert_eq!(parsed.data.as_ref().unwrap().uri, "https://arweave.net/updated");
            assert_eq!(parsed.data.as_ref().unwrap().seller_fee_basis_points, 250);
            assert_eq!(parsed.update_authority, Some(Pubkey::new_from_array([3; 32])));
            assert_eq!(parsed.primary_sale_happened, Some(true));
        }
        _ => panic!("expected UpdateMetadataAccount"),
    }
}

#[test]
fn token_metadata_update_metadata_account_v1_all_none() {
    let args = tm_ix::UpdateMetadataAccountArgs {
        data: None,
        update_authority: None,
        primary_sale_happened: None,
    };

    let mut data = vec![tm_ix::UPDATE_METADATA_ACCOUNT];
    data.extend_from_slice(&to_vec(&args).unwrap());

    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccount(parsed) => {
            assert!(parsed.data.is_none());
            assert!(parsed.update_authority.is_none());
            assert!(parsed.primary_sale_happened.is_none());
        }
        _ => panic!("expected UpdateMetadataAccount"),
    }
}

// ── Token Metadata: all create/update variants return name/symbol/uri ─

#[test]
fn all_create_metadata_variants_return_name_symbol_uri() {
    let name = "Test Token";
    let symbol = "TST";
    let uri = "https://example.com/meta.json";

    // V1 (disc 0)
    let v1_args = tm_ix::CreateMetadataAccountArgs {
        data: tm_ix::Data {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
        },
        is_mutable: false,
    };
    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT];
    data.extend_from_slice(&to_vec(&v1_args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccount(p) => {
            assert_eq!(p.data.name, name);
            assert_eq!(p.data.symbol, symbol);
            assert_eq!(p.data.uri, uri);
        }
        other => panic!("expected CreateMetadataAccount, got {:?}", other),
    }

    // V2 (disc 16)
    let v2_args = tm_ix::CreateMetadataAccountV2Args {
        data: tm_ix::DataV2 {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: false,
    };
    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT_V2];
    data.extend_from_slice(&to_vec(&v2_args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccountV2(p) => {
            assert_eq!(p.data.name, name);
            assert_eq!(p.data.symbol, symbol);
            assert_eq!(p.data.uri, uri);
        }
        other => panic!("expected CreateMetadataAccountV2, got {:?}", other),
    }

    // V3 (disc 33)
    let v3_args = tm_ix::CreateMetadataAccountV3Args {
        data: tm_ix::DataV2 {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: false,
        collection_details: None,
    };
    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT_V3];
    data.extend_from_slice(&to_vec(&v3_args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccountV3(p) => {
            assert_eq!(p.data.name, name);
            assert_eq!(p.data.symbol, symbol);
            assert_eq!(p.data.uri, uri);
        }
        other => panic!("expected CreateMetadataAccountV3, got {:?}", other),
    }

    // Create V1 unified (disc 42)
    let create_args = tm_ix::CreateV1InstructionArgs {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        primary_sale_happened: false,
        is_mutable: false,
        token_standard: tm_ix::TokenStandard::NonFungible,
        collection: None,
        uses: None,
        collection_details: None,
        rule_set: None,
        decimals: None,
        print_supply: None,
    };
    let mut data = vec![tm_ix::CREATE, 0];
    data.extend_from_slice(&to_vec(&create_args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::Create(p) => {
            assert_eq!(p.name, name);
            assert_eq!(p.symbol, symbol);
            assert_eq!(p.uri, uri);
        }
        other => panic!("expected Create, got {:?}", other),
    }
}

#[test]
fn all_update_metadata_variants_return_name_symbol_uri() {
    let name = "Updated Token";
    let symbol = "UPD";
    let uri = "https://example.com/updated.json";

    // V1 (disc 1)
    let v1_args = tm_ix::UpdateMetadataAccountArgs {
        data: Some(tm_ix::Data {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
        }),
        update_authority: None,
        primary_sale_happened: None,
    };
    let mut data = vec![tm_ix::UPDATE_METADATA_ACCOUNT];
    data.extend_from_slice(&to_vec(&v1_args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccount(p) => {
            assert_eq!(p.data.as_ref().unwrap().name, name);
            assert_eq!(p.data.as_ref().unwrap().symbol, symbol);
            assert_eq!(p.data.as_ref().unwrap().uri, uri);
        }
        other => panic!("expected UpdateMetadataAccount, got {:?}", other),
    }

    // V2 (disc 15)
    let v2_args = tm_ix::UpdateMetadataAccountV2Args {
        data: Some(tm_ix::DataV2 {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        }),
        update_authority: None,
        primary_sale_happened: None,
        is_mutable: None,
    };
    let mut data = vec![tm_ix::UPDATE_METADATA_ACCOUNT_V2];
    data.extend_from_slice(&to_vec(&v2_args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccountV2(p) => {
            assert_eq!(p.data.as_ref().unwrap().name, name);
            assert_eq!(p.data.as_ref().unwrap().symbol, symbol);
            assert_eq!(p.data.as_ref().unwrap().uri, uri);
        }
        other => panic!("expected UpdateMetadataAccountV2, got {:?}", other),
    }

    // Update V1 unified (disc 50)
    let update_args = tm_ix::UpdateV1InstructionArgs {
        new_update_authority: None,
        data: Some(tm_ix::Data {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
        }),
        primary_sale_happened: None,
        is_mutable: None,
        collection: tm_ix::CollectionToggle::None,
        collection_details: tm_ix::CollectionDetailsToggle::None,
        uses: tm_ix::UsesToggle::None,
        rule_set: tm_ix::RuleSetToggle::None,
        authorization_data: None,
    };
    let mut data = vec![tm_ix::UPDATE, 0];
    data.extend_from_slice(&to_vec(&update_args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::Update(p) => {
            assert_eq!(p.data.as_ref().unwrap().name, name);
            assert_eq!(p.data.as_ref().unwrap().symbol, symbol);
            assert_eq!(p.data.as_ref().unwrap().uri, uri);
        }
        other => panic!("expected Update, got {:?}", other),
    }
}

// ── Token Metadata: parsed instruction args ────────────────────────────

#[test]
fn token_metadata_create_metadata_account_v3() {
    let args = tm_ix::CreateMetadataAccountV3Args {
        data: tm_ix::DataV2 {
            name: "Test NFT".to_string(),
            symbol: "TNFT".to_string(),
            uri: "https://example.com/nft.json".to_string(),
            seller_fee_basis_points: 500,
            creators: Some(vec![tm_ix::Creator {
                address: Pubkey::new_from_array([1; 32]),
                verified: true,
                share: 100,
            }]),
            collection: Some(tm_ix::Collection {
                verified: false,
                key: Pubkey::new_from_array([2; 32]),
            }),
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    };

    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT_V3];
    data.extend_from_slice(&to_vec(&args).unwrap());

    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccountV3(parsed) => {
            assert_eq!(parsed.data.name, "Test NFT");
            assert_eq!(parsed.data.symbol, "TNFT");
            assert_eq!(parsed.data.seller_fee_basis_points, 500);
            assert_eq!(parsed.data.creators.as_ref().unwrap().len(), 1);
            assert_eq!(parsed.data.creators.as_ref().unwrap()[0].share, 100);
            assert!(parsed.data.collection.as_ref().unwrap().verified == false);
            assert!(parsed.is_mutable);
            assert!(parsed.collection_details.is_none());
        }
        _ => panic!("expected CreateMetadataAccountV3"),
    }
}

#[test]
fn token_metadata_create_metadata_account_v3_minimal() {
    let args = tm_ix::CreateMetadataAccountV3Args {
        data: tm_ix::DataV2 {
            name: "".to_string(),
            symbol: "".to_string(),
            uri: "".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: false,
        collection_details: None,
    };

    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT_V3];
    data.extend_from_slice(&to_vec(&args).unwrap());

    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccountV3(parsed) => {
            assert_eq!(parsed.data.name, "");
            assert!(parsed.data.creators.is_none());
            assert!(!parsed.is_mutable);
        }
        _ => panic!("expected CreateMetadataAccountV3"),
    }
}

#[test]
fn token_metadata_create_metadata_account_v2() {
    let args = tm_ix::CreateMetadataAccountV2Args {
        data: tm_ix::DataV2 {
            name: "Wrapped SOL".to_string(),
            symbol: "WSOL".to_string(),
            uri: "https://example.com/wsol.json".to_string(),
            seller_fee_basis_points: 0,
            creators: Some(vec![tm_ix::Creator {
                address: Pubkey::new_from_array([1; 32]),
                verified: true,
                share: 100,
            }]),
            collection: Some(tm_ix::Collection {
                verified: false,
                key: Pubkey::new_from_array([2; 32]),
            }),
            uses: None,
        },
        is_mutable: true,
    };

    let mut data = vec![tm_ix::CREATE_METADATA_ACCOUNT_V2];
    data.extend_from_slice(&to_vec(&args).unwrap());

    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccountV2(parsed) => {
            assert_eq!(parsed.data.name, "Wrapped SOL");
            assert_eq!(parsed.data.symbol, "WSOL");
            assert_eq!(parsed.data.creators, args.data.creators);
            assert_eq!(parsed.data.collection, args.data.collection);
            assert!(parsed.is_mutable);
        }
        _ => panic!("expected CreateMetadataAccountV2"),
    }
}

#[test]
fn token_metadata_create_master_edition_v3() {
    let mut data = vec![tm_ix::CREATE_MASTER_EDITION_V3];
    data.push(1); // Some
    data.extend_from_slice(&100u64.to_le_bytes());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMasterEditionV3(args) => {
            assert_eq!(args.max_supply, Some(100));
        }
        _ => panic!("expected CreateMasterEditionV3"),
    }
}

#[test]
fn token_metadata_create_master_edition_v3_unlimited() {
    let mut data = vec![tm_ix::CREATE_MASTER_EDITION_V3];
    data.push(0); // None (unlimited supply)
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMasterEditionV3(args) => {
            assert_eq!(args.max_supply, None);
        }
        _ => panic!("expected CreateMasterEditionV3"),
    }
}

#[test]
fn token_metadata_create_master_edition() {
    let args = tm_ix::CreateMasterEditionArgs { max_supply: Some(50) };
    let mut data = vec![tm_ix::CREATE_MASTER_EDITION];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMasterEdition(parsed) => {
            assert_eq!(parsed.max_supply, Some(50));
        }
        _ => panic!("expected CreateMasterEdition"),
    }
}

#[test]
fn token_metadata_mint_new_edition_via_token() {
    let args = tm_ix::MintNewEditionFromMasterEditionViaTokenArgs { edition: 42 };
    let mut data = vec![tm_ix::MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::MintNewEditionFromMasterEditionViaToken(parsed) => {
            assert_eq!(parsed.edition, 42);
        }
        _ => panic!("expected MintNewEditionFromMasterEditionViaToken"),
    }
}

#[test]
fn token_metadata_update_metadata_account_v2() {
    let args = tm_ix::UpdateMetadataAccountV2Args {
        data: Some(tm_ix::DataV2 {
            name: "Updated".to_string(),
            symbol: "UPD".to_string(),
            uri: "https://example.com/updated.json".to_string(),
            seller_fee_basis_points: 250,
            creators: None,
            collection: None,
            uses: None,
        }),
        update_authority: Some(Pubkey::new_from_array([3; 32])),
        primary_sale_happened: Some(true),
        is_mutable: Some(false),
    };

    let mut data = vec![tm_ix::UPDATE_METADATA_ACCOUNT_V2];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccountV2(parsed) => {
            assert_eq!(parsed.data.as_ref().unwrap().name, "Updated");
            assert_eq!(parsed.data.as_ref().unwrap().seller_fee_basis_points, 250);
            assert_eq!(parsed.update_authority, Some(Pubkey::new_from_array([3; 32])));
            assert_eq!(parsed.primary_sale_happened, Some(true));
            assert_eq!(parsed.is_mutable, Some(false));
        }
        _ => panic!("expected UpdateMetadataAccountV2"),
    }
}

#[test]
fn token_metadata_update_metadata_account_v2_all_none() {
    let args = tm_ix::UpdateMetadataAccountV2Args {
        data: None,
        update_authority: None,
        primary_sale_happened: None,
        is_mutable: None,
    };

    let mut data = vec![tm_ix::UPDATE_METADATA_ACCOUNT_V2];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccountV2(parsed) => {
            assert!(parsed.data.is_none());
            assert!(parsed.update_authority.is_none());
            assert!(parsed.primary_sale_happened.is_none());
            assert!(parsed.is_mutable.is_none());
        }
        _ => panic!("expected UpdateMetadataAccountV2"),
    }
}

#[test]
fn token_metadata_set_collection_size() {
    let args = tm_ix::SetCollectionSizeArgs { size: 10_000 };
    let mut data = vec![tm_ix::SET_COLLECTION_SIZE];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::SetCollectionSize(parsed) => {
            assert_eq!(parsed.size, 10_000);
        }
        _ => panic!("expected SetCollectionSize"),
    }
}

// ── Token Metadata: v1.13+ unified instructions ───────────────────────

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

    let mut data = vec![tm_ix::CREATE, 0];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::Create(parsed) => {
            assert_eq!(parsed.name, "Wrapped SOL");
            assert_eq!(parsed.symbol, "WSOL");
            assert_eq!(parsed.token_standard, tm_ix::TokenStandard::Fungible);
            assert_eq!(parsed.decimals, Some(9));
            assert!(parsed.print_supply.is_none());
        }
        _ => panic!("expected Create"),
    }
}

#[test]
fn token_metadata_create_v1_pnft() {
    let rule_set = Pubkey::new_from_array([99; 32]);
    let args = tm_ix::CreateV1InstructionArgs {
        name: "My pNFT".to_string(),
        symbol: "PNFT".to_string(),
        uri: "https://example.com/pnft.json".to_string(),
        seller_fee_basis_points: 500,
        creators: Some(vec![
            tm_ix::Creator { address: Pubkey::new_from_array([1; 32]), verified: true, share: 80 },
            tm_ix::Creator { address: Pubkey::new_from_array([2; 32]), verified: false, share: 20 },
        ]),
        primary_sale_happened: false,
        is_mutable: true,
        token_standard: tm_ix::TokenStandard::ProgrammableNonFungible,
        collection: Some(tm_ix::Collection { verified: false, key: Pubkey::new_from_array([3; 32]) }),
        uses: None,
        collection_details: None,
        rule_set: Some(rule_set),
        decimals: None,
        print_supply: Some(tm_ix::PrintSupply::Zero),
    };

    let mut data = vec![tm_ix::CREATE, 0];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::Create(parsed) => {
            assert_eq!(parsed.token_standard, tm_ix::TokenStandard::ProgrammableNonFungible);
            assert_eq!(parsed.rule_set, Some(rule_set));
            assert_eq!(parsed.creators.as_ref().unwrap().len(), 2);
            assert_eq!(parsed.creators.as_ref().unwrap()[0].share, 80);
            assert_eq!(parsed.print_supply, Some(tm_ix::PrintSupply::Zero));
        }
        _ => panic!("expected Create"),
    }
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

    let mut data = vec![tm_ix::UPDATE, 0];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::Update(parsed) => {
            let d = parsed.data.expect("expected data");
            assert_eq!(d.name, "Wrapped SOL");
            assert_eq!(d.symbol, "WSOL");
            assert_eq!(parsed.is_mutable, Some(true));
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn token_metadata_update_v1_clear_collection() {
    let args = tm_ix::UpdateV1InstructionArgs {
        new_update_authority: Some(Pubkey::new_from_array([5; 32])),
        data: None,
        primary_sale_happened: Some(true),
        is_mutable: None,
        collection: tm_ix::CollectionToggle::Clear,
        collection_details: tm_ix::CollectionDetailsToggle::None,
        uses: tm_ix::UsesToggle::Clear,
        rule_set: tm_ix::RuleSetToggle::Clear,
        authorization_data: None,
    };

    let mut data = vec![tm_ix::UPDATE, 0];
    data.extend_from_slice(&to_vec(&args).unwrap());
    match tm_ix::unpack(&data).unwrap() {
        tm_ix::TokenMetadataInstruction::Update(parsed) => {
            assert_eq!(parsed.new_update_authority, Some(Pubkey::new_from_array([5; 32])));
            assert!(parsed.data.is_none());
            assert_eq!(parsed.primary_sale_happened, Some(true));
            assert_eq!(parsed.collection, tm_ix::CollectionToggle::Clear);
            assert_eq!(parsed.uses, tm_ix::UsesToggle::Clear);
            assert_eq!(parsed.rule_set, tm_ix::RuleSetToggle::Clear);
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn token_metadata_burn_v1() {
    let args = tm_ix::BurnArgs::V1 { amount: 7 };
    let mut data = vec![tm_ix::BURN];
    data.extend_from_slice(&to_vec(&args).unwrap());
    assert_eq!(tm_ix::unpack(&data).unwrap(), tm_ix::TokenMetadataInstruction::Burn(tm_ix::BurnArgs::V1 { amount: 7 }));
}

#[test]
fn token_metadata_mint_v1() {
    let args = tm_ix::MintArgs::V1 { amount: 1, authorization_data: None };
    let mut data = vec![tm_ix::MINT];
    data.extend_from_slice(&to_vec(&args).unwrap());
    assert_eq!(
        tm_ix::unpack(&data).unwrap(),
        tm_ix::TokenMetadataInstruction::Mint(tm_ix::MintArgs::V1 { amount: 1, authorization_data: None })
    );
}

#[test]
fn token_metadata_transfer_v1() {
    let args = tm_ix::TransferArgs::V1 { amount: 11, authorization_data: None };
    let mut data = vec![tm_ix::TRANSFER];
    data.extend_from_slice(&to_vec(&args).unwrap());
    assert_eq!(
        tm_ix::unpack(&data).unwrap(),
        tm_ix::TokenMetadataInstruction::Transfer(tm_ix::TransferArgs::V1 { amount: 11, authorization_data: None })
    );
}

// ── Token Metadata: DelegateArgs variants ──────────────────────────────

#[test]
fn token_metadata_delegate_all_variants() {
    let variants: Vec<tm_ix::DelegateArgs> = vec![
        tm_ix::DelegateArgs::CollectionV1 { authorization_data: None },
        tm_ix::DelegateArgs::SaleV1 { amount: 1, authorization_data: None },
        tm_ix::DelegateArgs::TransferV1 { amount: 2, authorization_data: None },
        tm_ix::DelegateArgs::DataV1 { authorization_data: None },
        tm_ix::DelegateArgs::UtilityV1 { amount: 3, authorization_data: None },
        tm_ix::DelegateArgs::StakingV1 { amount: 4, authorization_data: None },
        tm_ix::DelegateArgs::StandardV1 { amount: 5 },
        tm_ix::DelegateArgs::LockedTransferV1 {
            amount: 6,
            locked_address: Pubkey::new_from_array([7; 32]),
            authorization_data: None,
        },
        tm_ix::DelegateArgs::ProgrammableConfigV1 { authorization_data: None },
        tm_ix::DelegateArgs::AuthorityItemV1 { authorization_data: None },
        tm_ix::DelegateArgs::DataItemV1 { authorization_data: None },
        tm_ix::DelegateArgs::CollectionItemV1 { authorization_data: None },
        tm_ix::DelegateArgs::ProgrammableConfigItemV1 { authorization_data: None },
        tm_ix::DelegateArgs::PrintDelegateV1 { authorization_data: None },
    ];

    for (i, variant) in variants.into_iter().enumerate() {
        let mut data = vec![tm_ix::DELEGATE];
        data.extend_from_slice(&to_vec(&variant).unwrap());
        let ix = tm_ix::unpack(&data).unwrap_or_else(|e| panic!("DelegateArgs variant {} failed: {}", i, e));
        assert_eq!(ix, tm_ix::TokenMetadataInstruction::Delegate(variant), "DelegateArgs variant {} mismatch", i);
    }
}

// ── Token Metadata: real mainnet fixtures ──────────────────────────────

#[test]
fn token_metadata_real_create_metadata_account_v3() {
    match tm_ix::unpack(metaplex_fixtures::REAL_CREATE_METADATA_ACCOUNT_V3_IX).unwrap() {
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
fn token_metadata_real_create_metadata_account() {
    match tm_ix::unpack(metaplex_fixtures::REAL_CREATE_METADATA_ACCOUNT_IX).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMetadataAccount(args) => {
            assert_eq!(args.data.name, "SaibaGang #1092\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
            assert_eq!(args.data.symbol, "SBAGNG\0\0\0\0");
            assert!(args.data.uri.starts_with("https://arweave.net/"));
            assert_eq!(args.data.seller_fee_basis_points, 500);
            assert_eq!(args.data.creators.as_ref().unwrap().len(), 5);
            assert!(args.is_mutable);
        }
        _ => panic!("expected CreateMetadataAccount"),
    }
}

#[test]
fn token_metadata_real_create_master_edition() {
    match tm_ix::unpack(metaplex_fixtures::REAL_CREATE_MASTER_EDITION_IX).unwrap() {
        tm_ix::TokenMetadataInstruction::CreateMasterEdition(args) => {
            assert_eq!(args.max_supply, Some(0));
        }
        _ => panic!("expected CreateMasterEdition"),
    }
}

#[test]
fn token_metadata_real_mint_new_edition_via_token() {
    match tm_ix::unpack(metaplex_fixtures::REAL_MINT_NEW_EDITION_VIA_TOKEN_IX).unwrap() {
        tm_ix::TokenMetadataInstruction::MintNewEditionFromMasterEditionViaToken(args) => {
            assert_eq!(args.edition, 430);
        }
        _ => panic!("expected MintNewEditionFromMasterEditionViaToken"),
    }
}

#[test]
fn token_metadata_real_update_metadata_account() {
    match tm_ix::unpack(metaplex_fixtures::REAL_UPDATE_METADATA_ACCOUNT_IX).unwrap() {
        tm_ix::TokenMetadataInstruction::UpdateMetadataAccount(args) => {
            assert!(args.data.is_none());
            assert!(args.update_authority.is_some());
            assert_eq!(args.primary_sale_happened, Some(true));
        }
        _ => panic!("expected UpdateMetadataAccount"),
    }
}

// ── Bubblegum: error handling ──────────────────────────────────────────

#[test]
fn bubblegum_empty() {
    assert!(matches!(bgum_ix::unpack(&[]), Err(ParseError::TooShort(0))));
}

#[test]
fn bubblegum_too_short() {
    assert!(matches!(bgum_ix::unpack(&[0u8; 4]), Err(ParseError::TooShort(4))));
}

#[test]
fn bubblegum_too_short_7_bytes() {
    assert!(matches!(bgum_ix::unpack(&[0u8; 7]), Err(ParseError::TooShort(7))));
}

#[test]
fn bubblegum_unknown_discriminator() {
    assert!(matches!(bgum_ix::unpack(&[0u8; 8]), Err(ParseError::Unknown(_))));
}

// ── Bubblegum: all instruction discriminators ──────────────────────────

#[test]
fn bubblegum_all_discriminators() {
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
        // With trailing payload bytes
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&[0u8; 32]);
        let ix = bgum_ix::unpack(&data).unwrap();
        assert_eq!(ix, expected);

        // With exactly 8 bytes (minimum)
        let ix2 = bgum_ix::unpack(&discriminator).unwrap();
        assert_eq!(ix2, expected);
    }
}
