//! Darklake on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

// Account discriminators
pub const AMM_CONFIG_ACCOUNT: [u8; 8] = [218, 244, 33, 104, 203, 203, 43, 111];
pub const ORDER_ACCOUNT: [u8; 8] = [134, 173, 223, 185, 77, 86, 28, 51];
pub const POOL_ACCOUNT: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AmmConfig {
    pub trade_fee_rate: u64,
    pub create_pool_fee: u64,
    pub protocol_fee_rate: u64,
    pub wsol_trade_deposit: u64,
    pub deadline_slot_duration: u64,
    pub ratio_change_tolerance_rate: u64,
    pub bump: u8,
    pub halted: bool,
    pub padding: [u64; 16],
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Order {
    pub trader: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub actual_in: u64,
    pub exchange_in: u64,
    pub actual_out: u64,
    pub from_to_lock: u64,
    pub d_in: u64,
    pub d_out: u64,
    pub deadline: u64,
    pub protocol_fee: u64,
    pub wsol_deposit: u64,
    pub c_min: [u8; 32],
    pub is_x_to_y: bool,
    pub bump: u8,
    pub padding: [u64; 4],
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Pool {
    pub creator: Pubkey,
    pub amm_config: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_lp_supply: u64,
    pub protocol_fee_x: u64,
    pub protocol_fee_y: u64,
    pub locked_x: u64,
    pub locked_y: u64,
    pub user_locked_x: u64,
    pub user_locked_y: u64,
    pub bump: u8,
    pub padding: [u64; 4],
}

// darklakefi/sdk-on-chain dab51a2, src/account_metas.rs::DarklakeAmmInitializePool.
accounts!(InitializePoolAccounts, get_initialize_pool_accounts, {
    user,
    pool,
    authority,
    amm_config,
    token_mint_x,
    token_mint_y,
    token_mint_wsol,
    token_mint_lp,
    metadata_account,
    metadata_account_x,
    metadata_account_y,
    user_token_account_x,
    user_token_account_y,
    user_token_account_lp,
    pool_token_reserve_x,
    pool_token_reserve_y,
    pool_wsol_reserve,
    create_pool_fee_vault,
    mpl_program,
    system_program,
    rent,
    associated_token_program,
    token_mint_x_program,
    token_mint_y_program,
    token_program,
});
