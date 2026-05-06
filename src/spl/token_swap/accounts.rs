//! SPL Token Swap on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts::AccountsError;

/// Token Swap pool state
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapV1 {
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub token_program_id: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub pool_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub pool_fee_account: Pubkey,
}

/// Trade fee numerator/denominator
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Fees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}

/// Constant price curve
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConstantPriceCurve {
    pub token_b_price: u64,
}

/// Constant product curve (empty — no params)
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConstantProductCurve;

/// Offset curve
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OffsetCurve {
    pub token_b_offset: u64,
}

/// Stable curve
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StableCurve {
    pub amp: u64,
}

// -----------------------------------------------------------------------------
// Swap instruction accounts
// -----------------------------------------------------------------------------
const IDX_SWAP_ACCOUNT: usize = 0;
const IDX_AUTHORITY: usize = 1;
const IDX_USER_TRANSFER_AUTHORITY: usize = 2;
const IDX_SOURCE: usize = 3;
const IDX_SWAP_SOURCE: usize = 4;
const IDX_SWAP_DESTINATION: usize = 5;
const IDX_DESTINATION: usize = 6;
const IDX_POOL_MINT: usize = 7;
const IDX_FEE_ACCOUNT: usize = 8;
const IDX_HOST_FEE_ACCOUNT: usize = 9;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapAccounts {
    pub swap_account: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub destination: Pubkey,
    pub pool_mint: Pubkey,
    pub fee_account: Pubkey,
    pub host_fee_account: Option<Pubkey>,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();

        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::common::accounts::to_pubkey(name, index, a.0)
        };

        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };

        Ok(SwapAccounts {
            swap_account: get_req(IDX_SWAP_ACCOUNT, "swap_account")?,
            authority: get_req(IDX_AUTHORITY, "authority")?,
            user_transfer_authority: get_req(IDX_USER_TRANSFER_AUTHORITY, "user_transfer_authority")?,
            source: get_req(IDX_SOURCE, "source")?,
            swap_source: get_req(IDX_SWAP_SOURCE, "swap_source")?,
            swap_destination: get_req(IDX_SWAP_DESTINATION, "swap_destination")?,
            destination: get_req(IDX_DESTINATION, "destination")?,
            pool_mint: get_req(IDX_POOL_MINT, "pool_mint")?,
            fee_account: get_req(IDX_FEE_ACCOUNT, "fee_account")?,
            host_fee_account: get_opt(IDX_HOST_FEE_ACCOUNT),
        })
    }
}

pub fn get_swap_accounts(ix: &InstructionView) -> Result<SwapAccounts, AccountsError> {
    SwapAccounts::try_from(ix)
}
