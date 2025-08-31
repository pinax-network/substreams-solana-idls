use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::accounts::{self as _, AccountsError};

// -----------------------------------------------------------------------------
// Simple instructions
// -----------------------------------------------------------------------------
accounts!(
    ClaimAccounts,
    get_claim_accounts,
    { wallet, program_authority, system_program }
);

accounts!(
    ClaimTokenAccounts,
    get_claim_token_accounts,
    { payer, wallet, program_authority, program_token_account, destination_token_account, mint, token_program, associated_token_program, system_program }
);

accounts!(
    CloseTokenAccounts,
    get_close_token_accounts,
    { operator, wallet, program_authority, program_token_account, mint, token_program }
);

accounts!(
    CreateOpenOrdersAccounts,
    get_create_open_orders_accounts,
    { open_orders, payer, dex_program, system_program, rent, market }
);

accounts!(
    CreateProgramOpenOrdersAccounts,
    get_create_program_open_orders_accounts,
    { open_orders, payer, program_authority, dex_program, system_program, rent, market }
);

accounts!(
    CreateTokenLedgerAccounts,
    get_create_token_ledger_accounts,
    { token_ledger, payer, system_program }
);

accounts!(
    CreateTokenAccountAccounts,
    get_create_token_account_accounts,
    { token_account, user, mint, token_program, system_program }
);

accounts!(
    SetTokenLedgerAccounts,
    get_set_token_ledger_accounts,
    { token_ledger, token_account }
);

// -----------------------------------------------------------------------------
// Instructions with optional accounts
// -----------------------------------------------------------------------------
const IDX_TOKEN_PROGRAM: usize = 0;
const IDX_USER_TRANSFER_AUTHORITY: usize = 1;
const IDX_USER_SOURCE_TOKEN_ACCOUNT: usize = 2;
const IDX_USER_DESTINATION_TOKEN_ACCOUNT: usize = 3;
const IDX_DESTINATION_TOKEN_ACCOUNT: usize = 4;
const IDX_SOURCE_MINT: usize = 5;
const IDX_DESTINATION_MINT: usize = 6;
const IDX_PLATFORM_FEE_ACCOUNT: usize = 7;
const IDX_TOKEN_2022_PROGRAM: usize = 8;
const IDX_EVENT_AUTHORITY: usize = 9;
const IDX_PROGRAM: usize = 10;

/// Accounts for the `exact_out_route` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ExactOutRouteAccounts {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Option<Pubkey>,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Option<Pubkey>,
    pub token_2022_program: Option<Pubkey>,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ExactOutRouteAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(ExactOutRouteAccounts {
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            user_transfer_authority: get_req(IDX_USER_TRANSFER_AUTHORITY, "user_transfer_authority")?,
            user_source_token_account: get_req(IDX_USER_SOURCE_TOKEN_ACCOUNT, "user_source_token_account")?,
            user_destination_token_account: get_req(IDX_USER_DESTINATION_TOKEN_ACCOUNT, "user_destination_token_account")?,
            destination_token_account: get_opt(IDX_DESTINATION_TOKEN_ACCOUNT),
            source_mint: get_req(IDX_SOURCE_MINT, "source_mint")?,
            destination_mint: get_req(IDX_DESTINATION_MINT, "destination_mint")?,
            platform_fee_account: get_opt(IDX_PLATFORM_FEE_ACCOUNT),
            token_2022_program: get_opt(IDX_TOKEN_2022_PROGRAM),
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_exact_out_route_accounts(ix: &InstructionView) -> Result<ExactOutRouteAccounts, AccountsError> {
    ExactOutRouteAccounts::try_from(ix)
}

const IDX_RT_TOKEN_PROGRAM: usize = 0;
const IDX_RT_USER_TRANSFER_AUTHORITY: usize = 1;
const IDX_RT_USER_SOURCE_TOKEN_ACCOUNT: usize = 2;
const IDX_RT_USER_DESTINATION_TOKEN_ACCOUNT: usize = 3;
const IDX_RT_DESTINATION_TOKEN_ACCOUNT: usize = 4;
const IDX_RT_DESTINATION_MINT: usize = 5;
const IDX_RT_PLATFORM_FEE_ACCOUNT: usize = 6;
const IDX_RT_EVENT_AUTHORITY: usize = 7;
const IDX_RT_PROGRAM: usize = 8;

/// Accounts for the `route` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RouteAccounts {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Option<Pubkey>,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Option<Pubkey>,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RouteAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RouteAccounts {
            token_program: get_req(IDX_RT_TOKEN_PROGRAM, "token_program")?,
            user_transfer_authority: get_req(IDX_RT_USER_TRANSFER_AUTHORITY, "user_transfer_authority")?,
            user_source_token_account: get_req(IDX_RT_USER_SOURCE_TOKEN_ACCOUNT, "user_source_token_account")?,
            user_destination_token_account: get_req(IDX_RT_USER_DESTINATION_TOKEN_ACCOUNT, "user_destination_token_account")?,
            destination_token_account: get_opt(IDX_RT_DESTINATION_TOKEN_ACCOUNT),
            destination_mint: get_req(IDX_RT_DESTINATION_MINT, "destination_mint")?,
            platform_fee_account: get_opt(IDX_RT_PLATFORM_FEE_ACCOUNT),
            event_authority: get_req(IDX_RT_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_RT_PROGRAM, "program")?,
        })
    }
}

