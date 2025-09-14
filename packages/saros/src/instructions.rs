//! Saros swap program instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

pub const SWAP: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SarosInstruction {
    Swap(SwapInstruction),
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

impl<'a> TryFrom<&'a [u8]> for SarosInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(data.len()));
        }
        let (&disc, payload) = data.split_first().unwrap();
        Ok(match disc {
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown([other, 0, 0, 0, 0, 0, 0, 0])),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<SarosInstruction, ParseError> {
    SarosInstruction::try_from(data)
}
