use raydium::cpmm::events::{self as cpmm_ev, RaydiumCpmmEvent};
use raydium::clmm::v3::events as clmm_ev;
use raydium::stable::events::{self as stable_ev, RaydiumStableEvent};

#[test]
fn cpmm_event_too_short() { assert!(cpmm_ev::unpack(&[0u8; 4]).is_err()); }
#[test]
fn cpmm_event_unknown_returns_variant() {
    assert!(matches!(cpmm_ev::unpack(&[0u8; 24]).unwrap(), RaydiumCpmmEvent::Unknown));
}

#[test]
fn clmm_event_unknown() { assert!(clmm_ev::unpack(&[0u8; 24]).is_err()); }

#[test]
fn stable_event_unknown_returns_variant() {
    assert!(matches!(stable_ev::unpack(&[0u8; 24]).unwrap(), RaydiumStableEvent::Unknown));
}
