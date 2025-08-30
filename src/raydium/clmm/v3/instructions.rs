//! Raydium CLMM instructions (currently undefined).

use crate::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumClmmInstruction {
    /// Unknown discriminator
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for RaydiumClmmInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        Err(ParseError::Unknown(disc))
    }
}

pub fn unpack(data: &[u8]) -> Result<RaydiumClmmInstruction, ParseError> {
    RaydiumClmmInstruction::try_from(data)
}
