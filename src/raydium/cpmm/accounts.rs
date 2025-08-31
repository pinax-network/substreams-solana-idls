use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::accounts::AccountsError;

/// Accounts for the `close_permission_pda` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClosePermissionPdaAccounts {
    pub owner: Pubkey,
    pub permission_authority: Pubkey,
    /// Initialize config state account to store protocol owner address and fee rates.
    pub permission: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClosePermissionPdaAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(ClosePermissionPdaAccounts {
            owner: get_req(0, "owner")?,
            permission_authority: get_req(1, "permission_authority")?,
            permission: get_req(2, "permission")?,
            system_program: get_req(3, "system_program")?,
        })
    }
}

pub fn get_close_permission_pda_accounts(ix: &InstructionView) -> Result<ClosePermissionPdaAccounts, AccountsError> {
    ClosePermissionPdaAccounts::try_from(ix)
}

/// Accounts for the `collect_creator_fee` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectCreatorFeeAccounts {
    /// Only pool creator can collect fee
    pub creator: Pubkey,
    pub authority: Pubkey,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: Pubkey,
    /// Amm config account stores fund_owner
    pub amm_config: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: Pubkey,
    /// The mint of token_0 vault
    pub vault_0_mint: Pubkey,
    /// The mint of token_1 vault
    pub vault_1_mint: Pubkey,
    /// The address that receives the collected token_0 fund fees
    pub creator_token_0: Pubkey,
    /// The address that receives the collected token_1 fund fees
    pub creator_token_1: Pubkey,
    /// Spl token program or token program 2022
    pub token_0_program: Pubkey,
    /// Spl token program or token program 2022
    pub token_1_program: Pubkey,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Pubkey,
    /// To create a new program account
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CollectCreatorFeeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(CollectCreatorFeeAccounts {
            creator: get_req(0, "creator")?,
            authority: get_req(1, "authority")?,
            pool_state: get_req(2, "pool_state")?,
            amm_config: get_req(3, "amm_config")?,
            token_0_vault: get_req(4, "token_0_vault")?,
            token_1_vault: get_req(5, "token_1_vault")?,
            vault_0_mint: get_req(6, "vault_0_mint")?,
            vault_1_mint: get_req(7, "vault_1_mint")?,
            creator_token_0: get_req(8, "creator_token_0")?,
            creator_token_1: get_req(9, "creator_token_1")?,
            token_0_program: get_req(10, "token_0_program")?,
            token_1_program: get_req(11, "token_1_program")?,
            associated_token_program: get_req(12, "associated_token_program")?,
            system_program: get_req(13, "system_program")?,
        })
    }
}

pub fn get_collect_creator_fee_accounts(ix: &InstructionView) -> Result<CollectCreatorFeeAccounts, AccountsError> {
    CollectCreatorFeeAccounts::try_from(ix)
}

/// Accounts for the `collect_fund_fee` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectFundFeeAccounts {
    /// Only admin or fund_owner can collect fee now
    pub owner: Pubkey,
    pub authority: Pubkey,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: Pubkey,
    /// Amm config account stores fund_owner
    pub amm_config: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: Pubkey,
    /// The mint of token_0 vault
    pub vault_0_mint: Pubkey,
    /// The mint of token_1 vault
    pub vault_1_mint: Pubkey,
    /// The address that receives the collected token_0 fund fees
    pub recipient_token_0_account: Pubkey,
    /// The address that receives the collected token_1 fund fees
    pub recipient_token_1_account: Pubkey,
    /// The SPL program to perform token transfers
    pub token_program: Pubkey,
    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CollectFundFeeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(CollectFundFeeAccounts {
            owner: get_req(0, "owner")?,
            authority: get_req(1, "authority")?,
            pool_state: get_req(2, "pool_state")?,
            amm_config: get_req(3, "amm_config")?,
            token_0_vault: get_req(4, "token_0_vault")?,
            token_1_vault: get_req(5, "token_1_vault")?,
            vault_0_mint: get_req(6, "vault_0_mint")?,
            vault_1_mint: get_req(7, "vault_1_mint")?,
            recipient_token_0_account: get_req(8, "recipient_token_0_account")?,
            recipient_token_1_account: get_req(9, "recipient_token_1_account")?,
            token_program: get_req(10, "token_program")?,
            token_program_2022: get_req(11, "token_program_2022")?,
        })
    }
}

