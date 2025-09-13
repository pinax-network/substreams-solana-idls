use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use common::accounts;

// -----------------------------------------------------------------------------
// Buy exact in accounts
// -----------------------------------------------------------------------------
accounts!(
    BuyExactInAccounts,
    get_buy_exact_in_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being bought)
        user_base_token,
        /// The user's token account for quote tokens (tokens being sold)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);

// -----------------------------------------------------------------------------
// Buy exact out accounts
// -----------------------------------------------------------------------------
accounts!(
    BuyExactOutAccounts,
    get_buy_exact_out_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being bought)
        user_base_token,
        /// The user's token account for quote tokens (tokens being sold)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);

// -----------------------------------------------------------------------------
// Sell exact in accounts
// -----------------------------------------------------------------------------
accounts!(
    SellExactInAccounts,
    get_sell_exact_in_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being sold)
        user_base_token,
        /// The user's token account for quote tokens (tokens being bought)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);

// -----------------------------------------------------------------------------
// Sell exact out accounts
// -----------------------------------------------------------------------------
accounts!(
    SellExactOutAccounts,
    get_sell_exact_out_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being sold)
        user_base_token,
        /// The user's token account for quote tokens (tokens being bought)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);
