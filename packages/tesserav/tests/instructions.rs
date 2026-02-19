use tesserav::instructions::*;

#[test]
fn parse_swap() {
    let instr = SwapInstruction { is_a_to_b: true, amount_in: 1_000_000, min_amount_out: 500_000 };
    let mut data = vec![SWAP];
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), TesseraVInstruction::Swap(s) if s.is_a_to_b));
}

#[test]
fn unknown() { assert!(unpack(&[255u8]).is_err()); }
#[test]
fn too_short() { assert!(unpack(&[]).is_err()); }
