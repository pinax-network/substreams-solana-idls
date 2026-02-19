//! Metaplex Bubblegum on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Account discriminators (Anchor sha256("account:<Name>")[..8])
pub const TREE_CONFIG_ACCOUNT: [u8; 8] = [122, 245, 175, 248, 171, 34, 0, 207];
pub const VOUCHER_ACCOUNT: [u8; 8] = [191, 204, 149, 234, 213, 165, 13, 65];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TreeConfig {
    pub tree_creator: Pubkey,
    pub tree_delegate: Pubkey,
    pub total_mint_capacity: u64,
    pub num_minted: u64,
    pub is_public: bool,
    pub is_decompressible: u8,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Voucher {
    pub leaf_schema: Pubkey,
    pub index: u32,
    pub merkle_tree: Pubkey,
}
