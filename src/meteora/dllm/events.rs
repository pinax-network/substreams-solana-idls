//! Meteora DLMM on-chain events.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the emitted log’s data)
// -----------------------------------------------------------------------------
const SWAP: [u8; 8] = [81, 108, 227, 190, 205, 208, 10, 196];
const ANCHOR_DISC: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];
// const ADD_LIQUIDITY: [u8; 8] = [31, 94, 125, 90, 227, 52, 61, 186];
// const CLAIM_FEE: [u8; 8] = [75, 122, 154, 48, 140, 74, 123, 163];
// const CLAIM_FEE2: [u8; 8] = [232, 171, 242, 97, 58, 77, 35, 45];
// const CLAIM_REWARD: [u8; 8] = [148, 116, 134, 204, 22, 171, 85, 95];
// const CLAIM_REWARD2: [u8; 8] = [27, 143, 244, 33, 80, 43, 110, 146];
// const COMPOSITION_FEE: [u8; 8] = [128, 151, 123, 106, 17, 102, 113, 142];
// const DECREASE_POSITION_LENGTH: [u8; 8] = [52, 118, 235, 85, 172, 169, 15, 128];
// const DYNAMIC_FEE_PARAMETER_UPDATE: [u8; 8] = [88, 88, 178, 135, 194, 146, 91, 243];
// const FEE_PARAMETER_UPDATE: [u8; 8] = [48, 76, 241, 117, 144, 215, 242, 44];
// const FUND_REWARD: [u8; 8] = [246, 228, 58, 130, 145, 170, 79, 204];
// const GO_TO_ABIN: [u8; 8] = [59, 138, 76, 68, 138, 131, 176, 67];
// const INCREASE_OBSERVATION: [u8; 8] = [99, 249, 17, 121, 166, 156, 207, 215];
// const INCREASE_POSITION_LENGTH: [u8; 8] = [157, 239, 42, 204, 30, 56, 223, 46];
// const INITIALIZE_REWARD: [u8; 8] = [211, 153, 88, 62, 149, 60, 177, 70];
// const LB_PAIR_CREATE: [u8; 8] = [185, 74, 252, 125, 27, 215, 188, 111];
// const POSITION_CLOSE: [u8; 8] = [255, 196, 16, 107, 28, 202, 53, 128];
// const POSITION_CREATE: [u8; 8] = [144, 142, 252, 84, 157, 53, 37, 121];
// const REBALANCING: [u8; 8] = [0, 109, 117, 179, 61, 91, 199, 200];
// const REMOVE_LIQUIDITY: [u8; 8] = [116, 244, 97, 232, 103, 31, 152, 58];
// const UPDATE_POSITION_LOCK_RELEASE_POINT: [u8; 8] = [133, 214, 66, 224, 64, 12, 7, 191];
// const UPDATE_POSITION_OPERATOR: [u8; 8] = [39, 115, 48, 204, 246, 47, 66, 57];
// const UPDATE_REWARD_DURATION: [u8; 8] = [223, 245, 224, 153, 49, 29, 163, 172];
// const UPDATE_REWARD_FUNDER: [u8; 8] = [224, 178, 174, 74, 252, 165, 85, 180];
// const WITHDRAW_INELIGIBLE_REWARD: [u8; 8] = [231, 189, 65, 149, 102, 215, 154, 244];

// -----------------------------------------------------------------------------
// High-level event enum
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeteoraDllmEvent {
    Swap(SwapEvent),
    // AddLiquidity,
    // ClaimFee,
    // ClaimFee2,
    // ClaimReward,
    // ClaimReward2,
    // CompositionFee,
    // DecreasePositionLength,
    // DynamicFeeParameterUpdate,
    // FeeParameterUpdate,
    // FundReward,
    // GoToABin,
    // IncreaseObservation,
    // IncreasePositionLength,
    // InitializeReward,
    // LbPairCreate,
    // PositionClose,
    // PositionCreate,
    // Rebalancing,
    // RemoveLiquidity,
    // UpdatePositionLockReleasePoint,
    // UpdatePositionOperator,
    // UpdateRewardDuration,
    // UpdateRewardFunder,
    // WithdrawIneligibleReward,
    Unknown,
}

/// Emitted when swap occurs.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapEvent {
    pub lb_pair: Pubkey,
    pub from: Pubkey,
    pub start_bin_id: i32,
    pub end_bin_id: i32,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_for_y: bool,
    pub fee: u64,
    pub protocol_fee: u64,
    pub fee_bps: u128,
    pub host_fee: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for MeteoraDllmEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = if data.len() >= 16 && &data[0..8] == ANCHOR_DISC {
            let disc: [u8; 8] = data[8..16].try_into().expect("slice len 8");
            (disc, &data[16..])
        } else {
            let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
            (disc, &data[8..])
        };
        Ok(match disc {
            SWAP => Self::Swap(SwapEvent::try_from_slice(payload)?),
            // ADD_LIQUIDITY => Self::AddLiquidity,
            // CLAIM_FEE => Self::ClaimFee,
            // CLAIM_FEE2 => Self::ClaimFee2,
            // CLAIM_REWARD => Self::ClaimReward,
            // CLAIM_REWARD2 => Self::ClaimReward2,
            // COMPOSITION_FEE => Self::CompositionFee,
            // DECREASE_POSITION_LENGTH => Self::DecreasePositionLength,
            // DYNAMIC_FEE_PARAMETER_UPDATE => Self::DynamicFeeParameterUpdate,
            // FEE_PARAMETER_UPDATE => Self::FeeParameterUpdate,
            // FUND_REWARD => Self::FundReward,
            // GO_TO_ABIN => Self::GoToABin,
            // INCREASE_OBSERVATION => Self::IncreaseObservation,
            // INCREASE_POSITION_LENGTH => Self::IncreasePositionLength,
            // INITIALIZE_REWARD => Self::InitializeReward,
            // LB_PAIR_CREATE => Self::LbPairCreate,
            // POSITION_CLOSE => Self::PositionClose,
            // POSITION_CREATE => Self::PositionCreate,
            // REBALANCING => Self::Rebalancing,
            // REMOVE_LIQUIDITY => Self::RemoveLiquidity,
            // UPDATE_POSITION_LOCK_RELEASE_POINT => Self::UpdatePositionLockReleasePoint,
            // UPDATE_POSITION_OPERATOR => Self::UpdatePositionOperator,
            // UPDATE_REWARD_DURATION => Self::UpdateRewardDuration,
            // UPDATE_REWARD_FUNDER => Self::UpdateRewardFunder,
            // WITHDRAW_INELIGIBLE_REWARD => Self::WithdrawIneligibleReward,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<MeteoraDllmEvent, ParseError> {
    MeteoraDllmEvent::try_from(data)
}
