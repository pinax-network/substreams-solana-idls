use substreams_solana_idls::stabble::events;
use substreams_solana_idls::stabble::instructions::*;

#[test]
fn parse_swap() {
    let instr = SwapInstruction {
        amount_in: Some(1_000_000),
        minimum_amount_out: 500_000,
    };
    let mut data = SWAP.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), StabbleInstruction::Swap(s) if s.minimum_amount_out == 500_000));
}

#[test]
fn unknown() {
    assert!(unpack(&[0u8; 16]).is_err());
}
#[test]
fn too_short() {
    assert!(unpack(&[0u8; 4]).is_err());
}
#[test]
fn event_unknown() {
    assert!(events::unpack(&[0u8; 24]).is_err());
}
