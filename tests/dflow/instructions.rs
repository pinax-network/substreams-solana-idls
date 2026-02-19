use substreams_solana_idls::dflow::v4::events::{self, DflowV4Event};
use substreams_solana_idls::dflow::v4::instructions::{self, DflowV4Instruction};

#[test]
fn unknown_returns_unknown_variant() {
    // dflow returns Ok(Unknown) for unknown discriminators
    assert!(matches!(instructions::unpack(&[0u8; 16]).unwrap(), DflowV4Instruction::Unknown));
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[0u8; 4]).is_err());
}

#[test]
fn event_unknown_returns_unknown_variant() {
    assert!(matches!(events::unpack(&[0u8; 24]).unwrap(), DflowV4Event::Unknown));
}
