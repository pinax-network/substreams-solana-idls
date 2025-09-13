use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use idls_common::accounts;

// -----------------------------------------------------------------------------
// Deposit accounts
// -----------------------------------------------------------------------------
accounts!(
    DepositAccounts,
    get_deposit_accounts,
    {
        /// The user providing liquidity
        user,
        /// The user's pool token account
        user_pool_token,
        /// Pool token mint
        mint,
        /// Pool state account
        pool,
        /// Pool authority PDA
        pool_authority,
        /// Vault state account
        vault,
        /// Vault authority PDA
        vault_authority,
        /// SPL Token program
        token_program,
        /// SPL Token 2022 program
        token_program_2022
    }
);

// -----------------------------------------------------------------------------
// Withdraw accounts
// -----------------------------------------------------------------------------
accounts!(
    WithdrawAccounts,
    get_withdraw_accounts,
    {
        /// The user removing liquidity
        user,
        /// The user's pool token account
        user_pool_token,
        /// Pool token mint
        mint,
        /// Pool state account
        pool,
        /// Withdraw authority PDA
        withdraw_authority,
        /// Vault state account
        vault,
        /// Vault authority PDA
        vault_authority,
        /// Vault program id
        vault_program,
        /// SPL Token program
        token_program,
        /// SPL Token 2022 program
        token_program_2022
    }
);

// -----------------------------------------------------------------------------
// Swap accounts
// -----------------------------------------------------------------------------
accounts!(
    SwapAccounts,
    get_swap_accounts,
    {
        /// The user performing the swap
        user,
        /// User token account for the input token
        user_token_in,
        /// User token account for the output token
        user_token_out,
        /// Pool vault for the input token
        vault_token_in,
        /// Pool vault for the output token
        vault_token_out,
        /// Beneficiary token account for fees
        beneficiary_token_out,
        /// Pool state account
        pool,
        /// Withdraw authority PDA
        withdraw_authority,
        /// Vault state account
        vault,
        /// Vault authority PDA
        vault_authority,
        /// Vault program id
        vault_program,
        /// SPL Token program
        token_program
    }
);
