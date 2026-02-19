//! Lifinity AMM V2 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (Anchor: sha256("global:<snake_case>")[..8])
// -----------------------------------------------------------------------------
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const DEPOSIT_ALL_TOKEN_TYPES: [u8; 8] = [32, 95, 69, 60, 75, 79, 205, 238];
pub const WITHDRAW_ALL_TOKEN_TYPES: [u8; 8] = [189, 254, 156, 174, 210, 9, 164, 216];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifinityInstruction {
    Swap(SwapInstruction),
    DepositAllTokenTypes(DepositAllTokenTypesInstruction),
    WithdrawAllTokenTypes(WithdrawAllTokenTypesInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositAllTokenTypesInstruction {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawAllTokenTypesInstruction {
    pub pool_token_amount: u64,
    pub minimum_token_a_amount: u64,
    pub minimum_token_b_amount: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for LifinityInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            DEPOSIT_ALL_TOKEN_TYPES => Self::DepositAllTokenTypes(DepositAllTokenTypesInstruction::try_from_slice(payload)?),
            WITHDRAW_ALL_TOKEN_TYPES => Self::WithdrawAllTokenTypes(WithdrawAllTokenTypesInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<LifinityInstruction, ParseError> {
    LifinityInstruction::try_from(data)
}
