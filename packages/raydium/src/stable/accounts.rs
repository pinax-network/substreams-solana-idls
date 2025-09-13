use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use idls_common::accounts;

// -----------------------------------------------------------------------------
// Pre-initialize accounts
// -----------------------------------------------------------------------------
accounts!(
    PreInitializeAccounts,
    get_pre_initialize_accounts,
    {
        /// SPL Token program
        spl_token_program,
        /// System program
        system_program,
        /// Rent sysvar
        rent,
        /// AMM target orders account
        amm_target_orders,
        /// AMM authority PDA
        amm_authority,
        /// Pool LP mint
        amm_lp_mint,
        /// Coin mint
        coin_mint,
        /// PC mint
        pc_mint,
        /// AMM coin vault
        amm_coin_vault,
        /// AMM pc vault
        amm_pc_vault,
        /// Serum market
        serum_market,
        /// User wallet (payer)
        user_wallet
    }
);

// -----------------------------------------------------------------------------
// Initialize accounts
// -----------------------------------------------------------------------------
accounts!(
    InitializeAccounts,
    get_initialize_accounts,
    {
        /// SPL Token program
        spl_token_program,
        /// System program
        system_program,
        /// Rent sysvar
        rent,
        /// AMM account
        amm,
        /// AMM authority PDA
        amm_authority,
        /// AMM open orders
        amm_open_orders,
        /// Pool LP mint
        amm_lp_mint,
        /// Coin mint
        coin_mint,
        /// PC mint
        pc_mint,
        /// AMM coin vault
        amm_coin_vault,
        /// AMM pc vault
        amm_pc_vault,
        /// AMM target orders
        amm_target_orders,
        /// Model data account
        model_data_account,
        /// Serum DEX program id
        serum_program,
        /// Serum market
        serum_market,
        /// User destination LP token account
        user_dest_lp_token,
        /// User wallet (payer)
        user_wallet
    }
);

// -----------------------------------------------------------------------------
// Deposit accounts
// -----------------------------------------------------------------------------
accounts!(
    DepositAccounts,
    get_deposit_accounts,
    {
        /// SPL Token program
        spl_token_program,
        /// AMM account
        amm,
        /// AMM authority PDA
        amm_authority,
        /// AMM open orders account
        amm_open_orders,
        /// AMM target orders account
        amm_target_orders,
        /// Pool LP mint
        amm_lp_mint,
        /// AMM coin vault
        amm_coin_vault,
        /// AMM pc vault
        amm_pc_vault,
        /// Model data account
        model_data_account,
        /// Serum market
        serum_market,
        /// User coin source token account
        user_source_coin_token,
        /// User pc source token account
        user_source_pc_token,
        /// User destination LP token account
        user_dest_lp_token,
        /// User wallet owner
        user_owner
    }
);

// -----------------------------------------------------------------------------
// Withdraw accounts
// -----------------------------------------------------------------------------
accounts!(
    WithdrawAccounts,
    get_withdraw_accounts,
    {
        /// SPL Token program
        spl_token_program,
        /// AMM account
        amm,
        /// AMM authority PDA
        amm_authority,
        /// AMM open orders account
        amm_open_orders,
        /// AMM target orders account
        amm_target_orders,
        /// Pool LP mint
        amm_lp_mint,
        /// AMM coin vault
        amm_coin_vault,
        /// AMM pc vault
        amm_pc_vault,
        /// Model data account
        model_data_account,
        /// Serum DEX program id
        serum_program,
        /// Serum market
        serum_market,
        /// Serum coin vault
        serum_coin_vault,
        /// Serum pc vault
        serum_pc_vault,
        /// Serum vault signer
        serum_vault_signer,
        /// User source LP token
        user_source_lp_token,
        /// User destination coin token
        user_dest_coin_token,
        /// User destination pc token
        user_dest_pc_token,
        /// User wallet owner
        user_owner
    }
);

// -----------------------------------------------------------------------------
// Swap base in accounts
// -----------------------------------------------------------------------------
accounts!(
    SwapBaseInAccounts,
    get_swap_base_in_accounts,
    {
        /// SPL Token program
        spl_token_program,
        /// AMM account
        amm,
        /// AMM authority PDA
        amm_authority,
        /// AMM open orders account
        amm_open_orders,
        /// AMM coin vault
        amm_coin_vault,
        /// AMM pc vault
        amm_pc_vault,
        /// Model data account
        model_data_account,
        /// Serum DEX program id
        serum_program,
        /// Serum market
        serum_market,
        /// Serum bids account
        serum_bids,
        /// Serum asks account
        serum_asks,
        /// Serum event queue
        serum_event_queue,
        /// Serum coin vault
        serum_coin_vault,
        /// Serum pc vault
        serum_pc_vault,
        /// Serum vault signer
        serum_vault_signer,
        /// User source token account
        user_source_token,
        /// User destination token account
        user_destination_token,
        /// User source owner
        user_source_owner
    }
);

// -----------------------------------------------------------------------------
// Swap base out accounts
// -----------------------------------------------------------------------------
accounts!(
    SwapBaseOutAccounts,
    get_swap_base_out_accounts,
    {
        /// SPL Token program
        spl_token_program,
        /// AMM account
        amm,
        /// AMM authority PDA
        amm_authority,
        /// AMM open orders account
        amm_open_orders,
        /// AMM coin vault
        amm_coin_vault,
        /// AMM pc vault
        amm_pc_vault,
        /// Model data account
        model_data_account,
        /// Serum DEX program id
        serum_program,
        /// Serum market
        serum_market,
        /// Serum bids account
        serum_bids,
        /// Serum asks account
        serum_asks,
        /// Serum event queue
        serum_event_queue,
        /// Serum coin vault
        serum_coin_vault,
        /// Serum pc vault
        serum_pc_vault,
        /// Serum vault signer
        serum_vault_signer,
        /// User source token account
        user_source_token,
        /// User destination token account
        user_destination_token,
        /// User source owner
        user_source_owner
    }
);

