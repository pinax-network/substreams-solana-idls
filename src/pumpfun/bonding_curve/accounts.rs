use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

// -----------------------------------------------------------------------------
// Account structs
// -----------------------------------------------------------------------------
accounts!(
    InitializeAccounts,
    get_initialize_accounts,
    {
        global_state,
        admin,
        system_program,
        event_authority,
        program
    }
);

accounts!(
    SetParamsAccounts,
    get_set_params_accounts,
    {
        curve_config,
        admin,
        global_state,
        event_authority,
        program
    }
);

accounts!(
    CreateAccounts,
    get_create_accounts,
    {
        mint,
        mint_authority,
        curve_config,
        curve,
        global_state,
        metadata_program,
        metadata,
        user,
        system_program,
        token_program,
        associated_token_program,
        rent,
        event_authority,
        program
    }
);

accounts!(
    BuyAccounts,
    get_buy_accounts,
    {
        global_state,
        fee_recipient,
        mint,
        curve_config,
        token_vault,
        user_state,
        user,
        system_program,
        token_program,
        creator_vault,
        event_authority,
        program
    }
);

accounts!(
    SellAccounts,
    get_sell_accounts,
    {
        global_state,
        fee_recipient,
        mint,
        curve_config,
        token_vault,
        user_state,
        user,
        system_program,
        creator_vault,
        token_program,
        event_authority,
        program
    }
);

accounts!(
    WithdrawAccounts,
    get_withdraw_accounts,
    {
        global_state,
        fee_recipient,
        system_program,
        event_authority,
        program
    }
);
