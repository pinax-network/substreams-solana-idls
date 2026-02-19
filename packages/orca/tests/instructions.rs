use orca::whirlpool::events::{self, WhirlpoolEvent};
use orca::whirlpool::instructions;

#[test]
fn whirlpool_unknown() {
    assert!(instructions::unpack(&[0u8; 16]).is_err());
}
#[test]
fn whirlpool_too_short() {
    assert!(instructions::unpack(&[0u8; 4]).is_err());
}
#[test]
fn whirlpool_event_unknown_returns_variant() {
    // orca events return Ok(Unknown)
    assert!(matches!(events::parse_event(&[0u8; 24]).unwrap(), WhirlpoolEvent::Unknown));
}
