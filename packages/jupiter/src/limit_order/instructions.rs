//! Jupiter Limit Order on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeOrderInstruction {
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expired_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FillOrderInstruction {
    pub making_amount: u64,
    pub max_taking_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PreFlashFillOrderInstruction {
    pub making_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FlashFillOrderInstruction {
    pub max_taking_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawFeeInstruction {
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitFeeInstruction {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateFeeInstruction {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const INITIALIZE_ORDER: [u8; 8] = [133, 110, 74, 175, 112, 159, 245, 159];
pub const FILL_ORDER: [u8; 8] = [232, 122, 115, 25, 199, 143, 136, 162];
pub const PRE_FLASH_FILL_ORDER: [u8; 8] = [240, 47, 153, 68, 13, 190, 225, 42];
pub const FLASH_FILL_ORDER: [u8; 8] = [252, 104, 18, 134, 164, 78, 18, 140];
pub const CANCEL_ORDER: [u8; 8] = [95, 129, 237, 240, 8, 49, 223, 132];
pub const CANCEL_EXPIRED_ORDER: [u8; 8] = [216, 120, 64, 235, 155, 19, 229, 99];
pub const WITHDRAW_FEE: [u8; 8] = [14, 122, 231, 218, 31, 238, 223, 150];
pub const INIT_FEE: [u8; 8] = [13, 9, 211, 107, 62, 172, 224, 67];
pub const UPDATE_FEE: [u8; 8] = [232, 253, 195, 247, 148, 212, 73, 222];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JupiterLimitOrderInstruction {
    InitializeOrder(InitializeOrderInstruction),
    FillOrder(FillOrderInstruction),
    PreFlashFillOrder(PreFlashFillOrderInstruction),
    FlashFillOrder(FlashFillOrderInstruction),
    CancelOrder,
    CancelExpiredOrder,
    WithdrawFee(WithdrawFeeInstruction),
    InitFee(InitFeeInstruction),
    UpdateFee(UpdateFeeInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for JupiterLimitOrderInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            INITIALIZE_ORDER => Self::InitializeOrder(InitializeOrderInstruction::try_from_slice(payload)?),
            FILL_ORDER => Self::FillOrder(FillOrderInstruction::try_from_slice(payload)?),
            PRE_FLASH_FILL_ORDER => Self::PreFlashFillOrder(PreFlashFillOrderInstruction::try_from_slice(payload)?),
            FLASH_FILL_ORDER => Self::FlashFillOrder(FlashFillOrderInstruction::try_from_slice(payload)?),
            CANCEL_ORDER => Self::CancelOrder,
            CANCEL_EXPIRED_ORDER => Self::CancelExpiredOrder,
            WITHDRAW_FEE => Self::WithdrawFee(WithdrawFeeInstruction::try_from_slice(payload)?),
            INIT_FEE => Self::InitFee(InitFeeInstruction::try_from_slice(payload)?),
            UPDATE_FEE => Self::UpdateFee(UpdateFeeInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<JupiterLimitOrderInstruction, ParseError> {
    JupiterLimitOrderInstruction::try_from(data)
}
