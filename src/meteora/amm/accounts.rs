use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::accounts::AccountsError;

// -----------------------------------------------------------------------------
// Accounts structs
// -----------------------------------------------------------------------------
/// Initialize a new permissioned pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePermissionedPoolAccounts {
    /// Pool account (arbitrary address)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: Pubkey,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: Pubkey,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token mint of vault A
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault B
    pub b_vault_lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Admin token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub admin_token_a: Pubkey,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub admin_token_b: Pubkey,
    /// Admin pool LP token account. Used to receive LP during first deposit (initialize pool)
    /// Admin pool LP token account. Used to receive LP during first deposit (initialize pool)
    pub admin_pool_lp: Pubkey,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: Pubkey,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: Pubkey,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub admin: Pubkey,
    pub fee_owner: Pubkey,
    /// Rent account.
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Associated token program.
    pub associated_token_program: Pubkey,
    /// System program.
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePermissionedPoolAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            token_a_mint: get(2, "token_a_mint")?,
            token_b_mint: get(3, "token_b_mint")?,
            a_vault: get(4, "a_vault")?,
            b_vault: get(5, "b_vault")?,
            a_vault_lp_mint: get(6, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(7, "b_vault_lp_mint")?,
            a_vault_lp: get(8, "a_vault_lp")?,
            b_vault_lp: get(9, "b_vault_lp")?,
            admin_token_a: get(10, "admin_token_a")?,
            admin_token_b: get(11, "admin_token_b")?,
            admin_pool_lp: get(12, "admin_pool_lp")?,
            protocol_token_a_fee: get(13, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(14, "protocol_token_b_fee")?,
            admin: get(15, "admin")?,
            fee_owner: get(16, "fee_owner")?,
            rent: get(17, "rent")?,
            mint_metadata: get(18, "mint_metadata")?,
            metadata_program: get(19, "metadata_program")?,
            vault_program: get(20, "vault_program")?,
            token_program: get(21, "token_program")?,
            associated_token_program: get(22, "associated_token_program")?,
            system_program: get(23, "system_program")?,
        })
    }
}

pub fn get_initialize_permissioned_pool_accounts(ix: &InstructionView) -> Result<InitializePermissionedPoolAccounts, AccountsError> {
    InitializePermissionedPoolAccounts::try_from(ix)
}

/// Initialize a new permissionless pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePermissionlessPoolAccounts {
    /// Pool account (PDA address)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: Pubkey,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: Pubkey,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// LP token mint of vault A
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault B
    pub b_vault_lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: Pubkey,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: Pubkey,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: Pubkey,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: Pubkey,
    pub fee_owner: Pubkey,
    /// Rent account.
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Associated token program.
    pub associated_token_program: Pubkey,
    /// System program.
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePermissionlessPoolAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            token_a_mint: get(2, "token_a_mint")?,
            token_b_mint: get(3, "token_b_mint")?,
            a_vault: get(4, "a_vault")?,
            b_vault: get(5, "b_vault")?,
            a_token_vault: get(6, "a_token_vault")?,
            b_token_vault: get(7, "b_token_vault")?,
            a_vault_lp_mint: get(8, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(9, "b_vault_lp_mint")?,
            a_vault_lp: get(10, "a_vault_lp")?,
            b_vault_lp: get(11, "b_vault_lp")?,
            payer_token_a: get(12, "payer_token_a")?,
            payer_token_b: get(13, "payer_token_b")?,
            payer_pool_lp: get(14, "payer_pool_lp")?,
            protocol_token_a_fee: get(15, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(16, "protocol_token_b_fee")?,
            payer: get(17, "payer")?,
            fee_owner: get(18, "fee_owner")?,
            rent: get(19, "rent")?,
            mint_metadata: get(20, "mint_metadata")?,
            metadata_program: get(21, "metadata_program")?,
            vault_program: get(22, "vault_program")?,
            token_program: get(23, "token_program")?,
            associated_token_program: get(24, "associated_token_program")?,
            system_program: get(25, "system_program")?,
        })
    }
}

