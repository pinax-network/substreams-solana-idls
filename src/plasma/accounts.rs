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
