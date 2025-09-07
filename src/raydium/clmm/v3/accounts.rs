use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::accounts::AccountsError;

/// Accounts for the `close_position` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClosePositionAccounts {
    /// The position nft owner
    pub nft_owner: Pubkey,
    /// Mint address bound to the personal position.
    pub position_nft_mint: Pubkey,
    /// User token account where position NFT be minted to
    pub position_nft_account: Pubkey,
    pub personal_position: Pubkey,
    /// System program to close the position state account
    pub system_program: Pubkey,
    /// Token/Token2022 program to close token/mint account
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClosePositionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(ClosePositionAccounts {
            nft_owner: get_req(0, "nft_owner")?,
            position_nft_mint: get_req(1, "position_nft_mint")?,
            position_nft_account: get_req(2, "position_nft_account")?,
            personal_position: get_req(3, "personal_position")?,
            system_program: get_req(4, "system_program")?,
            token_program: get_req(5, "token_program")?,
        })
    }
}

pub fn get_close_position_accounts(ix: &InstructionView) -> Result<ClosePositionAccounts, AccountsError> {
    ClosePositionAccounts::try_from(ix)
}

/// Accounts for the `collect_fund_fee` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectFundFeeAccounts {
    /// Only admin or fund_owner can collect fee now
    pub owner: Pubkey,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: Pubkey,
    /// Amm config account stores fund_owner
    pub amm_config: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_account_0: Pubkey,
    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_account_1: Pubkey,
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
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(CollectFundFeeAccounts {
            owner: get_req(0, "owner")?,
            pool_state: get_req(1, "pool_state")?,
            amm_config: get_req(2, "amm_config")?,
            token_vault_0: get_req(3, "token_vault_0")?,
            token_vault_1: get_req(4, "token_vault_1")?,
            vault_0_mint: get_req(5, "vault_0_mint")?,
            vault_1_mint: get_req(6, "vault_1_mint")?,
            recipient_token_account_0: get_req(7, "recipient_token_account_0")?,
            recipient_token_account_1: get_req(8, "recipient_token_account_1")?,
            token_program: get_req(9, "token_program")?,
            token_program_2022: get_req(10, "token_program_2022")?,
        })
    }
}

pub fn get_collect_fund_fee_accounts(ix: &InstructionView) -> Result<CollectFundFeeAccounts, AccountsError> {
    CollectFundFeeAccounts::try_from(ix)
}

/// Accounts for the `collect_protocol_fee` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectProtocolFeeAccounts {
    /// Only admin or config owner can collect fee now
    pub owner: Pubkey,
    /// Pool state stores accumulated protocol fee amount
    pub pool_state: Pubkey,
    /// Amm config account stores owner
    pub amm_config: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_account_0: Pubkey,
    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_account_1: Pubkey,
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
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(CollectProtocolFeeAccounts {
            owner: get_req(0, "owner")?,
            pool_state: get_req(1, "pool_state")?,
            amm_config: get_req(2, "amm_config")?,
            token_vault_0: get_req(3, "token_vault_0")?,
            token_vault_1: get_req(4, "token_vault_1")?,
            vault_0_mint: get_req(5, "vault_0_mint")?,
            vault_1_mint: get_req(6, "vault_1_mint")?,
            recipient_token_account_0: get_req(7, "recipient_token_account_0")?,
            recipient_token_account_1: get_req(8, "recipient_token_account_1")?,
            token_program: get_req(9, "token_program")?,
            token_program_2022: get_req(10, "token_program_2022")?,
        })
    }
}

pub fn get_collect_protocol_fee_accounts(ix: &InstructionView) -> Result<CollectProtocolFeeAccounts, AccountsError> {
    CollectProtocolFeeAccounts::try_from(ix)
}

