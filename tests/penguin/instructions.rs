use substreams_solana_idls::penguin::instructions::*;

#[test]
fn parse_swap() {
    // Manually encode: amount_in (u64 LE) + minimum_amount_out (u64 LE)
    let mut data = vec![SWAP];
    data.extend(1_000_000u64.to_le_bytes());
    data.extend(500_000u64.to_le_bytes());
    assert!(matches!(unpack(&data).unwrap(), TokenSwapInstruction::Swap { amount_in, .. } if amount_in == 1_000_000));
}

#[test]
fn unknown() {
    assert!(unpack(&[255u8]).is_err());
}

#[test]
fn too_short() {
    assert!(unpack(&[]).is_err());
}
