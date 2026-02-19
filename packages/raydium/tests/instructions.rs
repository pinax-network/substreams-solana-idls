use raydium::amm::v4::instructions::*;

#[test]
fn parse_swap_base_in() {
    let instr = SwapBaseInInstruction {
        amount_in: 1_000_000,
        minimum_amount_out: 500_000,
    };
    let mut data = vec![SWAP_BASE_IN];
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), RaydiumV4Instruction::SwapBaseIn(s) if s.amount_in == 1_000_000));
}

#[test]
fn parse_swap_base_out() {
    let instr = SwapBaseOutInstruction {
        max_amount_in: 2_000_000,
        amount_out: 1_000_000,
    };
    let mut data = vec![SWAP_BASE_OUT];
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), RaydiumV4Instruction::SwapBaseOut(s) if s.amount_out == 1_000_000));
}

#[test]
fn amm_v4_too_short() {
    assert!(unpack(&[]).is_err());
}

// --- CPMM ---
#[test]
fn cpmm_unknown() {
    assert!(raydium::cpmm::instructions::unpack(&[0u8; 16]).is_err());
}
#[test]
fn cpmm_too_short() {
    assert!(raydium::cpmm::instructions::unpack(&[0u8; 4]).is_err());
}

// --- CLMM V3 ---
#[test]
fn clmm_unknown() {
    assert!(raydium::clmm::v3::instructions::unpack(&[0u8; 16]).is_err());
}
#[test]
fn clmm_too_short() {
    assert!(raydium::clmm::v3::instructions::unpack(&[0u8; 4]).is_err());
}

// --- Stable ---
#[test]
fn stable_unknown() {
    // stable uses single-byte discriminator; 0 is a valid instruction
    assert!(raydium::stable::instructions::unpack(&[255u8]).is_err());
}

// --- Launchpad ---
#[test]
fn launchpad_unknown() {
    assert!(raydium::launchpad::instructions::unpack(&[0u8; 16]).is_err());
}