/// Accounts for the `collect_remaining_rewards` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectRemainingRewardsAccounts {
    /// The founder who init reward info previously
    pub reward_funder: Pubkey,
    /// The funder's reward token account
    pub funder_token_account: Pubkey,
    /// Set reward for this pool
    pub pool_state: Pubkey,
    /// Reward vault transfer remaining token to founder token account
    pub reward_token_vault: Pubkey,
    /// The mint of reward token vault
    pub reward_vault_mint: Pubkey,
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
    /// memo program
    pub memo_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CollectRemainingRewardsAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(CollectRemainingRewardsAccounts {
            reward_funder: get_req(0, "reward_funder")?,
            funder_token_account: get_req(1, "funder_token_account")?,
            pool_state: get_req(2, "pool_state")?,
            reward_token_vault: get_req(3, "reward_token_vault")?,
            reward_vault_mint: get_req(4, "reward_vault_mint")?,
            token_program: get_req(5, "token_program")?,
            token_program_2022: get_req(6, "token_program_2022")?,
            memo_program: get_req(7, "memo_program")?,
        })
    }
}

pub fn get_collect_remaining_rewards_accounts(ix: &InstructionView) -> Result<CollectRemainingRewardsAccounts, AccountsError> {
    CollectRemainingRewardsAccounts::try_from(ix)
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
            crate::accounts::to_pubkey(name, index, a.0)
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

/// Accounts for the `create_operation_account` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateOperationAccountAccounts {
    /// Address to be set as operation account owner.
    pub owner: Pubkey,
    /// Initialize operation state account to store operation owner address and white list mint.
    pub operation_state: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateOperationAccountAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(CreateOperationAccountAccounts {
            owner: get_req(0, "owner")?,
            operation_state: get_req(1, "operation_state")?,
            system_program: get_req(2, "system_program")?,
        })
    }
}

pub fn get_create_operation_account_accounts(ix: &InstructionView) -> Result<CreateOperationAccountAccounts, AccountsError> {
    CreateOperationAccountAccounts::try_from(ix)
}

/// Accounts for the `create_pool` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreatePoolAccounts {
    /// Address paying to create the pool. Can be anyone
    pub pool_creator: Pubkey,
    /// Which config the pool belongs to.
    pub amm_config: Pubkey,
    /// Initialize an account to store the pool state
    pub pool_state: Pubkey,
    /// Token_0 mint, the key must be smaller then token_1 mint.
    pub token_mint_0: Pubkey,
    /// Token_1 mint
    pub token_mint_1: Pubkey,
    /// Token_0 vault for the pool
    pub token_vault_0: Pubkey,
    /// Token_1 vault for the pool
    pub token_vault_1: Pubkey,
    /// Initialize an account to store oracle observations
    pub observation_state: Pubkey,
    /// Initialize an account to store if a tick array is initialized.
    pub tick_array_bitmap: Pubkey,
    /// Spl token program or token program 2022
    pub token_program_0: Pubkey,
    /// Spl token program or token program 2022
    pub token_program_1: Pubkey,
    /// To create a new program account
    pub system_program: Pubkey,
    /// Sysvar for program account
    pub rent: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreatePoolAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(CreatePoolAccounts {
            pool_creator: get_req(0, "pool_creator")?,
            amm_config: get_req(1, "amm_config")?,
            pool_state: get_req(2, "pool_state")?,
            token_mint_0: get_req(3, "token_mint_0")?,
            token_mint_1: get_req(4, "token_mint_1")?,
            token_vault_0: get_req(5, "token_vault_0")?,
            token_vault_1: get_req(6, "token_vault_1")?,
            observation_state: get_req(7, "observation_state")?,
            tick_array_bitmap: get_req(8, "tick_array_bitmap")?,
            token_program_0: get_req(9, "token_program_0")?,
            token_program_1: get_req(10, "token_program_1")?,
            system_program: get_req(11, "system_program")?,
            rent: get_req(12, "rent")?,
        })
    }
}

pub fn get_create_pool_accounts(ix: &InstructionView) -> Result<CreatePoolAccounts, AccountsError> {
    CreatePoolAccounts::try_from(ix)
}

