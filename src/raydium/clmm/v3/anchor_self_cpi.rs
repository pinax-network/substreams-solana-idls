//! Raydium CLMM swap events.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminator
// -----------------------------------------------------------------------------
pub const SWAP_EVENT: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumClmmEvent {
    SwapEvent(SwapEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload struct
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapEvent {
    pub pool_state: Pubkey,
    pub sender: Pubkey,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
    pub amount_0: u64,
    pub transfer_fee_0: u64,
    pub amount_1: u64,
    pub transfer_fee_1: u64,
    pub zero_for_one: bool,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick: i32,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumClmmEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[8..];

        Ok(match disc {
            SWAP_EVENT => Self::SwapEvent(SwapEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumClmmEvent, ParseError> {
    RaydiumClmmEvent::try_from(data)
}