pub fn get_initialize_permissionless_pool_accounts(ix: &InstructionView) -> Result<InitializePermissionlessPoolAccounts, AccountsError> {
    InitializePermissionlessPoolAccounts::try_from(ix)
}

/// Initialize a new permissionless pool with customized fee tier
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePermissionlessPoolWithFeeTierAccounts {
    /// Pool account (PDA address)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: Pubkey,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: Pubkey,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// LP token mint of vault A
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault B
    pub b_vault_lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: Pubkey,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: Pubkey,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: Pubkey,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: Pubkey,
    pub fee_owner: Pubkey,
    /// Rent account.
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Associated token program.
    pub associated_token_program: Pubkey,
    /// System program.
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePermissionlessPoolWithFeeTierAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            token_a_mint: get(2, "token_a_mint")?,
            token_b_mint: get(3, "token_b_mint")?,
            a_vault: get(4, "a_vault")?,
            b_vault: get(5, "b_vault")?,
            a_token_vault: get(6, "a_token_vault")?,
            b_token_vault: get(7, "b_token_vault")?,
            a_vault_lp_mint: get(8, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(9, "b_vault_lp_mint")?,
            a_vault_lp: get(10, "a_vault_lp")?,
            b_vault_lp: get(11, "b_vault_lp")?,
            payer_token_a: get(12, "payer_token_a")?,
            payer_token_b: get(13, "payer_token_b")?,
            payer_pool_lp: get(14, "payer_pool_lp")?,
            protocol_token_a_fee: get(15, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(16, "protocol_token_b_fee")?,
            payer: get(17, "payer")?,
            fee_owner: get(18, "fee_owner")?,
            rent: get(19, "rent")?,
            mint_metadata: get(20, "mint_metadata")?,
            metadata_program: get(21, "metadata_program")?,
            vault_program: get(22, "vault_program")?,
            token_program: get(23, "token_program")?,
            associated_token_program: get(24, "associated_token_program")?,
            system_program: get(25, "system_program")?,
        })
    }
}

pub fn get_initialize_permissionless_pool_with_fee_tier_accounts(
    ix: &InstructionView,
) -> Result<InitializePermissionlessPoolWithFeeTierAccounts, AccountsError> {
    InitializePermissionlessPoolWithFeeTierAccounts::try_from(ix)
}

/// Enable or disable a pool. A disabled pool allow only remove balanced liquidity operation.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct EnableOrDisablePoolAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// Admin account. Must be owner of the pool.
    pub admin: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for EnableOrDisablePoolAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            admin: get(1, "admin")?,
        })
    }
}

pub fn get_enable_or_disable_pool_accounts(ix: &InstructionView) -> Result<EnableOrDisablePoolAccounts, AccountsError> {
    EnableOrDisablePoolAccounts::try_from(ix)
}

/// Swap token A to B, or vice versa. An amount of trading fee will be charged for liquidity provider, and the admin of the pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// User token account. Token from this account will be transfer into the vault by the pool in exchange for another token of the pool.
    pub user_source_token: Pubkey,
    /// User token account. The exchanged token will be transfer into this account from the pool.
    pub user_destination_token: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// Lp token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// Lp token mint of vault b
    pub b_vault_lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Protocol fee token account. Used to receive trading fee. It's mint field must matched with user_source_token mint field.
    pub protocol_token_fee: Pubkey,
    /// User account. Must be owner of user_source_token.
    pub user: Pubkey,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            user_source_token: get(1, "user_source_token")?,
            user_destination_token: get(2, "user_destination_token")?,
            a_vault: get(3, "a_vault")?,
            b_vault: get(4, "b_vault")?,
            a_token_vault: get(5, "a_token_vault")?,
            b_token_vault: get(6, "b_token_vault")?,
            a_vault_lp_mint: get(7, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(8, "b_vault_lp_mint")?,
            a_vault_lp: get(9, "a_vault_lp")?,
            b_vault_lp: get(10, "b_vault_lp")?,
            protocol_token_fee: get(11, "protocol_token_fee")?,
            user: get(12, "user")?,
            vault_program: get(13, "vault_program")?,
            token_program: get(14, "token_program")?,
        })
    }
}