/// Accounts for the `create_support_mint_associated` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateSupportMintAssociatedAccounts {
    /// Address to be set as protocol owner.
    pub owner: Pubkey,
    /// Support token mint
    pub token_mint: Pubkey,
    /// Initialize support mint state account to store support mint address and bump.
    pub support_mint_associated: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateSupportMintAssociatedAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(CreateSupportMintAssociatedAccounts {
            owner: get_req(0, "owner")?,
            token_mint: get_req(1, "token_mint")?,
            support_mint_associated: get_req(2, "support_mint_associated")?,
            system_program: get_req(3, "system_program")?,
        })
    }
}

pub fn get_create_support_mint_associated_accounts(ix: &InstructionView) -> Result<CreateSupportMintAssociatedAccounts, AccountsError> {
    CreateSupportMintAssociatedAccounts::try_from(ix)
}

/// Accounts for the `decrease_liquidity` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DecreaseLiquidityAccounts {
    /// The position owner or delegated authority
    pub nft_owner: Pubkey,
    /// The token account for the tokenized position
    pub nft_account: Pubkey,
    /// Decrease liquidity for this position
    pub personal_position: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    /// Token_0 vault
    pub token_vault_0: Pubkey,
    /// Token_1 vault
    pub token_vault_1: Pubkey,
    /// Stores init state for the lower tick
    pub tick_array_lower: Pubkey,
    /// Stores init state for the upper tick
    pub tick_array_upper: Pubkey,
    /// The destination token account for receive amount_0
    pub recipient_token_account_0: Pubkey,
    /// The destination token account for receive amount_1
    pub recipient_token_account_1: Pubkey,
    /// SPL program to transfer out tokens
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for DecreaseLiquidityAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(DecreaseLiquidityAccounts {
            nft_owner: get_req(0, "nft_owner")?,
            nft_account: get_req(1, "nft_account")?,
            personal_position: get_req(2, "personal_position")?,
            pool_state: get_req(3, "pool_state")?,
            protocol_position: get_req(4, "protocol_position")?,
            token_vault_0: get_req(5, "token_vault_0")?,
            token_vault_1: get_req(6, "token_vault_1")?,
            tick_array_lower: get_req(7, "tick_array_lower")?,
            tick_array_upper: get_req(8, "tick_array_upper")?,
            recipient_token_account_0: get_req(9, "recipient_token_account_0")?,
            recipient_token_account_1: get_req(10, "recipient_token_account_1")?,
            token_program: get_req(11, "token_program")?,
        })
    }
}

pub fn get_decrease_liquidity_accounts(ix: &InstructionView) -> Result<DecreaseLiquidityAccounts, AccountsError> {
    DecreaseLiquidityAccounts::try_from(ix)
}

/// Accounts for the `decrease_liquidity_v2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DecreaseLiquidityV2Accounts {
    /// The position owner or delegated authority
    pub nft_owner: Pubkey,
    /// The token account for the tokenized position
    pub nft_account: Pubkey,
    /// Decrease liquidity for this position
    pub personal_position: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    /// Token_0 vault
    pub token_vault_0: Pubkey,
    /// Token_1 vault
    pub token_vault_1: Pubkey,
    /// Stores init state for the lower tick
    pub tick_array_lower: Pubkey,
    /// Stores init state for the upper tick
    pub tick_array_upper: Pubkey,
    /// The destination token account for receive amount_0
    pub recipient_token_account_0: Pubkey,
    /// The destination token account for receive amount_1
    pub recipient_token_account_1: Pubkey,
    /// SPL program to transfer out tokens
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
    /// memo program
    pub memo_program: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for DecreaseLiquidityV2Accounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(DecreaseLiquidityV2Accounts {
            nft_owner: get_req(0, "nft_owner")?,
            nft_account: get_req(1, "nft_account")?,
            personal_position: get_req(2, "personal_position")?,
            pool_state: get_req(3, "pool_state")?,
            protocol_position: get_req(4, "protocol_position")?,
            token_vault_0: get_req(5, "token_vault_0")?,
            token_vault_1: get_req(6, "token_vault_1")?,
            tick_array_lower: get_req(7, "tick_array_lower")?,
            tick_array_upper: get_req(8, "tick_array_upper")?,
            recipient_token_account_0: get_req(9, "recipient_token_account_0")?,
            recipient_token_account_1: get_req(10, "recipient_token_account_1")?,
            token_program: get_req(11, "token_program")?,
            token_program_2022: get_req(12, "token_program_2022")?,
            memo_program: get_req(13, "memo_program")?,
            vault_0_mint: get_req(14, "vault_0_mint")?,
            vault_1_mint: get_req(15, "vault_1_mint")?,
        })
    }
}

