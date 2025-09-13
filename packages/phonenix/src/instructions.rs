//! Phonenix trading instructions.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const SWAP: u8 = 0;
pub const SWAP_WITH_FREE_FUNDS: u8 = 1;

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhonenixInstruction {
    Swap(SwapInstruction),
    SwapWithFreeFunds(SwapWithFreeFundsInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapInstruction {
    /// Raw order packet payload
    pub order_packet: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapWithFreeFundsInstruction {
    /// Raw order packet payload
    pub order_packet: Vec<u8>,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PhonenixInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(1);
        let discriminator = disc[0];
        Ok(match discriminator {
            SWAP => Self::Swap(SwapInstruction {
                order_packet: payload.to_vec(),
            }),
            SWAP_WITH_FREE_FUNDS => Self::SwapWithFreeFunds(SwapWithFreeFundsInstruction {
                order_packet: payload.to_vec(),
            }),
            other => return Err(ParseError::Unknown([other, 0, 0, 0, 0, 0, 0, 0])),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PhonenixInstruction, ParseError> {
    PhonenixInstruction::try_from(data)
}
