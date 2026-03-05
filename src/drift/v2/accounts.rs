use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

// -----------------------------------------------------------------------------
// Account structs
// -----------------------------------------------------------------------------
accounts!(
    BeginSwapAccounts,
    get_begin_swap_accounts,
    {
        state,
        user,
        user_stats,
        authority,
        out_spot_market_vault,
        in_spot_market_vault,
        out_token_account,
        in_token_account,
        token_program,
        drift_signer,
        instructions
    }
);

accounts!(
    EndSwapAccounts,
    get_end_swap_accounts,
    {
        state,
        user,
        user_stats,
        authority,
        out_spot_market_vault,
        in_spot_market_vault,
        out_token_account,
        in_token_account,
        token_program,
        drift_signer,
        instructions
    }
);
