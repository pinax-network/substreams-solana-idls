//! PancakeSwap AMM on-chain instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

/// Exact create_pool discriminator from this module's bundled IDL.
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PancakeSwapInstruction {
    Swap(SwapInstruction),
    Unknown,
    CreatePool(CreatePoolInstruction),
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

/// Initial pool price and opening time, in IDL field order.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolInstruction {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PancakeSwapInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            CREATE_POOL => Self::CreatePool(CreatePoolInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PancakeSwapInstruction, ParseError> {
    PancakeSwapInstruction::try_from(data)
}
