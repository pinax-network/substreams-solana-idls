//! Obric V3 account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts;

accounts!(
    SwapXToYAccounts,
    get_swap_x_to_y_accounts,
    {
        trading_pair,
        mint_x,
        mint_y,
        x_price_feed,
        y_price_feed,
        reserve_x,
        reserve_y,
        user_token_account_x,
        user_token_account_y,
        protocol_fee_y,
        marginfi_program,
        marginfi_group,
        marginfi_bank_x,
        marginfi_price_x,
        marginfi_bank_y,
        marginfi_price_y,
        marginfi_account,
        marginfi_bank_x_liquidity_vault,
        marginfi_bank_y_liquidity_vault,
        marginfi_bank_y_liquidity_vault_authority,
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
        x_price_feed,
        y_price_feed,
        reserve_x,
        reserve_y,
        user_token_account_x,
        user_token_account_y,
        protocol_fee_x,
        marginfi_program,
        marginfi_group,
        marginfi_bank_x,
        marginfi_price_x,
        marginfi_bank_y,
        marginfi_price_y,
        marginfi_account,
        marginfi_bank_x_liquidity_vault,
        marginfi_bank_x_liquidity_vault_authority,
        marginfi_bank_y_liquidity_vault,
        user,
        token_program
    }
);
