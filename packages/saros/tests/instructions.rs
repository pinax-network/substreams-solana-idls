use saros::instructions::*;

#[test]
fn parse_swap() {
    let instr = SwapInstruction {
        amount_in: 1_000_000,
        minimum_amount_out: 500_000,
    };
    let mut data = vec![SWAP];
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), SarosInstruction::Swap(s) if s.amount_in == 1_000_000));
}

#[test]
fn unknown() {
    assert!(unpack(&[255u8]).is_err());
}
#[test]
fn too_short() {
    assert!(unpack(&[]).is_err());
}
