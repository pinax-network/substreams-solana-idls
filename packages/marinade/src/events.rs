//! Marinade Finance on-chain events (Anchor `#[event]`).

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (Anchor: sha256("event:<EventName>")[..8])
// -----------------------------------------------------------------------------
pub const DEPOSIT_EVENT: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];
pub const DEPOSIT_STAKE_ACCOUNT_EVENT: [u8; 8] = [231, 203, 118, 96, 75, 116, 70, 228];
pub const LIQUID_UNSTAKE_EVENT: [u8; 8] = [173, 5, 147, 15, 5, 14, 194, 116];
pub const ADD_LIQUIDITY_EVENT: [u8; 8] = [27, 178, 153, 186, 47, 196, 140, 45];
pub const REMOVE_LIQUIDITY_EVENT: [u8; 8] = [141, 199, 182, 123, 159, 94, 215, 102];
pub const WITHDRAW_STAKE_ACCOUNT_EVENT: [u8; 8] = [131, 238, 39, 48, 30, 27, 165, 28];

// ──────────────────────────────────────────────────────────────────────────────
// Event enum
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum MarinadeEvent {
    Deposit(DepositEvent),
    DepositStakeAccount(DepositStakeAccountEvent),
    LiquidUnstake(LiquidUnstakeEvent),
    AddLiquidity(AddLiquidityEvent),
    RemoveLiquidity(RemoveLiquidityEvent),
    WithdrawStakeAccount(WithdrawStakeAccountEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Event structs
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositEvent {
    pub state: Pubkey,
    pub sol_owner: Pubkey,
    pub user_sol_balance: u64,
    pub user_msol_balance: u64,
    pub sol_leg_balance: u64,
    pub msol_leg_balance: u64,
    pub reserve_balance: u64,
    pub sol_swapped: u64,
    pub msol_swapped: u64,
    pub sol_deposited: u64,
    pub msol_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositStakeAccountEvent {
    pub state: Pubkey,
    pub stake: Pubkey,
    pub delegated: u64,
    pub withdrawer: Pubkey,
    pub stake_index: u32,
    pub validator: Pubkey,
    pub validator_index: u32,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub user_msol_balance: u64,
    pub msol_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LiquidUnstakeEvent {
    pub state: Pubkey,
    pub msol_owner: Pubkey,
    pub liq_pool_sol_balance: u64,
    pub liq_pool_msol_balance: u64,
    pub treasury_msol_balance: Option<u64>,
    pub user_msol_balance: u64,
    pub user_sol_balance: u64,
    pub msol_amount: u64,
    pub msol_fee: u64,
    pub treasury_msol_cut: u64,
    pub sol_amount: u64,
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub treasury_cut: Fee,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityEvent {
    pub state: Pubkey,
    pub sol_owner: Pubkey,
    pub user_sol_balance: u64,
    pub user_lp_balance: u64,
    pub sol_leg_balance: u64,
    pub lp_supply: u64,
    pub sol_added_amount: u64,
    pub lp_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquidityEvent {
    pub state: Pubkey,
    pub sol_leg_balance: u64,
    pub msol_leg_balance: u64,
    pub user_lp_balance: u64,
    pub user_sol_balance: u64,
    pub user_msol_balance: u64,
    pub lp_mint_supply: u64,
    pub lp_burned: u64,
    pub sol_out_amount: u64,
    pub msol_out_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawStakeAccountEvent {
    pub state: Pubkey,
    pub epoch: u64,
    pub stake: Pubkey,
    pub last_update_stake_delegation: u64,
    pub stake_index: u32,
    pub validator: Pubkey,
    pub validator_index: u32,
    pub user_msol_balance: u64,
    pub user_msol_auth: Pubkey,
    pub msol_burned: u64,
    pub msol_fees: u64,
    pub split_stake: Pubkey,
    pub beneficiary: Pubkey,
    pub split_lamports: u64,
    pub fee_bp_cents: u32,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}

/// Fee structure used by Marinade (basis points).
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Fee {
    pub basis_points: u32,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation (Anchor events have 8-byte discriminator)
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for MarinadeEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            DEPOSIT_EVENT => Self::Deposit(DepositEvent::try_from_slice(payload)?),
            DEPOSIT_STAKE_ACCOUNT_EVENT => Self::DepositStakeAccount(DepositStakeAccountEvent::try_from_slice(payload)?),
            LIQUID_UNSTAKE_EVENT => Self::LiquidUnstake(LiquidUnstakeEvent::try_from_slice(payload)?),
            ADD_LIQUIDITY_EVENT => Self::AddLiquidity(AddLiquidityEvent::try_from_slice(payload)?),
            REMOVE_LIQUIDITY_EVENT => Self::RemoveLiquidity(RemoveLiquidityEvent::try_from_slice(payload)?),
            WITHDRAW_STAKE_ACCOUNT_EVENT => Self::WithdrawStakeAccount(WithdrawStakeAccountEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<MarinadeEvent, ParseError> {
    MarinadeEvent::try_from(data)
}
