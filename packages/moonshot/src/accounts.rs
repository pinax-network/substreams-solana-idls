//! Moonshot on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Account discriminators (sha256("account:<Name>")[..8])
pub const CONFIG_ACCOUNT: [u8; 8] = [189, 255, 97, 70, 186, 189, 24, 102];
pub const CURVE_ACCOUNT: [u8; 8] = [8, 91, 83, 28, 132, 216, 248, 22];

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Currency {
    Sol,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum CurveType {
    LinearV1,
}

// -----------------------------------------------------------------------------
// Account structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ConfigAccount {
    pub migration_authority: Pubkey,
    pub backend_authority: Pubkey,
    pub config_authority: Pubkey,
    pub helio_fee: Pubkey,
    pub dex_fee: Pubkey,
    pub fee_bps: u16,
    pub dex_fee_share: u8,
    pub migration_fee: u64,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub min_supported_decimal_places: u8,
    pub max_supported_decimal_places: u8,
    pub min_supported_token_supply: u64,
    pub max_supported_token_supply: u64,
    pub bump: u8,
    pub coef_b: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CurveAccount {
    pub total_supply: u64,
    pub curve_amount: u64,
    pub mint: Pubkey,
    pub decimals: u8,
    pub collateral_currency: Currency,
    pub curve_type: CurveType,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub migration_fee: u64,
    pub coef_b: u32,
    pub bump: u8,
}