pub fn get_collect_fund_fee_accounts(ix: &InstructionView) -> Result<CollectFundFeeAccounts, AccountsError> {
    CollectFundFeeAccounts::try_from(ix)
}

/// Accounts for the `collect_protocol_fee` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectProtocolFeeAccounts {
    /// Only admin or owner can collect fee now
    pub owner: Pubkey,
    pub authority: Pubkey,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: Pubkey,
    /// Amm config account stores owner
    pub amm_config: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: Pubkey,
    /// The mint of token_0 vault
    pub vault_0_mint: Pubkey,
    /// The mint of token_1 vault
    pub vault_1_mint: Pubkey,
    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_0_account: Pubkey,
    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_1_account: Pubkey,
    /// The SPL program to perform token transfers
    pub token_program: Pubkey,
    /// The SPL program 2022 to perform token transfers
    pub token_program_2022: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CollectProtocolFeeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(CollectProtocolFeeAccounts {
            owner: get_req(0, "owner")?,
            authority: get_req(1, "authority")?,
            pool_state: get_req(2, "pool_state")?,
            amm_config: get_req(3, "amm_config")?,
            token_0_vault: get_req(4, "token_0_vault")?,
            token_1_vault: get_req(5, "token_1_vault")?,
            vault_0_mint: get_req(6, "vault_0_mint")?,
            vault_1_mint: get_req(7, "vault_1_mint")?,
            recipient_token_0_account: get_req(8, "recipient_token_0_account")?,
            recipient_token_1_account: get_req(9, "recipient_token_1_account")?,
            token_program: get_req(10, "token_program")?,
            token_program_2022: get_req(11, "token_program_2022")?,
        })
    }
}

pub fn get_collect_protocol_fee_accounts(ix: &InstructionView) -> Result<CollectProtocolFeeAccounts, AccountsError> {
    CollectProtocolFeeAccounts::try_from(ix)
}

/// Accounts for the `create_amm_config` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateAmmConfigAccounts {
    /// Address to be set as protocol owner.
    pub owner: Pubkey,
    /// Initialize config state account to store protocol owner address and fee rates.
    pub amm_config: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateAmmConfigAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(CreateAmmConfigAccounts {
            owner: get_req(0, "owner")?,
            amm_config: get_req(1, "amm_config")?,
            system_program: get_req(2, "system_program")?,
        })
    }
}

pub fn get_create_amm_config_accounts(ix: &InstructionView) -> Result<CreateAmmConfigAccounts, AccountsError> {
    CreateAmmConfigAccounts::try_from(ix)
}

/// Accounts for the `create_permission_pda` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreatePermissionPdaAccounts {
    pub owner: Pubkey,
    pub permission_authority: Pubkey,
    /// Initialize config state account to store protocol owner address and fee rates.
    pub permission: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreatePermissionPdaAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(CreatePermissionPdaAccounts {
            owner: get_req(0, "owner")?,
            permission_authority: get_req(1, "permission_authority")?,
            permission: get_req(2, "permission")?,
            system_program: get_req(3, "system_program")?,
        })
    }
}

pub fn get_create_permission_pda_accounts(ix: &InstructionView) -> Result<CreatePermissionPdaAccounts, AccountsError> {
    CreatePermissionPdaAccounts::try_from(ix)
}

/// Accounts for the `deposit` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositAccounts {
    /// Pays to mint the position
    pub owner: Pubkey,
    pub authority: Pubkey,
    pub pool_state: Pubkey,
    /// Owner lp token account
    pub owner_lp_token: Pubkey,
    /// The payer's token account for token_0
    pub token_0_account: Pubkey,
    /// The payer's token account for token_1
    pub token_1_account: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: Pubkey,
    /// token Program
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
    /// The mint of token_0 vault
    pub vault_0_mint: Pubkey,
    /// The mint of token_1 vault
    pub vault_1_mint: Pubkey,
    /// Lp token mint
    pub lp_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for DepositAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(DepositAccounts {
            owner: get_req(0, "owner")?,
            authority: get_req(1, "authority")?,
            pool_state: get_req(2, "pool_state")?,
            owner_lp_token: get_req(3, "owner_lp_token")?,
            token_0_account: get_req(4, "token_0_account")?,
            token_1_account: get_req(5, "token_1_account")?,
            token_0_vault: get_req(6, "token_0_vault")?,
            token_1_vault: get_req(7, "token_1_vault")?,
            token_program: get_req(8, "token_program")?,
            token_program_2022: get_req(9, "token_program_2022")?,
            vault_0_mint: get_req(10, "vault_0_mint")?,
            vault_1_mint: get_req(11, "vault_1_mint")?,
            lp_mint: get_req(12, "lp_mint")?,
        })
    }
}

