use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;
use substreams_solana_idls::pumpswap::accounts::*;

#[test]
fn test_pool_account() {
    let pool = Pool {
        pool_bump: 254,
        index: 7,
        creator: Pubkey::new_unique(),
        base_mint: Pubkey::new_unique(),
        quote_mint: Pubkey::new_unique(),
        lp_mint: Pubkey::new_unique(),
        pool_base_token_account: Pubkey::new_unique(),
        pool_quote_token_account: Pubkey::new_unique(),
        lp_supply: 1_000_000,
        coin_creator: Pubkey::new_unique(),
        is_mayhem_mode: true,
    };
    let mut data = POOL_DISC.to_vec();
    data.extend_from_slice(&borsh::to_vec(&pool).unwrap());
    match unpack_account(&data).unwrap() {
        PumpSwapAccount::Pool(parsed) => {
            assert_eq!(parsed.pool_bump, 254);
            assert_eq!(parsed.index, 7);
            assert_eq!(parsed.lp_supply, 1_000_000);
            assert!(parsed.is_mayhem_mode);
        }
        other => panic!("expected Pool, got {:?}", other),
    }
}

#[test]
fn test_bonding_curve_account() {
    let bc = BondingCurve {
        virtual_token_reserves: 1_000_000_000,
        virtual_sol_reserves: 30_000_000_000,
        real_token_reserves: 500_000_000,
        real_sol_reserves: 15_000_000_000,
        token_total_supply: 1_000_000_000_000,
        complete: false,
        creator: Pubkey::new_unique(),
    };
    let mut data = BONDING_CURVE_DISC.to_vec();
    data.extend_from_slice(&borsh::to_vec(&bc).unwrap());
    match unpack_account(&data).unwrap() {
        PumpSwapAccount::BondingCurve(parsed) => {
            assert_eq!(parsed.virtual_token_reserves, 1_000_000_000);
            assert!(!parsed.complete);
        }
        other => panic!("expected BondingCurve, got {:?}", other),
    }
}

#[test]
fn test_unknown_account() {
    let data = [0u8; 16];
    assert!(unpack_account(&data).is_err());
}
