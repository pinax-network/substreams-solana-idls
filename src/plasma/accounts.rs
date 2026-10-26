//! Plasma account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

accounts!(
    SwapAccounts,
    get_swap_accounts,
    {
        plasma_program,
        log_authority,
        pool,
        trader,
        base_account,
        quote_account,
        base_vault,
        quote_vault,
        token_program
    }
);

accounts!(
    AddLiquidityAccounts,
    get_add_liquidity_accounts,
    {
        plasma_program,
        log_authority,
        pool,
        trader,
        lp_position,
        base_account,
        quote_account,
        base_vault,
        quote_vault,
        token_program
    }
);

// Ellipsis-Labs/plasma 86fb350, idl/plasma.json::InitializePool.
accounts!(InitializePoolAccounts, get_initialize_pool_accounts, {
    plasma_program,
    log_authority,
    pool,
    pool_creator,
    base_mint,
    quote_mint,
    base_vault,
    quote_vault,
    system_program,
    token_program,
});
