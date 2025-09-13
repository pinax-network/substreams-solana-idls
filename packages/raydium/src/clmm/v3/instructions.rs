//! Raydium CLMM instructions.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeRewardParam {
    /// Reward open time
    pub open_time: u64,
    /// Reward end time
    pub end_time: u64,
    /// Token reward per second are earned per unit of liquidity
    pub emissions_per_second_x64: u128,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const CLOSE_POSITION: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];
pub const COLLECT_FUND_FEE: [u8; 8] = [167, 138, 78, 149, 223, 194, 6, 126];
pub const COLLECT_PROTOCOL_FEE: [u8; 8] = [136, 136, 252, 221, 194, 66, 126, 89];
pub const COLLECT_REMAINING_REWARDS: [u8; 8] = [18, 237, 166, 197, 34, 16, 213, 144];
pub const CREATE_AMM_CONFIG: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
pub const CREATE_OPERATION_ACCOUNT: [u8; 8] = [63, 87, 148, 33, 109, 35, 8, 104];
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const CREATE_SUPPORT_MINT_ASSOCIATED: [u8; 8] = [17, 251, 65, 92, 136, 242, 14, 169];
pub const DECREASE_LIQUIDITY: [u8; 8] = [160, 38, 208, 111, 104, 91, 44, 1];
pub const DECREASE_LIQUIDITY_V2: [u8; 8] = [58, 127, 188, 62, 79, 82, 196, 96];
pub const INCREASE_LIQUIDITY: [u8; 8] = [46, 156, 243, 118, 13, 205, 251, 178];
pub const INCREASE_LIQUIDITY_V2: [u8; 8] = [133, 29, 89, 223, 69, 238, 176, 10];
pub const INITIALIZE_REWARD: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];
pub const OPEN_POSITION: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];
pub const OPEN_POSITION_V2: [u8; 8] = [77, 184, 74, 214, 112, 86, 241, 199];
pub const OPEN_POSITION_WITH_TOKEN22_NFT: [u8; 8] = [77, 255, 174, 82, 125, 29, 201, 46];
pub const SET_REWARD_PARAMS: [u8; 8] = [112, 52, 167, 75, 32, 201, 211, 137];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP_ROUTER_BASE_IN: [u8; 8] = [69, 125, 115, 218, 245, 186, 242, 196];
pub const SWAP_V2: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];
pub const TRANSFER_REWARD_OWNER: [u8; 8] = [7, 22, 12, 83, 242, 43, 48, 121];
pub const UPDATE_AMM_CONFIG: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];
pub const UPDATE_OPERATION_ACCOUNT: [u8; 8] = [127, 70, 119, 40, 188, 227, 61, 7];
pub const UPDATE_POOL_STATUS: [u8; 8] = [130, 87, 108, 6, 46, 224, 117, 123];
pub const UPDATE_REWARD_INFOS: [u8; 8] = [163, 172, 224, 52, 11, 154, 106, 223];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumClmmInstruction {
    ClosePosition,
    CollectFundFee(CollectFundFeeInstruction),
    CollectProtocolFee(CollectProtocolFeeInstruction),
    CollectRemainingRewards(CollectRemainingRewardsInstruction),
    CreateAmmConfig(CreateAmmConfigInstruction),
    CreateOperationAccount,
    CreatePool(CreatePoolInstruction),
    CreateSupportMintAssociated,
    DecreaseLiquidity(DecreaseLiquidityInstruction),
    DecreaseLiquidityV2(DecreaseLiquidityV2Instruction),
    IncreaseLiquidity(IncreaseLiquidityInstruction),
    IncreaseLiquidityV2(IncreaseLiquidityV2Instruction),
    InitializeReward(InitializeRewardInstruction),
    OpenPosition(OpenPositionInstruction),
    OpenPositionV2(OpenPositionV2Instruction),
    OpenPositionWithToken22Nft(OpenPositionWithToken22NftInstruction),
    SetRewardParams(SetRewardParamsInstruction),
    Swap(SwapInstruction),
    SwapRouterBaseIn(SwapRouterBaseInInstruction),
    SwapV2(SwapInstruction),
    TransferRewardOwner(TransferRewardOwnerInstruction),
    UpdateAmmConfig(UpdateAmmConfigInstruction),
    UpdateOperationAccount(UpdateOperationAccountInstruction),
    UpdatePoolStatus(UpdatePoolStatusInstruction),
    UpdateRewardInfos,
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Collect the fund fee accrued to the pool
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `amount_0_requested` - The maximum amount of token_0 to send, can be 0 to collect fees in only token_1
/// * `amount_1_requested` - The maximum amount of token_1 to send, can be 0 to collect fees in only token_0
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectFundFeeInstruction {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// Collect the protocol fee accrued to the pool
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `amount_0_requested` - The maximum amount of token_0 to send, can be 0 to collect fees in only token_1
/// * `amount_1_requested` - The maximum amount of token_1 to send, can be 0 to collect fees in only token_0
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectProtocolFeeInstruction {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// Collect remaining reward token for reward founder
///
/// # Arguments
///
/// * `ctx`- The context of accounts
/// * `reward_index` - the index to reward info
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectRemainingRewardsInstruction {
    pub reward_index: u8,
}

/// # Arguments
///
/// * `ctx`- The accounts needed by instruction.
/// * `index` - The index of amm config, there may be multiple config.
/// * `tick_spacing` - The tickspacing binding with config, cannot be changed.
/// * `trade_fee_rate` - Trade fee rate, can be changed.
/// * `protocol_fee_rate` - The rate of protocol fee within trade fee.
/// * `fund_fee_rate` - The rate of fund fee within trade fee.
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateAmmConfigInstruction {
    pub index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

/// Creates a pool for the given token pair and the initial price
///
/// # Arguments
///
/// * `ctx`- The context of accounts
/// * `sqrt_price_x64` - the initial sqrt price (amount_token_1 / amount_token_0) of the pool as a Q64.64
///   Note: The open_time must be smaller than the current block_timestamp on chain.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreatePoolInstruction {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

/// #[deprecated(note = "Use `decrease_liquidity_v2` instead.")]
/// Decreases liquidity for an existing position
///
/// # Arguments
///
/// * `ctx` -  The context of accounts
/// * `liquidity` - The amount by which liquidity will be decreased
/// * `amount_0_min` - The minimum amount of token_0 that should be accounted for the burned liquidity
/// * `amount_1_min` - The minimum amount of token_1 that should be accounted for the burned liquidity
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DecreaseLiquidityInstruction {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

/// Decreases liquidity for an existing position, support Token2022
///
/// # Arguments
///
/// * `ctx` -  The context of accounts
/// * `liquidity` - The amount by which liquidity will be decreased
/// * `amount_0_min` - The minimum amount of token_0 that should be accounted for the burned liquidity
/// * `amount_1_min` - The minimum amount of token_1 that should be accounted for the burned liquidity
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DecreaseLiquidityV2Instruction {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

/// #[deprecated(note = "Use `increase_liquidity_v2` instead.")]
/// Increases liquidity for an existing position, with amount paid by `payer`
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `liquidity` - The desired liquidity to be added, can't be zero
/// * `amount_0_max` - The max amount of token_0 to spend, which serves as a slippage check
/// * `amount_1_max` - The max amount of token_1 to spend, which serves as a slippage check
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct IncreaseLiquidityInstruction {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
}

/// Increases liquidity for an existing position, with amount paid by `payer`, support Token2022
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `liquidity` - The desired liquidity to be added, if zero, calculate liquidity base amount_0 or amount_1 according base_flag
/// * `amount_0_max` - The max amount of token_0 to spend, which serves as a slippage check
/// * `amount_1_max` - The max amount of token_1 to spend, which serves as a slippage check
/// * `base_flag` - must be specified if liquidity is zero, true: calculate liquidity base amount_0_max otherwise base amount_1_max
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct IncreaseLiquidityV2Instruction {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub base_flag: Option<bool>,
}

/// Initialize a reward info for a given pool and reward index
///
/// # Arguments
///
/// * `ctx`- The context of accounts
/// * `reward_index` - the index to reward info
/// * `open_time` - reward open timestamp
/// * `end_time` - reward end timestamp
/// * `emissions_per_second_x64` - Token reward per second are earned per unit of liquidity.
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeRewardInstruction {
    pub param: InitializeRewardParam,
}

/// #[deprecated(note = "Use `open_position_with_token22_nft` instead.")]
/// Creates a new position wrapped in a NFT
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `tick_lower_index` - The low boundary of market
/// * `tick_upper_index` - The upper boundary of market
/// * `tick_array_lower_start_index` - The start index of tick array which include tick low
/// * `tick_array_upper_start_index` - The start index of tick array which include tick upper
/// * `liquidity` - The liquidity to be added
/// * `amount_0_max` - The max amount of token_0 to spend, which serves as a slippage check
/// * `amount_1_max` - The max amount of token_1 to spend, which serves as a slippage check
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionInstruction {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
}

/// #[deprecated(note = "Use `open_position_with_token22_nft` instead.")]
/// Creates a new position wrapped in a NFT, support Token2022
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `tick_lower_index` - The low boundary of market
/// * `tick_upper_index` - The upper boundary of market
/// * `tick_array_lower_start_index` - The start index of tick array which include tick low
/// * `tick_array_upper_start_index` - The start index of tick array which include tick upper
/// * `liquidity` - The liquidity to be added, if zero, and the base_flag is specified, calculate liquidity base amount_0_max or amount_1_max according base_flag, otherwise open position with zero liquidity
/// * `amount_0_max` - The max amount of token_0 to spend, which serves as a slippage check
/// * `amount_1_max` - The max amount of token_1 to spend, which serves as a slippage check
/// * `with_metadata` - The flag indicating whether to create NFT mint metadata
/// * `base_flag` - if the liquidity specified as zero, true: calculate liquidity base amount_0_max otherwise base amount_1_max
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionV2Instruction {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

/// Creates a new position wrapped in a Token2022 NFT without relying on metadata_program and metadata_account, reduce the cost for user to create a personal position.
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `tick_lower_index` - The low boundary of market
/// * `tick_upper_index` - The upper boundary of market
/// * `tick_array_lower_start_index` - The start index of tick array which include tick low
/// * `tick_array_upper_start_index` - The start index of tick array which include tick upper
/// * `liquidity` - The liquidity to be added, if zero, and the base_flag is specified, calculate liquidity base amount_0_max or amount_1_max according base_flag, otherwise open position with zero liquidity
/// * `amount_0_max` - The max amount of token_0 to spend, which serves as a slippage check
/// * `amount_1_max` - The max amount of token_1 to spend, which serves as a slippage check
/// * `with_metadata` - The flag indicating whether to create NFT mint metadata
/// * `base_flag` - if the liquidity specified as zero, true: calculate liquidity base amount_0_max otherwise base amount_1_max
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionWithToken22NftInstruction {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

/// Reset reward param, start a new reward cycle or extend the current cycle.
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `reward_index` - The index of reward token in the pool.
/// * `emissions_per_second_x64` - The per second emission reward, when extend the current cycle,
///   new value can't be less than old value
/// * `open_time` - reward open timestamp, must be set when starting a new cycle
/// * `end_time` - reward end timestamp
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetRewardParamsInstruction {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
    pub open_time: u64,
    pub end_time: u64,
}

/// #[deprecated(note = "Use `swap_v2` instead.")]
/// Swaps one token for as much as possible of another token across a single pool
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `amount` - Arranged in pairs with other_amount_threshold. (amount_in, amount_out_minimum) or (amount_out, amount_in_maximum)
/// * `other_amount_threshold` - For slippage check
/// * `sqrt_price_limit` - The Q64.64 sqrt price √P limit. If zero for one, the price cannot
/// * `is_base_input` - swap base input or swap base output
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapInstruction {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

/// Swap token for as much as possible of another token across the path provided, base input
///
/// # Arguments
///
/// * `ctx` - The context of accounts
/// * `amount_in` - Token amount to be swapped in
/// * `amount_out_minimum` - Panic if output amount is below minimum amount. For slippage.
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapRouterBaseInInstruction {
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}

/// Transfer reward owner
///
/// # Arguments
///
/// * `ctx`- The context of accounts
/// * `new_owner`- new owner pubkey
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TransferRewardOwnerInstruction {
    pub new_owner: Pubkey,
}

/// Updates the owner of the amm config
/// Must be called by the current owner or admin
///
/// # Arguments
///
/// * `ctx`- The context of accounts
/// * `trade_fee_rate`- The new trade fee rate of amm config, be set when `param` is 0
/// * `protocol_fee_rate`- The new protocol fee rate of amm config, be set when `param` is 1
/// * `fund_fee_rate`- The new fund fee rate of amm config, be set when `param` is 2
/// * `new_owner`- The config's new owner, be set when `param` is 3
/// * `new_fund_owner`- The config's new fund owner, be set when `param` is 4
/// * `param`- The value can be 0 | 1 | 2 | 3 | 4, otherwise will report a error
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateAmmConfigInstruction {
    pub param: u8,
    pub value: u32,
}

/// Update the operation account
///
/// # Arguments
///
/// * `ctx`- The context of accounts
/// * `param`- The value can be 0 | 1 | 2 | 3, otherwise will report a error
/// * `keys`- update operation owner when the `param` is 0
///   remove operation owner when the `param` is 1
///   update whitelist mint when the `param` is 2
///   remove whitelist mint when the `param` is 3
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateOperationAccountInstruction {
    pub param: u8,
    pub keys: Vec<Pubkey>,
}

/// Update pool status for given value
///
/// # Arguments
///
/// * `ctx`- The context of accounts
/// * `status` - The value of status
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdatePoolStatusInstruction {
    pub status: u8,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumClmmInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            CLOSE_POSITION => Self::ClosePosition,
            COLLECT_FUND_FEE => Self::CollectFundFee(CollectFundFeeInstruction::try_from_slice(payload)?),
            COLLECT_PROTOCOL_FEE => Self::CollectProtocolFee(CollectProtocolFeeInstruction::try_from_slice(payload)?),
            COLLECT_REMAINING_REWARDS => Self::CollectRemainingRewards(CollectRemainingRewardsInstruction::try_from_slice(payload)?),
            CREATE_AMM_CONFIG => Self::CreateAmmConfig(CreateAmmConfigInstruction::try_from_slice(payload)?),
            CREATE_OPERATION_ACCOUNT => Self::CreateOperationAccount,
            CREATE_POOL => Self::CreatePool(CreatePoolInstruction::try_from_slice(payload)?),
            CREATE_SUPPORT_MINT_ASSOCIATED => Self::CreateSupportMintAssociated,
            DECREASE_LIQUIDITY => Self::DecreaseLiquidity(DecreaseLiquidityInstruction::try_from_slice(payload)?),
            DECREASE_LIQUIDITY_V2 => Self::DecreaseLiquidityV2(DecreaseLiquidityV2Instruction::try_from_slice(payload)?),
            INCREASE_LIQUIDITY => Self::IncreaseLiquidity(IncreaseLiquidityInstruction::try_from_slice(payload)?),
            INCREASE_LIQUIDITY_V2 => Self::IncreaseLiquidityV2(IncreaseLiquidityV2Instruction::try_from_slice(payload)?),
            INITIALIZE_REWARD => Self::InitializeReward(InitializeRewardInstruction::try_from_slice(payload)?),
            OPEN_POSITION => Self::OpenPosition(OpenPositionInstruction::try_from_slice(payload)?),
            OPEN_POSITION_V2 => Self::OpenPositionV2(OpenPositionV2Instruction::try_from_slice(payload)?),
            OPEN_POSITION_WITH_TOKEN22_NFT => Self::OpenPositionWithToken22Nft(OpenPositionWithToken22NftInstruction::try_from_slice(payload)?),
            SET_REWARD_PARAMS => Self::SetRewardParams(SetRewardParamsInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            SWAP_ROUTER_BASE_IN => Self::SwapRouterBaseIn(SwapRouterBaseInInstruction::try_from_slice(payload)?),
            SWAP_V2 => Self::SwapV2(SwapInstruction::try_from_slice(payload)?),
            TRANSFER_REWARD_OWNER => Self::TransferRewardOwner(TransferRewardOwnerInstruction::try_from_slice(payload)?),
            UPDATE_AMM_CONFIG => Self::UpdateAmmConfig(UpdateAmmConfigInstruction::try_from_slice(payload)?),
            UPDATE_OPERATION_ACCOUNT => Self::UpdateOperationAccount(UpdateOperationAccountInstruction::try_from_slice(payload)?),
            UPDATE_POOL_STATUS => Self::UpdatePoolStatus(UpdatePoolStatusInstruction::try_from_slice(payload)?),
            UPDATE_REWARD_INFOS => Self::UpdateRewardInfos,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumClmmInstruction, ParseError> {
    RaydiumClmmInstruction::try_from(data)
}
