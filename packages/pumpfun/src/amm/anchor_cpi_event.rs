//! on-chain **events** and their Borsh-deserialisation helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -------------------------------------------------------------------------
// Discriminators
// -------------------------------------------------------------------------
const BUY_ANCHOR_CPI_EVENT: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119]; // 67f4521f2cf57777
const SELL_ANCHOR_CPI_EVENT: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42]; // 3e2f370aa503dc2a

// -------------------------------------------------------------------------
// enumeration
// -------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PumpFunAmmAnchorCpiEvent {
    Buy(BuyAnchorCpiEvent),
    Sell(SellAnchorCpiEvent),
    Unknown,
}

// -------------------------------------------------------------------------
// Payload structs
// -------------------------------------------------------------------------

// -------------------------------------------------------------------------
// Payload structs
// -------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyAnchorCpiEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellAnchorCpiEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PumpFunAmmAnchorCpiEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            // 8 bytes discriminator
            return Err(ParseError::TooShort(data.len()));
        }

        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[8..]; // skip the discriminator

        Ok(match disc {
            BUY_ANCHOR_CPI_EVENT => Self::Buy(BuyAnchorCpiEvent::try_from_slice(payload)?),
            SELL_ANCHOR_CPI_EVENT => Self::Sell(SellAnchorCpiEvent::try_from_slice(payload)?),
            // If the discriminator does not match any known event, return Unknown
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PumpFunAmmAnchorCpiEvent, ParseError> {
    PumpFunAmmAnchorCpiEvent::try_from(data)
}
