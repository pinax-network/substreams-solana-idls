use pancakeswap::events;
use pancakeswap::instructions;

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
    assert!(events::unpack(&[0u8; 24]).is_err());
}
