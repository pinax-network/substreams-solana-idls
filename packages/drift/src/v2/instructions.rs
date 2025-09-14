//! Drift v2 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum SwapReduceOnly {
    In,
    Out,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BeginSwapInstruction {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub amount_in: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EndSwapInstruction {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub limit_price: Option<u64>,
    pub reduce_only: Option<SwapReduceOnly>,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const BEGIN_SWAP: [u8; 8] = [174, 109, 228, 1, 242, 105, 232, 105];
pub const END_SWAP: [u8; 8] = [177, 184, 27, 193, 34, 13, 210, 145];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum DriftInstruction {
    BeginSwap(BeginSwapInstruction),
    EndSwap(EndSwapInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for DriftInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            BEGIN_SWAP => Self::BeginSwap(BeginSwapInstruction::try_from_slice(payload)?),
            END_SWAP => Self::EndSwap(EndSwapInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<DriftInstruction, ParseError> {
    DriftInstruction::try_from(data)
}
