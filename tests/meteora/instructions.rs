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
fn daam_unpack_swap() {
    // SWAP discriminator + 8-byte amount_in + 8-byte minimum_amount_out (zero
    // values are fine — the Borsh deserializer doesn't care about content).
    let mut data = daam_ix::SWAP.to_vec();
    data.extend_from_slice(&0u64.to_le_bytes());
    data.extend_from_slice(&0u64.to_le_bytes());
    let parsed = daam_ix::unpack(&data).expect("SWAP must decode");
    assert!(matches!(parsed, daam_ix::MeteoraDammInstruction::Swap(_)));
}

#[test]
fn daam_unpack_swap2() {
    let mut data = daam_ix::SWAP2.to_vec();
    data.extend_from_slice(&1_087_808_068u64.to_le_bytes());
    data.extend_from_slice(&0u64.to_le_bytes());
    data.push(1);
    let parsed = daam_ix::unpack(&data).expect("SWAP2 must decode");
    match parsed {
        daam_ix::MeteoraDammInstruction::Swap2(ix) => {
            assert_eq!(ix.params.amount_0, 1_087_808_068);
            assert_eq!(ix.params.amount_1, 0);
            assert_eq!(ix.params.swap_mode, 1);
        }
        other => panic!("expected Swap2, got {other:?}"),
    }

    data.pop();
    assert!(daam_ix::unpack(&data).is_err(), "swap mode is required");
    data.extend_from_slice(&[1, 0]);
    assert!(daam_ix::unpack(&data).is_err(), "unexpected trailing bytes must fail");
}

#[test]
fn dlmm_unknown_discriminator() {
    assert!(dlmm_ix::unpack(&[0u8; 16]).is_err());
}

#[test]
fn dlmm_too_short() {
    assert!(dlmm_ix::unpack(&[0u8; 4]).is_err());
}
