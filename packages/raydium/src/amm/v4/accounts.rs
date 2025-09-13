use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use idls_common::accounts::AccountsError;

// -----------------------------------------------------------------------------
// Swap accounts (shared by `swap_base_in` and `swap_base_out`)
// -----------------------------------------------------------------------------
const IDX_TOKEN_PROGRAM: usize = 0;
const IDX_AMM: usize = 1;
const IDX_AMM_AUTHORITY: usize = 2;
const IDX_AMM_OPEN_ORDERS: usize = 3;
const IDX_AMM_TARGET_ORDERS: usize = 4; // optional
const IDX_POOL_COIN_TOKEN_ACCOUNT: usize = 5;
const IDX_POOL_PC_TOKEN_ACCOUNT: usize = 6;
const IDX_SERUM_PROGRAM: usize = 7;
const IDX_SERUM_MARKET: usize = 8;
const IDX_SERUM_BIDS: usize = 9;
const IDX_SERUM_ASKS: usize = 10;
const IDX_SERUM_EVENT_QUEUE: usize = 11;
const IDX_SERUM_COIN_VAULT_ACCOUNT: usize = 12;
const IDX_SERUM_PC_VAULT_ACCOUNT: usize = 13;
const IDX_SERUM_VAULT_SIGNER: usize = 14;
const IDX_UER_SOURCE_TOKEN_ACCOUNT: usize = 15;
const IDX_UER_DESTINATION_TOKEN_ACCOUNT: usize = 16;
const IDX_USER_SOURCE_OWNER: usize = 17;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseAccounts {
    pub token_program: Pubkey,
    pub amm: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_target_orders: Option<Pubkey>,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub uer_source_token_account: Pubkey,
    pub uer_destination_token_account: Pubkey,
    pub user_source_owner: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapBaseAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();

        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            idls_common::accounts::to_pubkey(name, index, a.0)
        };

        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };

        Ok(SwapBaseAccounts {
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            amm: get_req(IDX_AMM, "amm")?,
            amm_authority: get_req(IDX_AMM_AUTHORITY, "amm_authority")?,
            amm_open_orders: get_req(IDX_AMM_OPEN_ORDERS, "amm_open_orders")?,
            amm_target_orders: get_opt(IDX_AMM_TARGET_ORDERS),
            pool_coin_token_account: get_req(IDX_POOL_COIN_TOKEN_ACCOUNT, "pool_coin_token_account")?,
            pool_pc_token_account: get_req(IDX_POOL_PC_TOKEN_ACCOUNT, "pool_pc_token_account")?,
            serum_program: get_req(IDX_SERUM_PROGRAM, "serum_program")?,
            serum_market: get_req(IDX_SERUM_MARKET, "serum_market")?,
            serum_bids: get_req(IDX_SERUM_BIDS, "serum_bids")?,
            serum_asks: get_req(IDX_SERUM_ASKS, "serum_asks")?,
            serum_event_queue: get_req(IDX_SERUM_EVENT_QUEUE, "serum_event_queue")?,
            serum_coin_vault_account: get_req(IDX_SERUM_COIN_VAULT_ACCOUNT, "serum_coin_vault_account")?,
            serum_pc_vault_account: get_req(IDX_SERUM_PC_VAULT_ACCOUNT, "serum_pc_vault_account")?,
            serum_vault_signer: get_req(IDX_SERUM_VAULT_SIGNER, "serum_vault_signer")?,
            uer_source_token_account: get_req(IDX_UER_SOURCE_TOKEN_ACCOUNT, "uer_source_token_account")?,
            uer_destination_token_account: get_req(IDX_UER_DESTINATION_TOKEN_ACCOUNT, "uer_destination_token_account")?,
            user_source_owner: get_req(IDX_USER_SOURCE_OWNER, "user_source_owner")?,
        })
    }
}

pub fn get_swap_base_in_accounts(ix: &InstructionView) -> Result<SwapBaseAccounts, AccountsError> {
    SwapBaseAccounts::try_from(ix)
}

