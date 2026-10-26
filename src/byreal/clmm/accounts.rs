use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

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

// Exact create_pool account order from this module's bundled IDL.
accounts!(
    CreatePoolAccounts,
    get_create_pool_accounts,
    {
        pool_creator,
        pool_manager,
        admin_group,
        amm_config,
        pool_state,
        offchain_reward_config,
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
