//! DFlow Swap Aggregator V4 account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

// -----------------------------------------------------------------------------
// Swap accounts
// -----------------------------------------------------------------------------
accounts!(
    SwapAccounts,
    get_swap_accounts,
    {
        /// The user/payer performing the swap
        payer,
        /// Source token account (user input)
        source_token_account,
        /// Destination token account (user output)
        destination_token_account,
        /// Source token mint
        source_mint,
        /// Destination token mint
        destination_mint,
        /// SPL Token program
        token_program
    }
);