pub fn get_deposit_accounts(ix: &InstructionView) -> Result<DepositAccounts, AccountsError> {
    DepositAccounts::try_from(ix)
}

/// Accounts for the `initialize` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeAccounts {
    /// Address paying to create the pool. Can be anyone
    pub creator: Pubkey,
    /// Which config the pool belongs to.
    pub amm_config: Pubkey,
    /// pool vault and lp mint authority
    pub authority: Pubkey,
    /// PDA account:
    /// seeds = [
    /// POOL_SEED.as_bytes(),
    /// amm_config.key().as_ref(),
    /// token_0_mint.key().as_ref(),
    /// token_1_mint.key().as_ref(),
    /// ],
    ///
    /// Or random account: must be signed by cli
    pub pool_state: Pubkey,
    /// Token_0 mint, the key must smaller than token_1 mint.
    pub token_0_mint: Pubkey,
    /// Token_1 mint, the key must grater then token_0 mint.
    pub token_1_mint: Pubkey,
    /// pool lp mint
    pub lp_mint: Pubkey,
    /// payer token0 account
    pub creator_token_0: Pubkey,
    /// creator token1 account
    pub creator_token_1: Pubkey,
    /// creator lp token account
    pub creator_lp_token: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    /// create pool fee account
    pub create_pool_fee: Pubkey,
    /// an account to store oracle observations
    pub observation_state: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
    /// Spl token program or token program 2022
    pub token_0_program: Pubkey,
    /// Spl token program or token program 2022
    pub token_1_program: Pubkey,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Pubkey,
    /// To create a new program account
    pub system_program: Pubkey,
    /// Sysvar for program account
    pub rent: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(InitializeAccounts {
            creator: get_req(0, "creator")?,
            amm_config: get_req(1, "amm_config")?,
            authority: get_req(2, "authority")?,
            pool_state: get_req(3, "pool_state")?,
            token_0_mint: get_req(4, "token_0_mint")?,
            token_1_mint: get_req(5, "token_1_mint")?,
            lp_mint: get_req(6, "lp_mint")?,
            creator_token_0: get_req(7, "creator_token_0")?,
            creator_token_1: get_req(8, "creator_token_1")?,
            creator_lp_token: get_req(9, "creator_lp_token")?,
            token_0_vault: get_req(10, "token_0_vault")?,
            token_1_vault: get_req(11, "token_1_vault")?,
            create_pool_fee: get_req(12, "create_pool_fee")?,
            observation_state: get_req(13, "observation_state")?,
            token_program: get_req(14, "token_program")?,
            token_0_program: get_req(15, "token_0_program")?,
            token_1_program: get_req(16, "token_1_program")?,
            associated_token_program: get_req(17, "associated_token_program")?,
            system_program: get_req(18, "system_program")?,
            rent: get_req(19, "rent")?,
        })
    }
}

pub fn get_initialize_accounts(ix: &InstructionView) -> Result<InitializeAccounts, AccountsError> {
    InitializeAccounts::try_from(ix)
}

