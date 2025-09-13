use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use idls_common::accounts::AccountsError;

// -----------------------------------------------------------------------------
// Swap accounts (with optional referrer)
// -----------------------------------------------------------------------------
const IDX_STATE: usize = 0;
const IDX_POOL: usize = 1;
const IDX_TOKEN_X: usize = 2;
const IDX_TOKEN_Y: usize = 3;
const IDX_POOL_X_ACCOUNT: usize = 4;
const IDX_POOL_Y_ACCOUNT: usize = 5;
const IDX_SWAPPER_X_ACCOUNT: usize = 6;
const IDX_SWAPPER_Y_ACCOUNT: usize = 7;
const IDX_SWAPPER: usize = 8;
// Optional
const IDX_REFERRER_X_ACCOUNT: usize = 9;
const IDX_REFERRER_Y_ACCOUNT: usize = 10;
const IDX_REFERRER: usize = 11;
const IDX_PROGRAM_AUTHORITY: usize = 12;
const IDX_SYSTEM_PROGRAM: usize = 13;
const IDX_TOKEN_PROGRAM: usize = 14;
const IDX_ASSOCIATED_TOKEN_PROGRAM: usize = 15;
const IDX_RENT: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapAccounts {
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub swapper_x_account: Pubkey,
    pub swapper_y_account: Pubkey,
    pub swapper: Pubkey,
    pub referrer_x_account: Option<Pubkey>,
    pub referrer_y_account: Option<Pubkey>,
    pub referrer: Option<Pubkey>,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();

        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            idls_common::accounts::to_pubkey(name, index, a.0)
        };

        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };

        Ok(SwapAccounts {
            state: get_req(IDX_STATE, "state")?,
            pool: get_req(IDX_POOL, "pool")?,
            token_x: get_req(IDX_TOKEN_X, "token_x")?,
            token_y: get_req(IDX_TOKEN_Y, "token_y")?,
            pool_x_account: get_req(IDX_POOL_X_ACCOUNT, "pool_x_account")?,
            pool_y_account: get_req(IDX_POOL_Y_ACCOUNT, "pool_y_account")?,
            swapper_x_account: get_req(IDX_SWAPPER_X_ACCOUNT, "swapper_x_account")?,
            swapper_y_account: get_req(IDX_SWAPPER_Y_ACCOUNT, "swapper_y_account")?,
            swapper: get_req(IDX_SWAPPER, "swapper")?,
            referrer_x_account: get_opt(IDX_REFERRER_X_ACCOUNT),
            referrer_y_account: get_opt(IDX_REFERRER_Y_ACCOUNT),
            referrer: get_opt(IDX_REFERRER),
            program_authority: get_req(IDX_PROGRAM_AUTHORITY, "program_authority")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            associated_token_program: get_req(IDX_ASSOCIATED_TOKEN_PROGRAM, "associated_token_program")?,
            rent: get_req(IDX_RENT, "rent")?,
        })
    }
}

