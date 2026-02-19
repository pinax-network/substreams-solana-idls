//! Marinade Finance account extraction helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use common::accounts;

// -----------------------------------------------------------------------------
// Deposit accounts
// -----------------------------------------------------------------------------
accounts!(
    DepositAccounts,
    get_deposit_accounts,
    {
        /// Marinade state
        state,
        /// mSOL mint
        msol_mint,
        /// SOL leg of the liq pool
        liq_pool_sol_leg_pda,
        /// mSOL leg of the liq pool
        liq_pool_msol_leg,
        /// LP mint authority
        liq_pool_msol_leg_authority,
        /// Reserve PDA
        reserve_pda,
        /// User SOL account (signer)
        transfer_from,
        /// User mSOL token account
        mint_to,
        /// mSOL mint authority
        msol_mint_authority,
        /// System program
        system_program,
        /// Token program
        token_program
    }
);

// -----------------------------------------------------------------------------
// Liquid unstake accounts
// -----------------------------------------------------------------------------
accounts!(
    LiquidUnstakeAccounts,
    get_liquid_unstake_accounts,
    {
        /// Marinade state
        state,
        /// mSOL mint
        msol_mint,
        /// SOL leg of the liq pool
        liq_pool_sol_leg_pda,
        /// mSOL leg of the liq pool
        liq_pool_msol_leg,
        /// Treasury mSOL account
        treasury_msol_account,
        /// User mSOL token account (source)
        get_msol_from,
        /// User mSOL authority (signer)
        get_msol_from_authority,
        /// User SOL account (destination)
        transfer_sol_to,
        /// System program
        system_program,
        /// Token program
        token_program
    }
);

// -----------------------------------------------------------------------------
// Order unstake accounts
// -----------------------------------------------------------------------------
accounts!(
    OrderUnstakeAccounts,
    get_order_unstake_accounts,
    {
        /// Marinade state
        state,
        /// mSOL mint
        msol_mint,
        /// User mSOL token account
        burn_msol_from,
        /// User authority (signer)
        burn_msol_authority,
        /// New ticket account (writable)
        new_ticket_account,
        /// Clock sysvar
        clock,
        /// Rent sysvar
        rent,
        /// Token program
        token_program
    }
);