pub fn get_swap_accounts(ix: &InstructionView) -> Result<SwapAccounts, AccountsError> {
    SwapAccounts::try_from(ix)
}

/// Withdraw only single token from the pool. Only supported by pool with stable swap curve.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveLiquiditySingleSideAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// User pool lp token account. LP will be burned from this account upon success liquidity removal.
    pub user_pool_lp: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token mint of vault A
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault B
    pub b_vault_lp_mint: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// User token account to receive token upon success liquidity removal.
    pub user_destination_token: Pubkey,
    /// User account. Must be owner of the user_pool_lp account.
    pub user: Pubkey,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveLiquiditySingleSideAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            user_pool_lp: get(2, "user_pool_lp")?,
            a_vault_lp: get(3, "a_vault_lp")?,
            b_vault_lp: get(4, "b_vault_lp")?,
            a_vault: get(5, "a_vault")?,
            b_vault: get(6, "b_vault")?,
            a_vault_lp_mint: get(7, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(8, "b_vault_lp_mint")?,
            a_token_vault: get(9, "a_token_vault")?,
            b_token_vault: get(10, "b_token_vault")?,
            user_destination_token: get(11, "user_destination_token")?,
            user: get(12, "user")?,
            vault_program: get(13, "vault_program")?,
            token_program: get(14, "token_program")?,
        })
    }
}

pub fn get_remove_liquidity_single_side_accounts(ix: &InstructionView) -> Result<RemoveLiquiditySingleSideAccounts, AccountsError> {
    RemoveLiquiditySingleSideAccounts::try_from(ix)
}

/// Deposit tokens to the pool in an imbalance ratio. Only supported by pool with stable swap curve.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddImbalanceLiquidityAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault b
    pub b_vault_lp_mint: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: Pubkey,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: Pubkey,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: Pubkey,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddImbalanceLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            user_pool_lp: get(2, "user_pool_lp")?,
            a_vault_lp: get(3, "a_vault_lp")?,
            b_vault_lp: get(4, "b_vault_lp")?,
            a_vault: get(5, "a_vault")?,
            b_vault: get(6, "b_vault")?,
            a_vault_lp_mint: get(7, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(8, "b_vault_lp_mint")?,
            a_token_vault: get(9, "a_token_vault")?,
            b_token_vault: get(10, "b_token_vault")?,
            user_a_token: get(11, "user_a_token")?,
            user_b_token: get(12, "user_b_token")?,
            user: get(13, "user")?,
            vault_program: get(14, "vault_program")?,
            token_program: get(15, "token_program")?,
        })
    }
}

pub fn get_add_imbalance_liquidity_accounts(ix: &InstructionView) -> Result<AddImbalanceLiquidityAccounts, AccountsError> {
    AddImbalanceLiquidityAccounts::try_from(ix)
}

/// Withdraw tokens from the pool in a balanced ratio. User will still able to withdraw from pool even the pool is disabled. This allow user to exit their liquidity when there's some unforeseen event happen.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveBalanceLiquidityAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault b
    pub b_vault_lp_mint: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: Pubkey,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: Pubkey,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: Pubkey,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveBalanceLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            user_pool_lp: get(2, "user_pool_lp")?,
            a_vault_lp: get(3, "a_vault_lp")?,
            b_vault_lp: get(4, "b_vault_lp")?,
            a_vault: get(5, "a_vault")?,
            b_vault: get(6, "b_vault")?,
            a_vault_lp_mint: get(7, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(8, "b_vault_lp_mint")?,
            a_token_vault: get(9, "a_token_vault")?,
            b_token_vault: get(10, "b_token_vault")?,
            user_a_token: get(11, "user_a_token")?,
            user_b_token: get(12, "user_b_token")?,
            user: get(13, "user")?,
            vault_program: get(14, "vault_program")?,
            token_program: get(15, "token_program")?,
        })
    }
}

