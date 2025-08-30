//! Raydium Launchpad trading instructions.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const BUY_EXACT_IN: [u8; 8] = [250, 234, 13, 123, 213, 156, 19, 236];
pub const BUY_EXACT_OUT: [u8; 8] = [24, 211, 116, 40, 105, 3, 153, 56];
pub const SELL_EXACT_IN: [u8; 8] = [149, 39, 222, 155, 211, 124, 152, 26];
pub const SELL_EXACT_OUT: [u8; 8] = [95, 200, 71, 34, 8, 9, 11, 166];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumLaunchpadInstruction {
    BuyExactIn(BuyExactInInstruction),
    BuyExactOut(BuyExactOutInstruction),
    SellExactIn(SellExactInInstruction),
    SellExactOut(SellExactOutInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct BuyExactInInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct BuyExactOutInstruction {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SellExactInInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SellExactOutInstruction {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumLaunchpadInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            BUY_EXACT_IN => Self::BuyExactIn(BuyExactInInstruction::try_from_slice(payload)?),
            BUY_EXACT_OUT => Self::BuyExactOut(BuyExactOutInstruction::try_from_slice(payload)?),
            SELL_EXACT_IN => Self::SellExactIn(SellExactInInstruction::try_from_slice(payload)?),
            SELL_EXACT_OUT => Self::SellExactOut(SellExactOutInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumLaunchpadInstruction, ParseError> {
    RaydiumLaunchpadInstruction::try_from(data)
}
