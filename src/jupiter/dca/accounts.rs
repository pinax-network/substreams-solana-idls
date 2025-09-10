use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::accounts::{to_pubkey, AccountsError};

// -----------------------------------------------------------------------------
// Simple instructions
// -----------------------------------------------------------------------------
accounts!(
    OpenDcaAccounts,
    get_open_dca_accounts,
    {
        dca,
        user,
        input_mint,
        output_mint,
        user_ata,
        in_ata,
        out_ata,
        system_program,
        token_program,
        associated_token_program,
        event_authority,
        program
    }
);

accounts!(
    OpenDcaV2Accounts,
    get_open_dca_v2_accounts,
    {
        dca,
        user,
        payer,
        input_mint,
        output_mint,
        user_ata,
        in_ata,
        out_ata,
        system_program,
        token_program,
        associated_token_program,
        event_authority,
        program
    }
);

accounts!(
    CloseDcaAccounts,
    get_close_dca_accounts,
    {
        user,
        dca,
        input_mint,
        output_mint,
        in_ata,
        out_ata,
        user_in_ata,
        user_out_ata,
        system_program,
        token_program,
        associated_token_program,
        event_authority,
        program
    }
);

accounts!(
    DepositAccounts,
    get_deposit_accounts,
    {
        user,
        dca,
        in_ata,
        user_in_ata,
        token_program,
        event_authority,
        program
    }
);

accounts!(
    WithdrawFeesAccounts,
    get_withdraw_fees_accounts,
    {
        admin,
        mint,
        fee_authority,
        program_fee_ata,
        admin_fee_ata,
        system_program,
        token_program,
        associated_token_program
    }
);

accounts!(
    InitiateFlashFillAccounts,
    get_initiate_flash_fill_accounts,
    {
        keeper,
        dca,
        input_mint,
        keeper_in_ata,
        in_ata,
        out_ata,
        instructions_sysvar,
        system_program,
        token_program,
        associated_token_program
    }
);

accounts!(
    FulfillFlashFillAccounts,
    get_fulfill_flash_fill_accounts,
    {
        keeper,
        dca,
        input_mint,
        output_mint,
        keeper_in_ata,
        in_ata,
        out_ata,
        fee_authority,
        fee_ata,
        instructions_sysvar,
        system_program,
        token_program,
        associated_token_program,
        event_authority,
        program
    }
);

accounts!(
    InitiateDlmmFillAccounts,
    get_initiate_dlmm_fill_accounts,
    {
        keeper,
        dca,
        input_mint,
        keeper_in_ata,
        in_ata,
        out_ata,
        instructions_sysvar,
        system_program,
        token_program,
        associated_token_program
    }
);

accounts!(
    FulfillDlmmFillAccounts,
    get_fulfill_dlmm_fill_accounts,
    {
        keeper,
        dca,
        input_mint,
        output_mint,
        keeper_in_ata,
        in_ata,
        out_ata,
        fee_authority,
        fee_ata,
        instructions_sysvar,
        system_program,
        token_program,
        associated_token_program,
        event_authority,
        program
    }
);

// -----------------------------------------------------------------------------
// Instructions with optional accounts
// -----------------------------------------------------------------------------
const IDX_WD_USER: usize = 0;
const IDX_WD_DCA: usize = 1;
const IDX_WD_INPUT_MINT: usize = 2;
const IDX_WD_OUTPUT_MINT: usize = 3;
const IDX_WD_DCA_ATA: usize = 4;
const IDX_WD_USER_IN_ATA: usize = 5;
const IDX_WD_USER_OUT_ATA: usize = 6;
const IDX_WD_SYSTEM_PROGRAM: usize = 7;
const IDX_WD_TOKEN_PROGRAM: usize = 8;
const IDX_WD_ASSOCIATED_TOKEN_PROGRAM: usize = 9;
const IDX_WD_EVENT_AUTHORITY: usize = 10;
const IDX_WD_PROGRAM: usize = 11;

/// Accounts for the `withdraw` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawAccounts {
    pub user: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub dca_ata: Pubkey,
    pub user_in_ata: Option<Pubkey>,
    pub user_out_ata: Option<Pubkey>,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for WithdrawAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(WithdrawAccounts {
            user: get_req(IDX_WD_USER, "user")?,
            dca: get_req(IDX_WD_DCA, "dca")?,
            input_mint: get_req(IDX_WD_INPUT_MINT, "input_mint")?,
            output_mint: get_req(IDX_WD_OUTPUT_MINT, "output_mint")?,
            dca_ata: get_req(IDX_WD_DCA_ATA, "dca_ata")?,
            user_in_ata: get_opt(IDX_WD_USER_IN_ATA),
            user_out_ata: get_opt(IDX_WD_USER_OUT_ATA),
            system_program: get_req(IDX_WD_SYSTEM_PROGRAM, "system_program")?,
            token_program: get_req(IDX_WD_TOKEN_PROGRAM, "token_program")?,
            associated_token_program: get_req(IDX_WD_ASSOCIATED_TOKEN_PROGRAM, "associated_token_program")?,
            event_authority: get_req(IDX_WD_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_WD_PROGRAM, "program")?,
        })
    }
}

