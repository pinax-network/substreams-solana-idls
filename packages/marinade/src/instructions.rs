//! Marinade Finance on-chain instruction definitions and Borsh deserialisation.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
// -----------------------------------------------------------------------------
// Discriminators (Anchor: sha256("global:<name>")[..8])
// -----------------------------------------------------------------------------
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const DEPOSIT_STAKE_ACCOUNT: [u8; 8] = [110, 130, 115, 41, 164, 102, 2, 59];
pub const LIQUID_UNSTAKE: [u8; 8] = [30, 30, 119, 240, 191, 227, 12, 16];
pub const ADD_LIQUIDITY: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
pub const REMOVE_LIQUIDITY: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
pub const ORDER_UNSTAKE: [u8; 8] = [97, 167, 144, 107, 117, 190, 128, 36];
pub const CLAIM: [u8; 8] = [62, 198, 214, 193, 213, 159, 108, 210];
pub const WITHDRAW_STAKE_ACCOUNT: [u8; 8] = [211, 85, 184, 65, 183, 177, 233, 217];

// Admin / crank instructions (decoded but not parsed in detail)
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const CHANGE_AUTHORITY: [u8; 8] = [50, 106, 66, 104, 99, 118, 145, 88];
pub const ADD_VALIDATOR: [u8; 8] = [250, 113, 53, 54, 141, 117, 215, 185];
pub const REMOVE_VALIDATOR: [u8; 8] = [25, 96, 211, 155, 161, 14, 168, 188];
pub const SET_VALIDATOR_SCORE: [u8; 8] = [101, 41, 206, 33, 216, 111, 25, 78];
pub const CONFIG_VALIDATOR_SYSTEM: [u8; 8] = [27, 90, 97, 209, 17, 115, 7, 40];
pub const CONFIG_LP: [u8; 8] = [10, 24, 168, 119, 86, 48, 225, 17];
pub const CONFIG_MARINADE: [u8; 8] = [67, 3, 34, 114, 190, 185, 17, 62];
pub const STAKE_RESERVE: [u8; 8] = [87, 217, 23, 179, 205, 25, 113, 129];
pub const UPDATE_ACTIVE: [u8; 8] = [4, 67, 81, 64, 136, 245, 93, 152];
pub const UPDATE_DEACTIVATED: [u8; 8] = [16, 232, 131, 115, 156, 100, 239, 50];
pub const DEACTIVATE_STAKE: [u8; 8] = [165, 158, 229, 97, 168, 220, 187, 225];
pub const EMERGENCY_UNSTAKE: [u8; 8] = [123, 69, 168, 195, 183, 213, 199, 214];
pub const PARTIAL_UNSTAKE: [u8; 8] = [55, 241, 205, 221, 45, 114, 205, 163];
pub const MERGE_STAKES: [u8; 8] = [216, 36, 141, 225, 243, 78, 125, 237];
pub const REDELEGATE: [u8; 8] = [212, 82, 51, 160, 228, 80, 116, 35];
pub const PAUSE: [u8; 8] = [211, 22, 221, 251, 74, 121, 193, 47];
pub const RESUME: [u8; 8] = [1, 166, 51, 170, 127, 32, 141, 206];
pub const REALLOCATE_VALIDATOR_SCORE: [u8; 8] = [207, 202, 170, 75, 63, 240, 51, 234];
pub const REBALANCE_STAKE: [u8; 8] = [25, 77, 34, 6, 192, 58, 248, 230];

