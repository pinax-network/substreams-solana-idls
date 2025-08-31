//! Orca Whirlpool on-chain instructions.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum LockType {
    Permanent,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum LockTypeLabel {
    Permanent,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AdaptiveFeeConstants {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
    pub reserved: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AdaptiveFeeVariables {
    pub last_reference_update_timestamp: u64,
    pub last_major_swap_timestamp: u64,
    pub volatility_reference: u32,
    pub tick_group_index_reference: i32,
    pub volatility_accumulator: u32,
    pub reserved: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionBumps {
    pub position_bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionWithMetadataBumps {
    pub position_bump: u8,
    pub metadata_bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PositionRewardInfo {
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Tick {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WhirlpoolRewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}

pub const INITIALIZE_CONFIG: [u8; 8] = [208, 127, 21, 1, 194, 190, 196, 70];
pub const INITIALIZE_POOL: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
pub const INITIALIZE_TICK_ARRAY: [u8; 8] = [11, 188, 193, 214, 141, 91, 149, 184];
pub const INITIALIZE_FEE_TIER: [u8; 8] = [183, 74, 156, 160, 112, 2, 42, 30];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const SET_REWARD_EMISSIONS: [u8; 8] = [13, 197, 86, 168, 109, 176, 27, 244];
pub const OPEN_POSITION: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];
pub const OPEN_POSITION_WITH_METADATA: [u8; 8] = [242, 29, 134, 48, 58, 110, 14, 60];
pub const INCREASE_LIQUIDITY: [u8; 8] = [46, 156, 243, 118, 13, 205, 251, 178];
pub const DECREASE_LIQUIDITY: [u8; 8] = [160, 38, 208, 111, 104, 91, 44, 1];
pub const UPDATE_FEES_AND_REWARDS: [u8; 8] = [154, 230, 250, 13, 236, 209, 75, 223];
pub const COLLECT_FEES: [u8; 8] = [164, 152, 207, 99, 30, 186, 19, 182];
pub const COLLECT_REWARD: [u8; 8] = [70, 5, 132, 87, 86, 235, 177, 34];
pub const COLLECT_PROTOCOL_FEES: [u8; 8] = [22, 67, 23, 98, 150, 178, 70, 220];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const SET_DEFAULT_FEE_RATE: [u8; 8] = [118, 215, 214, 157, 182, 229, 208, 228];
pub const SET_DEFAULT_PROTOCOL_FEE_RATE: [u8; 8] = [107, 205, 249, 226, 151, 35, 86, 0];
pub const SET_FEE_RATE: [u8; 8] = [53, 243, 137, 65, 8, 140, 158, 6];
pub const SET_PROTOCOL_FEE_RATE: [u8; 8] = [95, 7, 4, 50, 154, 79, 156, 131];
pub const SET_FEE_AUTHORITY: [u8; 8] = [31, 1, 50, 87, 237, 101, 97, 132];
pub const SET_COLLECT_PROTOCOL_FEES_AUTHORITY: [u8; 8] = [34, 150, 93, 244, 139, 225, 233, 67];
pub const SET_REWARD_AUTHORITY: [u8; 8] = [34, 39, 183, 252, 83, 28, 85, 127];
pub const SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY: [u8; 8] = [240, 154, 201, 198, 148, 93, 56, 25];
pub const SET_REWARD_EMISSIONS_SUPER_AUTHORITY: [u8; 8] = [207, 5, 200, 209, 122, 56, 82, 183];
pub const TWO_HOP_SWAP: [u8; 8] = [195, 96, 237, 108, 68, 162, 219, 230];
pub const INITIALIZE_POSITION_BUNDLE: [u8; 8] = [117, 45, 241, 149, 24, 18, 194, 65];
pub const INITIALIZE_POSITION_BUNDLE_WITH_METADATA: [u8; 8] = [93, 124, 16, 179, 249, 131, 115, 245];
pub const DELETE_POSITION_BUNDLE: [u8; 8] = [100, 25, 99, 2, 217, 239, 124, 173];
pub const OPEN_BUNDLED_POSITION: [u8; 8] = [169, 113, 126, 171, 213, 172, 212, 49];
pub const CLOSE_BUNDLED_POSITION: [u8; 8] = [41, 36, 216, 245, 27, 85, 103, 67];
pub const OPEN_POSITION_WITH_TOKEN_EXTENSIONS: [u8; 8] = [212, 47, 95, 92, 114, 102, 131, 250];
pub const CLOSE_POSITION_WITH_TOKEN_EXTENSIONS: [u8; 8] = [1, 182, 135, 59, 155, 25, 99, 223];
pub const LOCK_POSITION: [u8; 8] = [227, 62, 2, 252, 247, 10, 171, 185];
pub const RESET_POSITION_RANGE: [u8; 8] = [164, 123, 180, 141, 194, 100, 160, 175];
pub const TRANSFER_LOCKED_POSITION: [u8; 8] = [179, 121, 229, 46, 67, 138, 194, 138];
pub const INITIALIZE_ADAPTIVE_FEE_TIER: [u8; 8] = [77, 99, 208, 200, 141, 123, 117, 48];
pub const SET_DEFAULT_BASE_FEE_RATE: [u8; 8] = [229, 66, 84, 251, 164, 134, 183, 7];
pub const SET_DELEGATED_FEE_AUTHORITY: [u8; 8] = [193, 234, 231, 147, 138, 57, 3, 122];
pub const SET_INITIALIZE_POOL_AUTHORITY: [u8; 8] = [125, 43, 127, 235, 149, 26, 106, 236];
pub const SET_PRESET_ADAPTIVE_FEE_CONSTANTS: [u8; 8] = [132, 185, 66, 148, 83, 88, 134, 198];
pub const INITIALIZE_POOL_WITH_ADAPTIVE_FEE: [u8; 8] = [143, 94, 96, 76, 172, 124, 119, 199];
pub const SET_FEE_RATE_BY_DELEGATED_FEE_AUTHORITY: [u8; 8] = [121, 121, 54, 114, 131, 230, 162, 104];
pub const COLLECT_FEES_V2: [u8; 8] = [207, 117, 95, 191, 229, 180, 226, 15];
pub const COLLECT_PROTOCOL_FEES_V2: [u8; 8] = [103, 128, 222, 134, 114, 200, 22, 200];
pub const COLLECT_REWARD_V2: [u8; 8] = [177, 107, 37, 180, 160, 19, 49, 209];
pub const DECREASE_LIQUIDITY_V2: [u8; 8] = [58, 127, 188, 62, 79, 82, 196, 96];
pub const INCREASE_LIQUIDITY_V2: [u8; 8] = [133, 29, 89, 223, 69, 238, 176, 10];
pub const INITIALIZE_POOL_V2: [u8; 8] = [207, 45, 87, 242, 27, 63, 204, 67];
pub const INITIALIZE_REWARD_V2: [u8; 8] = [91, 1, 77, 50, 235, 229, 133, 49];
pub const SET_REWARD_EMISSIONS_V2: [u8; 8] = [114, 228, 72, 32, 193, 48, 160, 102];
pub const SWAP_V2: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];
pub const TWO_HOP_SWAP_V2: [u8; 8] = [186, 143, 209, 29, 254, 2, 194, 117];
pub const INITIALIZE_CONFIG_EXTENSION: [u8; 8] = [55, 9, 53, 9, 114, 57, 209, 52];
pub const SET_CONFIG_EXTENSION_AUTHORITY: [u8; 8] = [44, 94, 241, 116, 24, 188, 60, 143];
pub const SET_TOKEN_BADGE_AUTHORITY: [u8; 8] = [207, 202, 4, 32, 205, 79, 13, 178];
pub const INITIALIZE_TOKEN_BADGE: [u8; 8] = [253, 77, 205, 95, 27, 224, 89, 223];
pub const DELETE_TOKEN_BADGE: [u8; 8] = [53, 146, 68, 8, 18, 117, 17, 185];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhirlpoolInstruction {
    InitializeConfig(InitializeConfigInstruction),
    InitializePool(InitializePoolInstruction),
    InitializeTickArray(InitializeTickArrayInstruction),
    InitializeFeeTier(InitializeFeeTierInstruction),
    InitializeReward(InitializeRewardInstruction),
    SetRewardEmissions(SetRewardEmissionsInstruction),
    OpenPosition(OpenPositionInstruction),
    OpenPositionWithMetadata(OpenPositionWithMetadataInstruction),
    IncreaseLiquidity(IncreaseLiquidityInstruction),
    DecreaseLiquidity(DecreaseLiquidityInstruction),
    UpdateFeesAndRewards,
    CollectFees,
    CollectReward(CollectRewardInstruction),
    CollectProtocolFees,
    Swap(SwapInstruction),
    ClosePosition,
    SetDefaultFeeRate(SetDefaultFeeRateInstruction),
    SetDefaultProtocolFeeRate(SetDefaultProtocolFeeRateInstruction),
    SetFeeRate(SetFeeRateInstruction),
    SetProtocolFeeRate(SetProtocolFeeRateInstruction),
    SetFeeAuthority,
    SetCollectProtocolFeesAuthority,
    SetRewardAuthority(SetRewardAuthorityInstruction),
    SetRewardAuthorityBySuperAuthority(SetRewardAuthorityBySuperAuthorityInstruction),
    SetRewardEmissionsSuperAuthority,
    TwoHopSwap(TwoHopSwapInstruction),
    InitializePositionBundle,
    InitializePositionBundleWithMetadata,
    DeletePositionBundle,
    OpenBundledPosition(OpenBundledPositionInstruction),
    CloseBundledPosition(CloseBundledPositionInstruction),
    OpenPositionWithTokenExtensions(OpenPositionWithTokenExtensionsInstruction),
    ClosePositionWithTokenExtensions,
    LockPosition(LockPositionInstruction),
    ResetPositionRange(ResetPositionRangeInstruction),
    TransferLockedPosition,
    InitializeAdaptiveFeeTier(InitializeAdaptiveFeeTierInstruction),
    SetDefaultBaseFeeRate(SetDefaultBaseFeeRateInstruction),
    SetDelegatedFeeAuthority,
    SetInitializePoolAuthority,
    SetPresetAdaptiveFeeConstants(SetPresetAdaptiveFeeConstantsInstruction),
    InitializePoolWithAdaptiveFee(InitializePoolWithAdaptiveFeeInstruction),
    SetFeeRateByDelegatedFeeAuthority(SetFeeRateByDelegatedFeeAuthorityInstruction),
    CollectFeesV2(CollectFeesV2Instruction),
    CollectProtocolFeesV2(CollectProtocolFeesV2Instruction),
    CollectRewardV2(CollectRewardV2Instruction),
    DecreaseLiquidityV2(DecreaseLiquidityV2Instruction),
    IncreaseLiquidityV2(IncreaseLiquidityV2Instruction),
    InitializePoolV2(InitializePoolV2Instruction),
    InitializeRewardV2(InitializeRewardV2Instruction),
    SetRewardEmissionsV2(SetRewardEmissionsV2Instruction),
    SwapV2(SwapV2Instruction),
    TwoHopSwapV2(TwoHopSwapV2Instruction),
    InitializeConfigExtension,
    SetConfigExtensionAuthority,
    SetTokenBadgeAuthority,
    InitializeTokenBadge,
    DeleteTokenBadge,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeConfigInstruction {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePoolInstruction {
    pub bumps: WhirlpoolBumps,
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeTickArrayInstruction {
    pub start_tick_index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeFeeTierInstruction {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeRewardInstruction {
    pub reward_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetRewardEmissionsInstruction {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionInstruction {
    pub bumps: OpenPositionBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionWithMetadataInstruction {
    pub bumps: OpenPositionWithMetadataBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct IncreaseLiquidityInstruction {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DecreaseLiquidityInstruction {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectRewardInstruction {
    pub reward_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapInstruction {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetDefaultFeeRateInstruction {
    pub default_fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetDefaultProtocolFeeRateInstruction {
    pub default_protocol_fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetFeeRateInstruction {
    pub fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetProtocolFeeRateInstruction {
    pub protocol_fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetRewardAuthorityInstruction {
    pub reward_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetRewardAuthorityBySuperAuthorityInstruction {
    pub reward_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TwoHopSwapInstruction {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenBundledPositionInstruction {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CloseBundledPositionInstruction {
    pub bundle_index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionWithTokenExtensionsInstruction {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub with_token_metadata_extension: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct LockPositionInstruction {
    pub lock_type: LockType,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ResetPositionRangeInstruction {
    pub new_tick_lower_index: i32,
    pub new_tick_upper_index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeAdaptiveFeeTierInstruction {
    pub fee_tier_index: u16,
    pub tick_spacing: u16,
    pub initialize_pool_authority: Pubkey,
    pub delegated_fee_authority: Pubkey,
    pub default_base_fee_rate: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetDefaultBaseFeeRateInstruction {
    pub default_base_fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetPresetAdaptiveFeeConstantsInstruction {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePoolWithAdaptiveFeeInstruction {
    pub initial_sqrt_price: u128,
    pub trade_enable_timestamp: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetFeeRateByDelegatedFeeAuthorityInstruction {
    pub fee_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectFeesV2Instruction {
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectProtocolFeesV2Instruction {
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectRewardV2Instruction {
    pub reward_index: u8,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DecreaseLiquidityV2Instruction {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct IncreaseLiquidityV2Instruction {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePoolV2Instruction {
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeRewardV2Instruction {
    pub reward_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetRewardEmissionsV2Instruction {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapV2Instruction {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TwoHopSwapV2Instruction {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

impl<'a> TryFrom<&'a [u8]> for WhirlpoolInstruction {
    type Error = ParseError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            INITIALIZE_CONFIG => Self::InitializeConfig(InitializeConfigInstruction::try_from_slice(payload)?),
            INITIALIZE_POOL => Self::InitializePool(InitializePoolInstruction::try_from_slice(payload)?),
            INITIALIZE_TICK_ARRAY => Self::InitializeTickArray(InitializeTickArrayInstruction::try_from_slice(payload)?),
            INITIALIZE_FEE_TIER => Self::InitializeFeeTier(InitializeFeeTierInstruction::try_from_slice(payload)?),
            INITIALIZE_REWARD => Self::InitializeReward(InitializeRewardInstruction::try_from_slice(payload)?),
            SET_REWARD_EMISSIONS => Self::SetRewardEmissions(SetRewardEmissionsInstruction::try_from_slice(payload)?),
            OPEN_POSITION => Self::OpenPosition(OpenPositionInstruction::try_from_slice(payload)?),
            OPEN_POSITION_WITH_METADATA => Self::OpenPositionWithMetadata(OpenPositionWithMetadataInstruction::try_from_slice(payload)?),
            INCREASE_LIQUIDITY => Self::IncreaseLiquidity(IncreaseLiquidityInstruction::try_from_slice(payload)?),
            DECREASE_LIQUIDITY => Self::DecreaseLiquidity(DecreaseLiquidityInstruction::try_from_slice(payload)?),
            UPDATE_FEES_AND_REWARDS => Self::UpdateFeesAndRewards,
            COLLECT_FEES => Self::CollectFees,
            COLLECT_REWARD => Self::CollectReward(CollectRewardInstruction::try_from_slice(payload)?),
            COLLECT_PROTOCOL_FEES => Self::CollectProtocolFees,
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            CLOSE_POSITION => Self::ClosePosition,
            SET_DEFAULT_FEE_RATE => Self::SetDefaultFeeRate(SetDefaultFeeRateInstruction::try_from_slice(payload)?),
            SET_DEFAULT_PROTOCOL_FEE_RATE => Self::SetDefaultProtocolFeeRate(SetDefaultProtocolFeeRateInstruction::try_from_slice(payload)?),
            SET_FEE_RATE => Self::SetFeeRate(SetFeeRateInstruction::try_from_slice(payload)?),
            SET_PROTOCOL_FEE_RATE => Self::SetProtocolFeeRate(SetProtocolFeeRateInstruction::try_from_slice(payload)?),
            SET_FEE_AUTHORITY => Self::SetFeeAuthority,
            SET_COLLECT_PROTOCOL_FEES_AUTHORITY => Self::SetCollectProtocolFeesAuthority,
            SET_REWARD_AUTHORITY => Self::SetRewardAuthority(SetRewardAuthorityInstruction::try_from_slice(payload)?),
            SET_REWARD_AUTHORITY_BY_SUPER_AUTHORITY => {
                Self::SetRewardAuthorityBySuperAuthority(SetRewardAuthorityBySuperAuthorityInstruction::try_from_slice(payload)?)
            }
            SET_REWARD_EMISSIONS_SUPER_AUTHORITY => Self::SetRewardEmissionsSuperAuthority,
            TWO_HOP_SWAP => Self::TwoHopSwap(TwoHopSwapInstruction::try_from_slice(payload)?),
            INITIALIZE_POSITION_BUNDLE => Self::InitializePositionBundle,
            INITIALIZE_POSITION_BUNDLE_WITH_METADATA => Self::InitializePositionBundleWithMetadata,
            DELETE_POSITION_BUNDLE => Self::DeletePositionBundle,
            OPEN_BUNDLED_POSITION => Self::OpenBundledPosition(OpenBundledPositionInstruction::try_from_slice(payload)?),
            CLOSE_BUNDLED_POSITION => Self::CloseBundledPosition(CloseBundledPositionInstruction::try_from_slice(payload)?),
            OPEN_POSITION_WITH_TOKEN_EXTENSIONS => Self::OpenPositionWithTokenExtensions(OpenPositionWithTokenExtensionsInstruction::try_from_slice(payload)?),
            CLOSE_POSITION_WITH_TOKEN_EXTENSIONS => Self::ClosePositionWithTokenExtensions,
            LOCK_POSITION => Self::LockPosition(LockPositionInstruction::try_from_slice(payload)?),
            RESET_POSITION_RANGE => Self::ResetPositionRange(ResetPositionRangeInstruction::try_from_slice(payload)?),
            TRANSFER_LOCKED_POSITION => Self::TransferLockedPosition,
            INITIALIZE_ADAPTIVE_FEE_TIER => Self::InitializeAdaptiveFeeTier(InitializeAdaptiveFeeTierInstruction::try_from_slice(payload)?),
            SET_DEFAULT_BASE_FEE_RATE => Self::SetDefaultBaseFeeRate(SetDefaultBaseFeeRateInstruction::try_from_slice(payload)?),
            SET_DELEGATED_FEE_AUTHORITY => Self::SetDelegatedFeeAuthority,
            SET_INITIALIZE_POOL_AUTHORITY => Self::SetInitializePoolAuthority,
            SET_PRESET_ADAPTIVE_FEE_CONSTANTS => Self::SetPresetAdaptiveFeeConstants(SetPresetAdaptiveFeeConstantsInstruction::try_from_slice(payload)?),
            INITIALIZE_POOL_WITH_ADAPTIVE_FEE => Self::InitializePoolWithAdaptiveFee(InitializePoolWithAdaptiveFeeInstruction::try_from_slice(payload)?),
            SET_FEE_RATE_BY_DELEGATED_FEE_AUTHORITY => {
                Self::SetFeeRateByDelegatedFeeAuthority(SetFeeRateByDelegatedFeeAuthorityInstruction::try_from_slice(payload)?)
            }
            COLLECT_FEES_V2 => Self::CollectFeesV2(CollectFeesV2Instruction::try_from_slice(payload)?),
            COLLECT_PROTOCOL_FEES_V2 => Self::CollectProtocolFeesV2(CollectProtocolFeesV2Instruction::try_from_slice(payload)?),
            COLLECT_REWARD_V2 => Self::CollectRewardV2(CollectRewardV2Instruction::try_from_slice(payload)?),
            DECREASE_LIQUIDITY_V2 => Self::DecreaseLiquidityV2(DecreaseLiquidityV2Instruction::try_from_slice(payload)?),
            INCREASE_LIQUIDITY_V2 => Self::IncreaseLiquidityV2(IncreaseLiquidityV2Instruction::try_from_slice(payload)?),
            INITIALIZE_POOL_V2 => Self::InitializePoolV2(InitializePoolV2Instruction::try_from_slice(payload)?),
            INITIALIZE_REWARD_V2 => Self::InitializeRewardV2(InitializeRewardV2Instruction::try_from_slice(payload)?),
            SET_REWARD_EMISSIONS_V2 => Self::SetRewardEmissionsV2(SetRewardEmissionsV2Instruction::try_from_slice(payload)?),
            SWAP_V2 => Self::SwapV2(SwapV2Instruction::try_from_slice(payload)?),
            TWO_HOP_SWAP_V2 => Self::TwoHopSwapV2(TwoHopSwapV2Instruction::try_from_slice(payload)?),
            INITIALIZE_CONFIG_EXTENSION => Self::InitializeConfigExtension,
            SET_CONFIG_EXTENSION_AUTHORITY => Self::SetConfigExtensionAuthority,
            SET_TOKEN_BADGE_AUTHORITY => Self::SetTokenBadgeAuthority,
            INITIALIZE_TOKEN_BADGE => Self::InitializeTokenBadge,
            DELETE_TOKEN_BADGE => Self::DeleteTokenBadge,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}
pub fn unpack(data: &[u8]) -> Result<WhirlpoolInstruction, ParseError> {
    WhirlpoolInstruction::try_from(data)
}
