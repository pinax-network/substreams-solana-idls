use substreams_solana_idls::meteora::amm::instructions as amm_ix;
use substreams_solana_idls::meteora::daam::instructions as daam_ix;
use substreams_solana_idls::meteora::dlmm::instructions as dlmm_ix;

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
fn dlmm_unknown_discriminator() {
    assert!(dlmm_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn dlmm_too_short() {
    assert!(dlmm_ix::unpack(&[0u8; 4]).is_err());
}
