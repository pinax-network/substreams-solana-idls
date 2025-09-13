//! Phonenix market events.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const FILL_EVENT: u8 = 2;

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhonenixEvent {
    Fill(FillEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FillEvent {
    pub index: u16,
    pub maker_id: Pubkey,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_filled: u64,
    pub base_lots_remaining: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PhonenixEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(1);
        let discriminator = disc[0];
        Ok(match discriminator {
            FILL_EVENT => Self::Fill(FillEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown([other, 0, 0, 0, 0, 0, 0, 0])),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PhonenixEvent, ParseError> {
    PhonenixEvent::try_from(data)
}
