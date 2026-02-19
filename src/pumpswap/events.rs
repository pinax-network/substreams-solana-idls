//! PumpSwap AMM events.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Event discriminators (from IDL spec 0.1.0)
pub const ADMIN_SET_COIN_CREATOR_EVENT: [u8; 8] = [45, 220, 93, 24, 25, 97, 172, 104];
pub const ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT: [u8; 8] = [147, 250, 108, 120, 247, 29, 67, 222];
pub const BUY_EVENT: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119];
pub const CLAIM_TOKEN_INCENTIVES_EVENT: [u8; 8] = [79, 172, 246, 49, 205, 91, 206, 232];
pub const CLOSE_USER_VOLUME_ACCUMULATOR_EVENT: [u8; 8] = [146, 159, 189, 172, 146, 88, 56, 244];
pub const COLLECT_COIN_CREATOR_FEE_EVENT: [u8; 8] = [232, 245, 194, 238, 234, 218, 58, 89];
pub const CREATE_CONFIG_EVENT: [u8; 8] = [107, 52, 89, 129, 55, 226, 81, 22];
pub const CREATE_POOL_EVENT: [u8; 8] = [177, 49, 12, 210, 160, 118, 167, 116];
pub const DEPOSIT_EVENT: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];
pub const DISABLE_EVENT: [u8; 8] = [107, 253, 193, 76, 228, 202, 27, 104];
pub const EXTEND_ACCOUNT_EVENT: [u8; 8] = [97, 97, 215, 144, 93, 146, 22, 124];
pub const INIT_USER_VOLUME_ACCUMULATOR_EVENT: [u8; 8] = [134, 36, 13, 72, 232, 101, 130, 216];
pub const SELL_EVENT: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42];
pub const SET_BONDING_CURVE_COIN_CREATOR_EVENT: [u8; 8] = [242, 231, 235, 102, 65, 99, 189, 211];
pub const SET_METAPLEX_COIN_CREATOR_EVENT: [u8; 8] = [150, 107, 199, 123, 124, 207, 102, 228];
pub const SYNC_USER_VOLUME_ACCUMULATOR_EVENT: [u8; 8] = [197, 122, 167, 124, 116, 81, 91, 255];
pub const UPDATE_ADMIN_EVENT: [u8; 8] = [225, 152, 171, 87, 246, 63, 66, 234];
pub const UPDATE_FEE_CONFIG_EVENT: [u8; 8] = [90, 23, 65, 35, 62, 244, 188, 208];
pub const WITHDRAW_EVENT: [u8; 8] = [22, 9, 133, 26, 160, 44, 71, 192];

