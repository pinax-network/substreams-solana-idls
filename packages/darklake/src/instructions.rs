//! Darklake on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// Discriminators
pub const ADD_LIQUIDITY: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
pub const CANCEL: [u8; 8] = [232, 219, 223, 41, 219, 236, 220, 190];
pub const COLLECT_PROTOCOL_FEES: [u8; 8] = [22, 67, 23, 98, 150, 178, 70, 220];
pub const CREATE_AMM_CONFIG: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
pub const INITIALIZE_POOL: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
pub const REMOVE_LIQUIDITY: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
pub const SETTLE: [u8; 8] = [175, 42, 185, 87, 144, 131, 102, 212];
pub const SLASH: [u8; 8] = [204, 141, 18, 161, 8, 177, 92, 142];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const UPDATE_AMM_CONFIG: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];

#[derive(Debug, Clone, PartialEq)]
pub enum DarklakeInstruction {
    AddLiquidity(AddLiquidityInstruction),
    Cancel(CancelInstruction),
    CollectProtocolFees(CollectProtocolFeesInstruction),
    CreateAmmConfig(CreateAmmConfigInstruction),
    InitializePool(InitializePoolInstruction),
    RemoveLiquidity(RemoveLiquidityInstruction),
    Settle(SettleInstruction),
    Slash(SlashInstruction),
    Swap(SwapInstruction),
    UpdateAmmConfig(UpdateAmmConfigInstruction),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityInstruction {
    pub amount_lp: u64,
    pub max_amount_x: u64,
    pub max_amount_y: u64,
    pub ref_code: Option<[u8; 20]>,
    pub label: Option<[u8; 21]>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CancelInstruction {
    pub proof_a: [u8; 64],
    pub proof_b: [u8; 128],
    pub proof_c: [u8; 64],
    pub public_inputs: [[u8; 32]; 2],
    pub label: Option<[u8; 21]>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CollectProtocolFeesInstruction {
    pub amount_x_requested: u64,
    pub amount_y_requested: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateAmmConfigInstruction {
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub create_pool_fee: u64,
    pub wsol_trade_deposit: u64,
    pub deadline_slot_duration: u64,
    pub ratio_change_tolerance_rate: u64,
    pub halted: bool,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct InitializePoolInstruction {
    pub amount_x: u64,
    pub amount_y: u64,
    pub label: Option<[u8; 21]>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityInstruction {
    pub amount_lp: u64,
    pub min_receive_x: u64,
    pub min_receive_y: u64,
    pub label: Option<[u8; 21]>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SettleInstruction {
    pub proof_a: [u8; 64],
    pub proof_b: [u8; 128],
    pub proof_c: [u8; 64],
    pub public_inputs: [[u8; 32]; 2],
    pub unwrap_wsol: bool,
    pub ref_code: Option<[u8; 20]>,
    pub label: Option<[u8; 21]>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SlashInstruction {
    pub label: Option<[u8; 21]>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub amount_in: u64,
    pub is_swap_x_to_y: bool,
    pub c_min: [u8; 32],
    pub label: Option<[u8; 21]>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdateAmmConfigInstruction {
    pub param: u8,
    pub value: u64,
}

impl<'a> TryFrom<&'a [u8]> for DarklakeInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            ADD_LIQUIDITY => Self::AddLiquidity(AddLiquidityInstruction::try_from_slice(payload)?),
            CANCEL => Self::Cancel(CancelInstruction::try_from_slice(payload)?),
            COLLECT_PROTOCOL_FEES => Self::CollectProtocolFees(CollectProtocolFeesInstruction::try_from_slice(payload)?),
            CREATE_AMM_CONFIG => Self::CreateAmmConfig(CreateAmmConfigInstruction::try_from_slice(payload)?),
            INITIALIZE_POOL => Self::InitializePool(InitializePoolInstruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY => Self::RemoveLiquidity(RemoveLiquidityInstruction::try_from_slice(payload)?),
            SETTLE => Self::Settle(SettleInstruction::try_from_slice(payload)?),
            SLASH => Self::Slash(SlashInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            UPDATE_AMM_CONFIG => Self::UpdateAmmConfig(UpdateAmmConfigInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<DarklakeInstruction, ParseError> {
    DarklakeInstruction::try_from(data)
}