pub fn get_remove_balance_liquidity_accounts(ix: &InstructionView) -> Result<RemoveBalanceLiquidityAccounts, AccountsError> {
    RemoveBalanceLiquidityAccounts::try_from(ix)
}

/// Deposit tokens to the pool in a balanced ratio.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddBalanceLiquidityAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault b
    pub b_vault_lp_mint: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: Pubkey,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: Pubkey,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: Pubkey,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddBalanceLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            user_pool_lp: get(2, "user_pool_lp")?,
            a_vault_lp: get(3, "a_vault_lp")?,
            b_vault_lp: get(4, "b_vault_lp")?,
            a_vault: get(5, "a_vault")?,
            b_vault: get(6, "b_vault")?,
            a_vault_lp_mint: get(7, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(8, "b_vault_lp_mint")?,
            a_token_vault: get(9, "a_token_vault")?,
            b_token_vault: get(10, "b_token_vault")?,
            user_a_token: get(11, "user_a_token")?,
            user_b_token: get(12, "user_b_token")?,
            user: get(13, "user")?,
            vault_program: get(14, "vault_program")?,
            token_program: get(15, "token_program")?,
        })
    }
}

pub fn get_add_balance_liquidity_accounts(ix: &InstructionView) -> Result<AddBalanceLiquidityAccounts, AccountsError> {
    AddBalanceLiquidityAccounts::try_from(ix)
}

/// Update trading fee charged for liquidity provider, and admin.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetPoolFeesAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// Fee operator account
    pub fee_operator: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SetPoolFeesAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            fee_operator: get(1, "fee_operator")?,
        })
    }
}

pub fn get_set_pool_fees_accounts(ix: &InstructionView) -> Result<SetPoolFeesAccounts, AccountsError> {
    SetPoolFeesAccounts::try_from(ix)
}

/// Update swap curve parameters. This function do not allow update of curve type. For example: stable swap curve to constant product curve. Only supported by pool with stable swap curve.
/// Only amp is allowed to be override. The other attributes of stable swap curve will be ignored.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OverrideCurveParamAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// Admin account.
    pub admin: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for OverrideCurveParamAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            admin: get(1, "admin")?,
        })
    }
}

pub fn get_override_curve_param_accounts(ix: &InstructionView) -> Result<OverrideCurveParamAccounts, AccountsError> {
    OverrideCurveParamAccounts::try_from(ix)
}

/// Get the general information of the pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct GetPoolInfoAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault b
    pub b_vault_lp_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for GetPoolInfoAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            a_vault_lp: get(2, "a_vault_lp")?,
            b_vault_lp: get(3, "b_vault_lp")?,
            a_vault: get(4, "a_vault")?,
            b_vault: get(5, "b_vault")?,
            a_vault_lp_mint: get(6, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(7, "b_vault_lp_mint")?,
        })
    }
}

pub fn get_get_pool_info_accounts(ix: &InstructionView) -> Result<GetPoolInfoAccounts, AccountsError> {
    GetPoolInfoAccounts::try_from(ix)
}

/// Bootstrap the pool when liquidity is depleted.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct BootstrapLiquidityAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault b
    pub b_vault_lp_mint: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: Pubkey,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: Pubkey,
    /// User account. Must be owner of user_a_token, and user_b_token.
    pub user: Pubkey,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for BootstrapLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            user_pool_lp: get(2, "user_pool_lp")?,
            a_vault_lp: get(3, "a_vault_lp")?,
            b_vault_lp: get(4, "b_vault_lp")?,
            a_vault: get(5, "a_vault")?,
            b_vault: get(6, "b_vault")?,
            a_vault_lp_mint: get(7, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(8, "b_vault_lp_mint")?,
            a_token_vault: get(9, "a_token_vault")?,
            b_token_vault: get(10, "b_token_vault")?,
            user_a_token: get(11, "user_a_token")?,
            user_b_token: get(12, "user_b_token")?,
            user: get(13, "user")?,
            vault_program: get(14, "vault_program")?,
            token_program: get(15, "token_program")?,
        })
    }
}

