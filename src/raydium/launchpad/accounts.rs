use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

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

// Exact initialize account order from this module's bundled IDL.
accounts!(
    InitializeAccounts,
    get_initialize_accounts,
    {
        payer,
        creator,
        global_config,
        platform_config,
        authority,
        pool_state,
        base_mint,
        quote_mint,
        base_vault,
        quote_vault,
        metadata_account,
        base_token_program,
        quote_token_program,
        metadata_program,
        system_program,
        rent_program,
        event_authority,
        program,
    }
);

// Exact initialize_v2 account order from this module's bundled IDL.
accounts!(
    InitializeV2Accounts,
    get_initialize_v2_accounts,
    {
        payer,
        creator,
        global_config,
        platform_config,
        authority,
        pool_state,
        base_mint,
        quote_mint,
        base_vault,
        quote_vault,
        metadata_account,
        base_token_program,
        quote_token_program,
        metadata_program,
        system_program,
        rent_program,
        event_authority,
        program,
    }
);

// Exact initialize_with_token_2022 account order from this module's bundled IDL.
accounts!(
    InitializeWithToken2022Accounts,
    get_initialize_with_token_2022_accounts,
    {
        payer,
        creator,
        global_config,
        platform_config,
        authority,
        pool_state,
        base_mint,
        quote_mint,
        base_vault,
        quote_vault,
        base_token_program,
        quote_token_program,
        system_program,
        event_authority,
        program,
    }
);
