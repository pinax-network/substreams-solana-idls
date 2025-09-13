//! Raydium Stable AMM events.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
/// `SwapEvent` discriminator reused from Raydium CPMM.
pub const SWAP_EVENT: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumStableEvent {
    /// Emitted when a swap occurs.
    SwapEvent(SwapEvent),
    /// Discriminator did not match any known event.
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Emitted when performing a swap on the Raydium Stable AMM.
///
/// The payload layout is:
/// `[u8 dex][u64 amount_in][u64 amount_out]`.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapEvent {
    /// DEX identifier as defined by Jupiter's aggregator.
    pub dex: u8,
    /// Amount of tokens supplied to the pool.
    pub amount_in: u64,
    /// Amount of tokens received from the pool.
    pub amount_out: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumStableEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SWAP_EVENT => Self::SwapEvent(SwapEvent::try_from_slice(payload)?),
            _ => Self::Unknown,
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumStableEvent, ParseError> {
    RaydiumStableEvent::try_from(data)
}