pub fn get_bootstrap_liquidity_accounts(ix: &InstructionView) -> Result<BootstrapLiquidityAccounts, AccountsError> {
    BootstrapLiquidityAccounts::try_from(ix)
}

/// Create mint metadata account for old pools
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateMintMetadataAccounts {
    /// Pool account
    pub pool: Pubkey,
    /// LP mint account of the pool
    pub lp_mint: Pubkey,
    /// Vault A LP account of the pool
    pub a_vault_lp: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    /// System program.
    pub system_program: Pubkey,
    /// Payer
    pub payer: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateMintMetadataAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            a_vault_lp: get(2, "a_vault_lp")?,
            mint_metadata: get(3, "mint_metadata")?,
            metadata_program: get(4, "metadata_program")?,
            system_program: get(5, "system_program")?,
            payer: get(6, "payer")?,
        })
    }
}

pub fn get_create_mint_metadata_accounts(ix: &InstructionView) -> Result<CreateMintMetadataAccounts, AccountsError> {
    CreateMintMetadataAccounts::try_from(ix)
}

/// Create lock account
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateLockEscrowAccounts {
    /// Pool account
    pub pool: Pubkey,
    /// Lock account
    pub lock_escrow: Pubkey,
    pub owner: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Payer account
    pub payer: Pubkey,
    /// System program.
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateLockEscrowAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lock_escrow: get(1, "lock_escrow")?,
            owner: get(2, "owner")?,
            lp_mint: get(3, "lp_mint")?,
            payer: get(4, "payer")?,
            system_program: get(5, "system_program")?,
        })
    }
}

pub fn get_create_lock_escrow_accounts(ix: &InstructionView) -> Result<CreateLockEscrowAccounts, AccountsError> {
    CreateLockEscrowAccounts::try_from(ix)
}

/// Lock Lp token
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct LockAccounts {
    /// Pool account
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Lock account
    pub lock_escrow: Pubkey,
    /// Can be anyone
    pub owner: Pubkey,
    /// owner lp token account
    pub source_tokens: Pubkey,
    /// Escrow vault
    pub escrow_vault: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// LP token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault b
    pub b_vault_lp_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for LockAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            lock_escrow: get(2, "lock_escrow")?,
            owner: get(3, "owner")?,
            source_tokens: get(4, "source_tokens")?,
            escrow_vault: get(5, "escrow_vault")?,
            token_program: get(6, "token_program")?,
            a_vault: get(7, "a_vault")?,
            b_vault: get(8, "b_vault")?,
            a_vault_lp: get(9, "a_vault_lp")?,
            b_vault_lp: get(10, "b_vault_lp")?,
            a_vault_lp_mint: get(11, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(12, "b_vault_lp_mint")?,
        })
    }
}

pub fn get_lock_accounts(ix: &InstructionView) -> Result<LockAccounts, AccountsError> {
    LockAccounts::try_from(ix)
}

/// Claim fee
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClaimFeeAccounts {
    /// Pool account
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Lock account
    pub lock_escrow: Pubkey,
    /// Owner of lock account
    pub owner: Pubkey,
    /// owner lp token account
    pub source_tokens: Pubkey,
    /// Escrow vault
    pub escrow_vault: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// LP token mint of vault a
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault b
    pub b_vault_lp_mint: Pubkey,
    /// User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: Pubkey,
    /// User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: Pubkey,
    /// Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClaimFeeAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            lock_escrow: get(2, "lock_escrow")?,
            owner: get(3, "owner")?,
            source_tokens: get(4, "source_tokens")?,
            escrow_vault: get(5, "escrow_vault")?,
            token_program: get(6, "token_program")?,
            a_token_vault: get(7, "a_token_vault")?,
            b_token_vault: get(8, "b_token_vault")?,
            a_vault: get(9, "a_vault")?,
            b_vault: get(10, "b_vault")?,
            a_vault_lp: get(11, "a_vault_lp")?,
            b_vault_lp: get(12, "b_vault_lp")?,
            a_vault_lp_mint: get(13, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(14, "b_vault_lp_mint")?,
            user_a_token: get(15, "user_a_token")?,
            user_b_token: get(16, "user_b_token")?,
            vault_program: get(17, "vault_program")?,
        })
    }
}

