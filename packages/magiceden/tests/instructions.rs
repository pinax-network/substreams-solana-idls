use magiceden::m2::instructions as m2_ix;
use magiceden::m3::instructions as m3_ix;

#[test]
fn m2_unknown_discriminator() {
    assert!(m2_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn m2_too_short() {
    assert!(m2_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn m3_unknown_discriminator() {
    assert!(m3_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn m3_too_short() {
    assert!(m3_ix::unpack(&[0u8; 4]).is_err());
}
