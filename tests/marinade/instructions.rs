use substreams_solana_idls::marinade::instructions::*;

#[test]
fn parse_deposit() {
    let instr = DepositInstruction { lamports: 1_000_000_000 };
    let mut data = DEPOSIT.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeInstruction::Deposit(d) if d.lamports == 1_000_000_000));
}

#[test]
fn parse_liquid_unstake() {
    let instr = LiquidUnstakeInstruction { msol_amount: 500_000_000 };
    let mut data = LIQUID_UNSTAKE.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeInstruction::LiquidUnstake(u) if u.msol_amount == 500_000_000));
}

#[test]
fn parse_add_liquidity() {
    let instr = AddLiquidityInstruction { lamports: 2_000_000_000 };
    let mut data = ADD_LIQUIDITY.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeInstruction::AddLiquidity(a) if a.lamports == 2_000_000_000));
}

#[test]
fn parse_remove_liquidity() {
    let instr = RemoveLiquidityInstruction { tokens: 100_000 };
    let mut data = REMOVE_LIQUIDITY.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeInstruction::RemoveLiquidity(r) if r.tokens == 100_000));
}

#[test]
fn parse_order_unstake() {
    let instr = OrderUnstakeInstruction { msol_amount: 300_000_000 };
    let mut data = ORDER_UNSTAKE.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeInstruction::OrderUnstake(o) if o.msol_amount == 300_000_000));
}

#[test]
fn parse_claim() {
    assert!(matches!(unpack(&CLAIM).unwrap(), MarinadeInstruction::Claim));
}

#[test]
fn parse_withdraw_stake_account() {
    let instr = WithdrawStakeAccountInstruction {
        msol_amount: 1_000_000,
        stake_index: 5,
        validator_index: 3,
    };
    let mut data = WITHDRAW_STAKE_ACCOUNT.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeInstruction::WithdrawStakeAccount(w) if w.stake_index == 5));
}

#[test]
fn parse_admin_instructions() {
    assert!(matches!(unpack(&INITIALIZE).unwrap(), MarinadeInstruction::Initialize));
    assert!(matches!(unpack(&PAUSE).unwrap(), MarinadeInstruction::Pause));
    assert!(matches!(unpack(&RESUME).unwrap(), MarinadeInstruction::Resume));
}

#[test]
fn unknown_discriminator() {
    assert!(unpack(&[0u8; 16]).is_err());
}

#[test]
fn too_short() {
    assert!(unpack(&[0u8; 4]).is_err());
}
