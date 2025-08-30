//! Raydium CPMM swap instructions.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const SWAP_BASE_INPUT: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];
pub const SWAP_BASE_OUTPUT: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumCpmmInstruction {
    SwapBaseInput(SwapBaseInputInstruction),
    SwapBaseOutput(SwapBaseOutputInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseInputInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseOutputInstruction {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumCpmmInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SWAP_BASE_INPUT => Self::SwapBaseInput(SwapBaseInputInstruction::try_from_slice(payload)?),
            SWAP_BASE_OUTPUT => Self::SwapBaseOutput(SwapBaseOutputInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumCpmmInstruction, ParseError> {
    RaydiumCpmmInstruction::try_from(data)
}
