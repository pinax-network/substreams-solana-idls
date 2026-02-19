//! Lifinity AMM V2 account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

accounts!(
    SwapAccounts,
    get_swap_accounts,
    {
        authority,
        amm,
        user_transfer_authority,
        source_info,
        destination_info,
        swap_source,
        swap_destination,
        pool_mint,
        fee_account,
        token_program,
        oracle_main_account,
        oracle_sub_account,
        oracle_pc_account
    }
);

accounts!(
    DepositAllTokenTypesAccounts,
    get_deposit_all_token_types_accounts,
    {
        amm,
        authority,
        user_transfer_authority_info,
        source_a_info,
        source_b_info,
        token_a,
        token_b,
        pool_mint,
        destination,
        token_program
    }
);

accounts!(
    WithdrawAllTokenTypesAccounts,
    get_withdraw_all_token_types_accounts,
    {
        amm,
        authority,
        user_transfer_authority_info,
        source_info,
        token_a,
        token_b,
        pool_mint,
        dest_token_a_info,
        dest_token_b_info,
        token_program
    }
);