// ──────────────────────────────────────────────────────────────────────────────
// Instruction enum
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum MarinadeInstruction {
    /// Deposit SOL, receive mSOL.
    Deposit(DepositInstruction),
    /// Deposit a stake account, receive mSOL.
    DepositStakeAccount,
    /// Instantly unstake mSOL → SOL via the liquidity pool.
    LiquidUnstake(LiquidUnstakeInstruction),
    /// Add SOL liquidity to the SOL/mSOL pool.
    AddLiquidity(AddLiquidityInstruction),
    /// Remove liquidity (burn LP tokens).
    RemoveLiquidity(RemoveLiquidityInstruction),
    /// Start delayed unstake (mSOL → ticket).
    OrderUnstake(OrderUnstakeInstruction),
    /// Claim SOL from a completed unstake ticket.
    Claim,
    /// Withdraw a stake account directly (mSOL burned).
    WithdrawStakeAccount(WithdrawStakeAccountInstruction),

    // Admin / crank
    Initialize,
    ChangeAuthority,
    AddValidator,
    RemoveValidator,
    SetValidatorScore,
    ConfigValidatorSystem,
    ConfigLp,
    ConfigMarinade,
    StakeReserve,
    UpdateActive,
    UpdateDeactivated,
    DeactivateStake,
    EmergencyUnstake,
    PartialUnstake,
    MergeStakes,
    Redelegate,
    Pause,
    Resume,
    ReallocateValidatorScore,
    RebalanceStake,

    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------

/// Deposit SOL to receive mSOL.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    /// Amount of SOL (lamports) to deposit.
    pub lamports: u64,
}

/// Liquid unstake mSOL → SOL via the liquidity pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidUnstakeInstruction {
    /// Amount of mSOL to unstake.
    pub msol_amount: u64,
}

/// Add SOL liquidity to the SOL/mSOL liquidity pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityInstruction {
    /// Amount of SOL (lamports) to add.
    pub lamports: u64,
}

/// Remove liquidity by burning LP tokens.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityInstruction {
    /// Amount of LP tokens to burn.
    pub tokens: u64,
}

/// Order a delayed unstake (mSOL → unstake ticket).
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OrderUnstakeInstruction {
    /// Amount of mSOL to unstake.
    pub msol_amount: u64,
}

/// Withdraw a stake account directly (burns mSOL, splits a validator stake).
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawStakeAccountInstruction {
    /// Amount of mSOL to burn.
    pub msol_amount: u64,
    /// Index of the stake in the validator list.
    pub stake_index: u32,
    /// Index of the validator in the validator list.
    pub validator_index: u32,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for MarinadeInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            DEPOSIT_STAKE_ACCOUNT => Self::DepositStakeAccount,
            LIQUID_UNSTAKE => Self::LiquidUnstake(LiquidUnstakeInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY => Self::AddLiquidity(AddLiquidityInstruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY => Self::RemoveLiquidity(RemoveLiquidityInstruction::try_from_slice(payload)?),
            ORDER_UNSTAKE => Self::OrderUnstake(OrderUnstakeInstruction::try_from_slice(payload)?),
            CLAIM => Self::Claim,
            WITHDRAW_STAKE_ACCOUNT => Self::WithdrawStakeAccount(WithdrawStakeAccountInstruction::try_from_slice(payload)?),
            INITIALIZE => Self::Initialize,
            CHANGE_AUTHORITY => Self::ChangeAuthority,
            ADD_VALIDATOR => Self::AddValidator,
            REMOVE_VALIDATOR => Self::RemoveValidator,
            SET_VALIDATOR_SCORE => Self::SetValidatorScore,
            CONFIG_VALIDATOR_SYSTEM => Self::ConfigValidatorSystem,
            CONFIG_LP => Self::ConfigLp,
            CONFIG_MARINADE => Self::ConfigMarinade,
            STAKE_RESERVE => Self::StakeReserve,
            UPDATE_ACTIVE => Self::UpdateActive,
            UPDATE_DEACTIVATED => Self::UpdateDeactivated,
            DEACTIVATE_STAKE => Self::DeactivateStake,
            EMERGENCY_UNSTAKE => Self::EmergencyUnstake,
            PARTIAL_UNSTAKE => Self::PartialUnstake,
            MERGE_STAKES => Self::MergeStakes,
            REDELEGATE => Self::Redelegate,
            PAUSE => Self::Pause,
            RESUME => Self::Resume,
            REALLOCATE_VALIDATOR_SCORE => Self::ReallocateValidatorScore,
            REBALANCE_STAKE => Self::RebalanceStake,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<MarinadeInstruction, ParseError> {
    MarinadeInstruction::try_from(data)
}
