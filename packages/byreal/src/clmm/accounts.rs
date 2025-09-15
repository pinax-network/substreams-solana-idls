use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use common::accounts;

// -----------------------------------------------------------------------------
// Swap v2 accounts
// -----------------------------------------------------------------------------
accounts!(
    SwapV2Accounts,
    get_swap_v2_accounts,
    {
        /// The user performing the swap
        payer,
        /// The factory state to read protocol fees
        amm_config,
        /// The program account of the pool in which the swap will be performed
        pool_state,
        /// The user token account for input token
        input_token_account,
        /// The user token account for output token
        output_token_account,
        /// The vault token account for input token
        input_vault,
        /// The vault token account for output token
        output_vault,
        /// The program account for the most recent oracle observation
        observation_state,
        /// SPL program for token transfers
        token_program,
        /// SPL token program 2022 for token transfers
        token_program_2022,
        /// Memo program
        memo_program,
        /// The mint of input token vault
        input_vault_mint,
        /// The mint of output token vault
        output_vault_mint
    }
);
