//! Meteora DLMM on-chain instructions.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum AccountsType {
    TransferHookX,
    TransferHookY,
    TransferHookReward,
    TransferHookMultiReward(u8),
}

/// Type of the activation
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum ActivationType {
    Slot,
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityParams {
    pub min_delta_id: i32,

    pub max_delta_id: i32,

    pub x0: u64,

    pub y0: u64,

    pub delta_x: u64,

    pub delta_y: u64,

    pub bit_flag: u8,

    pub favor_x_in_active_id: bool,

    pub padding: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquiditySingleSidePreciseParameter {
    pub bins: Vec<CompressedBinDepositAmount>,

    pub decompress_multiplier: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquiditySingleSidePreciseParameter2 {
    pub bins: Vec<CompressedBinDepositAmount>,

    pub decompress_multiplier: u64,

    pub max_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BaseFeeParameter {
    /// Portion of swap fees retained by the protocol by controlling protocol_share parameter. protocol_swap_fee = protocol_share * total_swap_fee
    pub protocol_share: u16,
    /// Base factor for base fee rate
    pub base_factor: u16,
    /// Base fee power factor
    pub base_fee_power_factor: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BinLiquidityDistribution {
    /// Define the bin ID wish to deposit to.
    pub bin_id: i32,
    /// DistributionX (or distributionY) is the percentages of amountX (or amountY) you want to add to each bin.
    pub distribution_x: u16,
    /// DistributionX (or distributionY) is the percentages of amountX (or amountY) you want to add to each bin.
    pub distribution_y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BinLiquidityDistributionByWeight {
    /// Define the bin ID wish to deposit to.
    pub bin_id: i32,
    /// weight of liquidity distributed for this bin id
    pub weight: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BinLiquidityReduction {
    pub bin_id: i32,

    pub bps_to_remove: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CompressedBinDepositAmount {
    pub bin_id: i32,

    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CustomizableParams {
    /// Pool price
    pub active_id: i32,
    /// Bin step
    pub bin_step: u16,
    /// Base factor
    pub base_factor: u16,
    /// Activation type. 0 = Slot, 1 = Time. Check ActivationType enum
    pub activation_type: u8,
    /// Whether the pool has an alpha vault
    pub has_alpha_vault: bool,
    /// Decide when does the pool start trade. None = Now
    pub activation_point: Option<u64>,
    /// Pool creator have permission to enable/disable pool with restricted program validation. Only applicable for customizable permissionless pool.
    pub creator_pool_on_off_control: bool,
    /// Base fee power factor
    pub base_fee_power_factor: u8,
    /// Padding, for future use
    pub padding: [u8; 62],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DummyIx {
    pub _pair_status: PairStatus,

    pub _pair_type: PairType,

    pub _activation_type: ActivationType,

    pub _token_program_flag: TokenProgramFlags,

    pub _resize_side: ResizeSide,

    pub _rounding: Rounding,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DynamicFeeParameter {
    /// Filter period determine high frequency trading time window.
    pub filter_period: u16,
    /// Decay period determine when the volatile fee start decay / decrease.
    pub decay_period: u16,
    /// Reduction factor controls the volatile fee rate decrement rate.
    pub reduction_factor: u16,
    /// Used to scale the variable fee component depending on the dynamic of the market
    pub variable_fee_control: u32,
    /// Maximum number of bin crossed can be accumulated. Used to cap volatile fee rate.
    pub max_volatility_accumulator: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitPermissionPairIx {
    pub active_id: i32,

    pub bin_step: u16,

    pub base_factor: u16,

    pub base_fee_power_factor: u8,

    pub activation_type: u8,

    pub protocol_share: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitPresetParameters2Ix {
    pub index: u16,
    /// Bin step. Represent the price increment / decrement.
    pub bin_step: u16,
    /// Used for base fee calculation. base_fee_rate = base_factor * bin_step * 10 * 10^base_fee_power_factor
    pub base_factor: u16,
    /// Filter period determine high frequency trading time window.
    pub filter_period: u16,
    /// Decay period determine when the volatile fee start decay / decrease.
    pub decay_period: u16,
    /// Reduction factor controls the volatile fee rate decrement rate.
    pub reduction_factor: u16,
    /// Used to scale the variable fee component depending on the dynamic of the market
    pub variable_fee_control: u32,
    /// Maximum number of bin crossed can be accumulated. Used to cap volatile fee rate.
    pub max_volatility_accumulator: u32,
    /// Portion of swap fees retained by the protocol by controlling protocol_share parameter. protocol_swap_fee = protocol_share * total_swap_fee
    pub protocol_share: u16,
    /// Base fee power factor
    pub base_fee_power_factor: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitPresetParametersIx {
    /// Bin step. Represent the price increment / decrement.
    pub bin_step: u16,
    /// Used for base fee calculation. base_fee_rate = base_factor * bin_step * 10 * 10^base_fee_power_factor
    pub base_factor: u16,
    /// Filter period determine high frequency trading time window.
    pub filter_period: u16,
    /// Decay period determine when the volatile fee start decay / decrease.
    pub decay_period: u16,
    /// Reduction factor controls the volatile fee rate decrement rate.
    pub reduction_factor: u16,
    /// Used to scale the variable fee component depending on the dynamic of the market
    pub variable_fee_control: u32,
    /// Maximum number of bin crossed can be accumulated. Used to cap volatile fee rate.
    pub max_volatility_accumulator: u32,
    /// Portion of swap fees retained by the protocol by controlling protocol_share parameter. protocol_swap_fee = protocol_share * total_swap_fee
    pub protocol_share: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeLbPair2Params {
    /// Pool price
    pub active_id: i32,
    /// Padding, for future use
    pub padding: [u8; 96],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityOneSideParameter {
    /// Amount of X token or Y token to deposit
    pub amount: u64,
    /// Active bin that integrator observe off-chain
    pub active_id: i32,
    /// max active bin slippage allowed
    pub max_active_bin_slippage: i32,
    /// Liquidity distribution to each bins
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityParameter {
    /// Amount of X token to deposit
    pub amount_x: u64,
    /// Amount of Y token to deposit
    pub amount_y: u64,
    /// Liquidity distribution to each bins
    pub bin_liquidity_dist: Vec<BinLiquidityDistribution>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityParameterByStrategy {
    /// Amount of X token to deposit
    pub amount_x: u64,
    /// Amount of Y token to deposit
    pub amount_y: u64,
    /// Active bin that integrator observe off-chain
    pub active_id: i32,
    /// max active bin slippage allowed
    pub max_active_bin_slippage: i32,
    /// strategy parameters
    pub strategy_parameters: StrategyParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityParameterByStrategyOneSide {
    /// Amount of X token or Y token to deposit
    pub amount: u64,
    /// Active bin that integrator observe off-chain
    pub active_id: i32,
    /// max active bin slippage allowed
    pub max_active_bin_slippage: i32,
    /// strategy parameters
    pub strategy_parameters: StrategyParameters,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityParameterByWeight {
    /// Amount of X token to deposit
    pub amount_x: u64,
    /// Amount of Y token to deposit
    pub amount_y: u64,
    /// Active bin that integrator observe off-chain
    pub active_id: i32,
    /// max active bin slippage allowed
    pub max_active_bin_slippage: i32,
    /// Liquidity distribution to each bins
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}

/// Pair status. 0 = Enabled, 1 = Disabled. Putting 0 as enabled for backward compatibility.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum PairStatus {
    Enabled,
    Disabled,
}

/// Type of the Pair. 0 = Permissionless, 1 = Permission, 2 = CustomizablePermissionless. Putting 0 as permissionless for backward compatibility.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum PairType {
    Permissionless,
    Permission,
    CustomizablePermissionless,
    PermissionlessV2,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RebalanceLiquidityParams {
    /// active id
    pub active_id: i32,
    /// max active bin slippage allowed
    pub max_active_bin_slippage: u16,
    /// a flag to indicate that whether fee should be harvested
    pub should_claim_fee: bool,
    /// a flag to indicate that whether rewards should be harvested
    pub should_claim_reward: bool,
    /// threshold for withdraw token x
    pub min_withdraw_x_amount: u64,
    /// threshold for deposit token x
    pub max_deposit_x_amount: u64,
    /// threshold for withdraw token y
    pub min_withdraw_y_amount: u64,
    /// threshold for deposit token y
    pub max_deposit_y_amount: u64,
    /// padding 32 bytes for future usage
    pub padding: [u8; 32],
    /// removes
    pub removes: Vec<RemoveLiquidityParams>,
    /// adds
    pub adds: Vec<AddLiquidityParams>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,

    pub length: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityParams {
    pub min_bin_id: Option<i32>,

    pub max_bin_id: Option<i32>,

    pub bps: u16,

    pub padding: [u8; 16],
}

/// Side of resize, 0 for lower and 1 for upper
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum ResizeSide {
    Lower,
    Upper,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Rounding {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct StrategyParameters {
    /// min bin id
    pub min_bin_id: i32,
    /// max bin id
    pub max_bin_id: i32,
    /// strategy type
    pub strategy_type: StrategyType,
    /// parameters
    pub parameteres: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum StrategyType {
    SpotOneSide,
    CurveOneSide,
    BidAskOneSide,
    SpotBalanced,
    CurveBalanced,
    BidAskBalanced,
    SpotImBalanced,
    CurveImBalanced,
    BidAskImBalanced,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum TokenProgramFlags {
    TokenProgram,
    TokenProgram2022,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const ADD_LIQUIDITY: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
pub const ADD_LIQUIDITY2: [u8; 8] = [228, 162, 78, 28, 70, 219, 116, 115];
pub const ADD_LIQUIDITY_BY_STRATEGY: [u8; 8] = [7, 3, 150, 127, 148, 40, 61, 200];
pub const ADD_LIQUIDITY_BY_STRATEGY2: [u8; 8] = [3, 221, 149, 218, 111, 141, 118, 213];
pub const ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE: [u8; 8] = [41, 5, 238, 175, 100, 225, 6, 205];
pub const ADD_LIQUIDITY_BY_WEIGHT: [u8; 8] = [28, 140, 238, 99, 231, 162, 21, 149];
pub const ADD_LIQUIDITY_ONE_SIDE: [u8; 8] = [94, 155, 103, 151, 70, 95, 220, 165];
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE: [u8; 8] = [161, 194, 103, 84, 171, 71, 250, 154];
pub const ADD_LIQUIDITY_ONE_SIDE_PRECISE2: [u8; 8] = [33, 51, 163, 201, 117, 98, 125, 231];
pub const CLAIM_FEE: [u8; 8] = [169, 32, 79, 137, 136, 232, 70, 137];
pub const CLAIM_FEE2: [u8; 8] = [112, 191, 101, 171, 28, 144, 127, 187];
pub const CLAIM_REWARD: [u8; 8] = [149, 95, 181, 242, 94, 90, 158, 162];
pub const CLAIM_REWARD2: [u8; 8] = [190, 3, 127, 119, 178, 87, 157, 183];
pub const CLOSE_CLAIM_PROTOCOL_FEE_OPERATOR: [u8; 8] = [8, 41, 87, 35, 80, 48, 121, 26];
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const CLOSE_POSITION2: [u8; 8] = [174, 90, 35, 115, 186, 40, 147, 226];
pub const CLOSE_POSITION_IF_EMPTY: [u8; 8] = [59, 124, 212, 118, 91, 152, 110, 157];
pub const CLOSE_PRESET_PARAMETER: [u8; 8] = [4, 148, 145, 100, 134, 26, 181, 61];
pub const CLOSE_PRESET_PARAMETER2: [u8; 8] = [39, 25, 95, 107, 116, 17, 115, 28];
pub const CREATE_CLAIM_PROTOCOL_FEE_OPERATOR: [u8; 8] = [51, 19, 150, 252, 105, 157, 48, 91];
pub const DECREASE_POSITION_LENGTH: [u8; 8] = [194, 219, 136, 32, 25, 96, 105, 37];
pub const FOR_IDL_TYPE_GENERATION_DO_NOT_CALL: [u8; 8] = [180, 105, 69, 80, 95, 50, 73, 108];
pub const FUND_REWARD: [u8; 8] = [188, 50, 249, 165, 93, 151, 38, 63];
pub const GO_TO_A_BIN: [u8; 8] = [146, 72, 174, 224, 40, 253, 84, 174];
pub const INCREASE_ORACLE_LENGTH: [u8; 8] = [190, 61, 125, 87, 103, 79, 158, 173];
pub const INCREASE_POSITION_LENGTH: [u8; 8] = [80, 83, 117, 211, 66, 13, 33, 149];
pub const INITIALIZE_BIN_ARRAY: [u8; 8] = [35, 86, 19, 185, 78, 212, 75, 211];
pub const INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION: [u8; 8] = [47, 157, 226, 180, 12, 240, 33, 71];
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR: [u8; 8] = [46, 39, 41, 135, 111, 183, 200, 64];
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2: [u8; 8] = [243, 73, 129, 126, 51, 19, 241, 107];
pub const INITIALIZE_LB_PAIR: [u8; 8] = [45, 154, 237, 210, 221, 15, 166, 92];
pub const INITIALIZE_LB_PAIR2: [u8; 8] = [73, 59, 36, 120, 237, 83, 108, 198];
pub const INITIALIZE_PERMISSION_LB_PAIR: [u8; 8] = [108, 102, 213, 85, 251, 3, 53, 21];
pub const INITIALIZE_POSITION: [u8; 8] = [219, 192, 234, 71, 190, 191, 102, 80];
pub const INITIALIZE_POSITION_BY_OPERATOR: [u8; 8] = [251, 189, 190, 244, 117, 254, 35, 148];
pub const INITIALIZE_POSITION_PDA: [u8; 8] = [46, 82, 125, 146, 85, 141, 228, 153];
pub const INITIALIZE_PRESET_PARAMETER: [u8; 8] = [66, 188, 71, 211, 98, 109, 14, 186];
pub const INITIALIZE_PRESET_PARAMETER2: [u8; 8] = [184, 7, 240, 171, 103, 47, 183, 121];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const INITIALIZE_TOKEN_BADGE: [u8; 8] = [253, 77, 205, 95, 27, 224, 89, 223];
pub const MIGRATE_BIN_ARRAY: [u8; 8] = [17, 23, 159, 211, 101, 184, 41, 241];
pub const MIGRATE_POSITION: [u8; 8] = [15, 132, 59, 50, 199, 6, 251, 46];
pub const REBALANCE_LIQUIDITY: [u8; 8] = [92, 4, 176, 193, 119, 185, 83, 9];
pub const REMOVE_ALL_LIQUIDITY: [u8; 8] = [10, 51, 61, 35, 112, 105, 24, 85];
pub const REMOVE_LIQUIDITY: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
pub const REMOVE_LIQUIDITY2: [u8; 8] = [230, 215, 82, 127, 241, 101, 227, 146];
pub const REMOVE_LIQUIDITY_BY_RANGE: [u8; 8] = [26, 82, 102, 152, 240, 74, 105, 26];
pub const REMOVE_LIQUIDITY_BY_RANGE2: [u8; 8] = [204, 2, 195, 145, 53, 145, 145, 205];
pub const SET_ACTIVATION_POINT: [u8; 8] = [91, 249, 15, 165, 26, 129, 254, 125];
pub const SET_PAIR_STATUS: [u8; 8] = [67, 248, 231, 137, 154, 149, 217, 174];
pub const SET_PAIR_STATUS_PERMISSIONLESS: [u8; 8] = [78, 59, 152, 211, 70, 183, 46, 208];
pub const SET_PRE_ACTIVATION_DURATION: [u8; 8] = [165, 61, 201, 244, 130, 159, 22, 100];
pub const SET_PRE_ACTIVATION_SWAP_ADDRESS: [u8; 8] = [57, 139, 47, 123, 216, 80, 223, 10];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP2: [u8; 8] = [65, 75, 63, 76, 235, 91, 91, 136];
pub const SWAP_EXACT_OUT: [u8; 8] = [250, 73, 101, 33, 38, 207, 75, 184];
pub const SWAP_EXACT_OUT2: [u8; 8] = [43, 215, 247, 132, 137, 60, 243, 81];
pub const SWAP_WITH_PRICE_IMPACT: [u8; 8] = [56, 173, 230, 208, 173, 228, 156, 205];
pub const SWAP_WITH_PRICE_IMPACT2: [u8; 8] = [74, 98, 192, 214, 177, 51, 75, 51];
pub const UPDATE_BASE_FEE_PARAMETERS: [u8; 8] = [75, 168, 223, 161, 16, 195, 3, 47];
pub const UPDATE_DYNAMIC_FEE_PARAMETERS: [u8; 8] = [92, 161, 46, 246, 255, 189, 22, 22];
pub const UPDATE_FEES_AND_REWARD2: [u8; 8] = [32, 142, 184, 154, 103, 65, 184, 88];
pub const UPDATE_FEES_AND_REWARDS: [u8; 8] = [154, 230, 250, 13, 236, 209, 75, 223];
pub const UPDATE_POSITION_OPERATOR: [u8; 8] = [202, 184, 103, 143, 180, 191, 116, 217];
pub const UPDATE_REWARD_DURATION: [u8; 8] = [138, 174, 196, 169, 213, 235, 254, 107];
pub const UPDATE_REWARD_FUNDER: [u8; 8] = [211, 28, 48, 32, 215, 160, 35, 23];
pub const WITHDRAW_INELIGIBLE_REWARD: [u8; 8] = [148, 206, 42, 195, 247, 49, 103, 8];
pub const WITHDRAW_PROTOCOL_FEE: [u8; 8] = [158, 201, 158, 189, 33, 93, 162, 103];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeteoraDllmInstruction {
    AddLiquidity(AddLiquidityInstruction),
    AddLiquidity2(AddLiquidity2Instruction),
    AddLiquidityByStrategy(AddLiquidityByStrategyInstruction),
    AddLiquidityByStrategy2(AddLiquidityByStrategy2Instruction),
    AddLiquidityByStrategyOneSide(AddLiquidityByStrategyOneSideInstruction),
    AddLiquidityByWeight(AddLiquidityByWeightInstruction),
    AddLiquidityOneSide(AddLiquidityOneSideInstruction),
    AddLiquidityOneSidePrecise(AddLiquidityOneSidePreciseInstruction),
    AddLiquidityOneSidePrecise2(AddLiquidityOneSidePrecise2Instruction),
    ClaimFee,
    ClaimFee2(ClaimFee2Instruction),
    ClaimReward(ClaimRewardInstruction),
    ClaimReward2(ClaimReward2Instruction),
    CloseClaimProtocolFeeOperator,
    ClosePosition,
    ClosePosition2,
    ClosePositionIfEmpty,
    ClosePresetParameter,
    ClosePresetParameter2,
    CreateClaimProtocolFeeOperator,
    DecreasePositionLength(DecreasePositionLengthInstruction),
    ForIdlTypeGenerationDoNotCall(ForIdlTypeGenerationDoNotCallInstruction),
    FundReward(FundRewardInstruction),
    GoToABin(GoToABinInstruction),
    IncreaseOracleLength(IncreaseOracleLengthInstruction),
    IncreasePositionLength(IncreasePositionLengthInstruction),
    InitializeBinArray(InitializeBinArrayInstruction),
    InitializeBinArrayBitmapExtension,
    InitializeCustomizablePermissionlessLbPair(InitializeCustomizablePermissionlessLbPairInstruction),
    InitializeCustomizablePermissionlessLbPair2(InitializeCustomizablePermissionlessLbPair2Instruction),
    InitializeLbPair(InitializeLbPairInstruction),
    InitializeLbPair2(InitializeLbPair2Instruction),
    InitializePermissionLbPair(InitializePermissionLbPairInstruction),
    InitializePosition(InitializePositionInstruction),
    InitializePositionByOperator(InitializePositionByOperatorInstruction),
    InitializePositionPda(InitializePositionPdaInstruction),
    InitializePresetParameter(InitializePresetParameterInstruction),
    InitializePresetParameter2(InitializePresetParameter2Instruction),
    InitializeReward(InitializeRewardInstruction),
    InitializeTokenBadge,
    MigrateBinArray,
    MigratePosition,
    RebalanceLiquidity(RebalanceLiquidityInstruction),
    RemoveAllLiquidity,
    RemoveLiquidity(RemoveLiquidityInstruction),
    RemoveLiquidity2(RemoveLiquidity2Instruction),
    RemoveLiquidityByRange(RemoveLiquidityByRangeInstruction),
    RemoveLiquidityByRange2(RemoveLiquidityByRange2Instruction),
    SetActivationPoint(SetActivationPointInstruction),
    SetPairStatus(SetPairStatusInstruction),
    SetPairStatusPermissionless(SetPairStatusPermissionlessInstruction),
    SetPreActivationDuration(SetPreActivationDurationInstruction),
    SetPreActivationSwapAddress(SetPreActivationSwapAddressInstruction),
    Swap(SwapInstruction),
    Swap2(Swap2Instruction),
    SwapExactOut(SwapExactOutInstruction),
    SwapExactOut2(SwapExactOut2Instruction),
    SwapWithPriceImpact(SwapWithPriceImpactInstruction),
    SwapWithPriceImpact2(SwapWithPriceImpact2Instruction),
    UpdateBaseFeeParameters(UpdateBaseFeeParametersInstruction),
    UpdateDynamicFeeParameters(UpdateDynamicFeeParametersInstruction),
    UpdateFeesAndReward2(UpdateFeesAndReward2Instruction),
    UpdateFeesAndRewards,
    UpdatePositionOperator(UpdatePositionOperatorInstruction),
    UpdateRewardDuration(UpdateRewardDurationInstruction),
    UpdateRewardFunder(UpdateRewardFunderInstruction),
    WithdrawIneligibleReward(WithdrawIneligibleRewardInstruction),
    WithdrawProtocolFee(WithdrawProtocolFeeInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityInstruction {
    pub liquidity_parameter: LiquidityParameter,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidity2Instruction {
    pub liquidity_parameter: LiquidityParameter,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityByStrategyInstruction {
    pub liquidity_parameter: LiquidityParameterByStrategy,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityByStrategy2Instruction {
    pub liquidity_parameter: LiquidityParameterByStrategy,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityByStrategyOneSideInstruction {
    pub liquidity_parameter: LiquidityParameterByStrategyOneSide,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityByWeightInstruction {
    pub liquidity_parameter: LiquidityParameterByWeight,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityOneSideInstruction {
    pub liquidity_parameter: LiquidityOneSideParameter,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityOneSidePreciseInstruction {
    pub parameter: AddLiquiditySingleSidePreciseParameter,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityOneSidePrecise2Instruction {
    pub liquidity_parameter: AddLiquiditySingleSidePreciseParameter2,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimFee2Instruction {
    pub min_bin_id: i32,

    pub max_bin_id: i32,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimRewardInstruction {
    pub reward_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimReward2Instruction {
    pub reward_index: u64,

    pub min_bin_id: i32,

    pub max_bin_id: i32,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DecreasePositionLengthInstruction {
    pub length_to_remove: u16,

    pub side: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ForIdlTypeGenerationDoNotCallInstruction {
    pub _ix: DummyIx,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FundRewardInstruction {
    pub reward_index: u64,

    pub amount: u64,

    pub carry_forward: bool,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct GoToABinInstruction {
    pub bin_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct IncreaseOracleLengthInstruction {
    pub length_to_add: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct IncreasePositionLengthInstruction {
    pub length_to_add: u16,

    pub side: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeBinArrayInstruction {
    pub index: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeCustomizablePermissionlessLbPairInstruction {
    pub params: CustomizableParams,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeCustomizablePermissionlessLbPair2Instruction {
    pub params: CustomizableParams,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeLbPairInstruction {
    pub active_id: i32,

    pub bin_step: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeLbPair2Instruction {
    pub params: InitializeLbPair2Params,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePermissionLbPairInstruction {
    pub ix_data: InitPermissionPairIx,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePositionInstruction {
    pub lower_bin_id: i32,

    pub width: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePositionByOperatorInstruction {
    pub lower_bin_id: i32,

    pub width: i32,

    pub fee_owner: Pubkey,

    pub lock_release_point: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePositionPdaInstruction {
    pub lower_bin_id: i32,

    pub width: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePresetParameterInstruction {
    pub ix: InitPresetParametersIx,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePresetParameter2Instruction {
    pub ix: InitPresetParameters2Ix,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeRewardInstruction {
    pub reward_index: u64,

    pub reward_duration: u64,

    pub funder: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RebalanceLiquidityInstruction {
    pub params: RebalanceLiquidityParams,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityInstruction {
    pub bin_liquidity_removal: Vec<BinLiquidityReduction>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidity2Instruction {
    pub bin_liquidity_removal: Vec<BinLiquidityReduction>,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityByRangeInstruction {
    pub from_bin_id: i32,

    pub to_bin_id: i32,

    pub bps_to_remove: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityByRange2Instruction {
    pub from_bin_id: i32,

    pub to_bin_id: i32,

    pub bps_to_remove: u16,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetActivationPointInstruction {
    pub activation_point: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetPairStatusInstruction {
    pub status: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetPairStatusPermissionlessInstruction {
    pub status: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetPreActivationDurationInstruction {
    pub pre_activation_duration: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetPreActivationSwapAddressInstruction {
    pub pre_activation_swap_address: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub amount_in: u64,

    pub min_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Swap2Instruction {
    pub amount_in: u64,

    pub min_amount_out: u64,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapExactOutInstruction {
    pub max_in_amount: u64,

    pub out_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapExactOut2Instruction {
    pub max_in_amount: u64,

    pub out_amount: u64,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapWithPriceImpactInstruction {
    pub amount_in: u64,

    pub active_id: Option<i32>,

    pub max_price_impact_bps: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapWithPriceImpact2Instruction {
    pub amount_in: u64,

    pub active_id: Option<i32>,

    pub max_price_impact_bps: u16,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateBaseFeeParametersInstruction {
    pub fee_parameter: BaseFeeParameter,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateDynamicFeeParametersInstruction {
    pub fee_parameter: DynamicFeeParameter,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateFeesAndReward2Instruction {
    pub min_bin_id: i32,

    pub max_bin_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdatePositionOperatorInstruction {
    pub operator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateRewardDurationInstruction {
    pub reward_index: u64,

    pub new_duration: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateRewardFunderInstruction {
    pub reward_index: u64,

    pub new_funder: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawIneligibleRewardInstruction {
    pub reward_index: u64,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawProtocolFeeInstruction {
    pub max_amount_x: u64,

    pub max_amount_y: u64,

    pub remaining_accounts_info: RemainingAccountsInfo,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for MeteoraDllmInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            ADD_LIQUIDITY => Self::AddLiquidity(AddLiquidityInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY2 => Self::AddLiquidity2(AddLiquidity2Instruction::try_from_slice(payload)?),
            ADD_LIQUIDITY_BY_STRATEGY => Self::AddLiquidityByStrategy(AddLiquidityByStrategyInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY_BY_STRATEGY2 => Self::AddLiquidityByStrategy2(AddLiquidityByStrategy2Instruction::try_from_slice(payload)?),
            ADD_LIQUIDITY_BY_STRATEGY_ONE_SIDE => Self::AddLiquidityByStrategyOneSide(AddLiquidityByStrategyOneSideInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY_BY_WEIGHT => Self::AddLiquidityByWeight(AddLiquidityByWeightInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY_ONE_SIDE => Self::AddLiquidityOneSide(AddLiquidityOneSideInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY_ONE_SIDE_PRECISE => Self::AddLiquidityOneSidePrecise(AddLiquidityOneSidePreciseInstruction::try_from_slice(payload)?),
            ADD_LIQUIDITY_ONE_SIDE_PRECISE2 => Self::AddLiquidityOneSidePrecise2(AddLiquidityOneSidePrecise2Instruction::try_from_slice(payload)?),
            CLAIM_FEE => Self::ClaimFee,
            CLAIM_FEE2 => Self::ClaimFee2(ClaimFee2Instruction::try_from_slice(payload)?),
            CLAIM_REWARD => Self::ClaimReward(ClaimRewardInstruction::try_from_slice(payload)?),
            CLAIM_REWARD2 => Self::ClaimReward2(ClaimReward2Instruction::try_from_slice(payload)?),
            CLOSE_CLAIM_PROTOCOL_FEE_OPERATOR => Self::CloseClaimProtocolFeeOperator,
            CLOSE_POSITION => Self::ClosePosition,
            CLOSE_POSITION2 => Self::ClosePosition2,
            CLOSE_POSITION_IF_EMPTY => Self::ClosePositionIfEmpty,
            CLOSE_PRESET_PARAMETER => Self::ClosePresetParameter,
            CLOSE_PRESET_PARAMETER2 => Self::ClosePresetParameter2,
            CREATE_CLAIM_PROTOCOL_FEE_OPERATOR => Self::CreateClaimProtocolFeeOperator,
            DECREASE_POSITION_LENGTH => Self::DecreasePositionLength(DecreasePositionLengthInstruction::try_from_slice(payload)?),
            FOR_IDL_TYPE_GENERATION_DO_NOT_CALL => Self::ForIdlTypeGenerationDoNotCall(ForIdlTypeGenerationDoNotCallInstruction::try_from_slice(payload)?),
            FUND_REWARD => Self::FundReward(FundRewardInstruction::try_from_slice(payload)?),
            GO_TO_A_BIN => Self::GoToABin(GoToABinInstruction::try_from_slice(payload)?),
            INCREASE_ORACLE_LENGTH => Self::IncreaseOracleLength(IncreaseOracleLengthInstruction::try_from_slice(payload)?),
            INCREASE_POSITION_LENGTH => Self::IncreasePositionLength(IncreasePositionLengthInstruction::try_from_slice(payload)?),
            INITIALIZE_BIN_ARRAY => Self::InitializeBinArray(InitializeBinArrayInstruction::try_from_slice(payload)?),
            INITIALIZE_BIN_ARRAY_BITMAP_EXTENSION => Self::InitializeBinArrayBitmapExtension,
            INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR => {
                Self::InitializeCustomizablePermissionlessLbPair(InitializeCustomizablePermissionlessLbPairInstruction::try_from_slice(payload)?)
            }
            INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_LB_PAIR2 => {
                Self::InitializeCustomizablePermissionlessLbPair2(InitializeCustomizablePermissionlessLbPair2Instruction::try_from_slice(payload)?)
            }
            INITIALIZE_LB_PAIR => Self::InitializeLbPair(InitializeLbPairInstruction::try_from_slice(payload)?),
            INITIALIZE_LB_PAIR2 => Self::InitializeLbPair2(InitializeLbPair2Instruction::try_from_slice(payload)?),
            INITIALIZE_PERMISSION_LB_PAIR => Self::InitializePermissionLbPair(InitializePermissionLbPairInstruction::try_from_slice(payload)?),
            INITIALIZE_POSITION => Self::InitializePosition(InitializePositionInstruction::try_from_slice(payload)?),
            INITIALIZE_POSITION_BY_OPERATOR => Self::InitializePositionByOperator(InitializePositionByOperatorInstruction::try_from_slice(payload)?),
            INITIALIZE_POSITION_PDA => Self::InitializePositionPda(InitializePositionPdaInstruction::try_from_slice(payload)?),
            INITIALIZE_PRESET_PARAMETER => Self::InitializePresetParameter(InitializePresetParameterInstruction::try_from_slice(payload)?),
            INITIALIZE_PRESET_PARAMETER2 => Self::InitializePresetParameter2(InitializePresetParameter2Instruction::try_from_slice(payload)?),
            INITIALIZE_REWARD => Self::InitializeReward(InitializeRewardInstruction::try_from_slice(payload)?),
            INITIALIZE_TOKEN_BADGE => Self::InitializeTokenBadge,
            MIGRATE_BIN_ARRAY => Self::MigrateBinArray,
            MIGRATE_POSITION => Self::MigratePosition,
            REBALANCE_LIQUIDITY => Self::RebalanceLiquidity(RebalanceLiquidityInstruction::try_from_slice(payload)?),
            REMOVE_ALL_LIQUIDITY => Self::RemoveAllLiquidity,
            REMOVE_LIQUIDITY => Self::RemoveLiquidity(RemoveLiquidityInstruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY2 => Self::RemoveLiquidity2(RemoveLiquidity2Instruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY_BY_RANGE => Self::RemoveLiquidityByRange(RemoveLiquidityByRangeInstruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY_BY_RANGE2 => Self::RemoveLiquidityByRange2(RemoveLiquidityByRange2Instruction::try_from_slice(payload)?),
            SET_ACTIVATION_POINT => Self::SetActivationPoint(SetActivationPointInstruction::try_from_slice(payload)?),
            SET_PAIR_STATUS => Self::SetPairStatus(SetPairStatusInstruction::try_from_slice(payload)?),
            SET_PAIR_STATUS_PERMISSIONLESS => Self::SetPairStatusPermissionless(SetPairStatusPermissionlessInstruction::try_from_slice(payload)?),
            SET_PRE_ACTIVATION_DURATION => Self::SetPreActivationDuration(SetPreActivationDurationInstruction::try_from_slice(payload)?),
            SET_PRE_ACTIVATION_SWAP_ADDRESS => Self::SetPreActivationSwapAddress(SetPreActivationSwapAddressInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            SWAP2 => Self::Swap2(Swap2Instruction::try_from_slice(payload)?),
            SWAP_EXACT_OUT => Self::SwapExactOut(SwapExactOutInstruction::try_from_slice(payload)?),
            SWAP_EXACT_OUT2 => Self::SwapExactOut2(SwapExactOut2Instruction::try_from_slice(payload)?),
            SWAP_WITH_PRICE_IMPACT => Self::SwapWithPriceImpact(SwapWithPriceImpactInstruction::try_from_slice(payload)?),
            SWAP_WITH_PRICE_IMPACT2 => Self::SwapWithPriceImpact2(SwapWithPriceImpact2Instruction::try_from_slice(payload)?),
            UPDATE_BASE_FEE_PARAMETERS => Self::UpdateBaseFeeParameters(UpdateBaseFeeParametersInstruction::try_from_slice(payload)?),
            UPDATE_DYNAMIC_FEE_PARAMETERS => Self::UpdateDynamicFeeParameters(UpdateDynamicFeeParametersInstruction::try_from_slice(payload)?),
            UPDATE_FEES_AND_REWARD2 => Self::UpdateFeesAndReward2(UpdateFeesAndReward2Instruction::try_from_slice(payload)?),
            UPDATE_FEES_AND_REWARDS => Self::UpdateFeesAndRewards,
            UPDATE_POSITION_OPERATOR => Self::UpdatePositionOperator(UpdatePositionOperatorInstruction::try_from_slice(payload)?),
            UPDATE_REWARD_DURATION => Self::UpdateRewardDuration(UpdateRewardDurationInstruction::try_from_slice(payload)?),
            UPDATE_REWARD_FUNDER => Self::UpdateRewardFunder(UpdateRewardFunderInstruction::try_from_slice(payload)?),
            WITHDRAW_INELIGIBLE_REWARD => Self::WithdrawIneligibleReward(WithdrawIneligibleRewardInstruction::try_from_slice(payload)?),
            WITHDRAW_PROTOCOL_FEE => Self::WithdrawProtocolFee(WithdrawProtocolFeeInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<MeteoraDllmInstruction, ParseError> {
    MeteoraDllmInstruction::try_from(data)
}
