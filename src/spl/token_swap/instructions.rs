//! SPL Token Swap on-chain instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

// -----------------------------------------------------------------------------
// Discriminators (single-byte index)
// -----------------------------------------------------------------------------
pub const INITIALIZE: u8 = 0;
pub const SWAP: u8 = 1;
pub const DEPOSIT_ALL_TOKEN_TYPES: u8 = 2;
pub const WITHDRAW_ALL_TOKEN_TYPES: u8 = 3;
pub const DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN: u8 = 4;
pub const WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT: u8 = 5;

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum TokenSwapInstruction {
    Initialize,
    Swap {
        amount_in: u64,
        minimum_amount_out: u64,
    },
    DepositAllTokenTypes {
        pool_token_amount: u64,
        maximum_token_a_amount: u64,
        maximum_token_b_amount: u64,
    },
    WithdrawAllTokenTypes {
        pool_token_amount: u64,
        minimum_token_a_amount: u64,
        minimum_token_b_amount: u64,
    },
    DepositSingleTokenTypeExactAmountIn {
        source_token_amount: u64,
        minimum_pool_token_amount: u64,
    },
    WithdrawSingleTokenTypeExactAmountOut {
        destination_token_amount: u64,
        maximum_pool_token_amount: u64,
    },
}

// -----------------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------------
impl TryFrom<&[u8]> for TokenSwapInstruction {
    type Error = ParseError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(data.len()));
        }
        let rest = &data[1..];
        match data[0] {
            INITIALIZE => Ok(Self::Initialize),
            SWAP => {
                if rest.len() < 16 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::Swap {
                    amount_in: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    minimum_amount_out: u64::from_le_bytes(rest[8..16].try_into().unwrap()),
                })
            }
            DEPOSIT_ALL_TOKEN_TYPES => {
                if rest.len() < 24 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::DepositAllTokenTypes {
                    pool_token_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    maximum_token_a_amount: u64::from_le_bytes(rest[8..16].try_into().unwrap()),
                    maximum_token_b_amount: u64::from_le_bytes(rest[16..24].try_into().unwrap()),
                })
            }
            WITHDRAW_ALL_TOKEN_TYPES => {
                if rest.len() < 24 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::WithdrawAllTokenTypes {
                    pool_token_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    minimum_token_a_amount: u64::from_le_bytes(rest[8..16].try_into().unwrap()),
                    minimum_token_b_amount: u64::from_le_bytes(rest[16..24].try_into().unwrap()),
                })
            }
            DEPOSIT_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_IN => {
                if rest.len() < 16 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::DepositSingleTokenTypeExactAmountIn {
                    source_token_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    minimum_pool_token_amount: u64::from_le_bytes(rest[8..16].try_into().unwrap()),
                })
            }
            WITHDRAW_SINGLE_TOKEN_TYPE_EXACT_AMOUNT_OUT => {
                if rest.len() < 16 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::WithdrawSingleTokenTypeExactAmountOut {
                    destination_token_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    maximum_pool_token_amount: u64::from_le_bytes(rest[8..16].try_into().unwrap()),
                })
            }
            other => Err(ParseError::SplUnknown(other)),
        }
    }
}

pub fn unpack(data: &[u8]) -> Result<TokenSwapInstruction, ParseError> {
    TokenSwapInstruction::try_from(data)
}

/// Complete canonical Token Swap initialization payload. Pool mints are not in
/// its instruction accounts; callers must resolve the vaults' token metadata.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeInstruction {
    pub fees: super::accounts::Fees,
    pub curve_type: u8,
    pub curve_parameters: [u8; 32],
}

/// Strict initialization decoder. Unlike the historical `unpack` enum's unit
/// Initialize variant, this checks the complete 97-byte payload.
pub fn unpack_initialize(data: &[u8]) -> Result<InitializeInstruction, ParseError> {
    let (&tag, payload) = data.split_first().ok_or(ParseError::TooShort(0))?;
    if tag != INITIALIZE {
        return Err(ParseError::SplUnknown(tag));
    }
    Ok(InitializeInstruction::try_from_slice(payload)?)
}
