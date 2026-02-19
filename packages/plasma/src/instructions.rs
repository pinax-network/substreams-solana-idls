//! Plasma on-chain instructions (Shank-based, single-byte discriminants).

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (Shank: sequential u8 values)
// -----------------------------------------------------------------------------
pub const SWAP: u8 = 0;
pub const ADD_LIQUIDITY: u8 = 1;
pub const REMOVE_LIQUIDITY: u8 = 2;
pub const RENOUNCE_LIQUIDITY: u8 = 3;
pub const WITHDRAW_LP_FEES: u8 = 4;
pub const INITIALIZE_LP_POSITION: u8 = 5;
pub const INITIALIZE_POOL: u8 = 6;
pub const WITHDRAW_PROTOCOL_FEES: u8 = 7;
pub const LOG: u8 = 8;
pub const TRANSFER_LIQUIDITY: u8 = 9;

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum SwapType {
    ExactIn { amount_in: u64, min_amount_out: u64 },
    ExactOut { amount_out: u64, max_amount_in: u64 },
}

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlasmaInstruction {
    Swap(SwapInstruction),
    AddLiquidity(AddLiquidityInstruction),
    RemoveLiquidity(RemoveLiquidityInstruction),
    RenounceLiquidity(RenounceLiquidityInstruction),
    WithdrawLpFees,
    InitializeLpPosition,
    InitializePool(InitializePoolInstruction),
    WithdrawProtocolFees,
    Log,
    TransferLiquidity,
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub side: Side,
    pub swap_type: SwapType,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityInstruction {
    pub desired_base_amount_in: u64,
    pub desired_quote_amount_in: u64,
    pub initial_lp_shares: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityInstruction {
    pub lp_shares: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RenounceLiquidityInstruction {
    pub allow_fee_withdrawal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ProtocolFeeRecipientParams {
    pub recipient: [u8; 32],
    pub shares: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePoolInstruction {
    pub lp_fee_in_bps: u64,
    pub protocol_lp_fee_allocation_in_pct: u64,
    pub fee_recipients_params: [ProtocolFeeRecipientParams; 3],
    pub num_slots_to_vest_lp_shares: Option<u64>,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PlasmaInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(1);
        let discriminator = disc[0];

        Ok(match discriminator {
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY => Self::AddLiquidity(AddLiquidityInstruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY => Self::RemoveLiquidity(RemoveLiquidityInstruction::try_from_slice(payload)?),
            RENOUNCE_LIQUIDITY => Self::RenounceLiquidity(RenounceLiquidityInstruction::try_from_slice(payload)?),
            WITHDRAW_LP_FEES => Self::WithdrawLpFees,
            INITIALIZE_LP_POSITION => Self::InitializeLpPosition,
            INITIALIZE_POOL => Self::InitializePool(InitializePoolInstruction::try_from_slice(payload)?),
            WITHDRAW_PROTOCOL_FEES => Self::WithdrawProtocolFees,
            LOG => Self::Log,
            TRANSFER_LIQUIDITY => Self::TransferLiquidity,
            other => return Err(ParseError::Unknown([other, 0, 0, 0, 0, 0, 0, 0])),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PlasmaInstruction, ParseError> {
    PlasmaInstruction::try_from(data)
}
