// --- V1 (raw data payload) ---
#[test]
fn v1_swap() {
    use solfi::v1::instructions::*;
    let mut data = vec![SWAP];
    data.extend_from_slice(&[1, 2, 3, 4]);
    let parsed = unpack(&data).unwrap();
    assert!(matches!(parsed, SolfiInstruction::Swap(s) if s.data == vec![1, 2, 3, 4]));
}

#[test]
fn v1_unknown() { assert!(solfi::v1::instructions::unpack(&[255u8]).is_err()); }
#[test]
fn v1_too_short() { assert!(solfi::v1::instructions::unpack(&[]).is_err()); }

// --- V2 (structured payload) ---
#[test]
fn v2_swap() {
    use solfi::v2::instructions::*;
    let instr = SwapInstruction { amount_in: 2_000_000, minimum_out: 1_000_000, direction: 0 };
    let mut data = vec![SWAP];
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), SolfiInstruction::Swap(s) if s.amount_in == 2_000_000));
}

#[test]
fn v2_unknown() { assert!(solfi::v2::instructions::unpack(&[255u8]).is_err()); }

// --- Events ---
#[test]
fn v1_event_unknown() { assert!(solfi::v1::events::unpack(&[0u8; 24]).is_err()); }
#[test]
fn v2_event_unknown() { assert!(solfi::v2::events::unpack(&[0u8; 24]).is_err()); }