pub fn get_decrease_liquidity_v2_accounts(ix: &InstructionView) -> Result<DecreaseLiquidityV2Accounts, AccountsError> {
    DecreaseLiquidityV2Accounts::try_from(ix)
}

/// Accounts for the `increase_liquidity` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct IncreaseLiquidityAccounts {
    /// Pays to mint the position
    pub nft_owner: Pubkey,
    /// The token account for nft
    pub nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    /// Increase liquidity for this position
    pub personal_position: Pubkey,
    /// Stores init state for the lower tick
    pub tick_array_lower: Pubkey,
    /// Stores init state for the upper tick
    pub tick_array_upper: Pubkey,
    /// The payer's token account for token_0
    pub token_account_0: Pubkey,
    /// The token account spending token_1 to mint the position
    pub token_account_1: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for IncreaseLiquidityAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(IncreaseLiquidityAccounts {
            nft_owner: get_req(0, "nft_owner")?,
            nft_account: get_req(1, "nft_account")?,
            pool_state: get_req(2, "pool_state")?,
            protocol_position: get_req(3, "protocol_position")?,
            personal_position: get_req(4, "personal_position")?,
            tick_array_lower: get_req(5, "tick_array_lower")?,
            tick_array_upper: get_req(6, "tick_array_upper")?,
            token_account_0: get_req(7, "token_account_0")?,
            token_account_1: get_req(8, "token_account_1")?,
            token_vault_0: get_req(9, "token_vault_0")?,
            token_vault_1: get_req(10, "token_vault_1")?,
            token_program: get_req(11, "token_program")?,
        })
    }
}

pub fn get_increase_liquidity_accounts(ix: &InstructionView) -> Result<IncreaseLiquidityAccounts, AccountsError> {
    IncreaseLiquidityAccounts::try_from(ix)
}

/// Accounts for the `increase_liquidity_v2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct IncreaseLiquidityV2Accounts {
    /// Pays to mint the position
    pub nft_owner: Pubkey,
    /// The token account for nft
    pub nft_account: Pubkey,
    pub pool_state: Pubkey,
    pub protocol_position: Pubkey,
    /// Increase liquidity for this position
    pub personal_position: Pubkey,
    /// Stores init state for the lower tick
    pub tick_array_lower: Pubkey,
    /// Stores init state for the upper tick
    pub tick_array_upper: Pubkey,
    /// The payer's token account for token_0
    pub token_account_0: Pubkey,
    /// The token account spending token_1 to mint the position
    pub token_account_1: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for IncreaseLiquidityV2Accounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(IncreaseLiquidityV2Accounts {
            nft_owner: get_req(0, "nft_owner")?,
            nft_account: get_req(1, "nft_account")?,
            pool_state: get_req(2, "pool_state")?,
            protocol_position: get_req(3, "protocol_position")?,
            personal_position: get_req(4, "personal_position")?,
            tick_array_lower: get_req(5, "tick_array_lower")?,
            tick_array_upper: get_req(6, "tick_array_upper")?,
            token_account_0: get_req(7, "token_account_0")?,
            token_account_1: get_req(8, "token_account_1")?,
            token_vault_0: get_req(9, "token_vault_0")?,
            token_vault_1: get_req(10, "token_vault_1")?,
            token_program: get_req(11, "token_program")?,
            token_program_2022: get_req(12, "token_program_2022")?,
            vault_0_mint: get_req(13, "vault_0_mint")?,
            vault_1_mint: get_req(14, "vault_1_mint")?,
        })
    }
}

