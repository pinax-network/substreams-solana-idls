//! Obric V2 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (Anchor: sha256("global:<snake_case>")[..8])
// -----------------------------------------------------------------------------
pub const CREATE_PAIR: [u8; 8] = [156, 190, 126, 151, 163, 62, 192, 220];
pub const CREATE_PAIR_V2: [u8; 8] = [55, 255, 159, 236, 123, 161, 174, 155];
pub const UPDATE_CONCENTRATION: [u8; 8] = [252, 177, 98, 133, 34, 227, 202, 11];
pub const UPDATE_VERSION: [u8; 8] = [248, 152, 89, 69, 41, 63, 148, 30];
pub const UPDATE_FEE_PARAMS: [u8; 8] = [223, 116, 30, 197, 161, 4, 201, 146];
pub const UPDATE_ORACLES: [u8; 8] = [209, 115, 103, 72, 108, 69, 218, 189];
pub const WITHDRAW_FEES: [u8; 8] = [198, 212, 171, 109, 144, 215, 174, 89];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
pub const SWAP_X_TO_Y: [u8; 8] = [226, 74, 41, 166, 87, 155, 41, 75];
pub const SWAP_Y_TO_X: [u8; 8] = [219, 168, 219, 174, 169, 221, 164, 95];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObricV2Instruction {
    CreatePair(CreatePairInstruction),
    CreatePairV2(CreatePairInstruction),
    UpdateConcentration(UpdateConcentrationInstruction),
    UpdateVersion(UpdateVersionInstruction),
    UpdateFeeParams(UpdateFeeParamsInstruction),
    UpdateOracles,
    WithdrawFees,
    Deposit(DepositInstruction),
    Withdraw(WithdrawInstruction),
    SwapXToY(SwapXToYInstruction),
    SwapYToX(SwapYToXInstruction),
    Swap(SwapInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePairInstruction {
    pub concentration: u64,
    pub fee_millionth: u64,
    pub rebate_percentage: u64,
    pub protocol_fee_share_thousandth: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateConcentrationInstruction {
    pub concentration: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateVersionInstruction {
    pub version: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateFeeParamsInstruction {
    pub fee_millionth: u64,
    pub protocol_fee_share_thousandth: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub input_x: u64,
    pub input_y: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInstruction {
    pub lp_amount_x: u64,
    pub lp_amount_y: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapXToYInstruction {
    pub input_x: u64,
    pub min_output_amt: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapYToXInstruction {
    pub input_y: u64,
    pub min_output_amt: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub is_x_to_y: bool,
    pub input_amt: u64,
    pub min_output_amt: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for ObricV2Instruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            CREATE_PAIR => Self::CreatePair(CreatePairInstruction::try_from_slice(payload)?),
            CREATE_PAIR_V2 => Self::CreatePairV2(CreatePairInstruction::try_from_slice(payload)?),
            UPDATE_CONCENTRATION => Self::UpdateConcentration(UpdateConcentrationInstruction::try_from_slice(payload)?),
            UPDATE_VERSION => Self::UpdateVersion(UpdateVersionInstruction::try_from_slice(payload)?),
            UPDATE_FEE_PARAMS => Self::UpdateFeeParams(UpdateFeeParamsInstruction::try_from_slice(payload)?),
            UPDATE_ORACLES => Self::UpdateOracles,
            WITHDRAW_FEES => Self::WithdrawFees,
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            SWAP_X_TO_Y => Self::SwapXToY(SwapXToYInstruction::try_from_slice(payload)?),
            SWAP_Y_TO_X => Self::SwapYToX(SwapYToXInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<ObricV2Instruction, ParseError> {
    ObricV2Instruction::try_from(data)
}
