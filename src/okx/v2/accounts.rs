//! OKX DEX Aggregation Router V2 account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

// -----------------------------------------------------------------------------
// Swap V3 accounts (fixed prefix accounts before dynamic route accounts)
// -----------------------------------------------------------------------------
accounts!(
    SwapV3Accounts,
    get_swap_v3_accounts,
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
        destination_mint
    }
);
