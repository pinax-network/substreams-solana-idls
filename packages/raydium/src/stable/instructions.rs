//! Raydium Stable AMM instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (first byte of instruction data)
// -----------------------------------------------------------------------------
pub const INITIALIZE: u8 = 0;
pub const DEPOSIT: u8 = 3;
pub const WITHDRAW: u8 = 4;
pub const SWAP_BASE_IN: u8 = 9;
pub const PRE_INITIALIZE: u8 = 10;
pub const SWAP_BASE_OUT: u8 = 11;

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaydiumStableInstruction {
    Initialize(InitializeInstruction),
    Deposit(DepositInstruction),
    Withdraw(WithdrawInstruction),
    SwapBaseIn(SwapBaseInInstruction),
    PreInitialize(PreInitializeInstruction),
    SwapBaseOut(SwapBaseOutInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeInstruction {
    pub nonce: u8,
    pub open_time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PreInitializeInstruction {
    pub nonce: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInstruction {
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapBaseInInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapBaseOutInstruction {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

// -----------------------------------------------------------------------------
// Deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumStableInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(data.len()));
        }
        let (&tag, payload) = data.split_first().unwrap();
        Ok(match tag {
            INITIALIZE => {
                if payload.len() < 9 {
                    return Err(ParseError::TooShort(data.len()));
                }
                let nonce = payload[0];
                let open_time = u64::from_le_bytes(payload[1..9].try_into().unwrap());
                Self::Initialize(InitializeInstruction { nonce, open_time })
            }
            DEPOSIT => {
                if payload.len() < 24 {
                    return Err(ParseError::TooShort(data.len()));
                }
                let max_coin_amount = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                let max_pc_amount = u64::from_le_bytes(payload[8..16].try_into().unwrap());
                let base_side = u64::from_le_bytes(payload[16..24].try_into().unwrap());
                Self::Deposit(DepositInstruction {
                    max_coin_amount,
                    max_pc_amount,
                    base_side,
                })
            }
            WITHDRAW => {
                if payload.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                let amount = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                Self::Withdraw(WithdrawInstruction { amount })
            }
            SWAP_BASE_IN => {
                if payload.len() < 16 {
                    return Err(ParseError::TooShort(data.len()));
                }
                let amount_in = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                let minimum_amount_out = u64::from_le_bytes(payload[8..16].try_into().unwrap());
                Self::SwapBaseIn(SwapBaseInInstruction { amount_in, minimum_amount_out })
            }
            PRE_INITIALIZE => {
                if payload.len() < 1 {
                    return Err(ParseError::TooShort(data.len()));
                }
                let nonce = payload[0];
                Self::PreInitialize(PreInitializeInstruction { nonce })
            }
            SWAP_BASE_OUT => {
                if payload.len() < 16 {
                    return Err(ParseError::TooShort(data.len()));
                }
                let max_amount_in = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                let amount_out = u64::from_le_bytes(payload[8..16].try_into().unwrap());
                Self::SwapBaseOut(SwapBaseOutInstruction { max_amount_in, amount_out })
            }
            other => return Err(ParseError::RaydiumUnknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<RaydiumStableInstruction, ParseError> {
    RaydiumStableInstruction::try_from(data)
}
