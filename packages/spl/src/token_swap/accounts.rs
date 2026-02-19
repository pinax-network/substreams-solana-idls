//! SPL Token Swap on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Token Swap pool state
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapV1 {
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub token_program_id: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub pool_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub pool_fee_account: Pubkey,
}

/// Trade fee numerator/denominator
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Fees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}

/// Constant price curve
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConstantPriceCurve {
    pub token_b_price: u64,
}

/// Constant product curve (empty — no params)
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConstantProductCurve;

/// Offset curve
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OffsetCurve {
    pub token_b_offset: u64,
}

/// Stable curve
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StableCurve {
    pub amp: u64,
}