pub fn get_increase_liquidity_v2_accounts(ix: &InstructionView) -> Result<IncreaseLiquidityV2Accounts, AccountsError> {
    IncreaseLiquidityV2Accounts::try_from(ix)
}

/// Accounts for the `initialize_reward` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeRewardAccounts {
    /// The founder deposit reward token to vault
    pub reward_funder: Pubkey,
    pub funder_token_account: Pubkey,
    /// For check the reward_funder authority
    pub amm_config: Pubkey,
    /// Set reward for this pool
    pub pool_state: Pubkey,
    /// load info from the account to judge reward permission
    pub operation_state: Pubkey,
    /// Reward mint
    pub reward_token_mint: Pubkey,
    /// A pda, reward vault
    pub reward_token_vault: Pubkey,
    pub reward_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeRewardAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(InitializeRewardAccounts {
            reward_funder: get_req(0, "reward_funder")?,
            funder_token_account: get_req(1, "funder_token_account")?,
            amm_config: get_req(2, "amm_config")?,
            pool_state: get_req(3, "pool_state")?,
            operation_state: get_req(4, "operation_state")?,
            reward_token_mint: get_req(5, "reward_token_mint")?,
            reward_token_vault: get_req(6, "reward_token_vault")?,
            reward_token_program: get_req(7, "reward_token_program")?,
            system_program: get_req(8, "system_program")?,
            rent: get_req(9, "rent")?,
        })
    }
}

pub fn get_initialize_reward_accounts(ix: &InstructionView) -> Result<InitializeRewardAccounts, AccountsError> {
    InitializeRewardAccounts::try_from(ix)
}

/// Accounts for the `open_position` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionAccounts {
    /// Pays to mint the position
    pub payer: Pubkey,
    pub position_nft_owner: Pubkey,
    /// Unique token mint address
    pub position_nft_mint: Pubkey,
    /// Token account where position NFT will be minted
    /// This account created in the contract by cpi to avoid large stack variables
    pub position_nft_account: Pubkey,
    /// To store metaplex metadata
    pub metadata_account: Pubkey,
    /// Add liquidity for this pool
    pub pool_state: Pubkey,
    /// Store the information of market marking in range
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    /// personal position state
    pub personal_position: Pubkey,
    /// The token_0 account deposit token to the pool
    pub token_account_0: Pubkey,
    /// The token_1 account deposit token to the pool
    pub token_account_1: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: Pubkey,
    /// Sysvar for token mint and ATA creation
    pub rent: Pubkey,
    /// Program to create the position manager state account
    pub system_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Pubkey,
    /// Program to create NFT metadata
    pub metadata_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for OpenPositionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(OpenPositionAccounts {
            payer: get_req(0, "payer")?,
            position_nft_owner: get_req(1, "position_nft_owner")?,
            position_nft_mint: get_req(2, "position_nft_mint")?,
            position_nft_account: get_req(3, "position_nft_account")?,
            metadata_account: get_req(4, "metadata_account")?,
            pool_state: get_req(5, "pool_state")?,
            protocol_position: get_req(6, "protocol_position")?,
            tick_array_lower: get_req(7, "tick_array_lower")?,
            tick_array_upper: get_req(8, "tick_array_upper")?,
            personal_position: get_req(9, "personal_position")?,
            token_account_0: get_req(10, "token_account_0")?,
            token_account_1: get_req(11, "token_account_1")?,
            token_vault_0: get_req(12, "token_vault_0")?,
            token_vault_1: get_req(13, "token_vault_1")?,
            rent: get_req(14, "rent")?,
            system_program: get_req(15, "system_program")?,
            token_program: get_req(16, "token_program")?,
            associated_token_program: get_req(17, "associated_token_program")?,
            metadata_program: get_req(18, "metadata_program")?,
        })
    }
}

