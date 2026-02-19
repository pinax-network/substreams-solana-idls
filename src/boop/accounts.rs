//! Boop on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum BondingCurveStatus {
    Trading,
    Graduated,
    PoolPriceCorrected,
    LiquidityProvisioned,
    LiquidityLocked,
}

// Account discriminators
pub const AMM_CONFIG_ACCOUNT: [u8; 8] = [218, 244, 33, 104, 203, 203, 43, 111];
pub const BONDING_CURVE_ACCOUNT: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];
pub const CONFIG_ACCOUNT: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
pub const LOCKED_CP_LIQUIDITY_STATE_ACCOUNT: [u8; 8] = [25, 10, 238, 197, 207, 234, 73, 22];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AmmConfig {
    pub bump: u8,
    pub disable_create_pool: bool,
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub protocol_owner: Pubkey,
    pub fund_owner: Pubkey,
    pub padding: [u64; 16],
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BondingCurve {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub sol_reserves: u64,
    pub token_reserves: u64,
    pub damping_term: u8,
    pub swap_fee_basis_points: u8,
    pub token_for_stakers_basis_points: u16,
    pub status: BondingCurveStatus,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Config {
    pub is_paused: bool,
    pub authority: Pubkey,
    pub pending_authority: Pubkey,
    pub operators: Vec<Pubkey>,
    pub protocol_fee_recipient: Pubkey,
    pub token_distributor: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub damping_term: u8,
    pub token_for_stakers_basis_points: u16,
    pub swap_fee_basis_points: u8,
    pub token_amount_for_raydium_liquidity: u64,
    pub max_graduation_price_deviation_basis_points: u16,
    pub max_swap_amount_for_pool_price_correction_basis_points: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct LockedCpLiquidityState {
    pub locked_lp_amount: u64,
    pub claimed_lp_amount: u64,
    pub unclaimed_lp_amount: u64,
    pub last_lp: u64,
    pub last_k: u128,
    pub recent_epoch: u64,
    pub pool_id: Pubkey,
    pub fee_nft_mint: Pubkey,
    pub locked_owner: Pubkey,
    pub locked_lp_mint: Pubkey,
    pub padding: [u64; 8],
}
