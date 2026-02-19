//! Metaplex Token Metadata on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Account discriminators (Anchor-style 8-byte, but token_metadata uses a Key enum byte prefix)
// These are placeholder discriminators based on IDL account order.
pub const COLLECTION_AUTHORITY_RECORD_ACCOUNT: u8 = 0;
pub const METADATA_DELEGATE_RECORD_ACCOUNT: u8 = 1;
pub const HOLDER_DELEGATE_RECORD_ACCOUNT: u8 = 2;
pub const EDITION_ACCOUNT: u8 = 3;
pub const EDITION_MARKER_ACCOUNT: u8 = 4;
pub const EDITION_MARKER_V2_ACCOUNT: u8 = 5;
pub const TOKEN_OWNED_ESCROW_ACCOUNT: u8 = 6;
pub const MASTER_EDITION_V2_ACCOUNT: u8 = 7;
pub const MASTER_EDITION_V1_ACCOUNT: u8 = 8;
pub const METADATA_ACCOUNT: u8 = 9;
pub const TOKEN_RECORD_ACCOUNT: u8 = 10;
pub const RESERVATION_LIST_V2_ACCOUNT: u8 = 11;
pub const RESERVATION_LIST_V1_ACCOUNT: u8 = 12;
pub const USE_AUTHORITY_RECORD_ACCOUNT: u8 = 13;

/// Metadata account (simplified)
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Metadata {
    pub update_authority: Pubkey,
    pub mint: Pubkey,
}
