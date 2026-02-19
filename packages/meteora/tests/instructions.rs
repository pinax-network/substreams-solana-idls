use meteora::amm::instructions as amm_ix;
use meteora::daam::instructions as daam_ix;
use meteora::dllm::instructions as dllm_ix;

#[test]
fn amm_unknown_discriminator() {
    assert!(amm_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn amm_too_short() {
    assert!(amm_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn daam_unknown_discriminator() {
    assert!(daam_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn daam_too_short() {
    assert!(daam_ix::unpack(&[0u8; 4]).is_err());
}

#[test]
fn dllm_unknown_discriminator() {
    assert!(dllm_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn dllm_too_short() {
    assert!(dllm_ix::unpack(&[0u8; 4]).is_err());
}
