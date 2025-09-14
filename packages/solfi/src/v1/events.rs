//! Solfi V1 on-chain events and their Borsh-deserialisation helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the emitted log's data)
// -----------------------------------------------------------------------------
const SWAP: [u8; 8] = [79, 54, 102, 82, 188, 211, 62, 178];

// -----------------------------------------------------------------------------
// High-level event enum
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolfiEvent {
    /// Swap. See [`SwapEvent`].
    Swap(SwapEvent),
    /// Discriminator did not match any known event.
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapEvent {
    pub user: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for SolfiEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[8..];
        Ok(match disc {
            SWAP => Self::Swap(SwapEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<SolfiEvent, ParseError> {
    SolfiEvent::try_from(data)
}
