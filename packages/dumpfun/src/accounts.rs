//! Dumpfun on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RampingLimit {
    pub start_sec: i64,
    pub end_sec: i64,
    pub limit_bps: u16,
}

// Account discriminators
pub const LIQUIDITY_POOL_ACCOUNT: [u8; 8] = [66, 38, 17, 64, 188, 80, 68, 129];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityPool {
    pub creator_wallet: Pubkey,
    pub platform_fee_wallet: Pubkey,
    pub company_tax_wallet: Pubkey,
    pub mint_account: Pubkey,
    pub sell_lock_period: i64,
    pub virtual_sol_reserve: u64,
    pub total_sol_volume: u64,
    pub real_token_reserve: u64,
    pub create_time: i64,
    pub ramping_limits: Vec<RampingLimit>,
    pub in_use: bool,
    pub closed: bool,
}
