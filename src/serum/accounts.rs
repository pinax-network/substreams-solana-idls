use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

// -----------------------------------------------------------------------------
// Instruction account layouts
// -----------------------------------------------------------------------------
accounts!(
    InitializeMarketAccounts,
    get_initialize_market_accounts,
    {
        /// The market to initialize
        market_to_initialize,
        /// Zeroed out request queue
        request_queue,
        /// Zeroed out event queue
        event_queue,
        /// Zeroed out bids
        bids,
        /// Zeroed out asks
        asks,
        /// SPL token account for the coin currency
        spl_token_account_coin,
        /// SPL token account for the price currency
        spl_token_account_price,
        /// Coin currency mint
        coin_currency_mint,
        /// Price currency mint
        price_currency_mint
    }
);

accounts!(
    NewOrderAccounts,
    get_new_order_accounts,
    {
        /// The market
        market,
        /// The OpenOrders account to use
        open_orders,
        /// The request queue
        request_queue,
        /// The (coin or price currency) account paying for the order
        order_payer,
        /// Owner of the OpenOrders account
        owner,
        /// Coin vault
        coin_vault,
        /// PC vault
        pc_vault,
        /// SPL token program
        spl_token_program,
        /// The rent sysvar
        rent_sysvar
    }
);

accounts!(
    MatchOrdersAccounts,
    get_match_orders_accounts,
    {
        /// Market
        market,
        /// Request queue
        request_queue,
        /// Event queue
        event_queue,
        /// Bids
        bids,
        /// Asks
        asks
    }
);

accounts!(
    ConsumeEventsAccounts,
    get_consume_events_accounts,
    {
        /// OpenOrders
        open_orders,
        /// Market
        market,
        /// Event queue
        event_queue,
        /// Coin fee receivable
        coin_fee_receivable,
        /// PC fee receivable
        pc_fee_receivable
    }
);

accounts!(
    CancelOrderAccounts,
    get_cancel_order_accounts,
    {
        /// Market
        market,
        /// OpenOrders
        open_orders,
        /// The request queue
        request_queue,
        /// The OpenOrders owner
        owner
    }
);

accounts!(
    SettleFundsAccounts,
    get_settle_funds_accounts,
    {
        /// Market
        market,
        /// OpenOrders
        open_orders,
        /// The OpenOrders owner
        owner,
        /// Coin vault
        coin_vault,
        /// PC vault
        pc_vault,
        /// Coin wallet
        coin_wallet,
        /// PC wallet
        pc_wallet,
        /// Vault signer
        vault_signer,
        /// SPL token program
        spl_token_program
    }
);

accounts!(
    CancelOrderByClientIdAccounts,
    get_cancel_order_by_client_id_accounts,
    {
        /// Market
        market,
        /// OpenOrders
        open_orders,
        /// The request queue
        request_queue,
        /// The OpenOrders owner
        owner
    }
);

accounts!(
    DisableMarketAccounts,
    get_disable_market_accounts,
    {
        /// Market
        market,
        /// Disable authority
        disable_authority
    }
);

accounts!(
    SweepFeesAccounts,
    get_sweep_fees_accounts,
    {
        /// Market
        market,
        /// PC vault
        pc_vault,
        /// Fee sweeping authority
        fee_sweeping_authority,
        /// Fee receivable account
        fee_receivable,
        /// Vault signer
        vault_signer,
        /// SPL token program
        spl_token_program
    }
);