/// Accounts for the `initialize_with_permission` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeWithPermissionAccounts {
    /// Address paying to create the pool. Can be anyone
    pub payer: Pubkey,
    pub creator: Pubkey,
    /// Which config the pool belongs to.
    pub amm_config: Pubkey,
    /// pool vault and lp mint authority
    pub authority: Pubkey,
    /// PDA account:
    /// seeds = [
    /// POOL_SEED.as_bytes(),
    /// amm_config.key().as_ref(),
    /// token_0_mint.key().as_ref(),
    /// token_1_mint.key().as_ref(),
    /// ],
    ///
    /// Or random account: must be signed by cli
    pub pool_state: Pubkey,
    /// Token_0 mint, the key must smaller than token_1 mint.
    pub token_0_mint: Pubkey,
    /// Token_1 mint, the key must grater then token_0 mint.
    pub token_1_mint: Pubkey,
    /// pool lp mint
    pub lp_mint: Pubkey,
    /// payer token0 account
    pub payer_token_0: Pubkey,
    /// payer token1 account
    pub payer_token_1: Pubkey,
    /// payer lp token account
    pub payer_lp_token: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    /// create pool fee account
    pub create_pool_fee: Pubkey,
    /// an account to store oracle observations
    pub observation_state: Pubkey,
    pub permission: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
    /// Spl token program or token program 2022
    pub token_0_program: Pubkey,
    /// Spl token program or token program 2022
    pub token_1_program: Pubkey,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Pubkey,
    /// To create a new program account
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeWithPermissionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(InitializeWithPermissionAccounts {
            payer: get_req(0, "payer")?,
            creator: get_req(1, "creator")?,
            amm_config: get_req(2, "amm_config")?,
            authority: get_req(3, "authority")?,
            pool_state: get_req(4, "pool_state")?,
            token_0_mint: get_req(5, "token_0_mint")?,
            token_1_mint: get_req(6, "token_1_mint")?,
            lp_mint: get_req(7, "lp_mint")?,
            payer_token_0: get_req(8, "payer_token_0")?,
            payer_token_1: get_req(9, "payer_token_1")?,
            payer_lp_token: get_req(10, "payer_lp_token")?,
            token_0_vault: get_req(11, "token_0_vault")?,
            token_1_vault: get_req(12, "token_1_vault")?,
            create_pool_fee: get_req(13, "create_pool_fee")?,
            observation_state: get_req(14, "observation_state")?,
            permission: get_req(15, "permission")?,
            token_program: get_req(16, "token_program")?,
            token_0_program: get_req(17, "token_0_program")?,
            token_1_program: get_req(18, "token_1_program")?,
            associated_token_program: get_req(19, "associated_token_program")?,
            system_program: get_req(20, "system_program")?,
        })
    }
}

pub fn get_initialize_with_permission_accounts(ix: &InstructionView) -> Result<InitializeWithPermissionAccounts, AccountsError> {
    InitializeWithPermissionAccounts::try_from(ix)
}

/// Accounts for the `swap_base_input` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseInputAccounts {
    /// The user performing the swap
    pub payer: Pubkey,
    pub authority: Pubkey,
    /// The factory state to read protocol fees
    pub amm_config: Pubkey,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: Pubkey,
    /// The user token account for input token
    pub input_token_account: Pubkey,
    /// The user token account for output token
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    pub input_vault: Pubkey,
    /// The vault token account for output token
    pub output_vault: Pubkey,
    /// SPL program for input token transfers
    pub input_token_program: Pubkey,
    /// SPL program for output token transfers
    pub output_token_program: Pubkey,
    /// The mint of input token
    pub input_token_mint: Pubkey,
    /// The mint of output token
    pub output_token_mint: Pubkey,
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapBaseInputAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(SwapBaseInputAccounts {
            payer: get_req(0, "payer")?,
            authority: get_req(1, "authority")?,
            amm_config: get_req(2, "amm_config")?,
            pool_state: get_req(3, "pool_state")?,
            input_token_account: get_req(4, "input_token_account")?,
            output_token_account: get_req(5, "output_token_account")?,
            input_vault: get_req(6, "input_vault")?,
            output_vault: get_req(7, "output_vault")?,
            input_token_program: get_req(8, "input_token_program")?,
            output_token_program: get_req(9, "output_token_program")?,
            input_token_mint: get_req(10, "input_token_mint")?,
            output_token_mint: get_req(11, "output_token_mint")?,
            observation_state: get_req(12, "observation_state")?,
        })
    }
}

pub fn get_swap_base_input_accounts(ix: &InstructionView) -> Result<SwapBaseInputAccounts, AccountsError> {
    SwapBaseInputAccounts::try_from(ix)
}

