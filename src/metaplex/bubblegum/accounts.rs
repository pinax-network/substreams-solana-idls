//! Metaplex Bubblegum on-chain accounts.

use crate::common::ParseError;
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

#[derive(Debug, Clone, PartialEq)]
pub enum BubblegumAccount {
    TreeConfig(TreeConfig),
    Voucher(Voucher),
}

pub fn unpack(data: &[u8]) -> Result<BubblegumAccount, ParseError> {
    if data.len() < 8 {
        return Err(ParseError::TooShort(data.len()));
    }

    let (disc, rest) = data.split_at(8);
    let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

    Ok(match discriminator {
        TREE_CONFIG_ACCOUNT => BubblegumAccount::TreeConfig(TreeConfig::try_from_slice(rest)?),
        VOUCHER_ACCOUNT => BubblegumAccount::Voucher(Voucher::try_from_slice(rest)?),
        other => return Err(ParseError::AnchorUnknown(other)),
    })
}
