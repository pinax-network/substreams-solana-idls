//! Pump.fun on-chain **events** and their Borsh-deserialisation helpers.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the emitted log’s data)
// -----------------------------------------------------------------------------
const SWAP: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29]; // e445a52e51cb9a1d

// -----------------------------------------------------------------------------
// High-level event enum (concise; rich docs live in each struct)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JupiterV4Event {
    /// Swap. See [`SwapEvent`].
    Swap(SwapEvent),

    /// Discriminator did not match any known event.
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs (inline field comments instead of tables)
// -----------------------------------------------------------------------------

/// Emitted once when a new bonding-curve pool is created.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapEvent {
    pub amm: Pubkey,
    pub input_mint: Pubkey,
    pub input_amount: u64,
    pub output_mint: Pubkey,
    pub output_amount: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for JupiterV4Event {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 16 {
            // 8 bytes Pump.fun discriminator + 8 bytes Anchor discriminator
            return Err(ParseError::TooShort(data.len()));
        }

        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[16..]; // skip both discriminators

        Ok(match disc {
            SWAP => Self::Swap(SwapEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
#[deprecated(since = "0.0.0", note = "not yet implemented")]
pub fn unpack(data: &[u8]) -> Result<JupiterV4Event, ParseError> {
    JupiterV4Event::try_from(data)
}
