//! Orca Whirlpool on-chain events.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum LockType {
    Permanent,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum LockTypeLabel {
    Permanent,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AdaptiveFeeVariables {
    pub last_reference_update_timestamp: u64,
    pub last_major_swap_timestamp: u64,
    pub volatility_reference: u32,
    pub tick_group_index_reference: i32,
    pub volatility_accumulator: u32,
    pub reserved: [u8; 16],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OpenPositionBumps {
    pub position_bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OpenPositionWithMetadataBumps {
    pub position_bump: u8,
    pub metadata_bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PositionRewardInfo {
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Tick {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WhirlpoolRewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}

const LIQUIDITY_DECREASED: [u8; 8] = [166, 1, 36, 71, 112, 202, 181, 171];
const LIQUIDITY_INCREASED: [u8; 8] = [30, 7, 144, 181, 102, 254, 155, 161];
const POOL_INITIALIZED: [u8; 8] = [100, 118, 173, 87, 12, 198, 254, 229];
const TRADED: [u8; 8] = [225, 202, 73, 175, 147, 43, 160, 150];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhirlpoolEvent {
    LiquidityDecreased(LiquidityDecreased),
    LiquidityIncreased(LiquidityIncreased),
    PoolInitialized(PoolInitialized),
    Traded(Traded),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityDecreased {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_a_transfer_fee: u64,
    pub token_b_transfer_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityIncreased {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_a_transfer_fee: u64,
    pub token_b_transfer_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PoolInitialized {
    pub whirlpool: Pubkey,
    pub whirlpools_config: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub tick_spacing: u16,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub decimals_a: u8,
    pub decimals_b: u8,
    pub initial_sqrt_price: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Traded {
    pub whirlpool: Pubkey,
    pub a_to_b: bool,
    pub pre_sqrt_price: u128,
    pub post_sqrt_price: u128,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub lp_fee: u64,
    pub protocol_fee: u64,
}

impl<'a> TryFrom<&'a [u8]> for WhirlpoolEvent {
    type Error = ParseError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            LIQUIDITY_DECREASED => Self::LiquidityDecreased(LiquidityDecreased::try_from_slice(payload)?),
            LIQUIDITY_INCREASED => Self::LiquidityIncreased(LiquidityIncreased::try_from_slice(payload)?),
            POOL_INITIALIZED => Self::PoolInitialized(PoolInitialized::try_from_slice(payload)?),
            TRADED => Self::Traded(Traded::try_from_slice(payload)?),
            _ => Self::Unknown,
        })
    }
}
pub fn parse_event(data: &[u8]) -> Result<WhirlpoolEvent, ParseError> {
    WhirlpoolEvent::try_from(data)
}
