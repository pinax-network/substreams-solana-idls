//! Raydium CLMM events.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const COLLECT_PERSONAL_FEE_EVENT: [u8; 8] = [166, 174, 105, 192, 81, 161, 83, 105];
pub const COLLECT_PROTOCOL_FEE_EVENT: [u8; 8] = [206, 87, 17, 79, 45, 41, 213, 61];
pub const CONFIG_CHANGE_EVENT: [u8; 8] = [247, 189, 7, 119, 106, 112, 95, 151];
pub const CREATE_PERSONAL_POSITION_EVENT: [u8; 8] = [100, 30, 87, 249, 196, 223, 154, 206];
pub const DECREASE_LIQUIDITY_EVENT: [u8; 8] = [58, 222, 86, 58, 68, 50, 85, 56];
pub const INCREASE_LIQUIDITY_EVENT: [u8; 8] = [49, 79, 105, 212, 32, 34, 30, 84];
pub const LIQUIDITY_CALCULATE_EVENT: [u8; 8] = [237, 112, 148, 230, 57, 84, 180, 162];
pub const LIQUIDITY_CHANGE_EVENT: [u8; 8] = [126, 240, 175, 206, 158, 88, 153, 107];
pub const POOL_CREATED_EVENT: [u8; 8] = [25, 94, 75, 47, 112, 99, 53, 63];
pub const SWAP_EVENT: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
pub const UPDATE_REWARD_INFOS_EVENT: [u8; 8] = [109, 127, 186, 78, 114, 65, 37, 236];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaydiumClmmEvent {
    CollectPersonalFeeEvent(CollectPersonalFeeEvent),
    CollectProtocolFeeEvent(CollectProtocolFeeEvent),
    ConfigChangeEvent(ConfigChangeEvent),
    CreatePersonalPositionEvent(CreatePersonalPositionEvent),
    DecreaseLiquidityEvent(DecreaseLiquidityEvent),
    IncreaseLiquidityEvent(IncreaseLiquidityEvent),
    LiquidityCalculateEvent(LiquidityCalculateEvent),
    LiquidityChangeEvent(LiquidityChangeEvent),
    PoolCreatedEvent(PoolCreatedEvent),
    SwapEvent(SwapEvent),
    UpdateRewardInfosEvent(UpdateRewardInfosEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Emitted when tokens are collected for a position
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CollectPersonalFeeEvent {
    /// The ID of the token for which underlying tokens were collected
    pub position_nft_mint: Pubkey,
    /// The token account that received the collected token_0 tokens
    pub recipient_token_account_0: Pubkey,
    /// The token account that received the collected token_1 tokens
    pub recipient_token_account_1: Pubkey,
    /// The amount of token_0 owed to the position that was collected
    pub amount_0: u64,
    /// The amount of token_1 owed to the position that was collected
    pub amount_1: u64,
}

/// Emitted when the collected protocol fees are withdrawn by the factory owner
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CollectProtocolFeeEvent {
    /// The pool whose protocol fee is collected
    pub pool_state: Pubkey,
    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_account_0: Pubkey,
    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_account_1: Pubkey,
    /// The amount of token_0 protocol fees that is withdrawn
    pub amount_0: u64,
    /// The amount of token_0 protocol fees that is withdrawn
    pub amount_1: u64,
}

/// Emitted when create or update a config
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ConfigChangeEvent {
    pub index: u16,
    pub owner: Pubkey,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u16,
    pub fund_fee_rate: u32,
    pub fund_owner: Pubkey,
}

/// Emitted when create a new position
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePersonalPositionEvent {
    /// The pool for which liquidity was added
    pub pool_state: Pubkey,
    /// The address that create the position
    pub minter: Pubkey,
    /// The owner of the position and recipient of any minted liquidity
    pub nft_owner: Pubkey,
    /// The lower tick of the position
    pub tick_lower_index: i32,
    /// The upper tick of the position
    pub tick_upper_index: i32,
    /// The amount of liquidity minted to the position range
    pub liquidity: u128,
    /// The amount of token_0 was deposit for the liquidity
    pub deposit_amount_0: u64,
    /// The amount of token_1 was deposit for the liquidity
    pub deposit_amount_1: u64,
    /// The token transfer fee for deposit_amount_0
    pub deposit_amount_0_transfer_fee: u64,
    /// The token transfer fee for deposit_amount_1
    pub deposit_amount_1_transfer_fee: u64,
}

/// Emitted when liquidity is decreased.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DecreaseLiquidityEvent {
    /// The ID of the token for which liquidity was decreased
    pub position_nft_mint: Pubkey,
    /// The amount by which liquidity for the position was decreased
    pub liquidity: u128,
    /// The amount of token_0 that was paid for the decrease in liquidity
    pub decrease_amount_0: u64,
    /// The amount of token_1 that was paid for the decrease in liquidity
    pub decrease_amount_1: u64,
    pub fee_amount_0: u64,
    /// The amount of token_1 fee
    pub fee_amount_1: u64,
    /// The amount of rewards
    pub reward_amounts: [u64; 3],
    /// The amount of token_0 transfer fee
    pub transfer_fee_0: u64,
    /// The amount of token_1 transfer fee
    pub transfer_fee_1: u64,
}

/// Emitted when liquidity is increased.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct IncreaseLiquidityEvent {
    /// The ID of the token for which liquidity was increased
    pub position_nft_mint: Pubkey,
    /// The amount by which liquidity for the NFT position was increased
    pub liquidity: u128,
    /// The amount of token_0 that was paid for the increase in liquidity
    pub amount_0: u64,
    /// The amount of token_1 that was paid for the increase in liquidity
    pub amount_1: u64,
    /// The token transfer fee for amount_0
    pub amount_0_transfer_fee: u64,
    /// The token transfer fee for amount_1
    pub amount_1_transfer_fee: u64,
}