pub fn get_swap_base_out_accounts(ix: &InstructionView) -> Result<SwapBaseAccounts, AccountsError> {
    SwapBaseAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// Account structs for other instructions
// -----------------------------------------------------------------------------
accounts!(
    Initialize2Accounts,
    get_initialize2_accounts,
    {
        token_program,
        spl_associated_token_account,
        system_program,
        rent,
        amm,
        amm_authority,
        amm_open_orders,
        lp_mint,
        coin_mint,
        pc_mint,
        pool_coin_token_account,
        pool_pc_token_account,
        pool_withdraw_queue,
        amm_target_orders,
        pool_temp_lp,
        serum_program,
        serum_market,
        user_wallet,
        user_token_coin,
        user_token_pc,
        user_lp_token_account
    }
);

accounts!(
    InitializeAccounts,
    get_initialize_accounts,
    {
        token_program,
        system_program,
        rent,
        amm,
        amm_authority,
        amm_open_orders,
        lp_mint_address,
        coin_mint_address,
        pc_mint_address,
        pool_coin_token_account,
        pool_pc_token_account,
        pool_withdraw_queue,
        pool_target_orders_account,
        user_lp_token_account,
        pool_temp_lp_token_account,
        serum_program,
        serum_market,
        user_wallet
    }
);

accounts!(
    MonitorStepAccounts,
    get_monitor_step_accounts,
    {
        token_program,
        rent,
        clock,
        amm,
        amm_authority,
        amm_open_orders,
        amm_target_orders,
        pool_coin_token_account,
        pool_pc_token_account,
        pool_withdraw_queue,
        serum_program,
        serum_market,
        serum_coin_vault_account,
        serum_pc_vault_account,
        serum_vault_signer,
        serum_req_q,
        serum_event_q,
        serum_bids,
        serum_asks
    }
);

accounts!(
    DepositAccounts,
    get_deposit_accounts,
    {
        token_program,
        amm,
        amm_authority,
        amm_open_orders,
        amm_target_orders,
        lp_mint_address,
        pool_coin_token_account,
        pool_pc_token_account,
        serum_market,
        user_coin_token_account,
        user_pc_token_account,
        user_lp_token_account,
        user_owner,
        serum_event_queue
    }
);

accounts!(
    WithdrawAccounts,
    get_withdraw_accounts,
    {
        token_program,
        amm,
        amm_authority,
        amm_open_orders,
        amm_target_orders,
        lp_mint_address,
        pool_coin_token_account,
        pool_pc_token_account,
        pool_withdraw_queue,
        pool_temp_lp_token_account,
        serum_program,
        serum_market,
        serum_coin_vault_account,
        serum_pc_vault_account,
        serum_vault_signer,
        user_lp_token_account,
        uer_coin_token_account,
        uer_pc_token_account,
        user_owner,
        serum_event_q,
        serum_bids,
        serum_asks
    }
);

accounts!(
    MigrateToOpenBookAccounts,
    get_migrate_to_open_book_accounts,
    {
        token_program,
        system_program,
        rent,
        amm,
        amm_authority,
        amm_open_orders,
        amm_token_coin,
        amm_token_pc,
        amm_target_orders,
        serum_program,
        serum_market,
        serum_bids,
        serum_asks,
        serum_event_queue,
        serum_coin_vault,
        serum_pc_vault,
        serum_vault_signer,
        new_amm_open_orders,
        new_serum_program,
        new_serum_market,
        admin
    }
);

accounts!(
    SetParamsAccounts,
    get_set_params_accounts,
    {
        token_program,
        amm,
        amm_authority,
        amm_open_orders,
        amm_target_orders,
        amm_coin_vault,
        amm_pc_vault,
        serum_program,
        serum_market,
        serum_coin_vault,
        serum_pc_vault,
        serum_vault_signer,
        serum_event_queue,
        serum_bids,
        serum_asks,
        amm_admin_account
    }
);

accounts!(
    WithdrawPnlAccounts,
    get_withdraw_pnl_accounts,
    {
        token_program,
        amm,
        amm_config,
        amm_authority,
        amm_open_orders,
        pool_coin_token_account,
        pool_pc_token_account,
        coin_pnl_token_account,
        pc_pnl_token_account,
        pnl_owner_account,
        amm_target_orders,
        serum_program,
        serum_market,
        serum_event_queue,
        serum_coin_vault_account,
        serum_pc_vault_account,
        serum_vault_signer
    }
);

accounts!(
    WithdrawSrmAccounts,
    get_withdraw_srm_accounts,
    {
        token_program,
        amm,
        amm_owner_account,
        amm_authority,
        srm_token,
        dest_srm_token
    }
);

accounts!(
    SimulateInfoAccounts,
    get_simulate_info_accounts,
    {
        amm,
        amm_authority,
        amm_open_orders,
        pool_coin_token_account,
        pool_pc_token_account,
        lp_mint_address,
        serum_market,
        serum_event_queue
    }
);

accounts!(
    AdminCancelOrdersAccounts,
    get_admin_cancel_orders_accounts,
    {
        token_program,
        amm,
        amm_authority,
        amm_open_orders,
        amm_target_orders,
        pool_coin_token_account,
        pool_pc_token_account,
        amm_owner_account,
        amm_config,
        serum_program,
        serum_market,
        serum_coin_vault_account,
        serum_pc_vault_account,
        serum_vault_signer,
        serum_event_q,
        serum_bids,
        serum_asks
    }
);

accounts!(
    CreateConfigAccountAccounts,
    get_create_config_account_accounts,
    {
        admin,
        amm_config,
        owner,
        system_program,
        rent
    }
);

accounts!(
    UpdateConfigAccountAccounts,
    get_update_config_account_accounts,
    {
        admin,
        amm_config
    }
);

accounts!(
    PreInitializeAccounts,
    get_pre_initialize_accounts,
    {
        token_program,
        system_program,
        rent,
        amm_target_orders,
        pool_withdraw_queue,
        amm_authority,
        lp_mint_address,
        coin_mint_address,
        pc_mint_address,
        pool_coin_token_account,
        pool_pc_token_account,
        pool_temp_lp_token_account,
        serum_market,
        user_wallet
    }
);
