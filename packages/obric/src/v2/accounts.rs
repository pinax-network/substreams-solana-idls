//! Obric V2 account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use common::accounts;

accounts!(
    SwapXToYAccounts,
    get_swap_x_to_y_accounts,
    {
        trading_pair,
        mint_x,
        mint_y,
        reserve_x,
        reserve_y,
        user_token_account_x,
        user_token_account_y,
        protocol_fee_y,
        x_price_feed,
        y_price_feed,
        user,
        token_program
    }
);

accounts!(
    SwapYToXAccounts,
    get_swap_y_to_x_accounts,
    {
        trading_pair,
        mint_x,
        mint_y,
        reserve_x,
        reserve_y,
        user_token_account_x,
        user_token_account_y,
        protocol_fee_x,
        x_price_feed,
        y_price_feed,
        user,
        token_program
    }
);

accounts!(
    SwapAccounts,
    get_swap_accounts,
    {
        trading_pair,
        mint_x,
        mint_y,
        reserve_x,
        reserve_y,
        user_token_account_x,
        user_token_account_y,
        protocol_fee,
        x_price_feed,
        y_price_feed,
        user,
        token_program
    }
);