#[derive(Debug, Clone, PartialEq)]
pub enum PumpSwapEvent {
    AdminSetCoinCreator(AdminSetCoinCreatorEvent),
    AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesEvent),
    Buy(BuyEvent),
    ClaimTokenIncentives(ClaimTokenIncentivesEvent),
    CloseUserVolumeAccumulator(CloseUserVolumeAccumulatorEvent),
    CollectCoinCreatorFee(CollectCoinCreatorFeeEvent),
    CreateConfig(CreateConfigEvent),
    CreatePool(CreatePoolEvent),
    Deposit(DepositEvent),
    Disable(DisableEvent),
    ExtendAccount(ExtendAccountEvent),
    InitUserVolumeAccumulator(InitUserVolumeAccumulatorEvent),
    Sell(SellEvent),
    SetBondingCurveCoinCreator(SetBondingCurveCoinCreatorEvent),
    SetMetaplexCoinCreator(SetMetaplexCoinCreatorEvent),
    SyncUserVolumeAccumulator(SyncUserVolumeAccumulatorEvent),
    UpdateAdmin(UpdateAdminEvent),
    UpdateFeeConfig(UpdateFeeConfigEvent),
    Withdraw(WithdrawEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AdminSetCoinCreatorEvent {
    pub timestamp: i64,
    pub admin_set_coin_creator_authority: Pubkey,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub old_coin_creator: Pubkey,
    pub new_coin_creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AdminUpdateTokenIncentivesEvent {
    pub start_time: i64,
    pub end_time: i64,
    pub day_number: u64,
    pub token_supply_per_day: u64,
    pub mint: Pubkey,
    pub seconds_in_a_day: i64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
    pub track_volume: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub min_base_amount_out: u64,
    pub ix_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimTokenIncentivesEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CloseUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub timestamp: i64,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CollectCoinCreatorFeeEvent {
    pub timestamp: i64,
    pub coin_creator: Pubkey,
    pub coin_creator_fee: u64,
    pub coin_creator_vault_ata: Pubkey,
    pub coin_creator_token_account: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositEvent {
    pub timestamp: i64,
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DisableEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: Pubkey,
    pub user: Pubkey,
    pub current_size: u64,
    pub new_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitUserVolumeAccumulatorEvent {
    pub payer: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetBondingCurveCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub bonding_curve: Pubkey,
    pub coin_creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetMetaplexCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub metadata: Pubkey,
    pub coin_creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SyncUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub total_claimed_tokens_before: u64,
    pub total_claimed_tokens_after: u64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateAdminEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateFeeConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawEvent {
    pub timestamp: i64,
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_out: u64,
    pub quote_amount_out: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}

pub fn unpack_event(data: &[u8]) -> Result<PumpSwapEvent, ParseError> {
    if data.len() < 8 {
        return Err(ParseError::TooShort(data.len()));
    }
    let (disc, rest) = data.split_at(8);
    let disc: [u8; 8] = disc.try_into().unwrap();
    Ok(match disc {
        ADMIN_SET_COIN_CREATOR_EVENT => PumpSwapEvent::AdminSetCoinCreator(AdminSetCoinCreatorEvent::try_from_slice(rest)?),
        ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT => PumpSwapEvent::AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesEvent::try_from_slice(rest)?),
        BUY_EVENT => PumpSwapEvent::Buy(BuyEvent::try_from_slice(rest)?),
        CLAIM_TOKEN_INCENTIVES_EVENT => PumpSwapEvent::ClaimTokenIncentives(ClaimTokenIncentivesEvent::try_from_slice(rest)?),
        CLOSE_USER_VOLUME_ACCUMULATOR_EVENT => PumpSwapEvent::CloseUserVolumeAccumulator(CloseUserVolumeAccumulatorEvent::try_from_slice(rest)?),
        COLLECT_COIN_CREATOR_FEE_EVENT => PumpSwapEvent::CollectCoinCreatorFee(CollectCoinCreatorFeeEvent::try_from_slice(rest)?),
        CREATE_CONFIG_EVENT => PumpSwapEvent::CreateConfig(CreateConfigEvent::try_from_slice(rest)?),
        CREATE_POOL_EVENT => PumpSwapEvent::CreatePool(CreatePoolEvent::try_from_slice(rest)?),
        DEPOSIT_EVENT => PumpSwapEvent::Deposit(DepositEvent::try_from_slice(rest)?),
        DISABLE_EVENT => PumpSwapEvent::Disable(DisableEvent::try_from_slice(rest)?),
        EXTEND_ACCOUNT_EVENT => PumpSwapEvent::ExtendAccount(ExtendAccountEvent::try_from_slice(rest)?),
        INIT_USER_VOLUME_ACCUMULATOR_EVENT => PumpSwapEvent::InitUserVolumeAccumulator(InitUserVolumeAccumulatorEvent::try_from_slice(rest)?),
        SELL_EVENT => PumpSwapEvent::Sell(SellEvent::try_from_slice(rest)?),
        SET_BONDING_CURVE_COIN_CREATOR_EVENT => PumpSwapEvent::SetBondingCurveCoinCreator(SetBondingCurveCoinCreatorEvent::try_from_slice(rest)?),
        SET_METAPLEX_COIN_CREATOR_EVENT => PumpSwapEvent::SetMetaplexCoinCreator(SetMetaplexCoinCreatorEvent::try_from_slice(rest)?),
        SYNC_USER_VOLUME_ACCUMULATOR_EVENT => PumpSwapEvent::SyncUserVolumeAccumulator(SyncUserVolumeAccumulatorEvent::try_from_slice(rest)?),
        UPDATE_ADMIN_EVENT => PumpSwapEvent::UpdateAdmin(UpdateAdminEvent::try_from_slice(rest)?),
        UPDATE_FEE_CONFIG_EVENT => PumpSwapEvent::UpdateFeeConfig(UpdateFeeConfigEvent::try_from_slice(rest)?),
        WITHDRAW_EVENT => PumpSwapEvent::Withdraw(WithdrawEvent::try_from_slice(rest)?),
        _ => return Err(ParseError::Unknown(disc)),
    })
}