pub fn get_claim_fee_accounts(ix: &InstructionView) -> Result<ClaimFeeAccounts, AccountsError> {
    ClaimFeeAccounts::try_from(ix)
}

/// Create config
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateConfigAccounts {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateConfigAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            config: get(0, "config")?,
            admin: get(1, "admin")?,
            system_program: get(2, "system_program")?,
        })
    }
}

pub fn get_create_config_accounts(ix: &InstructionView) -> Result<CreateConfigAccounts, AccountsError> {
    CreateConfigAccounts::try_from(ix)
}

/// Close config
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CloseConfigAccounts {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CloseConfigAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            config: get(0, "config")?,
            admin: get(1, "admin")?,
            rent_receiver: get(2, "rent_receiver")?,
        })
    }
}

pub fn get_close_config_accounts(ix: &InstructionView) -> Result<CloseConfigAccounts, AccountsError> {
    CloseConfigAccounts::try_from(ix)
}

/// Initialize permissionless pool with config
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePermissionlessConstantProductPoolWithConfigAccounts {
    /// Pool account (PDA address)
    pub pool: Pubkey,
    pub config: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: Pubkey,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: Pubkey,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// LP token mint of vault A
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault B
    pub b_vault_lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: Pubkey,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: Pubkey,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: Pubkey,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: Pubkey,
    /// Rent account.
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Associated token program.
    pub associated_token_program: Pubkey,
    /// System program.
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePermissionlessConstantProductPoolWithConfigAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            config: get(1, "config")?,
            lp_mint: get(2, "lp_mint")?,
            token_a_mint: get(3, "token_a_mint")?,
            token_b_mint: get(4, "token_b_mint")?,
            a_vault: get(5, "a_vault")?,
            b_vault: get(6, "b_vault")?,
            a_token_vault: get(7, "a_token_vault")?,
            b_token_vault: get(8, "b_token_vault")?,
            a_vault_lp_mint: get(9, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(10, "b_vault_lp_mint")?,
            a_vault_lp: get(11, "a_vault_lp")?,
            b_vault_lp: get(12, "b_vault_lp")?,
            payer_token_a: get(13, "payer_token_a")?,
            payer_token_b: get(14, "payer_token_b")?,
            payer_pool_lp: get(15, "payer_pool_lp")?,
            protocol_token_a_fee: get(16, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(17, "protocol_token_b_fee")?,
            payer: get(18, "payer")?,
            rent: get(19, "rent")?,
            mint_metadata: get(20, "mint_metadata")?,
            metadata_program: get(21, "metadata_program")?,
            vault_program: get(22, "vault_program")?,
            token_program: get(23, "token_program")?,
            associated_token_program: get(24, "associated_token_program")?,
            system_program: get(25, "system_program")?,
        })
    }
}

pub fn get_initialize_permissionless_constant_product_pool_with_config_accounts(
    ix: &InstructionView,
) -> Result<InitializePermissionlessConstantProductPoolWithConfigAccounts, AccountsError> {
    InitializePermissionlessConstantProductPoolWithConfigAccounts::try_from(ix)
}

