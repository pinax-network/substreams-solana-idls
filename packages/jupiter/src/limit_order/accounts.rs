use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use idls_common::accounts::{to_pubkey, AccountsError};

// -----------------------------------------------------------------------------
// Simple instructions
// -----------------------------------------------------------------------------
accounts!(
    PreFlashFillOrderAccounts,
    get_pre_flash_fill_order_accounts,
    { order, reserve, taker, taker_output_account, input_mint, input_mint_token_program, instruction, system_program }
);

accounts!(
    WithdrawFeeAccounts,
    get_withdraw_fee_accounts,
    { admin, fee_authority, program_fee_account, admin_token_acocunt, token_program, mint }
);

accounts!(
    InitFeeAccounts,
    get_init_fee_accounts,
    { keeper, fee_authority, system_program }
);

accounts!(
    UpdateFeeAccounts,
    get_update_fee_accounts,
    { keeper, fee_authority }
);

// -----------------------------------------------------------------------------
// Instructions with optional accounts
// -----------------------------------------------------------------------------
const IDX_BASE: usize = 0;
const IDX_MAKER: usize = 1;
const IDX_ORDER: usize = 2;
const IDX_RESERVE: usize = 3;
const IDX_MAKER_INPUT_ACCOUNT: usize = 4;
const IDX_INPUT_MINT: usize = 5;
const IDX_MAKER_OUTPUT_ACCOUNT: usize = 6;
const IDX_REFERRAL: usize = 7;
const IDX_OUTPUT_MINT: usize = 8;
const IDX_SYSTEM_PROGRAM: usize = 9;
const IDX_TOKEN_PROGRAM: usize = 10;
const IDX_RENT: usize = 11;

/// Accounts for the `initialize_order` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeOrderAccounts {
    pub base: Pubkey,
    pub maker: Pubkey,
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker_input_account: Pubkey,
    pub input_mint: Pubkey,
    pub maker_output_account: Pubkey,
    pub referral: Option<Pubkey>,
    pub output_mint: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeOrderAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(InitializeOrderAccounts {
            base: get_req(IDX_BASE, "base")?,
            maker: get_req(IDX_MAKER, "maker")?,
            order: get_req(IDX_ORDER, "order")?,
            reserve: get_req(IDX_RESERVE, "reserve")?,
            maker_input_account: get_req(IDX_MAKER_INPUT_ACCOUNT, "maker_input_account")?,
            input_mint: get_req(IDX_INPUT_MINT, "input_mint")?,
            maker_output_account: get_req(IDX_MAKER_OUTPUT_ACCOUNT, "maker_output_account")?,
            referral: get_opt(IDX_REFERRAL),
            output_mint: get_req(IDX_OUTPUT_MINT, "output_mint")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            rent: get_req(IDX_RENT, "rent")?,
        })
    }
}

pub fn get_initialize_order_accounts(ix: &InstructionView) -> Result<InitializeOrderAccounts, AccountsError> {
    InitializeOrderAccounts::try_from(ix)
}

const IDX_FO_ORDER: usize = 0;
const IDX_FO_RESERVE: usize = 1;
const IDX_FO_MAKER: usize = 2;
const IDX_FO_TAKER: usize = 3;
const IDX_FO_TAKER_OUTPUT_ACCOUNT: usize = 4;
const IDX_FO_MAKER_OUTPUT_ACCOUNT: usize = 5;
const IDX_FO_TAKER_INPUT_ACCOUNT: usize = 6;
const IDX_FO_FEE_AUTHORITY: usize = 7;
const IDX_FO_PROGRAM_FEE_ACCOUNT: usize = 8;
const IDX_FO_REFERRAL: usize = 9;
const IDX_FO_TOKEN_PROGRAM: usize = 10;
const IDX_FO_SYSTEM_PROGRAM: usize = 11;

/// Accounts for the `fill_order` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct FillOrderAccounts {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub taker_output_account: Pubkey,
    pub maker_output_account: Pubkey,
    pub taker_input_account: Pubkey,
    pub fee_authority: Pubkey,
    pub program_fee_account: Pubkey,
    pub referral: Option<Pubkey>,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for FillOrderAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(FillOrderAccounts {
            order: get_req(IDX_FO_ORDER, "order")?,
            reserve: get_req(IDX_FO_RESERVE, "reserve")?,
            maker: get_req(IDX_FO_MAKER, "maker")?,
            taker: get_req(IDX_FO_TAKER, "taker")?,
            taker_output_account: get_req(IDX_FO_TAKER_OUTPUT_ACCOUNT, "taker_output_account")?,
            maker_output_account: get_req(IDX_FO_MAKER_OUTPUT_ACCOUNT, "maker_output_account")?,
            taker_input_account: get_req(IDX_FO_TAKER_INPUT_ACCOUNT, "taker_input_account")?,
            fee_authority: get_req(IDX_FO_FEE_AUTHORITY, "fee_authority")?,
            program_fee_account: get_req(IDX_FO_PROGRAM_FEE_ACCOUNT, "program_fee_account")?,
            referral: get_opt(IDX_FO_REFERRAL),
            token_program: get_req(IDX_FO_TOKEN_PROGRAM, "token_program")?,
            system_program: get_req(IDX_FO_SYSTEM_PROGRAM, "system_program")?,
        })
    }
}

pub fn get_fill_order_accounts(ix: &InstructionView) -> Result<FillOrderAccounts, AccountsError> {
    FillOrderAccounts::try_from(ix)
}

