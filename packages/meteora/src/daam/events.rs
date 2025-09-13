//! Meteora DAMM v2 events.

use super::instructions::{
    AddLiquidityParameters, PoolFeeParameters, RemoveLiquidityParameters, SplitAmountInfo, SplitPositionInfo, SplitPositionParameters, SwapParameters,
    SwapResult,
};
use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const EVTADDLIQUIDITY: [u8; 8] = [175, 242, 8, 157, 30, 247, 185, 169];
pub const EVTCLAIMPARTNERFEE: [u8; 8] = [118, 99, 77, 10, 226, 1, 1, 87];
pub const EVTCLAIMPOSITIONFEE: [u8; 8] = [198, 182, 183, 52, 97, 12, 49, 56];
pub const EVTCLAIMPROTOCOLFEE: [u8; 8] = [186, 244, 75, 251, 188, 13, 25, 33];
pub const EVTCLAIMREWARD: [u8; 8] = [218, 86, 147, 200, 235, 188, 215, 231];
pub const EVTCLOSECLAIMFEEOPERATOR: [u8; 8] = [111, 39, 37, 55, 110, 216, 194, 23];
pub const EVTCLOSECONFIG: [u8; 8] = [36, 30, 239, 45, 58, 132, 14, 5];
pub const EVTCLOSEPOSITION: [u8; 8] = [20, 145, 144, 68, 143, 142, 214, 178];
pub const EVTCREATECLAIMFEEOPERATOR: [u8; 8] = [21, 6, 153, 120, 68, 116, 28, 177];
pub const EVTCREATECONFIG: [u8; 8] = [131, 207, 180, 174, 180, 73, 165, 54];
pub const EVTCREATEDYNAMICCONFIG: [u8; 8] = [231, 197, 13, 164, 248, 213, 133, 152];
pub const EVTCREATEPOSITION: [u8; 8] = [156, 15, 119, 198, 29, 181, 221, 55];
pub const EVTCREATETOKENBADGE: [u8; 8] = [141, 120, 134, 116, 34, 28, 114, 160];
pub const EVTFUNDREWARD: [u8; 8] = [104, 233, 237, 122, 199, 191, 121, 85];
pub const EVTINITIALIZEPOOL: [u8; 8] = [228, 50, 246, 85, 203, 66, 134, 37];
pub const EVTINITIALIZEREWARD: [u8; 8] = [129, 91, 188, 3, 246, 52, 185, 249];
pub const EVTLOCKPOSITION: [u8; 8] = [168, 63, 108, 83, 219, 82, 2, 200];
pub const EVTPERMANENTLOCKPOSITION: [u8; 8] = [145, 143, 162, 218, 218, 80, 67, 11];
pub const EVTREMOVELIQUIDITY: [u8; 8] = [87, 46, 88, 98, 175, 96, 34, 91];
pub const EVTSETPOOLSTATUS: [u8; 8] = [100, 213, 74, 3, 95, 91, 228, 146];
pub const EVTSPLITPOSITION: [u8; 8] = [182, 138, 42, 254, 27, 94, 82, 221];
pub const EVTSWAP: [u8; 8] = [27, 60, 21, 213, 138, 170, 187, 147];
pub const EVTUPDATEREWARDDURATION: [u8; 8] = [149, 135, 65, 231, 129, 153, 65, 57];
pub const EVTUPDATEREWARDFUNDER: [u8; 8] = [76, 154, 208, 13, 40, 115, 246, 146];
pub const EVTWITHDRAWINELIGIBLEREWARD: [u8; 8] = [248, 215, 184, 78, 31, 180, 179, 168];
const ANCHOR_DISC: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeteoraDammEvent {
    EvtAddLiquidity(EvtAddLiquidity),
    EvtClaimPartnerFee(EvtClaimPartnerFee),
    EvtClaimPositionFee(EvtClaimPositionFee),
    EvtClaimProtocolFee(EvtClaimProtocolFee),
    EvtClaimReward(EvtClaimReward),
    EvtCloseClaimFeeOperator(EvtCloseClaimFeeOperator),
    EvtCloseConfig(EvtCloseConfig),
    EvtClosePosition(EvtClosePosition),
    EvtCreateClaimFeeOperator(EvtCreateClaimFeeOperator),
    EvtCreateConfig(EvtCreateConfig),
    EvtCreateDynamicConfig(EvtCreateDynamicConfig),
    EvtCreatePosition(EvtCreatePosition),
    EvtCreateTokenBadge(EvtCreateTokenBadge),
    EvtFundReward(EvtFundReward),
    EvtInitializePool(EvtInitializePool),
    EvtInitializeReward(EvtInitializeReward),
    EvtLockPosition(EvtLockPosition),
    EvtPermanentLockPosition(EvtPermanentLockPosition),
    EvtRemoveLiquidity(EvtRemoveLiquidity),
    EvtSetPoolStatus(EvtSetPoolStatus),
    EvtSplitPosition(EvtSplitPosition),
    EvtSwap(EvtSwap),
    EvtUpdateRewardDuration(EvtUpdateRewardDuration),
    EvtUpdateRewardFunder(EvtUpdateRewardFunder),
    EvtWithdrawIneligibleReward(EvtWithdrawIneligibleReward),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtAddLiquidity {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub params: AddLiquidityParameters,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtClaimPartnerFee {
    pub pool: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtClaimPositionFee {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub fee_a_claimed: u64,
    pub fee_b_claimed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtClaimProtocolFee {
    pub pool: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtClaimReward {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub mint_reward: Pubkey,
    pub reward_index: u8,
    pub total_reward: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtCloseClaimFeeOperator {
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtCloseConfig {
    /// Config pubkey
    pub config: Pubkey,
    /// admin pk
    pub admin: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtClosePosition {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtCreateClaimFeeOperator {
    pub operator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtCreateConfig {
    pub pool_fees: PoolFeeParameters,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub collect_fee_mode: u8,
    pub index: u64,
    pub config: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtCreateDynamicConfig {
    pub config: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtCreatePosition {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position: Pubkey,
    pub position_nft_mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtCreateTokenBadge {
    pub token_mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtFundReward {
    pub pool: Pubkey,
    pub funder: Pubkey,
    pub mint_reward: Pubkey,
    pub reward_index: u8,
    pub amount: u64,
    pub transfer_fee_excluded_amount_in: u64,
    pub reward_duration_end: u64,
    pub pre_reward_rate: u128,
    pub post_reward_rate: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtInitializePool {
    pub pool: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub alpha_vault: Pubkey,
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
    pub pool_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtInitializeReward {
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub funder: Pubkey,
    pub creator: Pubkey,
    pub reward_index: u8,
    pub reward_duration: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtLockPosition {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub vesting: Pubkey,
    pub cliff_point: u64,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtPermanentLockPosition {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub lock_liquidity_amount: u128,
    pub total_permanent_locked_liquidity: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtRemoveLiquidity {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub params: RemoveLiquidityParameters,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtSetPoolStatus {
    pub pool: Pubkey,
    pub status: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtSplitPosition {
    pub pool: Pubkey,
    pub first_owner: Pubkey,
    pub second_owner: Pubkey,
    pub first_position: Pubkey,
    pub second_position: Pubkey,
    pub current_sqrt_price: u128,
    pub amount_splits: SplitAmountInfo,
    pub first_position_info: SplitPositionInfo,
    pub second_position_info: SplitPositionInfo,
    pub split_position_parameters: SplitPositionParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtSwap {
    pub pool: Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub params: SwapParameters,
    pub swap_result: SwapResult,
    pub actual_amount_in: u64,
    pub current_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtUpdateRewardDuration {
    pub pool: Pubkey,
    pub reward_index: u8,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtUpdateRewardFunder {
    pub pool: Pubkey,
    pub reward_index: u8,
    pub old_funder: Pubkey,
    pub new_funder: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EvtWithdrawIneligibleReward {
    pub pool: Pubkey,
    pub reward_mint: Pubkey,
    pub amount: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for MeteoraDammEvent {
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
            EVTADDLIQUIDITY => Self::EvtAddLiquidity(EvtAddLiquidity::try_from_slice(payload)?),
            EVTCLAIMPARTNERFEE => Self::EvtClaimPartnerFee(EvtClaimPartnerFee::try_from_slice(payload)?),
            EVTCLAIMPOSITIONFEE => Self::EvtClaimPositionFee(EvtClaimPositionFee::try_from_slice(payload)?),
            EVTCLAIMPROTOCOLFEE => Self::EvtClaimProtocolFee(EvtClaimProtocolFee::try_from_slice(payload)?),
            EVTCLAIMREWARD => Self::EvtClaimReward(EvtClaimReward::try_from_slice(payload)?),
            EVTCLOSECLAIMFEEOPERATOR => Self::EvtCloseClaimFeeOperator(EvtCloseClaimFeeOperator::try_from_slice(payload)?),
            EVTCLOSECONFIG => Self::EvtCloseConfig(EvtCloseConfig::try_from_slice(payload)?),
            EVTCLOSEPOSITION => Self::EvtClosePosition(EvtClosePosition::try_from_slice(payload)?),
            EVTCREATECLAIMFEEOPERATOR => Self::EvtCreateClaimFeeOperator(EvtCreateClaimFeeOperator::try_from_slice(payload)?),
            EVTCREATECONFIG => Self::EvtCreateConfig(EvtCreateConfig::try_from_slice(payload)?),
            EVTCREATEDYNAMICCONFIG => Self::EvtCreateDynamicConfig(EvtCreateDynamicConfig::try_from_slice(payload)?),
            EVTCREATEPOSITION => Self::EvtCreatePosition(EvtCreatePosition::try_from_slice(payload)?),
            EVTCREATETOKENBADGE => Self::EvtCreateTokenBadge(EvtCreateTokenBadge::try_from_slice(payload)?),
            EVTFUNDREWARD => Self::EvtFundReward(EvtFundReward::try_from_slice(payload)?),
            EVTINITIALIZEPOOL => Self::EvtInitializePool(EvtInitializePool::try_from_slice(payload)?),
            EVTINITIALIZEREWARD => Self::EvtInitializeReward(EvtInitializeReward::try_from_slice(payload)?),
            EVTLOCKPOSITION => Self::EvtLockPosition(EvtLockPosition::try_from_slice(payload)?),
            EVTPERMANENTLOCKPOSITION => Self::EvtPermanentLockPosition(EvtPermanentLockPosition::try_from_slice(payload)?),
            EVTREMOVELIQUIDITY => Self::EvtRemoveLiquidity(EvtRemoveLiquidity::try_from_slice(payload)?),
            EVTSETPOOLSTATUS => Self::EvtSetPoolStatus(EvtSetPoolStatus::try_from_slice(payload)?),
            EVTSPLITPOSITION => Self::EvtSplitPosition(EvtSplitPosition::try_from_slice(payload)?),
            EVTSWAP => Self::EvtSwap(EvtSwap::try_from_slice(payload)?),
            EVTUPDATEREWARDDURATION => Self::EvtUpdateRewardDuration(EvtUpdateRewardDuration::try_from_slice(payload)?),
            EVTUPDATEREWARDFUNDER => Self::EvtUpdateRewardFunder(EvtUpdateRewardFunder::try_from_slice(payload)?),
            EVTWITHDRAWINELIGIBLEREWARD => Self::EvtWithdrawIneligibleReward(EvtWithdrawIneligibleReward::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<MeteoraDammEvent, ParseError> {
    MeteoraDammEvent::try_from(data)
}