pub fn get_open_position_accounts(ix: &InstructionView) -> Result<OpenPositionAccounts, AccountsError> {
    OpenPositionAccounts::try_from(ix)
}

/// Accounts for the `open_position_v2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionV2Accounts {
    /// Pays to mint the position
    pub payer: Pubkey,
    pub position_nft_owner: Pubkey,
    /// Unique token mint address
    pub position_nft_mint: Pubkey,
    /// Token account where position NFT will be minted
    pub position_nft_account: Pubkey,
    /// To store metaplex metadata
    pub metadata_account: Pubkey,
    /// Add liquidity for this pool
    pub pool_state: Pubkey,
    /// Store the information of market marking in range
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    /// personal position state
    pub personal_position: Pubkey,
    /// The token_0 account deposit token to the pool
    pub token_account_0: Pubkey,
    /// The token_1 account deposit token to the pool
    pub token_account_1: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: Pubkey,
    /// Sysvar for token mint and ATA creation
    pub rent: Pubkey,
    /// Program to create the position manager state account
    pub system_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Pubkey,
    /// Program to create NFT metadata
    pub metadata_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program_2022: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for OpenPositionV2Accounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(OpenPositionV2Accounts {
            payer: get_req(0, "payer")?,
            position_nft_owner: get_req(1, "position_nft_owner")?,
            position_nft_mint: get_req(2, "position_nft_mint")?,
            position_nft_account: get_req(3, "position_nft_account")?,
            metadata_account: get_req(4, "metadata_account")?,
            pool_state: get_req(5, "pool_state")?,
            protocol_position: get_req(6, "protocol_position")?,
            tick_array_lower: get_req(7, "tick_array_lower")?,
            tick_array_upper: get_req(8, "tick_array_upper")?,
            personal_position: get_req(9, "personal_position")?,
            token_account_0: get_req(10, "token_account_0")?,
            token_account_1: get_req(11, "token_account_1")?,
            token_vault_0: get_req(12, "token_vault_0")?,
            token_vault_1: get_req(13, "token_vault_1")?,
            rent: get_req(14, "rent")?,
            system_program: get_req(15, "system_program")?,
            token_program: get_req(16, "token_program")?,
            associated_token_program: get_req(17, "associated_token_program")?,
            metadata_program: get_req(18, "metadata_program")?,
            token_program_2022: get_req(19, "token_program_2022")?,
            vault_0_mint: get_req(20, "vault_0_mint")?,
            vault_1_mint: get_req(21, "vault_1_mint")?,
        })
    }
}

pub fn get_open_position_v2_accounts(ix: &InstructionView) -> Result<OpenPositionV2Accounts, AccountsError> {
    OpenPositionV2Accounts::try_from(ix)
}

