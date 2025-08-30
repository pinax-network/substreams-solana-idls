//! BonkSwap on-chain events emitted via `emit!`.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const SWAP_EVENT: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BonkSwapEvent {
    Swap(SwapEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapEvent {
    pub delta_in: u64,
    pub price_limit: u128,
    pub x_to_y: bool,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for BonkSwapEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SWAP_EVENT => Self::Swap(SwapEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<BonkSwapEvent, ParseError> {
    BonkSwapEvent::try_from(data)
}
