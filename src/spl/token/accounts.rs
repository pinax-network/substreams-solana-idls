//! SPL Token on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// SPL Token Mint account
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Mint {
    pub mint_authority_option: u32,
    pub mint_authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority_option: u32,
    pub freeze_authority: Pubkey,
}

/// SPL Token Account
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub delegate_option: u32,
    pub delegate: Pubkey,
    pub state: u8,
    pub is_native_option: u32,
    pub is_native: u64,
    pub delegated_amount: u64,
    pub close_authority_option: u32,
    pub close_authority: Pubkey,
}
