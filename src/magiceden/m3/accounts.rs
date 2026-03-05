//! Magic Eden M3 on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Allowlist {
    pub kind: u8,
    pub value: Pubkey,
}

// Account discriminators
pub const POOL_ACCOUNT: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
pub const SELL_STATE_ACCOUNT: [u8; 8] = [183, 195, 195, 180, 139, 112, 255, 193];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Pool {
    pub spot_price: u64,
    pub curve_type: u8,
    pub curve_delta: u64,
    pub reinvest_fulfill_buy: bool,
    pub reinvest_fulfill_sell: bool,
    pub expiry: i64,
    pub lp_fee_bp: u16,
    pub referral: Pubkey,
    pub referral_bp: u16,
    pub buyside_creator_royalty_bp: u16,
    pub cosigner_annotation: [u8; 32],
    pub sellside_asset_amount: u64,
    pub lp_fee_earned: u64,
    pub owner: Pubkey,
    pub cosigner: Pubkey,
    pub uuid: Pubkey,
    pub payment_mint: Pubkey,
    pub allowlists: [Allowlist; 6],
    pub buyside_payment_amount: u64,
    pub shared_escrow_account: Pubkey,
    pub shared_escrow_count: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellState {
    pub pool: Pubkey,
    pub pool_owner: Pubkey,
    pub asset_mint: Pubkey,
    pub asset_amount: u64,
    pub cosigner_annotation: [u8; 32],
}