/// Accounts for the `open_position_with_token22_nft` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OpenPositionWithToken22NftAccounts {
    /// Pays to mint the position
    pub payer: Pubkey,
    pub position_nft_owner: Pubkey,
    /// Unique token mint address, initialize in contract
    pub position_nft_mint: Pubkey,
    pub position_nft_account: Pubkey,
    /// Add liquidity for this pool
    pub pool_state: Pubkey,
    /// Store the information of market marking in range
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    /// personal position state
    pub personal_position: Pubkey,
    /// The token_0 account deposit token to the pool
    pub token_account_0: Pubkey,
    /// The token_1 account deposit token to the pool
    pub token_account_1: Pubkey,
    /// The address that holds pool tokens for token_0
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    pub token_vault_1: Pubkey,
    /// Sysvar for token mint and ATA creation
    pub rent: Pubkey,
    /// Program to create the position manager state account
    pub system_program: Pubkey,
    /// Program to transfer for token account
    pub token_program: Pubkey,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Pubkey,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program_2022: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for OpenPositionWithToken22NftAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(OpenPositionWithToken22NftAccounts {
            payer: get_req(0, "payer")?,
            position_nft_owner: get_req(1, "position_nft_owner")?,
            position_nft_mint: get_req(2, "position_nft_mint")?,
            position_nft_account: get_req(3, "position_nft_account")?,
            pool_state: get_req(4, "pool_state")?,
            protocol_position: get_req(5, "protocol_position")?,
            tick_array_lower: get_req(6, "tick_array_lower")?,
            tick_array_upper: get_req(7, "tick_array_upper")?,
            personal_position: get_req(8, "personal_position")?,
            token_account_0: get_req(9, "token_account_0")?,
            token_account_1: get_req(10, "token_account_1")?,
            token_vault_0: get_req(11, "token_vault_0")?,
            token_vault_1: get_req(12, "token_vault_1")?,
            rent: get_req(13, "rent")?,
            system_program: get_req(14, "system_program")?,
            token_program: get_req(15, "token_program")?,
            associated_token_program: get_req(16, "associated_token_program")?,
            token_program_2022: get_req(17, "token_program_2022")?,
            vault_0_mint: get_req(18, "vault_0_mint")?,
            vault_1_mint: get_req(19, "vault_1_mint")?,
        })
    }
}

pub fn get_open_position_with_token22_nft_accounts(ix: &InstructionView) -> Result<OpenPositionWithToken22NftAccounts, AccountsError> {
    OpenPositionWithToken22NftAccounts::try_from(ix)
}

/// Accounts for the `set_reward_params` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetRewardParamsAccounts {
    /// Address to be set as protocol owner. It pays to create factory state account.
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    /// load info from the account to judge reward permission
    pub operation_state: Pubkey,
    /// Token program
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SetRewardParamsAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(SetRewardParamsAccounts {
            authority: get_req(0, "authority")?,
            amm_config: get_req(1, "amm_config")?,
            pool_state: get_req(2, "pool_state")?,
            operation_state: get_req(3, "operation_state")?,
            token_program: get_req(4, "token_program")?,
            token_program_2022: get_req(5, "token_program_2022")?,
        })
    }
}

pub fn get_set_reward_params_accounts(ix: &InstructionView) -> Result<SetRewardParamsAccounts, AccountsError> {
    SetRewardParamsAccounts::try_from(ix)
}

/// Accounts for the `swap` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapAccounts {
    /// The user performing the swap
    pub payer: Pubkey,
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
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    pub tick_array: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(SwapAccounts {
            payer: get_req(0, "payer")?,
            amm_config: get_req(1, "amm_config")?,
            pool_state: get_req(2, "pool_state")?,
            input_token_account: get_req(3, "input_token_account")?,
            output_token_account: get_req(4, "output_token_account")?,
            input_vault: get_req(5, "input_vault")?,
            output_vault: get_req(6, "output_vault")?,
            observation_state: get_req(7, "observation_state")?,
            token_program: get_req(8, "token_program")?,
            tick_array: get_req(9, "tick_array")?,
        })
    }
}

pub fn get_swap_accounts(ix: &InstructionView) -> Result<SwapAccounts, AccountsError> {
    SwapAccounts::try_from(ix)
}

/// Accounts for the `swap_router_base_in` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapRouterBaseInAccounts {
    /// The user performing the swap
    pub payer: Pubkey,
    /// The token account that pays input tokens for the swap
    pub input_token_account: Pubkey,
    /// The mint of input token
    pub input_token_mint: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    /// SPL program 2022 for token transfers
    pub token_program_2022: Pubkey,
    /// Memo program
    pub memo_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapRouterBaseInAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(SwapRouterBaseInAccounts {
            payer: get_req(0, "payer")?,
            input_token_account: get_req(1, "input_token_account")?,
            input_token_mint: get_req(2, "input_token_mint")?,
            token_program: get_req(3, "token_program")?,
            token_program_2022: get_req(4, "token_program_2022")?,
            memo_program: get_req(5, "memo_program")?,
        })
    }
}

