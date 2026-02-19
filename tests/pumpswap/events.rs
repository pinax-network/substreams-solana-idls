use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;
use substreams_solana_idls::pumpswap::events::*;

#[test]
fn test_sell_event() {
    let event = SellEvent {
        timestamp: 1234567890,
        base_amount_in: 1_000_000,
        min_quote_amount_out: 500_000,
        user_base_token_reserves: 100,
        user_quote_token_reserves: 200,
        pool_base_token_reserves: 300,
        pool_quote_token_reserves: 400,
        quote_amount_out: 450_000,
        lp_fee_basis_points: 25,
        lp_fee: 1000,
        protocol_fee_basis_points: 10,
        protocol_fee: 500,
        quote_amount_out_without_lp_fee: 451_000,
        user_quote_amount_out: 449_000,
        pool: Pubkey::new_unique(),
        user: Pubkey::new_unique(),
        user_base_token_account: Pubkey::new_unique(),
        user_quote_token_account: Pubkey::new_unique(),
        protocol_fee_recipient: Pubkey::new_unique(),
        protocol_fee_recipient_token_account: Pubkey::new_unique(),
        coin_creator: Pubkey::new_unique(),
        coin_creator_fee_basis_points: 5,
        coin_creator_fee: 250,
    };
    let mut data = SELL_EVENT.to_vec();
    data.extend_from_slice(&borsh::to_vec(&event).unwrap());
    match unpack_event(&data).unwrap() {
        PumpSwapEvent::Sell(parsed) => {
            assert_eq!(parsed.timestamp, 1234567890);
            assert_eq!(parsed.base_amount_in, 1_000_000);
            assert_eq!(parsed.quote_amount_out, 450_000);
        }
        other => panic!("expected Sell, got {:?}", other),
    }
}

#[test]
fn test_create_pool_event() {
    let event = CreatePoolEvent {
        timestamp: 1234567890,
        index: 1,
        creator: Pubkey::new_unique(),
        base_mint: Pubkey::new_unique(),
        quote_mint: Pubkey::new_unique(),
        base_mint_decimals: 9,
        quote_mint_decimals: 6,
        base_amount_in: 1_000_000,
        quote_amount_in: 500_000,
        pool_base_amount: 900_000,
        pool_quote_amount: 450_000,
        minimum_liquidity: 100,
        initial_liquidity: 10_000,
        lp_token_amount_out: 9_900,
        pool_bump: 255,
        pool: Pubkey::new_unique(),
        lp_mint: Pubkey::new_unique(),
        user_base_token_account: Pubkey::new_unique(),
        user_quote_token_account: Pubkey::new_unique(),
        coin_creator: Pubkey::new_unique(),
        is_mayhem_mode: false,
    };
    let mut data = CREATE_POOL_EVENT.to_vec();
    data.extend_from_slice(&borsh::to_vec(&event).unwrap());
    match unpack_event(&data).unwrap() {
        PumpSwapEvent::CreatePool(parsed) => {
            assert_eq!(parsed.index, 1);
            assert_eq!(parsed.base_mint_decimals, 9);
            assert!(!parsed.is_mayhem_mode);
        }
        other => panic!("expected CreatePool, got {:?}", other),
    }
}

#[test]
fn test_withdraw_event() {
    let event = WithdrawEvent {
        timestamp: 999,
        lp_token_amount_in: 100,
        min_base_amount_out: 50,
        min_quote_amount_out: 25,
        user_base_token_reserves: 1000,
        user_quote_token_reserves: 2000,
        pool_base_token_reserves: 3000,
        pool_quote_token_reserves: 4000,
        base_amount_out: 55,
        quote_amount_out: 28,
        lp_mint_supply: 5000,
        pool: Pubkey::new_unique(),
        user: Pubkey::new_unique(),
        user_base_token_account: Pubkey::new_unique(),
        user_quote_token_account: Pubkey::new_unique(),
        user_pool_token_account: Pubkey::new_unique(),
    };
    let mut data = WITHDRAW_EVENT.to_vec();
    data.extend_from_slice(&borsh::to_vec(&event).unwrap());
    match unpack_event(&data).unwrap() {
        PumpSwapEvent::Withdraw(parsed) => {
            assert_eq!(parsed.timestamp, 999);
            assert_eq!(parsed.base_amount_out, 55);
        }
        other => panic!("expected Withdraw, got {:?}", other),
    }
}

#[test]
fn test_unknown_event() {
    let data = [0u8; 16];
    assert!(unpack_event(&data).is_err());
}
