//! Darklake on-chain events.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Event discriminators
pub const ADD_LIQUIDITY_EVENT: [u8; 8] = [31, 94, 125, 90, 227, 52, 61, 186];
pub const CANCEL_EVENT: [u8; 8] = [196, 40, 17, 225, 87, 58, 126, 44];
pub const INITIALIZE_POOL_EVENT: [u8; 8] = [145, 104, 208, 79, 8, 159, 145, 240];
pub const REMOVE_LIQUIDITY_EVENT: [u8; 8] = [116, 244, 97, 232, 103, 31, 152, 58];
pub const SETTLE_EVENT: [u8; 8] = [172, 88, 86, 73, 227, 209, 204, 56];
pub const SLASH_EVENT: [u8; 8] = [157, 91, 23, 33, 129, 182, 68, 120];
pub const SWAP_EVENT: [u8; 8] = [81, 108, 227, 190, 205, 208, 10, 196];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityEvent {
    pub supplier: Pubkey,
    pub max_amount_x: u64,
    pub max_amount_y: u64,
    pub transfer_in_x: u64,
    pub transfer_in_y: u64,
    pub liquidity_minted: u64,
    pub user_token_lp_balance: u64,
    pub new_reserve_x: u64,
    pub new_reserve_y: u64,
    pub available_reserve_x: u64,
    pub available_reserve_y: u64,
    pub token_mint_lp: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub ref_code: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CancelEvent {
    pub caller: Pubkey,
    pub trader: Pubkey,
    pub direction: u8,
    pub deadline: u64,
    pub protocol_fee: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub wsol_to_order_owner: u64,
    pub wsol_to_caller: u64,
    pub sol_to_caller: u64,
    pub actual_amount_in: u64,
    pub new_reserve_x: u64,
    pub new_reserve_y: u64,
    pub available_reserve_x: u64,
    pub available_reserve_y: u64,
    pub locked_x: u64,
    pub locked_y: u64,
    pub user_locked_x: u64,
    pub user_locked_y: u64,
    pub protocol_fee_x: u64,
    pub protocol_fee_y: u64,
    pub user_token_account_x: Pubkey,
    pub user_token_account_y: Pubkey,
    pub token_mint_lp: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct InitializePoolEvent {
    pub trader: Pubkey,
    pub liquidity_minted: u64,
    pub sol_create_pool_fee: u64,
    pub new_reserve_x: u64,
    pub new_reserve_y: u64,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub token_mint_lp: Pubkey,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityEvent {
    pub supplier: Pubkey,
    pub min_amount_x: u64,
    pub min_amount_y: u64,
    pub transfer_out_x: u64,
    pub transfer_out_y: u64,
    pub liquidity_burned: u64,
    pub user_token_lp_balance: u64,
    pub new_reserve_x: u64,
    pub new_reserve_y: u64,
    pub available_reserve_x: u64,
    pub available_reserve_y: u64,
    pub token_mint_lp: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SettleEvent {
    pub caller: Pubkey,
    pub trader: Pubkey,
    pub direction: u8,
    pub deadline: u64,
    pub protocol_fee: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub actual_amount_in: u64,
    pub wsol_to_trader: u64,
    pub wsol_to_caller: u64,
    pub sol_to_trader: u64,
    pub actual_amount_out: u64,
    pub new_reserve_x: u64,
    pub new_reserve_y: u64,
    pub available_reserve_x: u64,
    pub available_reserve_y: u64,
    pub locked_x: u64,
    pub locked_y: u64,
    pub user_locked_x: u64,
    pub user_locked_y: u64,
    pub protocol_fee_x: u64,
    pub protocol_fee_y: u64,
    pub user_token_account_x: Pubkey,
    pub user_token_account_y: Pubkey,
    pub token_mint_lp: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub ref_code: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SlashEvent {
    pub caller: Pubkey,
    pub trader: Pubkey,
    pub direction: u8,
    pub deadline: u64,
    pub protocol_fee: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub wsol_to_trader: u64,
    pub wsol_to_caller: u64,
    pub sol_to_caller: u64,
    pub actual_amount_in: u64,
    pub new_reserve_x: u64,
    pub new_reserve_y: u64,
    pub available_reserve_x: u64,
    pub available_reserve_y: u64,
    pub locked_x: u64,
    pub locked_y: u64,
    pub user_locked_x: u64,
    pub user_locked_y: u64,
    pub protocol_fee_x: u64,
    pub protocol_fee_y: u64,
    pub user_token_account_x: Pubkey,
    pub user_token_account_y: Pubkey,
    pub token_mint_lp: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapEvent {
    pub trader: Pubkey,
    pub direction: u8,
    pub deadline: u64,
    pub trade_fee: u64,
    pub protocol_fee: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub actual_amount_in: u64,
    pub wsol_deposit: u64,
    pub actual_amount_out: u64,
    pub new_reserve_x: u64,
    pub new_reserve_y: u64,
    pub available_reserve_x: u64,
    pub available_reserve_y: u64,
    pub locked_x: u64,
    pub locked_y: u64,
    pub user_locked_x: u64,
    pub user_locked_y: u64,
    pub protocol_fee_x: u64,
    pub protocol_fee_y: u64,
    pub user_token_account_x: Pubkey,
    pub user_token_account_y: Pubkey,
    pub token_mint_lp: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DarklakeEvent {
    AddLiquidity(AddLiquidityEvent),
    Cancel(CancelEvent),
    InitializePool(InitializePoolEvent),
    RemoveLiquidity(RemoveLiquidityEvent),
    Settle(SettleEvent),
    Slash(SlashEvent),
    Swap(SwapEvent),
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for DarklakeEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            ADD_LIQUIDITY_EVENT => Self::AddLiquidity(AddLiquidityEvent::try_from_slice(payload)?),
            CANCEL_EVENT => Self::Cancel(CancelEvent::try_from_slice(payload)?),
            INITIALIZE_POOL_EVENT => Self::InitializePool(InitializePoolEvent::try_from_slice(payload)?),
            REMOVE_LIQUIDITY_EVENT => Self::RemoveLiquidity(RemoveLiquidityEvent::try_from_slice(payload)?),
            SETTLE_EVENT => Self::Settle(SettleEvent::try_from_slice(payload)?),
            SLASH_EVENT => Self::Slash(SlashEvent::try_from_slice(payload)?),
            SWAP_EVENT => Self::Swap(SwapEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack_event(data: &[u8]) -> Result<DarklakeEvent, ParseError> {
    DarklakeEvent::try_from(data)
}
