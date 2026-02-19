//! GoonFi on-chain instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

// -----------------------------------------------------------------------------
// Discriminators  (sha256("global:<snake_case>")[..8])
// -----------------------------------------------------------------------------
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoonFiInstruction {
    Swap(SwapInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub is_bid: bool,
}

// -----------------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for GoonFiInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<GoonFiInstruction, ParseError> {
    GoonFiInstruction::try_from(data)
}
