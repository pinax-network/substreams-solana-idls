use substreams_solana_idls::metaplex::bubblegum::instructions as bgum_ix;
use substreams_solana_idls::metaplex::token_metadata::instructions as tm_ix;

#[test]
fn token_metadata_unknown_discriminator() {
    // discriminator 255 is not valid
    assert!(tm_ix::unpack(&[255]).is_err());
}

#[test]
fn token_metadata_empty() {
    assert!(tm_ix::unpack(&[]).is_err());
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
fn bubblegum_unknown_discriminator() {
    assert!(bgum_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn bubblegum_too_short() {
    assert!(bgum_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn bubblegum_transfer() {
    let mut data = bgum_ix::TRANSFER.to_vec();
    data.extend_from_slice(&[0u8; 64]); // dummy payload
    let ix = bgum_ix::unpack(&data).unwrap();
    assert_eq!(ix, bgum_ix::BubblegumInstruction::Transfer);
}
