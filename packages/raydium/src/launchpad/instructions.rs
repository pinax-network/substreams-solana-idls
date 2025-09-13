//! Raydium Launchpad trading instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

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
#[derive(Debug, Clone, PartialEq, Eq)]
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
/// Use the given amount of quote tokens to purchase base tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyExactInInstruction {
    /// Amount of quote token to purchase
    pub amount_in: u64,
    /// Minimum amount of base token to receive (slippage protection)
    pub minimum_amount_out: u64,
    /// Fee rate for the share
    pub share_fee_rate: u64,
}

/// Use quote tokens to purchase the given amount of base tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyExactOutInstruction {
    /// Amount of base token to receive
    pub amount_out: u64,
    /// Maximum amount of quote token to purchase (slippage protection)
    pub maximum_amount_in: u64,
    /// Fee rate for the share
    pub share_fee_rate: u64,
}

/// Use the given amount of base tokens to sell for quote tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellExactInInstruction {
    /// Amount of base token to sell
    pub amount_in: u64,
    /// Minimum amount of quote token to receive (slippage protection)
    pub minimum_amount_out: u64,
    /// Fee rate for the share
    pub share_fee_rate: u64,
}

/// Sell base tokens for the given amount of quote tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellExactOutInstruction {
    /// Amount of quote token to receive
    pub amount_out: u64,
    /// Maximum amount of base token to purchase (slippage protection)
    pub maximum_amount_in: u64,
    /// Fee rate for the share
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