pub fn get_withdraw_accounts(ix: &InstructionView) -> Result<WithdrawAccounts, AccountsError> {
    WithdrawAccounts::try_from(ix)
}

const IDX_TR_KEEPER: usize = 0;
const IDX_TR_DCA: usize = 1;
const IDX_TR_USER: usize = 2;
const IDX_TR_OUTPUT_MINT: usize = 3;
const IDX_TR_DCA_OUT_ATA: usize = 4;
const IDX_TR_USER_OUT_ATA: usize = 5;
const IDX_TR_INTERMEDIATE_ACCOUNT: usize = 6;
const IDX_TR_SYSTEM_PROGRAM: usize = 7;
const IDX_TR_TOKEN_PROGRAM: usize = 8;
const IDX_TR_ASSOCIATED_TOKEN_PROGRAM: usize = 9;
const IDX_TR_EVENT_AUTHORITY: usize = 10;
const IDX_TR_PROGRAM: usize = 11;

/// Accounts for the `transfer` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TransferAccounts {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub user: Pubkey,
    pub output_mint: Pubkey,
    pub dca_out_ata: Pubkey,
    pub user_out_ata: Option<Pubkey>,
    pub intermediate_account: Option<Pubkey>,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for TransferAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(TransferAccounts {
            keeper: get_req(IDX_TR_KEEPER, "keeper")?,
            dca: get_req(IDX_TR_DCA, "dca")?,
            user: get_req(IDX_TR_USER, "user")?,
            output_mint: get_req(IDX_TR_OUTPUT_MINT, "output_mint")?,
            dca_out_ata: get_req(IDX_TR_DCA_OUT_ATA, "dca_out_ata")?,
            user_out_ata: get_opt(IDX_TR_USER_OUT_ATA),
            intermediate_account: get_opt(IDX_TR_INTERMEDIATE_ACCOUNT),
            system_program: get_req(IDX_TR_SYSTEM_PROGRAM, "system_program")?,
            token_program: get_req(IDX_TR_TOKEN_PROGRAM, "token_program")?,
            associated_token_program: get_req(IDX_TR_ASSOCIATED_TOKEN_PROGRAM, "associated_token_program")?,
            event_authority: get_req(IDX_TR_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_TR_PROGRAM, "program")?,
        })
    }
}

pub fn get_transfer_accounts(ix: &InstructionView) -> Result<TransferAccounts, AccountsError> {
    TransferAccounts::try_from(ix)
}

const IDX_EC_KEEPER: usize = 0;
const IDX_EC_DCA: usize = 1;
const IDX_EC_INPUT_MINT: usize = 2;
const IDX_EC_OUTPUT_MINT: usize = 3;
const IDX_EC_IN_ATA: usize = 4;
const IDX_EC_OUT_ATA: usize = 5;
const IDX_EC_USER: usize = 6;
const IDX_EC_USER_OUT_ATA: usize = 7;
const IDX_EC_INIT_USER_OUT_ATA: usize = 8;
const IDX_EC_INTERMEDIATE_ACCOUNT: usize = 9;
const IDX_EC_SYSTEM_PROGRAM: usize = 10;
const IDX_EC_TOKEN_PROGRAM: usize = 11;
const IDX_EC_ASSOCIATED_TOKEN_PROGRAM: usize = 12;
const IDX_EC_EVENT_AUTHORITY: usize = 13;
const IDX_EC_PROGRAM: usize = 14;

/// Accounts for the `end_and_close` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct EndAndCloseAccounts {
    pub keeper: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub in_ata: Pubkey,
    pub out_ata: Pubkey,
    pub user: Pubkey,
    pub user_out_ata: Option<Pubkey>,
    pub init_user_out_ata: Option<Pubkey>,
    pub intermediate_account: Option<Pubkey>,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for EndAndCloseAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(EndAndCloseAccounts {
            keeper: get_req(IDX_EC_KEEPER, "keeper")?,
            dca: get_req(IDX_EC_DCA, "dca")?,
            input_mint: get_req(IDX_EC_INPUT_MINT, "input_mint")?,
            output_mint: get_req(IDX_EC_OUTPUT_MINT, "output_mint")?,
            in_ata: get_req(IDX_EC_IN_ATA, "in_ata")?,
            out_ata: get_req(IDX_EC_OUT_ATA, "out_ata")?,
            user: get_req(IDX_EC_USER, "user")?,
            user_out_ata: get_opt(IDX_EC_USER_OUT_ATA),
            init_user_out_ata: get_opt(IDX_EC_INIT_USER_OUT_ATA),
            intermediate_account: get_opt(IDX_EC_INTERMEDIATE_ACCOUNT),
            system_program: get_req(IDX_EC_SYSTEM_PROGRAM, "system_program")?,
            token_program: get_req(IDX_EC_TOKEN_PROGRAM, "token_program")?,
            associated_token_program: get_req(IDX_EC_ASSOCIATED_TOKEN_PROGRAM, "associated_token_program")?,
            event_authority: get_req(IDX_EC_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_EC_PROGRAM, "program")?,
        })
    }
}

pub fn get_end_and_close_accounts(ix: &InstructionView) -> Result<EndAndCloseAccounts, AccountsError> {
    EndAndCloseAccounts::try_from(ix)
}
