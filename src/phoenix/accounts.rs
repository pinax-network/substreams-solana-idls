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
        /// Phoenix program
        phoenix_program,
        /// Phoenix log authority
        log_authority,
        /// Market state account
        market,
        /// Trader performing the swap
        trader,
        /// Trader base token account
        base_account,
        /// Trader quote token account
        quote_account,
        /// Base vault PDA
        base_vault,
        /// Quote vault PDA
        quote_vault,
        /// SPL token program
        token_program
    }
);

// Exact InitializeMarket account order from this module's bundled IDL.
accounts!(
    InitializeMarketAccounts,
    get_initialize_market_accounts,
    {
        phoenix_program,
        log_authority,
        market,
        market_creator,
        base_mint,
        quote_mint,
        base_vault,
        quote_vault,
        system_program,
        token_program,
    }
);
