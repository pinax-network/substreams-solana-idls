use phoenix::instructions::{self, PhonenixInstruction};
use phoenix::events;

#[test]
fn parse_swap() {
    let data = [0u8, 1, 2, 3]; // disc=0 is SWAP
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, PhonenixInstruction::Swap(_)));
}

#[test]
fn parse_swap_with_free_funds() {
    let data = [1u8, 4, 5, 6]; // disc=1 is SWAP_WITH_FREE_FUNDS
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, PhonenixInstruction::SwapWithFreeFunds(_)));
}

#[test]
fn unknown_discriminator() {
    assert!(instructions::unpack(&[255u8]).is_err());
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[]).is_err());
}

#[test]
fn event_unknown() { assert!(events::unpack(&[0u8; 24]).is_err()); }
