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
        /// Tick array account used in the swap
        tick_array
    }
);

// Exact create_pool account order from this module's bundled IDL.
accounts!(
    CreatePoolAccounts,
    get_create_pool_accounts,
    {
        pool_creator,
        amm_config,
        pool_state,
        token_mint_0,
        token_mint_1,
        token_vault_0,
        token_vault_1,
        observation_state,
        tick_array_bitmap,
        token_program_0,
        token_program_1,
        system_program,
        rent,
    }
);