pub fn get_route_accounts(ix: &InstructionView) -> Result<RouteAccounts, AccountsError> {
    RouteAccounts::try_from(ix)
}

const IDX_RTL_TOKEN_PROGRAM: usize = 0;
const IDX_RTL_USER_TRANSFER_AUTHORITY: usize = 1;
const IDX_RTL_USER_SOURCE_TOKEN_ACCOUNT: usize = 2;
const IDX_RTL_USER_DESTINATION_TOKEN_ACCOUNT: usize = 3;
const IDX_RTL_DESTINATION_TOKEN_ACCOUNT: usize = 4;
const IDX_RTL_DESTINATION_MINT: usize = 5;
const IDX_RTL_PLATFORM_FEE_ACCOUNT: usize = 6;
const IDX_RTL_TOKEN_LEDGER: usize = 7;
const IDX_RTL_EVENT_AUTHORITY: usize = 8;
const IDX_RTL_PROGRAM: usize = 9;

/// Accounts for the `route_with_token_ledger` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RouteWithTokenLedgerAccounts {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Option<Pubkey>,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Option<Pubkey>,
    pub token_ledger: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RouteWithTokenLedgerAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RouteWithTokenLedgerAccounts {
            token_program: get_req(IDX_RTL_TOKEN_PROGRAM, "token_program")?,
            user_transfer_authority: get_req(IDX_RTL_USER_TRANSFER_AUTHORITY, "user_transfer_authority")?,
            user_source_token_account: get_req(IDX_RTL_USER_SOURCE_TOKEN_ACCOUNT, "user_source_token_account")?,
            user_destination_token_account: get_req(IDX_RTL_USER_DESTINATION_TOKEN_ACCOUNT, "user_destination_token_account")?,
            destination_token_account: get_opt(IDX_RTL_DESTINATION_TOKEN_ACCOUNT),
            destination_mint: get_req(IDX_RTL_DESTINATION_MINT, "destination_mint")?,
            platform_fee_account: get_opt(IDX_RTL_PLATFORM_FEE_ACCOUNT),
            token_ledger: get_req(IDX_RTL_TOKEN_LEDGER, "token_ledger")?,
            event_authority: get_req(IDX_RTL_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_RTL_PROGRAM, "program")?,
        })
    }
}

pub fn get_route_with_token_ledger_accounts(ix: &InstructionView) -> Result<RouteWithTokenLedgerAccounts, AccountsError> {
    RouteWithTokenLedgerAccounts::try_from(ix)
}

const IDX_SA_TOKEN_PROGRAM: usize = 0;
const IDX_SA_PROGRAM_AUTHORITY: usize = 1;
const IDX_SA_USER_TRANSFER_AUTHORITY: usize = 2;
const IDX_SA_SOURCE_TOKEN_ACCOUNT: usize = 3;
const IDX_SA_PROGRAM_SOURCE_TOKEN_ACCOUNT: usize = 4;
const IDX_SA_PROGRAM_DESTINATION_TOKEN_ACCOUNT: usize = 5;
const IDX_SA_DESTINATION_TOKEN_ACCOUNT: usize = 6;
const IDX_SA_SOURCE_MINT: usize = 7;
const IDX_SA_DESTINATION_MINT: usize = 8;
const IDX_SA_PLATFORM_FEE_ACCOUNT: usize = 9;
const IDX_SA_TOKEN_2022_PROGRAM: usize = 10;
const IDX_SA_EVENT_AUTHORITY: usize = 11;
const IDX_SA_PROGRAM: usize = 12;

/// Accounts for the `shared_accounts_exact_out_route` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SharedAccountsExactOutRouteAccounts {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Option<Pubkey>,
    pub token_2022_program: Option<Pubkey>,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SharedAccountsExactOutRouteAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SharedAccountsExactOutRouteAccounts {
            token_program: get_req(IDX_SA_TOKEN_PROGRAM, "token_program")?,
            program_authority: get_req(IDX_SA_PROGRAM_AUTHORITY, "program_authority")?,
            user_transfer_authority: get_req(IDX_SA_USER_TRANSFER_AUTHORITY, "user_transfer_authority")?,
            source_token_account: get_req(IDX_SA_SOURCE_TOKEN_ACCOUNT, "source_token_account")?,
            program_source_token_account: get_req(IDX_SA_PROGRAM_SOURCE_TOKEN_ACCOUNT, "program_source_token_account")?,
            program_destination_token_account: get_req(IDX_SA_PROGRAM_DESTINATION_TOKEN_ACCOUNT, "program_destination_token_account")?,
            destination_token_account: get_req(IDX_SA_DESTINATION_TOKEN_ACCOUNT, "destination_token_account")?,
            source_mint: get_req(IDX_SA_SOURCE_MINT, "source_mint")?,
            destination_mint: get_req(IDX_SA_DESTINATION_MINT, "destination_mint")?,
            platform_fee_account: get_opt(IDX_SA_PLATFORM_FEE_ACCOUNT),
            token_2022_program: get_opt(IDX_SA_TOKEN_2022_PROGRAM),
            event_authority: get_req(IDX_SA_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_SA_PROGRAM, "program")?,
        })
    }
}

