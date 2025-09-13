//! Jupiter DCA on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawParams {
    pub withdraw_amount: u64,
    pub withdrawal: Withdrawal,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Withdrawal {
    In,
    Out,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OpenDcaInstruction {
    pub application_idx: u64,
    pub in_amount: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub min_out_amount: Option<u64>,
    pub max_out_amount: Option<u64>,
    pub start_at: Option<i64>,
    pub close_wsol_in_ata: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OpenDcaV2Instruction {
    pub application_idx: u64,
    pub in_amount: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub min_out_amount: Option<u64>,
    pub max_out_amount: Option<u64>,
    pub start_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInstruction {
    pub withdraw_params: WithdrawParams,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub deposit_in: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawFeesInstruction {
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FulfillFlashFillInstruction {
    pub repay_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FulfillDlmmFillInstruction {
    pub repay_amount: u64,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const OPEN_DCA: [u8; 8] = [36, 65, 185, 54, 1, 210, 100, 163];
pub const OPEN_DCA_V2: [u8; 8] = [142, 119, 43, 109, 162, 52, 11, 177];
pub const CLOSE_DCA: [u8; 8] = [22, 7, 33, 98, 168, 183, 34, 243];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const WITHDRAW_FEES: [u8; 8] = [198, 212, 171, 109, 144, 215, 174, 89];
pub const INITIATE_FLASH_FILL: [u8; 8] = [143, 205, 3, 191, 162, 215, 245, 49];
pub const FULFILL_FLASH_FILL: [u8; 8] = [115, 64, 226, 78, 33, 211, 105, 162];
pub const INITIATE_DLMM_FILL: [u8; 8] = [155, 193, 80, 121, 91, 147, 254, 187];
pub const FULFILL_DLMM_FILL: [u8; 8] = [1, 230, 118, 251, 45, 177, 101, 187];
pub const TRANSFER: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];
pub const END_AND_CLOSE: [u8; 8] = [83, 125, 166, 69, 247, 252, 103, 133];

// -----------------------------------------------------------------------------
// High-level instruction enum
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JupiterDcaInstruction {
    OpenDca(OpenDcaInstruction),
    OpenDcaV2(OpenDcaV2Instruction),
    CloseDca,
    Withdraw(WithdrawInstruction),
    Deposit(DepositInstruction),
    WithdrawFees(WithdrawFeesInstruction),
    InitiateFlashFill,
    FulfillFlashFill(FulfillFlashFillInstruction),
    InitiateDlmmFill,
    FulfillDlmmFill(FulfillDlmmFillInstruction),
    Transfer,
    EndAndClose,
    Unknown,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for JupiterDcaInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            OPEN_DCA => Self::OpenDca(OpenDcaInstruction::try_from_slice(payload)?),
            OPEN_DCA_V2 => Self::OpenDcaV2(OpenDcaV2Instruction::try_from_slice(payload)?),
            CLOSE_DCA => Self::CloseDca,
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            WITHDRAW_FEES => Self::WithdrawFees(WithdrawFeesInstruction::try_from_slice(payload)?),
            INITIATE_FLASH_FILL => Self::InitiateFlashFill,
            FULFILL_FLASH_FILL => Self::FulfillFlashFill(FulfillFlashFillInstruction::try_from_slice(payload)?),
            INITIATE_DLMM_FILL => Self::InitiateDlmmFill,
            FULFILL_DLMM_FILL => Self::FulfillDlmmFill(FulfillDlmmFillInstruction::try_from_slice(payload)?),
            TRANSFER => Self::Transfer,
            END_AND_CLOSE => Self::EndAndClose,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<JupiterDcaInstruction, ParseError> {
    JupiterDcaInstruction::try_from(data)
}
