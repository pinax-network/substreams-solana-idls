use jupiter::dca::events::{self as dca_ev, JupiterDcaEvent};
use jupiter::limit_order::events as lo_ev;
use jupiter::v4::events as v4_ev;
use jupiter::v6::events as v6_ev;

#[test]
fn v4_event_unknown() { assert!(v4_ev::unpack(&[0u8; 24]).is_err()); }
#[test]
fn v6_event_unknown() { assert!(v6_ev::unpack(&[0u8; 24]).is_err()); }
#[test]
fn dca_event_unknown_returns_variant() {
    // dca events return Ok(Unknown)
    assert!(matches!(dca_ev::unpack(&[0u8; 24]).unwrap(), JupiterDcaEvent::Unknown));
}
#[test]
fn limit_order_event_unknown() { assert!(lo_ev::unpack(&[0u8; 24]).is_err()); }