pub fn get_shared_accounts_exact_out_route_accounts(ix: &InstructionView) -> Result<SharedAccountsExactOutRouteAccounts, AccountsError> {
    SharedAccountsExactOutRouteAccounts::try_from(ix)
}

/// Accounts for the `shared_accounts_route` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SharedAccountsRouteAccounts {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Option<Pubkey>,
    pub token_2022_program: Option<Pubkey>,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SharedAccountsRouteAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SharedAccountsRouteAccounts {
            token_program: get_req(IDX_SA_TOKEN_PROGRAM, "token_program")?,
            program_authority: get_req(IDX_SA_PROGRAM_AUTHORITY, "program_authority")?,
            user_transfer_authority: get_req(IDX_SA_USER_TRANSFER_AUTHORITY, "user_transfer_authority")?,
            source_token_account: get_req(IDX_SA_SOURCE_TOKEN_ACCOUNT, "source_token_account")?,
            program_source_token_account: get_req(IDX_SA_PROGRAM_SOURCE_TOKEN_ACCOUNT, "program_source_token_account")?,
            program_destination_token_account: get_req(IDX_SA_PROGRAM_DESTINATION_TOKEN_ACCOUNT, "program_destination_token_account")?,
            destination_token_account: get_req(IDX_SA_DESTINATION_TOKEN_ACCOUNT, "destination_token_account")?,
            source_mint: get_req(IDX_SA_SOURCE_MINT, "source_mint")?,
            destination_mint: get_req(IDX_SA_DESTINATION_MINT, "destination_mint")?,
            platform_fee_account: get_opt(IDX_SA_PLATFORM_FEE_ACCOUNT),
            token_2022_program: get_opt(IDX_SA_TOKEN_2022_PROGRAM),
            event_authority: get_req(IDX_SA_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_SA_PROGRAM, "program")?,
        })
    }
}

pub fn get_shared_accounts_route_accounts(ix: &InstructionView) -> Result<SharedAccountsRouteAccounts, AccountsError> {
    SharedAccountsRouteAccounts::try_from(ix)
}

const IDX_SAL_TOKEN_LEDGER: usize = 11; // for with_token_ledger variant, before event_authority

/// Accounts for the `shared_accounts_route_with_token_ledger` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SharedAccountsRouteWithTokenLedgerAccounts {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Option<Pubkey>,
    pub token_2022_program: Option<Pubkey>,
    pub token_ledger: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SharedAccountsRouteWithTokenLedgerAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SharedAccountsRouteWithTokenLedgerAccounts {
            token_program: get_req(IDX_SA_TOKEN_PROGRAM, "token_program")?,
            program_authority: get_req(IDX_SA_PROGRAM_AUTHORITY, "program_authority")?,
            user_transfer_authority: get_req(IDX_SA_USER_TRANSFER_AUTHORITY, "user_transfer_authority")?,
            source_token_account: get_req(IDX_SA_SOURCE_TOKEN_ACCOUNT, "source_token_account")?,
            program_source_token_account: get_req(IDX_SA_PROGRAM_SOURCE_TOKEN_ACCOUNT, "program_source_token_account")?,
            program_destination_token_account: get_req(IDX_SA_PROGRAM_DESTINATION_TOKEN_ACCOUNT, "program_destination_token_account")?,
            destination_token_account: get_req(IDX_SA_DESTINATION_TOKEN_ACCOUNT, "destination_token_account")?,
            source_mint: get_req(IDX_SA_SOURCE_MINT, "source_mint")?,
            destination_mint: get_req(IDX_SA_DESTINATION_MINT, "destination_mint")?,
            platform_fee_account: get_opt(IDX_SA_PLATFORM_FEE_ACCOUNT),
            token_2022_program: get_opt(IDX_SA_TOKEN_2022_PROGRAM),
            token_ledger: get_req(IDX_SAL_TOKEN_LEDGER, "token_ledger")?,
            event_authority: get_req(IDX_SA_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_SA_PROGRAM, "program")?,
        })
    }
}

pub fn get_shared_accounts_route_with_token_ledger_accounts(ix: &InstructionView) -> Result<SharedAccountsRouteWithTokenLedgerAccounts, AccountsError> {
    SharedAccountsRouteWithTokenLedgerAccounts::try_from(ix)
}
