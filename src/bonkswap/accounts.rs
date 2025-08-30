use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;
use thiserror::Error;

// Zero-based indices into `InstructionView.accounts()`
const IDX_POOL: usize = 1;
const IDX_TOKEN_X: usize = 2;
const IDX_TOKEN_Y: usize = 3;
const IDX_POOL_X_ACCOUNT: usize = 4;
const IDX_POOL_Y_ACCOUNT: usize = 5;
const IDX_SWAPPER_X_ACCOUNT: usize = 6;
const IDX_SWAPPER_Y_ACCOUNT: usize = 7;
const IDX_SWAPPER: usize = 8;
// Optional
const IDX_REFERRER_X_ACCOUNT: usize = 9;
const IDX_REFERRER_Y_ACCOUNT: usize = 10;
const IDX_REFERRER: usize = 11;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapAccounts {
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub swapper_x_account: Pubkey,
    pub swapper_y_account: Pubkey,
    pub swapper: Pubkey,
    pub referrer_x_account: Option<Pubkey>,
    pub referrer_y_account: Option<Pubkey>,
    pub referrer: Option<Pubkey>,
}

#[derive(Debug, Error)]
pub enum SwapAccountsError {
    #[error("missing required account `{name}` at index {index}")]
    Missing { name: &'static str, index: usize },
    #[error("invalid key length for `{name}` at index {index}: got {got}, want 32")]
    InvalidLen { name: &'static str, index: usize, got: usize },
}

#[inline]
fn to_pubkey(name: &'static str, index: usize, bytes: &[u8]) -> Result<Pubkey, SwapAccountsError> {
    let arr: [u8; 32] = bytes.try_into().map_err(|_| SwapAccountsError::InvalidLen { name, index, got: bytes.len() })?;
    Ok(Pubkey::new_from_array(arr))
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapAccounts {
    type Error = SwapAccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        // IMPORTANT: call once to avoid borrowing from a temporary
        // If `accounts()` returns a Vec, we own it; if it returns a slice, call `.to_vec()`.
        let accounts = ix.accounts();

        // Required account getter
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, SwapAccountsError> {
            let a = accounts.get(index).ok_or(SwapAccountsError::Missing { name, index })?;
            to_pubkey(name, index, &a.0)
        };

        // Optional account getter
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };

        Ok(SwapAccounts {
            pool: get_req(IDX_POOL, "pool")?,
            token_x: get_req(IDX_TOKEN_X, "token_x")?,
            token_y: get_req(IDX_TOKEN_Y, "token_y")?,
            pool_x_account: get_req(IDX_POOL_X_ACCOUNT, "pool_x_account")?,
            pool_y_account: get_req(IDX_POOL_Y_ACCOUNT, "pool_y_account")?,
            swapper_x_account: get_req(IDX_SWAPPER_X_ACCOUNT, "swapper_x_account")?,
            swapper_y_account: get_req(IDX_SWAPPER_Y_ACCOUNT, "swapper_y_account")?,
            swapper: get_req(IDX_SWAPPER, "swapper")?,
            referrer_x_account: get_opt(IDX_REFERRER_X_ACCOUNT),
            referrer_y_account: get_opt(IDX_REFERRER_Y_ACCOUNT),
            referrer: get_opt(IDX_REFERRER),
        })
    }
}

// Convenience wrapper matching your original API
pub fn get_swap_accounts(ix: &InstructionView) -> Result<SwapAccounts, SwapAccountsError> {
    SwapAccounts::try_from(ix)
}
