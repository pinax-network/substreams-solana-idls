//! SPL Token Lending on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Lending market state
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct LendingMarket {
    pub version: u8,
    pub bump_seed: u8,
    pub owner: Pubkey,
    pub quote_currency: [u8; 32],
    pub token_program_id: Pubkey,
    pub oracle_program_id: Pubkey,
}

/// Reserve state
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Reserve {
    pub version: u8,
    pub last_update_slot: u64,
    pub last_update_stale: bool,
    pub lending_market: Pubkey,
    pub liquidity_mint_pubkey: Pubkey,
    pub liquidity_supply_pubkey: Pubkey,
}

/// Obligation state
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Obligation {
    pub version: u8,
    pub last_update_slot: u64,
    pub last_update_stale: bool,
    pub lending_market: Pubkey,
    pub owner: Pubkey,
}
