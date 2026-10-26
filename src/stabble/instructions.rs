//! Stabble swap instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabbleInstruction {
    Deposit(DepositInstruction),
    Withdraw(WithdrawInstruction),
    Swap(SwapInstruction),
    Unknown,
    Initialize(InitializeInstruction),
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Add liquidity to the pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub amounts: Vec<u64>,
    pub minimum_amount_out: u64,
}

/// Remove liquidity from the pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInstruction {
    pub amount: u64,
    pub minimum_amounts_out: Vec<u64>,
}

/// Swap tokens through the pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub amount_in: Option<u64>,
    pub minimum_amount_out: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for StabbleInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            INITIALIZE => Self::Initialize(InitializeInstruction::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<StabbleInstruction, ParseError> {
    StabbleInstruction::try_from(data)
}

/// Pool weights and caps. Asset mints are not part of these arguments; consumers
/// need the program's remaining-account contract to identify the ordered assets.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeInstruction {
    pub swap_fee: u64,
    pub weights: Vec<u64>,
    pub max_caps: Vec<u64>,
}
