use drift::v2::instructions;
use drift::v2::events::{self, DriftEvent};

#[test]
fn unknown() { assert!(instructions::unpack(&[0u8; 16]).is_err()); }
#[test]
fn too_short() { assert!(instructions::unpack(&[0u8; 4]).is_err()); }
#[test]
fn event_unknown_returns_variant() {
    // drift events return Ok(Unknown)
    assert!(matches!(events::unpack(&[0u8; 24]).unwrap(), DriftEvent::Unknown));
}