/// Emitted when liquidity decreased or increase.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityCalculateEvent {
    /// The pool liquidity before decrease or increase
    pub pool_liquidity: u128,
    /// The pool price when decrease or increase in liquidity
    pub pool_sqrt_price_x64: u128,
    /// The pool tick when decrease or increase in liquidity
    pub pool_tick: i32,
    /// The amount of token_0 that was calculated for the decrease or increase in liquidity
    pub calc_amount_0: u64,
    /// The amount of token_1 that was calculated for the decrease or increase in liquidity
    pub calc_amount_1: u64,
    pub trade_fee_owed_0: u64,
    /// The amount of token_1 fee
    pub trade_fee_owed_1: u64,
    /// The amount of token_0 transfer fee without trade_fee_amount_0
    pub transfer_fee_0: u64,
    /// The amount of token_1 transfer fee without trade_fee_amount_0
    pub transfer_fee_1: u64,
}

/// Emitted pool liquidity change when increase and decrease liquidity
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityChangeEvent {
    /// The pool for swap
    pub pool_state: Pubkey,
    /// The tick of the pool
    pub tick: i32,
    /// The tick lower of position
    pub tick_lower: i32,
    /// The tick lower of position
    pub tick_upper: i32,
    /// The liquidity of the pool before liquidity change
    pub liquidity_before: u128,
    /// The liquidity of the pool after liquidity change
    pub liquidity_after: u128,
}

/// Emitted when a pool is created and initialized with a starting price
///
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PoolCreatedEvent {
    /// The first token of the pool by address sort order
    pub token_mint_0: Pubkey,
    /// The second token of the pool by address sort order
    pub token_mint_1: Pubkey,
    /// The minimum number of ticks between initialized ticks
    pub tick_spacing: u16,
    /// The address of the created pool
    pub pool_state: Pubkey,
    /// The initial sqrt price of the pool, as a Q64.64
    pub sqrt_price_x64: u128,
    /// The initial tick of the pool, i.e. log base 1.0001 of the starting price of the pool
    pub tick: i32,
    /// Vault of token_0
    pub token_vault_0: Pubkey,
    /// Vault of token_1
    pub token_vault_1: Pubkey,
}

/// Emitted by when a swap is performed for a pool
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapEvent {
    /// The pool for which token_0 and token_1 were swapped
    pub pool_state: Pubkey,
    /// The address that initiated the swap call, and that received the callback
    pub sender: Pubkey,
    /// The payer token account in zero for one swaps, or the recipient token account
    /// in one for zero swaps
    pub token_account_0: Pubkey,
    /// The payer token account in one for zero swaps, or the recipient token account
    /// in zero for one swaps
    pub token_account_1: Pubkey,
    /// The real delta amount of the token_0 of the pool or user
    pub amount_0: u64,
    /// The transfer fee charged by the withheld_amount of the token_0
    pub transfer_fee_0: u64,
    /// The real delta of the token_1 of the pool or user
    pub amount_1: u64,
    /// The transfer fee charged by the withheld_amount of the token_1
    pub transfer_fee_1: u64,
    /// if true, amount_0 is negtive and amount_1 is positive
    pub zero_for_one: bool,
    /// The sqrt(price) of the pool after the swap, as a Q64.64
    pub sqrt_price_x64: u128,
    /// The liquidity of the pool after the swap
    pub liquidity: u128,
    /// The log base 1.0001 of price of the pool after the swap
    pub tick: i32,
}

/// Emitted when Reward are updated for a pool
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateRewardInfosEvent {
    /// Reward info
    pub reward_growth_global_x64: [u128; 3],
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumClmmEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[8..];
        Ok(match disc {
            COLLECT_PERSONAL_FEE_EVENT => Self::CollectPersonalFeeEvent(CollectPersonalFeeEvent::try_from_slice(payload)?),
            COLLECT_PROTOCOL_FEE_EVENT => Self::CollectProtocolFeeEvent(CollectProtocolFeeEvent::try_from_slice(payload)?),
            CONFIG_CHANGE_EVENT => Self::ConfigChangeEvent(ConfigChangeEvent::try_from_slice(payload)?),
            CREATE_PERSONAL_POSITION_EVENT => Self::CreatePersonalPositionEvent(CreatePersonalPositionEvent::try_from_slice(payload)?),
            DECREASE_LIQUIDITY_EVENT => Self::DecreaseLiquidityEvent(DecreaseLiquidityEvent::try_from_slice(payload)?),
            INCREASE_LIQUIDITY_EVENT => Self::IncreaseLiquidityEvent(IncreaseLiquidityEvent::try_from_slice(payload)?),
            LIQUIDITY_CALCULATE_EVENT => Self::LiquidityCalculateEvent(LiquidityCalculateEvent::try_from_slice(payload)?),
            LIQUIDITY_CHANGE_EVENT => Self::LiquidityChangeEvent(LiquidityChangeEvent::try_from_slice(payload)?),
            POOL_CREATED_EVENT => Self::PoolCreatedEvent(PoolCreatedEvent::try_from_slice(payload)?),
            SWAP_EVENT => Self::SwapEvent(SwapEvent::try_from_slice(payload)?),
            UPDATE_REWARD_INFOS_EVENT => Self::UpdateRewardInfosEvent(UpdateRewardInfosEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumClmmEvent, ParseError> {
    RaydiumClmmEvent::try_from(data)
}
