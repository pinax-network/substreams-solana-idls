use substreams_solana_idls::okx::v2::instructions;
use substreams_solana_idls::okx::v2::{events, events::OkxV2Event};

#[test]
fn unknown_discriminator() {
    assert!(instructions::unpack(&[0u8; 16]).is_err());
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[0u8; 4]).is_err());
}

#[test]
fn event_unknown() {
    assert!(matches!(events::unpack(&[0u8; 24]).unwrap(), OkxV2Event::Unknown));
}