/// Accounts for the `swap_base_output` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseOutputAccounts {
    /// The user performing the swap
    pub payer: Pubkey,
    pub authority: Pubkey,
    /// The factory state to read protocol fees
    pub amm_config: Pubkey,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: Pubkey,
    /// The user token account for input token
    pub input_token_account: Pubkey,
    /// The user token account for output token
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    pub input_vault: Pubkey,
    /// The vault token account for output token
    pub output_vault: Pubkey,
    /// SPL program for input token transfers
    pub input_token_program: Pubkey,
    /// SPL program for output token transfers
    pub output_token_program: Pubkey,
    /// The mint of input token
    pub input_token_mint: Pubkey,
    /// The mint of output token
    pub output_token_mint: Pubkey,
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapBaseOutputAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(SwapBaseOutputAccounts {
            payer: get_req(0, "payer")?,
            authority: get_req(1, "authority")?,
            amm_config: get_req(2, "amm_config")?,
            pool_state: get_req(3, "pool_state")?,
            input_token_account: get_req(4, "input_token_account")?,
            output_token_account: get_req(5, "output_token_account")?,
            input_vault: get_req(6, "input_vault")?,
            output_vault: get_req(7, "output_vault")?,
            input_token_program: get_req(8, "input_token_program")?,
            output_token_program: get_req(9, "output_token_program")?,
            input_token_mint: get_req(10, "input_token_mint")?,
            output_token_mint: get_req(11, "output_token_mint")?,
            observation_state: get_req(12, "observation_state")?,
        })
    }
}

pub fn get_swap_base_output_accounts(ix: &InstructionView) -> Result<SwapBaseOutputAccounts, AccountsError> {
    SwapBaseOutputAccounts::try_from(ix)
}

/// Accounts for the `update_amm_config` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateAmmConfigAccounts {
    /// The amm config owner or admin
    pub owner: Pubkey,
    /// Amm config account to be changed
    pub amm_config: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for UpdateAmmConfigAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(UpdateAmmConfigAccounts {
            owner: get_req(0, "owner")?,
            amm_config: get_req(1, "amm_config")?,
        })
    }
}

pub fn get_update_amm_config_accounts(ix: &InstructionView) -> Result<UpdateAmmConfigAccounts, AccountsError> {
    UpdateAmmConfigAccounts::try_from(ix)
}

/// Accounts for the `update_pool_status` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdatePoolStatusAccounts {
    pub authority: Pubkey,
    pub pool_state: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for UpdatePoolStatusAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(UpdatePoolStatusAccounts {
            authority: get_req(0, "authority")?,
            pool_state: get_req(1, "pool_state")?,
        })
    }
}

pub fn get_update_pool_status_accounts(ix: &InstructionView) -> Result<UpdatePoolStatusAccounts, AccountsError> {
    UpdatePoolStatusAccounts::try_from(ix)
}

/// Accounts for the `withdraw` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawAccounts {
    /// Pays to mint the position
    pub owner: Pubkey,
    pub authority: Pubkey,
    /// Pool state account
    pub pool_state: Pubkey,
    /// Owner lp token account
    pub owner_lp_token: Pubkey,
    /// The token account for receive token_0,
    pub token_0_account: Pubkey,
    /// The token account for receive token_1
    pub token_1_account: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: Pubkey,
    /// token Program
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
    /// The mint of token_0 vault
    pub vault_0_mint: Pubkey,
    /// The mint of token_1 vault
    pub vault_1_mint: Pubkey,
    /// Pool lp token mint
    pub lp_mint: Pubkey,
    /// memo program
    pub memo_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for WithdrawAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, &a.0)
        };
        Ok(WithdrawAccounts {
            owner: get_req(0, "owner")?,
            authority: get_req(1, "authority")?,
            pool_state: get_req(2, "pool_state")?,
            owner_lp_token: get_req(3, "owner_lp_token")?,
            token_0_account: get_req(4, "token_0_account")?,
            token_1_account: get_req(5, "token_1_account")?,
            token_0_vault: get_req(6, "token_0_vault")?,
            token_1_vault: get_req(7, "token_1_vault")?,
            token_program: get_req(8, "token_program")?,
            token_program_2022: get_req(9, "token_program_2022")?,
            vault_0_mint: get_req(10, "vault_0_mint")?,
            vault_1_mint: get_req(11, "vault_1_mint")?,
            lp_mint: get_req(12, "lp_mint")?,
            memo_program: get_req(13, "memo_program")?,
        })
    }
}

pub fn get_withdraw_accounts(ix: &InstructionView) -> Result<WithdrawAccounts, AccountsError> {
    WithdrawAccounts::try_from(ix)
}