/// Initialize permissionless pool with config 2
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePermissionlessConstantProductPoolWithConfig2Accounts {
    /// Pool account (PDA address)
    pub pool: Pubkey,
    pub config: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: Pubkey,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: Pubkey,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// LP token mint of vault A
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault B
    pub b_vault_lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: Pubkey,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: Pubkey,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: Pubkey,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: Pubkey,
    /// Rent account.
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Associated token program.
    pub associated_token_program: Pubkey,
    /// System program.
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePermissionlessConstantProductPoolWithConfig2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            config: get(1, "config")?,
            lp_mint: get(2, "lp_mint")?,
            token_a_mint: get(3, "token_a_mint")?,
            token_b_mint: get(4, "token_b_mint")?,
            a_vault: get(5, "a_vault")?,
            b_vault: get(6, "b_vault")?,
            a_token_vault: get(7, "a_token_vault")?,
            b_token_vault: get(8, "b_token_vault")?,
            a_vault_lp_mint: get(9, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(10, "b_vault_lp_mint")?,
            a_vault_lp: get(11, "a_vault_lp")?,
            b_vault_lp: get(12, "b_vault_lp")?,
            payer_token_a: get(13, "payer_token_a")?,
            payer_token_b: get(14, "payer_token_b")?,
            payer_pool_lp: get(15, "payer_pool_lp")?,
            protocol_token_a_fee: get(16, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(17, "protocol_token_b_fee")?,
            payer: get(18, "payer")?,
            rent: get(19, "rent")?,
            mint_metadata: get(20, "mint_metadata")?,
            metadata_program: get(21, "metadata_program")?,
            vault_program: get(22, "vault_program")?,
            token_program: get(23, "token_program")?,
            associated_token_program: get(24, "associated_token_program")?,
            system_program: get(25, "system_program")?,
        })
    }
}

pub fn get_initialize_permissionless_constant_product_pool_with_config2_accounts(
    ix: &InstructionView,
) -> Result<InitializePermissionlessConstantProductPoolWithConfig2Accounts, AccountsError> {
    InitializePermissionlessConstantProductPoolWithConfig2Accounts::try_from(ix)
}

/// Initialize permissionless pool with customizable params
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeCustomizablePermissionlessConstantProductPoolAccounts {
    /// Pool account (PDA address)
    pub pool: Pubkey,
    /// LP token mint of the pool
    pub lp_mint: Pubkey,
    /// Token A mint of the pool. Eg: USDT
    pub token_a_mint: Pubkey,
    /// Token B mint of the pool. Eg: USDC
    pub token_b_mint: Pubkey,
    /// Vault account for token A. Token A of the pool will be deposit / withdraw from this vault account.
    pub a_vault: Pubkey,
    /// Vault account for token B. Token B of the pool will be deposit / withdraw from this vault account.
    pub b_vault: Pubkey,
    /// Token vault account of vault A
    pub a_token_vault: Pubkey,
    /// Token vault account of vault B
    pub b_token_vault: Pubkey,
    /// LP token mint of vault A
    pub a_vault_lp_mint: Pubkey,
    /// LP token mint of vault B
    pub b_vault_lp_mint: Pubkey,
    /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: Pubkey,
    /// LP token account of vault B. Used to receive/burn vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: Pubkey,
    /// Payer token account for pool token A mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_a: Pubkey,
    /// Admin token account for pool token B mint. Used to bootstrap the pool with initial liquidity.
    pub payer_token_b: Pubkey,
    pub payer_pool_lp: Pubkey,
    /// Protocol fee token account for token A. Used to receive trading fee.
    pub protocol_token_a_fee: Pubkey,
    /// Protocol fee token account for token B. Used to receive trading fee.
    pub protocol_token_b_fee: Pubkey,
    /// Admin account. This account will be the admin of the pool, and the payer for PDA during initialize pool.
    pub payer: Pubkey,
    /// Rent account.
    pub rent: Pubkey,
    pub mint_metadata: Pubkey,
    pub metadata_program: Pubkey,
    /// Vault program. The pool will deposit/withdraw liquidity from the vault.
    pub vault_program: Pubkey,
    /// Token program.
    pub token_program: Pubkey,
    /// Associated token program.
    pub associated_token_program: Pubkey,
    /// System program.
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeCustomizablePermissionlessConstantProductPoolAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            lp_mint: get(1, "lp_mint")?,
            token_a_mint: get(2, "token_a_mint")?,
            token_b_mint: get(3, "token_b_mint")?,
            a_vault: get(4, "a_vault")?,
            b_vault: get(5, "b_vault")?,
            a_token_vault: get(6, "a_token_vault")?,
            b_token_vault: get(7, "b_token_vault")?,
            a_vault_lp_mint: get(8, "a_vault_lp_mint")?,
            b_vault_lp_mint: get(9, "b_vault_lp_mint")?,
            a_vault_lp: get(10, "a_vault_lp")?,
            b_vault_lp: get(11, "b_vault_lp")?,
            payer_token_a: get(12, "payer_token_a")?,
            payer_token_b: get(13, "payer_token_b")?,
            payer_pool_lp: get(14, "payer_pool_lp")?,
            protocol_token_a_fee: get(15, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(16, "protocol_token_b_fee")?,
            payer: get(17, "payer")?,
            rent: get(18, "rent")?,
            mint_metadata: get(19, "mint_metadata")?,
            metadata_program: get(20, "metadata_program")?,
            vault_program: get(21, "vault_program")?,
            token_program: get(22, "token_program")?,
            associated_token_program: get(23, "associated_token_program")?,
            system_program: get(24, "system_program")?,
        })
    }
}

