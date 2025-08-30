//! Raydium Launchpad trade events.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const TRADE_EVENT: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumLaunchpadEvent {
    TradeEvent(TradeEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload struct
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TradeEvent {
    pub pool_state: Pubkey,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base_before: u64,
    pub real_quote_before: u64,
    pub real_base_after: u64,
    pub real_quote_after: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub protocol_fee: u64,
    pub platform_fee: u64,
    pub creator_fee: u64,
    pub share_fee: u64,
    pub trade_direction: u8,
    pub pool_status: u8,
    pub exact_in: bool,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumLaunchpadEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[8..];

        Ok(match disc {
            TRADE_EVENT => Self::TradeEvent(TradeEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumLaunchpadEvent, ParseError> {
    RaydiumLaunchpadEvent::try_from(data)
}