const IDX_FFL_ORDER: usize = 0;
const IDX_FFL_RESERVE: usize = 1;
const IDX_FFL_MAKER: usize = 2;
const IDX_FFL_TAKER: usize = 3;
const IDX_FFL_MAKER_OUTPUT_ACCOUNT: usize = 4;
const IDX_FFL_TAKER_INPUT_ACCOUNT: usize = 5;
const IDX_FFL_FEE_AUTHORITY: usize = 6;
const IDX_FFL_PROGRAM_FEE_ACCOUNT: usize = 7;
const IDX_FFL_REFERRAL: usize = 8;
const IDX_FFL_INPUT_MINT: usize = 9;
const IDX_FFL_INPUT_MINT_TOKEN_PROGRAM: usize = 10;
const IDX_FFL_OUTPUT_MINT: usize = 11;
const IDX_FFL_OUTPUT_MINT_TOKEN_PROGRAM: usize = 12;
const IDX_FFL_SYSTEM_PROGRAM: usize = 13;

/// Accounts for the `flash_fill_order` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct FlashFillOrderAccounts {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub maker_output_account: Pubkey,
    pub taker_input_account: Pubkey,
    pub fee_authority: Pubkey,
    pub program_fee_account: Pubkey,
    pub referral: Option<Pubkey>,
    pub input_mint: Pubkey,
    pub input_mint_token_program: Pubkey,
    pub output_mint: Pubkey,
    pub output_mint_token_program: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for FlashFillOrderAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(FlashFillOrderAccounts {
            order: get_req(IDX_FFL_ORDER, "order")?,
            reserve: get_req(IDX_FFL_RESERVE, "reserve")?,
            maker: get_req(IDX_FFL_MAKER, "maker")?,
            taker: get_req(IDX_FFL_TAKER, "taker")?,
            maker_output_account: get_req(IDX_FFL_MAKER_OUTPUT_ACCOUNT, "maker_output_account")?,
            taker_input_account: get_req(IDX_FFL_TAKER_INPUT_ACCOUNT, "taker_input_account")?,
            fee_authority: get_req(IDX_FFL_FEE_AUTHORITY, "fee_authority")?,
            program_fee_account: get_req(IDX_FFL_PROGRAM_FEE_ACCOUNT, "program_fee_account")?,
            referral: get_opt(IDX_FFL_REFERRAL),
            input_mint: get_req(IDX_FFL_INPUT_MINT, "input_mint")?,
            input_mint_token_program: get_req(IDX_FFL_INPUT_MINT_TOKEN_PROGRAM, "input_mint_token_program")?,
            output_mint: get_req(IDX_FFL_OUTPUT_MINT, "output_mint")?,
            output_mint_token_program: get_req(IDX_FFL_OUTPUT_MINT_TOKEN_PROGRAM, "output_mint_token_program")?,
            system_program: get_req(IDX_FFL_SYSTEM_PROGRAM, "system_program")?,
        })
    }
}

pub fn get_flash_fill_order_accounts(ix: &InstructionView) -> Result<FlashFillOrderAccounts, AccountsError> {
    FlashFillOrderAccounts::try_from(ix)
}

const IDX_CO_ORDER: usize = 0;
const IDX_CO_RESERVE: usize = 1;
const IDX_CO_MAKER: usize = 2;
const IDX_CO_MAKER_INPUT_ACCOUNT: usize = 3;
const IDX_CO_SYSTEM_PROGRAM: usize = 4;
const IDX_CO_TOKEN_PROGRAM: usize = 5;
const IDX_CO_INPUT_MINT: usize = 6;

/// Accounts for the `cancel_order` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CancelOrderAccounts {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub maker_input_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub input_mint: Option<Pubkey>,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CancelOrderAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(CancelOrderAccounts {
            order: get_req(IDX_CO_ORDER, "order")?,
            reserve: get_req(IDX_CO_RESERVE, "reserve")?,
            maker: get_req(IDX_CO_MAKER, "maker")?,
            maker_input_account: get_req(IDX_CO_MAKER_INPUT_ACCOUNT, "maker_input_account")?,
            system_program: get_req(IDX_CO_SYSTEM_PROGRAM, "system_program")?,
            token_program: get_req(IDX_CO_TOKEN_PROGRAM, "token_program")?,
            input_mint: get_opt(IDX_CO_INPUT_MINT),
        })
    }
}

pub fn get_cancel_order_accounts(ix: &InstructionView) -> Result<CancelOrderAccounts, AccountsError> {
    CancelOrderAccounts::try_from(ix)
}

/// Accounts for the `cancel_expired_order` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CancelExpiredOrderAccounts {
    pub order: Pubkey,
    pub reserve: Pubkey,
    pub maker: Pubkey,
    pub maker_input_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub input_mint: Option<Pubkey>,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CancelExpiredOrderAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(CancelExpiredOrderAccounts {
            order: get_req(IDX_CO_ORDER, "order")?,
            reserve: get_req(IDX_CO_RESERVE, "reserve")?,
            maker: get_req(IDX_CO_MAKER, "maker")?,
            maker_input_account: get_req(IDX_CO_MAKER_INPUT_ACCOUNT, "maker_input_account")?,
            system_program: get_req(IDX_CO_SYSTEM_PROGRAM, "system_program")?,
            token_program: get_req(IDX_CO_TOKEN_PROGRAM, "token_program")?,
            input_mint: get_opt(IDX_CO_INPUT_MINT),
        })
    }
}

pub fn get_cancel_expired_order_accounts(ix: &InstructionView) -> Result<CancelExpiredOrderAccounts, AccountsError> {
    CancelExpiredOrderAccounts::try_from(ix)
}