pub fn get_swap_accounts(ix: &InstructionView) -> Result<SwapAccounts, AccountsError> {
    SwapAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// Account structs for other instructions
// -----------------------------------------------------------------------------
accounts!(
    CreatePoolAccounts,
    get_create_pool_accounts,
    {
        state,
        pool,
        token_x,
        token_y,
        pool_x_account,
        pool_y_account,
        admin_x_account,
        admin_y_account,
        admin,
        project_owner,
        program_authority,
        system_program,
        token_program,
        rent
    }
);

accounts!(
    CreateProviderAccounts,
    get_create_provider_accounts,
    {
        pool,
        farm,
        provider,
        token_x,
        token_y,
        pool_x_account,
        pool_y_account,
        owner_x_account,
        owner_y_account,
        owner,
        system_program,
        token_program,
        rent
    }
);

accounts!(
    CreateStateAccounts,
    get_create_state_accounts,
    {
        state,
        admin,
        program_authority,
        system_program
    }
);

accounts!(
    AddTokensAccounts,
    get_add_tokens_accounts,
    {
        state,
        pool,
        farm,
        provider,
        token_x,
        token_y,
        token_marco,
        token_project_first,
        token_project_second,
        owner_x_account,
        owner_y_account,
        pool_x_account,
        pool_y_account,
        owner_marco_account,
        owner_project_first_account,
        owner_project_second_account,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        owner,
        program_authority,
        system_program,
        token_program,
        associated_token_program,
        rent
    }
);

accounts!(
    WithdrawBuybackAccounts,
    get_withdraw_buyback_accounts,
    {
        state,
        pool,
        token_x,
        token_y,
        buyback_x_account,
        buyback_y_account,
        pool_x_account,
        pool_y_account,
        admin,
        program_authority,
        system_program,
        token_program,
        associated_token_program,
        rent
    }
);

accounts!(
    WithdrawSharesAccounts,
    get_withdraw_shares_accounts,
    {
        state,
        pool,
        farm,
        provider,
        token_x,
        token_y,
        token_marco,
        token_project_first,
        token_project_second,
        pool_x_account,
        pool_y_account,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        owner_x_account,
        owner_y_account,
        owner_marco_account,
        owner_project_first_account,
        owner_project_second_account,
        owner,
        program_authority,
        system_program,
        token_program,
        associated_token_program,
        rent
    }
);

accounts!(
    WithdrawLpFeeAccounts,
    get_withdraw_lp_fee_accounts,
    {
        state,
        pool,
        provider,
        token_x,
        token_y,
        owner_x_account,
        owner_y_account,
        pool_x_account,
        pool_y_account,
        owner,
        program_authority,
        system_program,
        token_program,
        associated_token_program,
        rent
    }
);

accounts!(
    WithdrawProjectFeeAccounts,
    get_withdraw_project_fee_accounts,
    {
        state,
        pool,
        token_x,
        token_y,
        project_owner_x_account,
        project_owner_y_account,
        pool_x_account,
        pool_y_account,
        project_owner,
        program_authority,
        system_program,
        token_program,
        associated_token_program,
        rent
    }
);

accounts!(
    CreateFarmAccounts,
    get_create_farm_accounts,
    {
        state,
        pool,
        farm,
        token_x,
        token_y,
        token_marco,
        token_marco_account,
        admin_marco_account,
        admin,
        program_authority,
        system_program,
        token_program,
        rent
    }
);

accounts!(
    CreateDualFarmAccounts,
    get_create_dual_farm_accounts,
    {
        state,
        pool,
        farm,
        token_x,
        token_y,
        token_marco,
        token_project_first,
        token_marco_account,
        token_project_first_account,
        admin_marco_account,
        admin_project_first_account,
        admin,
        program_authority,
        system_program,
        token_program,
        rent
    }
);

accounts!(
    CreateTripleFarmAccounts,
    get_create_triple_farm_accounts,
    {
        state,
        pool,
        farm,
        token_x,
        token_y,
        token_marco,
        token_project_first,
        token_project_second,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        admin_marco_account,
        admin_project_first_account,
        admin_project_second_account,
        admin,
        program_authority,
        system_program,
        token_program,
        rent
    }
);

accounts!(
    WithdrawRewardsAccounts,
    get_withdraw_rewards_accounts,
    {
        state,
        pool,
        farm,
        provider,
        token_x,
        token_y,
        token_marco,
        token_project_first,
        token_project_second,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        owner_marco_account,
        owner_project_first_account,
        owner_project_second_account,
        owner,
        program_authority,
        system_program,
        token_program,
        associated_token_program,
        rent
    }
);

accounts!(
    ClosePoolAccounts,
    get_close_pool_accounts,
    {
        state,
        pool,
        farm,
        token_x,
        token_y,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        pool_x_account,
        pool_y_account,
        buyback_x_account,
        buyback_y_account,
        admin,
        program_authority,
        token_program
    }
);

accounts!(
    WithdrawMercantiFeeAccounts,
    get_withdraw_mercanti_fee_accounts,
    {
        state,
        pool,
        token_x,
        token_y,
        mercanti_x_account,
        mercanti_y_account,
        pool_x_account,
        pool_y_account,
        admin,
        program_authority,
        token_program
    }
);

accounts!(
    AddSupplyAccounts,
    get_add_supply_accounts,
    {
        state,
        pool,
        farm,
        token_x,
        token_y,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        admin_marco_account,
        admin_project_first_account,
        admin_project_second_account,
        admin,
        token_program
    }
);

accounts!(
    UpdateFeesAccounts,
    get_update_fees_accounts,
    {
        state,
        pool,
        token_x,
        token_y,
        admin,
        program_authority
    }
);

accounts!(
    ResetFarmAccounts,
    get_reset_farm_accounts,
    {
        state,
        pool,
        farm,
        token_x,
        token_y,
        token_marco,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        admin_marco_account,
        admin_project_first_account,
        admin_project_second_account,
        admin,
        program_authority,
        system_program,
        token_program,
        rent
    }
);

accounts!(
    UpdateRewardTokensAccounts,
    get_update_reward_tokens_accounts,
    {
        state,
        pool,
        farm,
        token_marco_account,
        token_project_first_account,
        token_project_second_account,
        token_marco,
        new_token_marco_account,
        admin,
        program_authority,
        system_program,
        token_program
    }
);
