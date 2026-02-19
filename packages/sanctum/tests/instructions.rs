use sanctum::instructions::*;

#[test]
fn parse_stake_wrapped_sol() {
    let mut data = vec![STAKE_WRAPPED_SOL];
    data.extend_from_slice(&1_000_000_000u64.to_le_bytes());
    match unpack(&data).unwrap() {
        SanctumInstruction::StakeWrappedSol { amount } => assert_eq!(amount, 1_000_000_000),
        other => panic!("expected StakeWrappedSol, got {:?}", other),
    }
}

#[test]
fn parse_swap_via_stake() {
    let mut data = vec![SWAP_VIA_STAKE];
    data.extend_from_slice(&500_000_000u64.to_le_bytes());
    match unpack(&data).unwrap() {
        SanctumInstruction::SwapViaStake { amount } => assert_eq!(amount, 500_000_000),
        other => panic!("expected SwapViaStake, got {:?}", other),
    }
}

#[test]
fn parse_create_fee_token_account() {
    assert!(matches!(
        unpack(&[CREATE_FEE_TOKEN_ACCOUNT]).unwrap(),
        SanctumInstruction::CreateFeeTokenAccount
    ));
}

#[test]
fn parse_deposit_stake() {
    assert!(matches!(unpack(&[DEPOSIT_STAKE]).unwrap(), SanctumInstruction::DepositStake));
}

#[test]
fn parse_withdraw_wrapped_sol() {
    let mut data = vec![WITHDRAW_WRAPPED_SOL];
    data.extend_from_slice(&2_000_000_000u64.to_le_bytes());
    match unpack(&data).unwrap() {
        SanctumInstruction::WithdrawWrappedSol { amount } => assert_eq!(amount, 2_000_000_000),
        other => panic!("expected WithdrawWrappedSol, got {:?}", other),
    }
}

#[test]
fn unknown_discriminator() {
    assert!(unpack(&[255u8]).is_err());
}

#[test]
fn too_short() {
    assert!(unpack(&[]).is_err());
}
