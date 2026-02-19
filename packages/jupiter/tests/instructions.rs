use jupiter::dca::instructions as dca_ix;
use jupiter::limit_order::instructions as lo_ix;
use jupiter::v4::instructions as v4_ix;
use jupiter::v6::instructions as v6_ix;

#[test]
fn v4_unknown_discriminator() {
    assert!(v4_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn v4_too_short() {
    assert!(v4_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn v6_unknown_discriminator() {
    assert!(v6_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn v6_too_short() {
    assert!(v6_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn dca_unknown_discriminator() {
    assert!(dca_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn dca_too_short() {
    assert!(dca_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn limit_order_unknown_discriminator() {
    assert!(lo_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn limit_order_too_short() {
    assert!(lo_ix::unpack(&[0u8; 4]).is_err());
}