pub fn get_initialize_customizable_permissionless_constant_product_pool_accounts(
    ix: &InstructionView,
) -> Result<InitializeCustomizablePermissionlessConstantProductPoolAccounts, AccountsError> {
    InitializeCustomizablePermissionlessConstantProductPoolAccounts::try_from(ix)
}

/// Update activation slot
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateActivationPointAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    /// Admin account.
    pub admin: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for UpdateActivationPointAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            admin: get(1, "admin")?,
        })
    }
}

pub fn get_update_activation_point_accounts(ix: &InstructionView) -> Result<UpdateActivationPointAccounts, AccountsError> {
    UpdateActivationPointAccounts::try_from(ix)
}

/// Withdraw protocol fee
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawProtocolFeesAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    pub a_vault_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub treasury_token_a: Pubkey,
    pub treasury_token_b: Pubkey,
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for WithdrawProtocolFeesAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            a_vault_lp: get(1, "a_vault_lp")?,
            protocol_token_a_fee: get(2, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(3, "protocol_token_b_fee")?,
            treasury_token_a: get(4, "treasury_token_a")?,
            treasury_token_b: get(5, "treasury_token_b")?,
            token_program: get(6, "token_program")?,
        })
    }
}

pub fn get_withdraw_protocol_fees_accounts(ix: &InstructionView) -> Result<WithdrawProtocolFeesAccounts, AccountsError> {
    WithdrawProtocolFeesAccounts::try_from(ix)
}

/// Set whitelisted vault
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetWhitelistedVaultAccounts {
    pub pool: Pubkey,
    pub admin: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SetWhitelistedVaultAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            admin: get(1, "admin")?,
        })
    }
}

pub fn get_set_whitelisted_vault_accounts(ix: &InstructionView) -> Result<SetWhitelistedVaultAccounts, AccountsError> {
    SetWhitelistedVaultAccounts::try_from(ix)
}

/// Partner claim fee
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PartnerClaimFeeAccounts {
    /// Pool account (PDA)
    pub pool: Pubkey,
    pub a_vault_lp: Pubkey,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub partner_token_a: Pubkey,
    pub partner_token_b: Pubkey,
    pub token_program: Pubkey,
    pub partner_authority: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for PartnerClaimFeeAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get(0, "pool")?,
            a_vault_lp: get(1, "a_vault_lp")?,
            protocol_token_a_fee: get(2, "protocol_token_a_fee")?,
            protocol_token_b_fee: get(3, "protocol_token_b_fee")?,
            partner_token_a: get(4, "partner_token_a")?,
            partner_token_b: get(5, "partner_token_b")?,
            token_program: get(6, "token_program")?,
            partner_authority: get(7, "partner_authority")?,
        })
    }
}

pub fn get_partner_claim_fee_accounts(ix: &InstructionView) -> Result<PartnerClaimFeeAccounts, AccountsError> {
    PartnerClaimFeeAccounts::try_from(ix)
}
