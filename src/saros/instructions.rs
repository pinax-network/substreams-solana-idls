//! Saros swap program instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

pub const INITIALIZE: u8 = 0;
pub const SWAP: u8 = 1;
pub use crate::spl::token_swap::instructions::InitializeInstruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SarosInstruction {
    Swap(SwapInstruction),
    Unknown,
    Initialize(InitializeInstruction),
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
            INITIALIZE => Self::Initialize(InitializeInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown([other, 0, 0, 0, 0, 0, 0, 0])),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<SarosInstruction, ParseError> {
    SarosInstruction::try_from(data)
}
