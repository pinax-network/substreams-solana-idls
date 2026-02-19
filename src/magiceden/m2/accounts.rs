//! Magic Eden M2 on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Account discriminators
pub const BUYER_TRADE_STATE_ACCOUNT: [u8; 8] = [200, 164, 153, 187, 118, 60, 200, 51];
pub const SELLER_TRADE_STATE_ACCOUNT: [u8; 8] = [1, 238, 72, 137, 138, 21, 254, 249];
pub const SELLER_TRADE_STATE_V2_ACCOUNT: [u8; 8] = [164, 14, 92, 100, 123, 57, 234, 204];
pub const AUCTION_HOUSE_ACCOUNT: [u8; 8] = [40, 108, 215, 107, 213, 85, 245, 48];
pub const BUYER_TRADE_STATE_V2_ACCOUNT: [u8; 8] = [195, 55, 46, 41, 54, 7, 225, 155];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyerTradeState {
    pub auction_house_key: Pubkey,
    pub buyer: Pubkey,
    pub buyer_referral: Pubkey,
    pub buyer_price: u64,
    pub token_mint: Pubkey,
    pub token_size: u64,
    pub bump: u8,
    pub expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellerTradeState {
    pub auction_house_key: Pubkey,
    pub seller: Pubkey,
    pub seller_referral: Pubkey,
    pub buyer_price: u64,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub token_size: u64,
    pub bump: u8,
    pub expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellerTradeStateV2 {
    pub auction_house_key: Pubkey,
    pub seller: Pubkey,
    pub seller_referral: Pubkey,
    pub buyer_price: u64,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub token_size: u64,
    pub bump: u8,
    pub expiry: i64,
    pub payment_mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AuctionHouse {
    pub auction_house_treasury: Pubkey,
    pub treasury_withdrawal_destination: Pubkey,
    pub authority: Pubkey,
    pub creator: Pubkey,
    pub notary: Pubkey,
    pub bump: u8,
    pub treasury_bump: u8,
    pub seller_fee_basis_points: u16,
    pub buyer_referral_bp: u16,
    pub seller_referral_bp: u16,
    pub requires_notary: bool,
    pub nprob: u8,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyerTradeStateV2 {
    pub auction_house_key: Pubkey,
    pub buyer: Pubkey,
    pub buyer_referral: Pubkey,
    pub buyer_price: u64,
    pub token_mint: Pubkey,
    pub token_size: u64,
    pub bump: u8,
    pub expiry: i64,
    pub buyer_creator_royalty_bp: u16,
    pub payment_mint: Pubkey,
}
