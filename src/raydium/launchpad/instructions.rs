//! Raydium Launchpad trading instructions.

pub use super::anchor_cpi_event::{AmmCreatorFeeOn, CurveParams, MintParams, VestingParams};
use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIALIZE_V2: [u8; 8] = [67, 153, 175, 39, 218, 16, 38, 32];
pub const INITIALIZE_WITH_TOKEN_2022: [u8; 8] = [37, 190, 126, 222, 44, 154, 171, 17];

pub const BUY_EXACT_IN: [u8; 8] = [250, 234, 13, 123, 213, 156, 19, 236];
pub const BUY_EXACT_OUT: [u8; 8] = [24, 211, 116, 40, 105, 3, 153, 56];
pub const SELL_EXACT_IN: [u8; 8] = [149, 39, 222, 155, 211, 124, 152, 26];
pub const SELL_EXACT_OUT: [u8; 8] = [95, 200, 71, 34, 8, 9, 11, 166];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaydiumLaunchpadInstruction {
    BuyExactIn(BuyExactInInstruction),
    BuyExactOut(BuyExactOutInstruction),
    SellExactIn(SellExactInInstruction),
    SellExactOut(SellExactOutInstruction),
    Unknown,
    Initialize(InitializeInstruction),
    InitializeV2(InitializeV2Instruction),
    InitializeWithToken2022(InitializeWithToken2022Instruction),
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Use the given amount of quote tokens to purchase base tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyExactInInstruction {
    /// Amount of quote token to purchase
    pub amount_in: u64,
    /// Minimum amount of base token to receive (slippage protection)
    pub minimum_amount_out: u64,
    /// Fee rate for the share
    pub share_fee_rate: u64,
}

/// Use quote tokens to purchase the given amount of base tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyExactOutInstruction {
    /// Amount of base token to receive
    pub amount_out: u64,
    /// Maximum amount of quote token to purchase (slippage protection)
    pub maximum_amount_in: u64,
    /// Fee rate for the share
    pub share_fee_rate: u64,
}

/// Use the given amount of base tokens to sell for quote tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellExactInInstruction {
    /// Amount of base token to sell
    pub amount_in: u64,
    /// Minimum amount of quote token to receive (slippage protection)
    pub minimum_amount_out: u64,
    /// Fee rate for the share
    pub share_fee_rate: u64,
}

/// Sell base tokens for the given amount of quote tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellExactOutInstruction {
    /// Amount of quote token to receive
    pub amount_out: u64,
    /// Maximum amount of base token to purchase (slippage protection)
    pub maximum_amount_in: u64,
    /// Fee rate for the share
    pub share_fee_rate: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeInstruction {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeV2Instruction {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeWithToken2022Instruction {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
    pub transfer_fee_extension_param: Option<TransferFeeExtensionParams>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TransferFeeExtensionParams {
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumLaunchpadInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            INITIALIZE => Self::Initialize(InitializeInstruction::try_from_slice(payload)?),
            INITIALIZE_V2 => Self::InitializeV2(InitializeV2Instruction::try_from_slice(payload)?),
            INITIALIZE_WITH_TOKEN_2022 => Self::InitializeWithToken2022(InitializeWithToken2022Instruction::try_from_slice(payload)?),
            BUY_EXACT_IN => Self::BuyExactIn(BuyExactInInstruction::try_from_slice(payload)?),
            BUY_EXACT_OUT => Self::BuyExactOut(BuyExactOutInstruction::try_from_slice(payload)?),
            SELL_EXACT_IN => Self::SellExactIn(SellExactInInstruction::try_from_slice(payload)?),
            SELL_EXACT_OUT => Self::SellExactOut(SellExactOutInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumLaunchpadInstruction, ParseError> {
    RaydiumLaunchpadInstruction::try_from(data)
}
