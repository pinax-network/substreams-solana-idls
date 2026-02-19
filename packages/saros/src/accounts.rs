//! Saros swap account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use common::accounts;

// -----------------------------------------------------------------------------
// Swap accounts (SPL Token Swap compatible layout)
// -----------------------------------------------------------------------------
accounts!(
    SwapAccounts,
    get_swap_accounts,
    {
        /// The swap pool account
        swap,
        /// Pool authority (PDA)
        authority,
        /// User transfer authority
        user_transfer_authority,
        /// User source token account
        user_source,
        /// Pool source token account (vault)
        pool_source,
        /// Pool destination token account (vault)
        pool_destination,
        /// User destination token account
        user_destination,
        /// Pool mint for LP tokens
        pool_mint,
        /// Fee account
        fee_account,
        /// SPL Token program
        token_program
    }
);
