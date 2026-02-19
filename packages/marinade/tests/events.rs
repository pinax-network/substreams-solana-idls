use marinade::events::*;
use solana_program::pubkey::Pubkey;

#[test]
fn parse_deposit_event() {
    let event = DepositEvent {
        state: Pubkey::new_unique(),
        sol_owner: Pubkey::new_unique(),
        user_sol_balance: 10_000_000_000,
        user_msol_balance: 5_000_000_000,
        sol_leg_balance: 100_000_000_000,
        msol_leg_balance: 50_000_000_000,
        reserve_balance: 200_000_000_000,
        sol_swapped: 0,
        msol_swapped: 0,
        sol_deposited: 1_000_000_000,
        msol_minted: 900_000_000,
        total_virtual_staked_lamports: 500_000_000_000,
        msol_supply: 450_000_000_000,
    };
    let mut data = DEPOSIT_EVENT.to_vec();
    data.extend(borsh::to_vec(&event).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeEvent::Deposit(e) if e.msol_minted == 900_000_000));
}

#[test]
fn parse_add_liquidity_event() {
    let event = AddLiquidityEvent {
        state: Pubkey::new_unique(),
        sol_owner: Pubkey::new_unique(),
        user_sol_balance: 5_000_000_000,
        user_lp_balance: 1_000_000,
        sol_leg_balance: 100_000_000_000,
        lp_supply: 50_000_000,
        sol_added_amount: 2_000_000_000,
        lp_minted: 1_000_000,
        total_virtual_staked_lamports: 500_000_000_000,
        msol_supply: 450_000_000_000,
    };
    let mut data = ADD_LIQUIDITY_EVENT.to_vec();
    data.extend(borsh::to_vec(&event).unwrap());
    assert!(matches!(unpack(&data).unwrap(), MarinadeEvent::AddLiquidity(e) if e.lp_minted == 1_000_000));
}

#[test]
fn unknown_event() {
    assert!(unpack(&[0u8; 16]).is_err());
}

#[test]
fn event_too_short() {
    assert!(unpack(&[0u8; 4]).is_err());
}
