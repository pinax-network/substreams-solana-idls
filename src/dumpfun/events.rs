//! Dumpfun on-chain events.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Event discriminators
pub const BUY_TOKEN_EVENT_EVENT: [u8; 8] = [90, 138, 104, 84, 222, 143, 82, 123];
pub const DRAIN_POOL_EVENT_EVENT: [u8; 8] = [116, 220, 198, 208, 98, 61, 234, 65];
pub const SELL_TOKEN_EVENT_EVENT: [u8; 8] = [148, 185, 126, 171, 239, 120, 196, 178];
pub const TOKEN_CREATED_EVENT_EVENT: [u8; 8] = [96, 122, 113, 138, 50, 227, 149, 57];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyTokenEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub sol_in: u64,
    pub token_out: u64,
    pub buy_time: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DrainPoolEvent {
    pub pool: Pubkey,
    pub mint: Pubkey,
    pub creator_wallet: Pubkey,
    pub company_wallet: Pubkey,
    pub creator_amount: u64,
    pub company_amount: u64,
    pub total_drained: u64,
    pub drain_time: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellTokenEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub token_in: u64,
    pub sol_out: u64,
    pub sell_time: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TokenCreatedEvent {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub create_time: i64,
    pub sell_lock_period: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DumpfunEvent {
    BuyTokenEvent(BuyTokenEvent),
    DrainPoolEvent(DrainPoolEvent),
    SellTokenEvent(SellTokenEvent),
    TokenCreatedEvent(TokenCreatedEvent),
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for DumpfunEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            BUY_TOKEN_EVENT_EVENT => Self::BuyTokenEvent(BuyTokenEvent::try_from_slice(payload)?),
            DRAIN_POOL_EVENT_EVENT => Self::DrainPoolEvent(DrainPoolEvent::try_from_slice(payload)?),
            SELL_TOKEN_EVENT_EVENT => Self::SellTokenEvent(SellTokenEvent::try_from_slice(payload)?),
            TOKEN_CREATED_EVENT_EVENT => Self::TokenCreatedEvent(TokenCreatedEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack_event(data: &[u8]) -> Result<DumpfunEvent, ParseError> {
    DumpfunEvent::try_from(data)
}
