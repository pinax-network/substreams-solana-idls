//! Meteora DAMM v2 instructions.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityParameters {
    /// delta liquidity
    pub liquidity_delta: u128,
    /// maximum token a amount
    pub token_a_amount_threshold: u64,
    /// maximum token b amount
    pub token_b_amount_threshold: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BaseFeeParameters {
    pub cliff_fee_numerator: u64,
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
    pub fee_scheduler_mode: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DynamicConfigParameters {
    pub pool_creator_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DynamicFeeParameters {
    pub bin_step: u16,
    pub bin_step_u128: u128,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeCustomizablePoolParameters {
    /// pool fees
    pub pool_fees: PoolFeeParameters,
    /// sqrt min price
    pub sqrt_min_price: u128,
    /// sqrt max price
    pub sqrt_max_price: u128,
    /// has alpha vault
    pub has_alpha_vault: bool,
    /// initialize liquidity
    pub liquidity: u128,
    /// The init price of the pool as a sqrt(token_b/token_a) Q64.64 value
    pub sqrt_price: u128,
    /// activation type
    pub activation_type: u8,
    /// collect fee mode
    pub collect_fee_mode: u8,
    /// activation point
    pub activation_point: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePoolParameters {
    /// initialize liquidity
    pub liquidity: u128,
    /// The init price of the pool as a sqrt(token_b/token_a) Q64.64 value
    pub sqrt_price: u128,
    /// activation point
    pub activation_point: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PoolFeeParameters {
    /// Base fee
    pub base_fee: BaseFeeParameters,
    /// padding
    pub padding: [u8; 3],
    /// dynamic fee
    pub dynamic_fee: Option<DynamicFeeParameters>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityParameters {
    /// delta liquidity
    pub liquidity_delta: u128,
    /// minimum token a amount
    pub token_a_amount_threshold: u64,
    /// minimum token b amount
    pub token_b_amount_threshold: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SplitAmountInfo {
    pub permanent_locked_liquidity: u128,
    pub unlocked_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SplitPositionInfo {
    pub liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SplitPositionParameters {
    /// Percentage of unlocked liquidity to split to the second position
    pub unlocked_liquidity_percentage: u8,
    /// Percentage of permanent locked liquidity to split to the second position
    pub permanent_locked_liquidity_percentage: u8,
    /// Percentage of fee A pending to split to the second position
    pub fee_a_percentage: u8,
    /// Percentage of fee B pending to split to the second position
    pub fee_b_percentage: u8,
    /// Percentage of reward 0 pending to split to the second position
    pub reward_0_percentage: u8,
    /// Percentage of reward 1 pending to split to the second position
    pub reward_1_percentage: u8,
    /// padding for future
    pub padding: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct StaticConfigParameters {
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapParameters {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapResult {
    pub output_amount: u64,
    pub next_sqrt_price: u128,
    pub lp_fee: u64,
    pub protocol_fee: u64,
    pub partner_fee: u64,
    pub referral_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct VestingParameters {
    pub cliff_point: Option<u64>,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const ADD_LIQUIDITY: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
pub const CLAIM_PARTNER_FEE: [u8; 8] = [97, 206, 39, 105, 94, 94, 126, 148];
pub const CLAIM_POSITION_FEE: [u8; 8] = [180, 38, 154, 17, 133, 33, 162, 211];
pub const CLAIM_PROTOCOL_FEE: [u8; 8] = [165, 228, 133, 48, 99, 249, 255, 33];
pub const CLAIM_REWARD: [u8; 8] = [149, 95, 181, 242, 94, 90, 158, 162];
pub const CLOSE_CLAIM_FEE_OPERATOR: [u8; 8] = [38, 134, 82, 216, 95, 124, 17, 99];
pub const CLOSE_CONFIG: [u8; 8] = [145, 9, 72, 157, 95, 125, 61, 85];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const CLOSE_TOKEN_BADGE: [u8; 8] = [108, 146, 86, 110, 179, 254, 10, 104];
pub const CREATE_CLAIM_FEE_OPERATOR: [u8; 8] = [169, 62, 207, 107, 58, 187, 162, 109];
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CREATE_DYNAMIC_CONFIG: [u8; 8] = [81, 251, 122, 78, 66, 57, 208, 82];
pub const CREATE_POSITION: [u8; 8] = [48, 215, 197, 153, 96, 203, 180, 133];
pub const CREATE_TOKEN_BADGE: [u8; 8] = [88, 206, 0, 91, 60, 175, 151, 118];
pub const FUND_REWARD: [u8; 8] = [188, 50, 249, 165, 93, 151, 38, 63];
pub const INITIALIZE_CUSTOMIZABLE_POOL: [u8; 8] = [20, 161, 241, 24, 189, 221, 180, 2];
pub const INITIALIZE_POOL: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
pub const INITIALIZE_POOL_WITH_DYNAMIC_CONFIG: [u8; 8] = [149, 82, 72, 197, 253, 252, 68, 15];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const LOCK_POSITION: [u8; 8] = [227, 62, 2, 252, 247, 10, 171, 185];
pub const PERMANENT_LOCK_POSITION: [u8; 8] = [165, 176, 125, 6, 231, 171, 186, 213];
pub const REFRESH_VESTING: [u8; 8] = [9, 94, 216, 14, 116, 204, 247, 0];
pub const REMOVE_ALL_LIQUIDITY: [u8; 8] = [10, 51, 61, 35, 112, 105, 24, 85];
pub const REMOVE_LIQUIDITY: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
pub const SET_POOL_STATUS: [u8; 8] = [112, 87, 135, 223, 83, 204, 132, 53];
pub const SPLIT_POSITION: [u8; 8] = [172, 241, 221, 138, 161, 29, 253, 42];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const UPDATE_REWARD_DURATION: [u8; 8] = [138, 174, 196, 169, 213, 235, 254, 107];
pub const UPDATE_REWARD_FUNDER: [u8; 8] = [211, 28, 48, 32, 215, 160, 35, 23];
pub const WITHDRAW_INELIGIBLE_REWARD: [u8; 8] = [148, 206, 42, 195, 247, 49, 103, 8];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeteoraDammInstruction {
    AddLiquidity(AddLiquidityInstruction),
    ClaimPartnerFee(ClaimPartnerFeeInstruction),
    ClaimPositionFee,
    ClaimProtocolFee(ClaimProtocolFeeInstruction),
    ClaimReward(ClaimRewardInstruction),
    CloseClaimFeeOperator,
    CloseConfig,
    ClosePosition,
    CloseTokenBadge,
    CreateClaimFeeOperator,
    CreateConfig(CreateConfigInstruction),
    CreateDynamicConfig(CreateDynamicConfigInstruction),
    CreatePosition,
    CreateTokenBadge,
    FundReward(FundRewardInstruction),
    InitializeCustomizablePool(InitializeCustomizablePoolInstruction),
    InitializePool(InitializePoolInstruction),
    InitializePoolWithDynamicConfig(InitializePoolWithDynamicConfigInstruction),
    InitializeReward(InitializeRewardInstruction),
    LockPosition(LockPositionInstruction),
    PermanentLockPosition(PermanentLockPositionInstruction),
    RefreshVesting,
    RemoveAllLiquidity(RemoveAllLiquidityInstruction),
    RemoveLiquidity(RemoveLiquidityInstruction),
    SetPoolStatus(SetPoolStatusInstruction),
    SplitPosition(SplitPositionInstruction),
    Swap(SwapInstruction),
    UpdateRewardDuration(UpdateRewardDurationInstruction),
    UpdateRewardFunder(UpdateRewardFunderInstruction),
    WithdrawIneligibleReward(WithdrawIneligibleRewardInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityInstruction {
    pub params: AddLiquidityParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimPartnerFeeInstruction {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimProtocolFeeInstruction {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimRewardInstruction {
    pub reward_index: u8,
    pub skip_reward: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateConfigInstruction {
    pub index: u64,
    pub config_parameters: StaticConfigParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateDynamicConfigInstruction {
    pub index: u64,
    pub config_parameters: DynamicConfigParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FundRewardInstruction {
    pub reward_index: u8,
    pub amount: u64,
    pub carry_forward: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeCustomizablePoolInstruction {
    pub params: InitializeCustomizablePoolParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePoolInstruction {
    pub params: InitializePoolParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePoolWithDynamicConfigInstruction {
    pub params: InitializeCustomizablePoolParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeRewardInstruction {
    pub reward_index: u8,
    pub reward_duration: u64,
    pub funder: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LockPositionInstruction {
    pub params: VestingParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PermanentLockPositionInstruction {
    pub permanent_lock_liquidity: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveAllLiquidityInstruction {
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityInstruction {
    pub params: RemoveLiquidityParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetPoolStatusInstruction {
    pub status: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SplitPositionInstruction {
    pub params: SplitPositionParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub params: SwapParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateRewardDurationInstruction {
    pub reward_index: u8,
    pub new_duration: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateRewardFunderInstruction {
    pub reward_index: u8,
    pub new_funder: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawIneligibleRewardInstruction {
    pub reward_index: u8,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for MeteoraDammInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            ADD_LIQUIDITY => Self::AddLiquidity(AddLiquidityInstruction::try_from_slice(payload)?),
            CLAIM_PARTNER_FEE => Self::ClaimPartnerFee(ClaimPartnerFeeInstruction::try_from_slice(payload)?),
            CLAIM_POSITION_FEE => Self::ClaimPositionFee,
            CLAIM_PROTOCOL_FEE => Self::ClaimProtocolFee(ClaimProtocolFeeInstruction::try_from_slice(payload)?),
            CLAIM_REWARD => Self::ClaimReward(ClaimRewardInstruction::try_from_slice(payload)?),
            CLOSE_CLAIM_FEE_OPERATOR => Self::CloseClaimFeeOperator,
            CLOSE_CONFIG => Self::CloseConfig,
            CLOSE_POSITION => Self::ClosePosition,
            CLOSE_TOKEN_BADGE => Self::CloseTokenBadge,
            CREATE_CLAIM_FEE_OPERATOR => Self::CreateClaimFeeOperator,
            CREATE_CONFIG => Self::CreateConfig(CreateConfigInstruction::try_from_slice(payload)?),
            CREATE_DYNAMIC_CONFIG => Self::CreateDynamicConfig(CreateDynamicConfigInstruction::try_from_slice(payload)?),
            CREATE_POSITION => Self::CreatePosition,
            CREATE_TOKEN_BADGE => Self::CreateTokenBadge,
            FUND_REWARD => Self::FundReward(FundRewardInstruction::try_from_slice(payload)?),
            INITIALIZE_CUSTOMIZABLE_POOL => Self::InitializeCustomizablePool(InitializeCustomizablePoolInstruction::try_from_slice(payload)?),
            INITIALIZE_POOL => Self::InitializePool(InitializePoolInstruction::try_from_slice(payload)?),
            INITIALIZE_POOL_WITH_DYNAMIC_CONFIG => Self::InitializePoolWithDynamicConfig(InitializePoolWithDynamicConfigInstruction::try_from_slice(payload)?),
            INITIALIZE_REWARD => Self::InitializeReward(InitializeRewardInstruction::try_from_slice(payload)?),
            LOCK_POSITION => Self::LockPosition(LockPositionInstruction::try_from_slice(payload)?),
            PERMANENT_LOCK_POSITION => Self::PermanentLockPosition(PermanentLockPositionInstruction::try_from_slice(payload)?),
            REFRESH_VESTING => Self::RefreshVesting,
            REMOVE_ALL_LIQUIDITY => Self::RemoveAllLiquidity(RemoveAllLiquidityInstruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY => Self::RemoveLiquidity(RemoveLiquidityInstruction::try_from_slice(payload)?),
            SET_POOL_STATUS => Self::SetPoolStatus(SetPoolStatusInstruction::try_from_slice(payload)?),
            SPLIT_POSITION => Self::SplitPosition(SplitPositionInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            UPDATE_REWARD_DURATION => Self::UpdateRewardDuration(UpdateRewardDurationInstruction::try_from_slice(payload)?),
            UPDATE_REWARD_FUNDER => Self::UpdateRewardFunder(UpdateRewardFunderInstruction::try_from_slice(payload)?),
            WITHDRAW_INELIGIBLE_REWARD => Self::WithdrawIneligibleReward(WithdrawIneligibleRewardInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<MeteoraDammInstruction, ParseError> {
    MeteoraDammInstruction::try_from(data)
}