pub fn get_swap_router_base_in_accounts(ix: &InstructionView) -> Result<SwapRouterBaseInAccounts, AccountsError> {
    SwapRouterBaseInAccounts::try_from(ix)
}

/// Accounts for the `swap_v2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapV2Accounts {
    /// The user performing the swap
    pub payer: Pubkey,
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
    /// The program account for the most recent oracle observation
    pub observation_state: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    /// SPL program 2022 for token transfers
    pub token_program_2022: Pubkey,
    /// Memo program
    pub memo_program: Pubkey,
    /// The mint of token vault 0
    pub input_vault_mint: Pubkey,
    /// The mint of token vault 1
    pub output_vault_mint: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapV2Accounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(SwapV2Accounts {
            payer: get_req(0, "payer")?,
            amm_config: get_req(1, "amm_config")?,
            pool_state: get_req(2, "pool_state")?,
            input_token_account: get_req(3, "input_token_account")?,
            output_token_account: get_req(4, "output_token_account")?,
            input_vault: get_req(5, "input_vault")?,
            output_vault: get_req(6, "output_vault")?,
            observation_state: get_req(7, "observation_state")?,
            token_program: get_req(8, "token_program")?,
            token_program_2022: get_req(9, "token_program_2022")?,
            memo_program: get_req(10, "memo_program")?,
            input_vault_mint: get_req(11, "input_vault_mint")?,
            output_vault_mint: get_req(12, "output_vault_mint")?,
        })
    }
}

pub fn get_swap_v2_accounts(ix: &InstructionView) -> Result<SwapV2Accounts, AccountsError> {
    SwapV2Accounts::try_from(ix)
}

/// Accounts for the `transfer_reward_owner` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TransferRewardOwnerAccounts {
    /// Address to be set as operation account owner.
    pub authority: Pubkey,
    pub pool_state: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for TransferRewardOwnerAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(TransferRewardOwnerAccounts {
            authority: get_req(0, "authority")?,
            pool_state: get_req(1, "pool_state")?,
        })
    }
}

pub fn get_transfer_reward_owner_accounts(ix: &InstructionView) -> Result<TransferRewardOwnerAccounts, AccountsError> {
    TransferRewardOwnerAccounts::try_from(ix)
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
            crate::accounts::to_pubkey(name, index, a.0)
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

/// Accounts for the `update_operation_account` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateOperationAccountAccounts {
    /// Address to be set as operation account owner.
    pub owner: Pubkey,
    /// Initialize operation state account to store operation owner address and white list mint.
    pub operation_state: Pubkey,
    pub system_program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for UpdateOperationAccountAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(UpdateOperationAccountAccounts {
            owner: get_req(0, "owner")?,
            operation_state: get_req(1, "operation_state")?,
            system_program: get_req(2, "system_program")?,
        })
    }
}

pub fn get_update_operation_account_accounts(ix: &InstructionView) -> Result<UpdateOperationAccountAccounts, AccountsError> {
    UpdateOperationAccountAccounts::try_from(ix)
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
            crate::accounts::to_pubkey(name, index, a.0)
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

/// Accounts for the `update_reward_infos` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateRewardInfosAccounts {
    /// The liquidity pool for which reward info to update
    pub pool_state: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for UpdateRewardInfosAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(UpdateRewardInfosAccounts {
            pool_state: get_req(0, "pool_state")?,
        })
    }
}

pub fn get_update_reward_infos_accounts(ix: &InstructionView) -> Result<UpdateRewardInfosAccounts, AccountsError> {
    UpdateRewardInfosAccounts::try_from(ix)
}
