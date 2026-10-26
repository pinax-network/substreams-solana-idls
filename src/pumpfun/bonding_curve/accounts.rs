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
        bonding_curve,
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
        bonding_curve,
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
        bonding_curve,
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
        bonding_curve,
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

// Official pump-public-docs 8cda1fa, idl/pump.json::create_v2.
accounts!(CreateV2Accounts, get_create_v2_accounts, {
    mint,
    mint_authority,
    bonding_curve,
    associated_bonding_curve,
    global,
    user,
    system_program,
    token_program,
    associated_token_program,
    mayhem_program_id,
    global_params,
    sol_vault,
    mayhem_state,
    mayhem_token_vault,
    event_authority,
    program,
});

/// Optional token quote selected by `create_v2` remaining accounts.
/// Absence means native SOL. This does not validate the quote-control/ancestor
/// curve accounts that the program checks while executing the instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateV2QuoteAccounts {
    pub quote_mint: Pubkey,
    pub associated_quote_bonding_curve: Pubkey,
    pub quote_token_program: Pubkey,
}

pub fn get_create_v2_quote_accounts(ix: &InstructionView) -> Result<Option<CreateV2QuoteAccounts>, crate::common::accounts::AccountsError> {
    use crate::common::accounts::{to_pubkey, AccountsError};
    get_create_v2_accounts(ix)?;
    let accounts = ix.accounts();
    if accounts.len() == 16 {
        return Ok(None);
    }
    let required = |name: &'static str, index: usize| {
        let account = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
        to_pubkey(name, index, account.0)
    };
    Ok(Some(CreateV2QuoteAccounts {
        quote_mint: required("quote_mint", 16)?,
        associated_quote_bonding_curve: required("associated_quote_bonding_curve", 17)?,
        quote_token_program: required("quote_token_program", 18)?,
    }))
}
