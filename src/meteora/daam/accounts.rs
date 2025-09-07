use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::accounts::AccountsError;

// -----------------------------------------------------------------------------
// AddLiquidity accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityAccounts {
    pub pool: Pubkey,
    pub position: Pubkey,
    /// The user token a account
    pub token_a_account: Pubkey,
    /// The user token b account
    pub token_b_account: Pubkey,
    /// The vault token account for input token
    pub token_a_vault: Pubkey,
    /// The vault token account for output token
    pub token_b_vault: Pubkey,
    /// The mint of token a
    pub token_a_mint: Pubkey,
    /// The mint of token b
    pub token_b_mint: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    /// owner of position
    pub owner: Pubkey,
    /// Token a program
    pub token_a_program: Pubkey,
    /// Token b program
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_POSITION: usize = 1;
        const IDX_TOKEN_A_ACCOUNT: usize = 2;
        const IDX_TOKEN_B_ACCOUNT: usize = 3;
        const IDX_TOKEN_A_VAULT: usize = 4;
        const IDX_TOKEN_B_VAULT: usize = 5;
        const IDX_TOKEN_A_MINT: usize = 6;
        const IDX_TOKEN_B_MINT: usize = 7;
        const IDX_POSITION_NFT_ACCOUNT: usize = 8;
        const IDX_OWNER: usize = 9;
        const IDX_TOKEN_A_PROGRAM: usize = 10;
        const IDX_TOKEN_B_PROGRAM: usize = 11;
        const IDX_EVENT_AUTHORITY: usize = 12;
        const IDX_PROGRAM: usize = 13;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            token_a_account: get_req(IDX_TOKEN_A_ACCOUNT, "token_a_account")?,
            token_b_account: get_req(IDX_TOKEN_B_ACCOUNT, "token_b_account")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_add_liquidity_accounts(ix: &InstructionView) -> Result<AddLiquidityAccounts, AccountsError> {
    AddLiquidityAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// ClaimPartnerFee accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClaimPartnerFeeAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    /// The treasury token a account
    pub token_a_account: Pubkey,
    /// The treasury token b account
    pub token_b_account: Pubkey,
    /// The vault token account for input token
    pub token_a_vault: Pubkey,
    /// The vault token account for output token
    pub token_b_vault: Pubkey,
    /// The mint of token a
    pub token_a_mint: Pubkey,
    /// The mint of token b
    pub token_b_mint: Pubkey,
    pub partner: Pubkey,
    /// Token a program
    pub token_a_program: Pubkey,
    /// Token b program
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClaimPartnerFeeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_TOKEN_A_ACCOUNT: usize = 2;
        const IDX_TOKEN_B_ACCOUNT: usize = 3;
        const IDX_TOKEN_A_VAULT: usize = 4;
        const IDX_TOKEN_B_VAULT: usize = 5;
        const IDX_TOKEN_A_MINT: usize = 6;
        const IDX_TOKEN_B_MINT: usize = 7;
        const IDX_PARTNER: usize = 8;
        const IDX_TOKEN_A_PROGRAM: usize = 9;
        const IDX_TOKEN_B_PROGRAM: usize = 10;
        const IDX_EVENT_AUTHORITY: usize = 11;
        const IDX_PROGRAM: usize = 12;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            token_a_account: get_req(IDX_TOKEN_A_ACCOUNT, "token_a_account")?,
            token_b_account: get_req(IDX_TOKEN_B_ACCOUNT, "token_b_account")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            partner: get_req(IDX_PARTNER, "partner")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_claim_partner_fee_accounts(ix: &InstructionView) -> Result<ClaimPartnerFeeAccounts, AccountsError> {
    ClaimPartnerFeeAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// ClaimPositionFee accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClaimPositionFeeAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    /// The user token a account
    pub token_a_account: Pubkey,
    /// The user token b account
    pub token_b_account: Pubkey,
    /// The vault token account for input token
    pub token_a_vault: Pubkey,
    /// The vault token account for output token
    pub token_b_vault: Pubkey,
    /// The mint of token a
    pub token_a_mint: Pubkey,
    /// The mint of token b
    pub token_b_mint: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    /// owner of position
    pub owner: Pubkey,
    /// Token a program
    pub token_a_program: Pubkey,
    /// Token b program
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClaimPositionFeeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_POSITION: usize = 2;
        const IDX_TOKEN_A_ACCOUNT: usize = 3;
        const IDX_TOKEN_B_ACCOUNT: usize = 4;
        const IDX_TOKEN_A_VAULT: usize = 5;
        const IDX_TOKEN_B_VAULT: usize = 6;
        const IDX_TOKEN_A_MINT: usize = 7;
        const IDX_TOKEN_B_MINT: usize = 8;
        const IDX_POSITION_NFT_ACCOUNT: usize = 9;
        const IDX_OWNER: usize = 10;
        const IDX_TOKEN_A_PROGRAM: usize = 11;
        const IDX_TOKEN_B_PROGRAM: usize = 12;
        const IDX_EVENT_AUTHORITY: usize = 13;
        const IDX_PROGRAM: usize = 14;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            token_a_account: get_req(IDX_TOKEN_A_ACCOUNT, "token_a_account")?,
            token_b_account: get_req(IDX_TOKEN_B_ACCOUNT, "token_b_account")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_claim_position_fee_accounts(ix: &InstructionView) -> Result<ClaimPositionFeeAccounts, AccountsError> {
    ClaimPositionFeeAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// ClaimProtocolFee accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClaimProtocolFeeAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    /// The vault token account for input token
    pub token_a_vault: Pubkey,
    /// The vault token account for output token
    pub token_b_vault: Pubkey,
    /// The mint of token a
    pub token_a_mint: Pubkey,
    /// The mint of token b
    pub token_b_mint: Pubkey,
    /// The treasury token a account
    pub token_a_account: Pubkey,
    /// The treasury token b account
    pub token_b_account: Pubkey,
    /// Claim fee operator
    pub claim_fee_operator: Pubkey,
    /// Operator
    pub operator: Pubkey,
    /// Token a program
    pub token_a_program: Pubkey,
    /// Token b program
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClaimProtocolFeeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_TOKEN_A_VAULT: usize = 2;
        const IDX_TOKEN_B_VAULT: usize = 3;
        const IDX_TOKEN_A_MINT: usize = 4;
        const IDX_TOKEN_B_MINT: usize = 5;
        const IDX_TOKEN_A_ACCOUNT: usize = 6;
        const IDX_TOKEN_B_ACCOUNT: usize = 7;
        const IDX_CLAIM_FEE_OPERATOR: usize = 8;
        const IDX_OPERATOR: usize = 9;
        const IDX_TOKEN_A_PROGRAM: usize = 10;
        const IDX_TOKEN_B_PROGRAM: usize = 11;
        const IDX_EVENT_AUTHORITY: usize = 12;
        const IDX_PROGRAM: usize = 13;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            token_a_account: get_req(IDX_TOKEN_A_ACCOUNT, "token_a_account")?,
            token_b_account: get_req(IDX_TOKEN_B_ACCOUNT, "token_b_account")?,
            claim_fee_operator: get_req(IDX_CLAIM_FEE_OPERATOR, "claim_fee_operator")?,
            operator: get_req(IDX_OPERATOR, "operator")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_claim_protocol_fee_accounts(ix: &InstructionView) -> Result<ClaimProtocolFeeAccounts, AccountsError> {
    ClaimProtocolFeeAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// ClaimReward accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClaimRewardAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    /// The vault token account for reward token
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub user_token_account: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    /// owner of position
    pub owner: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClaimRewardAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_POSITION: usize = 2;
        const IDX_REWARD_VAULT: usize = 3;
        const IDX_REWARD_MINT: usize = 4;
        const IDX_USER_TOKEN_ACCOUNT: usize = 5;
        const IDX_POSITION_NFT_ACCOUNT: usize = 6;
        const IDX_OWNER: usize = 7;
        const IDX_TOKEN_PROGRAM: usize = 8;
        const IDX_EVENT_AUTHORITY: usize = 9;
        const IDX_PROGRAM: usize = 10;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            reward_vault: get_req(IDX_REWARD_VAULT, "reward_vault")?,
            reward_mint: get_req(IDX_REWARD_MINT, "reward_mint")?,
            user_token_account: get_req(IDX_USER_TOKEN_ACCOUNT, "user_token_account")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_claim_reward_accounts(ix: &InstructionView) -> Result<ClaimRewardAccounts, AccountsError> {
    ClaimRewardAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CloseClaimFeeOperator accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CloseClaimFeeOperatorAccounts {
    pub claim_fee_operator: Pubkey,
    pub rent_receiver: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CloseClaimFeeOperatorAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CLAIM_FEE_OPERATOR: usize = 0;
        const IDX_RENT_RECEIVER: usize = 1;
        const IDX_ADMIN: usize = 2;
        const IDX_EVENT_AUTHORITY: usize = 3;
        const IDX_PROGRAM: usize = 4;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            claim_fee_operator: get_req(IDX_CLAIM_FEE_OPERATOR, "claim_fee_operator")?,
            rent_receiver: get_req(IDX_RENT_RECEIVER, "rent_receiver")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_close_claim_fee_operator_accounts(ix: &InstructionView) -> Result<CloseClaimFeeOperatorAccounts, AccountsError> {
    CloseClaimFeeOperatorAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CloseConfig accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CloseConfigAccounts {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CloseConfigAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CONFIG: usize = 0;
        const IDX_ADMIN: usize = 1;
        const IDX_RENT_RECEIVER: usize = 2;
        const IDX_EVENT_AUTHORITY: usize = 3;
        const IDX_PROGRAM: usize = 4;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            config: get_req(IDX_CONFIG, "config")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            rent_receiver: get_req(IDX_RENT_RECEIVER, "rent_receiver")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_close_config_accounts(ix: &InstructionView) -> Result<CloseConfigAccounts, AccountsError> {
    CloseConfigAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// ClosePosition accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClosePositionAccounts {
    /// position_nft_mint
    pub position_nft_mint: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub pool_authority: Pubkey,
    pub rent_receiver: Pubkey,
    /// Owner of position
    pub owner: Pubkey,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for ClosePositionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POSITION_NFT_MINT: usize = 0;
        const IDX_POSITION_NFT_ACCOUNT: usize = 1;
        const IDX_POOL: usize = 2;
        const IDX_POSITION: usize = 3;
        const IDX_POOL_AUTHORITY: usize = 4;
        const IDX_RENT_RECEIVER: usize = 5;
        const IDX_OWNER: usize = 6;
        const IDX_TOKEN_PROGRAM: usize = 7;
        const IDX_EVENT_AUTHORITY: usize = 8;
        const IDX_PROGRAM: usize = 9;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            position_nft_mint: get_req(IDX_POSITION_NFT_MINT, "position_nft_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            rent_receiver: get_req(IDX_RENT_RECEIVER, "rent_receiver")?,
            owner: get_req(IDX_OWNER, "owner")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_close_position_accounts(ix: &InstructionView) -> Result<ClosePositionAccounts, AccountsError> {
    ClosePositionAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CloseTokenBadge accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CloseTokenBadgeAccounts {
    pub token_badge: Pubkey,
    pub admin: Pubkey,
    pub rent_receiver: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CloseTokenBadgeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_TOKEN_BADGE: usize = 0;
        const IDX_ADMIN: usize = 1;
        const IDX_RENT_RECEIVER: usize = 2;
        const IDX_EVENT_AUTHORITY: usize = 3;
        const IDX_PROGRAM: usize = 4;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            token_badge: get_req(IDX_TOKEN_BADGE, "token_badge")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            rent_receiver: get_req(IDX_RENT_RECEIVER, "rent_receiver")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_close_token_badge_accounts(ix: &InstructionView) -> Result<CloseTokenBadgeAccounts, AccountsError> {
    CloseTokenBadgeAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CreateClaimFeeOperator accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateClaimFeeOperatorAccounts {
    pub claim_fee_operator: Pubkey,
    pub operator: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateClaimFeeOperatorAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CLAIM_FEE_OPERATOR: usize = 0;
        const IDX_OPERATOR: usize = 1;
        const IDX_ADMIN: usize = 2;
        const IDX_SYSTEM_PROGRAM: usize = 3;
        const IDX_EVENT_AUTHORITY: usize = 4;
        const IDX_PROGRAM: usize = 5;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            claim_fee_operator: get_req(IDX_CLAIM_FEE_OPERATOR, "claim_fee_operator")?,
            operator: get_req(IDX_OPERATOR, "operator")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_create_claim_fee_operator_accounts(ix: &InstructionView) -> Result<CreateClaimFeeOperatorAccounts, AccountsError> {
    CreateClaimFeeOperatorAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CreateConfig accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateConfigAccounts {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateConfigAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CONFIG: usize = 0;
        const IDX_ADMIN: usize = 1;
        const IDX_SYSTEM_PROGRAM: usize = 2;
        const IDX_EVENT_AUTHORITY: usize = 3;
        const IDX_PROGRAM: usize = 4;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            config: get_req(IDX_CONFIG, "config")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_create_config_accounts(ix: &InstructionView) -> Result<CreateConfigAccounts, AccountsError> {
    CreateConfigAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CreateDynamicConfig accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateDynamicConfigAccounts {
    pub config: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateDynamicConfigAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CONFIG: usize = 0;
        const IDX_ADMIN: usize = 1;
        const IDX_SYSTEM_PROGRAM: usize = 2;
        const IDX_EVENT_AUTHORITY: usize = 3;
        const IDX_PROGRAM: usize = 4;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            config: get_req(IDX_CONFIG, "config")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_create_dynamic_config_accounts(ix: &InstructionView) -> Result<CreateDynamicConfigAccounts, AccountsError> {
    CreateDynamicConfigAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CreatePosition accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreatePositionAccounts {
    pub owner: Pubkey,
    /// position_nft_mint
    pub position_nft_mint: Pubkey,
    /// position nft account
    pub position_nft_account: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub pool_authority: Pubkey,
    /// Address paying to create the position. Can be anyone
    pub payer: Pubkey,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreatePositionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_OWNER: usize = 0;
        const IDX_POSITION_NFT_MINT: usize = 1;
        const IDX_POSITION_NFT_ACCOUNT: usize = 2;
        const IDX_POOL: usize = 3;
        const IDX_POSITION: usize = 4;
        const IDX_POOL_AUTHORITY: usize = 5;
        const IDX_PAYER: usize = 6;
        const IDX_TOKEN_PROGRAM: usize = 7;
        const IDX_SYSTEM_PROGRAM: usize = 8;
        const IDX_EVENT_AUTHORITY: usize = 9;
        const IDX_PROGRAM: usize = 10;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            owner: get_req(IDX_OWNER, "owner")?,
            position_nft_mint: get_req(IDX_POSITION_NFT_MINT, "position_nft_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            payer: get_req(IDX_PAYER, "payer")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_create_position_accounts(ix: &InstructionView) -> Result<CreatePositionAccounts, AccountsError> {
    CreatePositionAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// CreateTokenBadge accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateTokenBadgeAccounts {
    pub token_badge: Pubkey,
    pub token_mint: Pubkey,
    pub admin: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for CreateTokenBadgeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_TOKEN_BADGE: usize = 0;
        const IDX_TOKEN_MINT: usize = 1;
        const IDX_ADMIN: usize = 2;
        const IDX_SYSTEM_PROGRAM: usize = 3;
        const IDX_EVENT_AUTHORITY: usize = 4;
        const IDX_PROGRAM: usize = 5;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            token_badge: get_req(IDX_TOKEN_BADGE, "token_badge")?,
            token_mint: get_req(IDX_TOKEN_MINT, "token_mint")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_create_token_badge_accounts(ix: &InstructionView) -> Result<CreateTokenBadgeAccounts, AccountsError> {
    CreateTokenBadgeAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// FundReward accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct FundRewardAccounts {
    pub pool: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for FundRewardAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_REWARD_VAULT: usize = 1;
        const IDX_REWARD_MINT: usize = 2;
        const IDX_FUNDER_TOKEN_ACCOUNT: usize = 3;
        const IDX_FUNDER: usize = 4;
        const IDX_TOKEN_PROGRAM: usize = 5;
        const IDX_EVENT_AUTHORITY: usize = 6;
        const IDX_PROGRAM: usize = 7;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            reward_vault: get_req(IDX_REWARD_VAULT, "reward_vault")?,
            reward_mint: get_req(IDX_REWARD_MINT, "reward_mint")?,
            funder_token_account: get_req(IDX_FUNDER_TOKEN_ACCOUNT, "funder_token_account")?,
            funder: get_req(IDX_FUNDER, "funder")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_fund_reward_accounts(ix: &InstructionView) -> Result<FundRewardAccounts, AccountsError> {
    FundRewardAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// InitializeCustomizablePool accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeCustomizablePoolAccounts {
    pub creator: Pubkey,
    /// position_nft_mint
    pub position_nft_mint: Pubkey,
    /// position nft account
    pub position_nft_account: Pubkey,
    /// Address paying to create the pool. Can be anyone
    pub payer: Pubkey,
    pub pool_authority: Pubkey,
    /// Initialize an account to store the pool state
    pub pool: Pubkey,
    pub position: Pubkey,
    /// Token a mint
    pub token_a_mint: Pubkey,
    /// Token b mint
    pub token_b_mint: Pubkey,
    /// Token a vault for the pool
    pub token_a_vault: Pubkey,
    /// Token b vault for the pool
    pub token_b_vault: Pubkey,
    /// payer token a account
    pub payer_token_a: Pubkey,
    /// creator token b account
    pub payer_token_b: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_a_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_b_program: Pubkey,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeCustomizablePoolAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CREATOR: usize = 0;
        const IDX_POSITION_NFT_MINT: usize = 1;
        const IDX_POSITION_NFT_ACCOUNT: usize = 2;
        const IDX_PAYER: usize = 3;
        const IDX_POOL_AUTHORITY: usize = 4;
        const IDX_POOL: usize = 5;
        const IDX_POSITION: usize = 6;
        const IDX_TOKEN_A_MINT: usize = 7;
        const IDX_TOKEN_B_MINT: usize = 8;
        const IDX_TOKEN_A_VAULT: usize = 9;
        const IDX_TOKEN_B_VAULT: usize = 10;
        const IDX_PAYER_TOKEN_A: usize = 11;
        const IDX_PAYER_TOKEN_B: usize = 12;
        const IDX_TOKEN_A_PROGRAM: usize = 13;
        const IDX_TOKEN_B_PROGRAM: usize = 14;
        const IDX_TOKEN_2022_PROGRAM: usize = 15;
        const IDX_SYSTEM_PROGRAM: usize = 16;
        const IDX_EVENT_AUTHORITY: usize = 17;
        const IDX_PROGRAM: usize = 18;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            creator: get_req(IDX_CREATOR, "creator")?,
            position_nft_mint: get_req(IDX_POSITION_NFT_MINT, "position_nft_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            payer: get_req(IDX_PAYER, "payer")?,
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            payer_token_a: get_req(IDX_PAYER_TOKEN_A, "payer_token_a")?,
            payer_token_b: get_req(IDX_PAYER_TOKEN_B, "payer_token_b")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            token_2022_program: get_req(IDX_TOKEN_2022_PROGRAM, "token_2022_program")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_initialize_customizable_pool_accounts(ix: &InstructionView) -> Result<InitializeCustomizablePoolAccounts, AccountsError> {
    InitializeCustomizablePoolAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// InitializePool accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePoolAccounts {
    pub creator: Pubkey,
    /// position_nft_mint
    pub position_nft_mint: Pubkey,
    /// position nft account
    pub position_nft_account: Pubkey,
    /// Address paying to create the pool. Can be anyone
    pub payer: Pubkey,
    /// Which config the pool belongs to.
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    /// Initialize an account to store the pool state
    pub pool: Pubkey,
    pub position: Pubkey,
    /// Token a mint
    pub token_a_mint: Pubkey,
    /// Token b mint
    pub token_b_mint: Pubkey,
    /// Token a vault for the pool
    pub token_a_vault: Pubkey,
    /// Token b vault for the pool
    pub token_b_vault: Pubkey,
    /// payer token a account
    pub payer_token_a: Pubkey,
    /// creator token b account
    pub payer_token_b: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_a_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_b_program: Pubkey,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePoolAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CREATOR: usize = 0;
        const IDX_POSITION_NFT_MINT: usize = 1;
        const IDX_POSITION_NFT_ACCOUNT: usize = 2;
        const IDX_PAYER: usize = 3;
        const IDX_CONFIG: usize = 4;
        const IDX_POOL_AUTHORITY: usize = 5;
        const IDX_POOL: usize = 6;
        const IDX_POSITION: usize = 7;
        const IDX_TOKEN_A_MINT: usize = 8;
        const IDX_TOKEN_B_MINT: usize = 9;
        const IDX_TOKEN_A_VAULT: usize = 10;
        const IDX_TOKEN_B_VAULT: usize = 11;
        const IDX_PAYER_TOKEN_A: usize = 12;
        const IDX_PAYER_TOKEN_B: usize = 13;
        const IDX_TOKEN_A_PROGRAM: usize = 14;
        const IDX_TOKEN_B_PROGRAM: usize = 15;
        const IDX_TOKEN_2022_PROGRAM: usize = 16;
        const IDX_SYSTEM_PROGRAM: usize = 17;
        const IDX_EVENT_AUTHORITY: usize = 18;
        const IDX_PROGRAM: usize = 19;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            creator: get_req(IDX_CREATOR, "creator")?,
            position_nft_mint: get_req(IDX_POSITION_NFT_MINT, "position_nft_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            payer: get_req(IDX_PAYER, "payer")?,
            config: get_req(IDX_CONFIG, "config")?,
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            payer_token_a: get_req(IDX_PAYER_TOKEN_A, "payer_token_a")?,
            payer_token_b: get_req(IDX_PAYER_TOKEN_B, "payer_token_b")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            token_2022_program: get_req(IDX_TOKEN_2022_PROGRAM, "token_2022_program")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_initialize_pool_accounts(ix: &InstructionView) -> Result<InitializePoolAccounts, AccountsError> {
    InitializePoolAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// InitializePoolWithDynamicConfig accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePoolWithDynamicConfigAccounts {
    pub creator: Pubkey,
    /// position_nft_mint
    pub position_nft_mint: Pubkey,
    /// position nft account
    pub position_nft_account: Pubkey,
    /// Address paying to create the pool. Can be anyone
    pub payer: Pubkey,
    pub pool_creator_authority: Pubkey,
    /// Which config the pool belongs to.
    pub config: Pubkey,
    pub pool_authority: Pubkey,
    /// Initialize an account to store the pool state
    pub pool: Pubkey,
    pub position: Pubkey,
    /// Token a mint
    pub token_a_mint: Pubkey,
    /// Token b mint
    pub token_b_mint: Pubkey,
    /// Token a vault for the pool
    pub token_a_vault: Pubkey,
    /// Token b vault for the pool
    pub token_b_vault: Pubkey,
    /// payer token a account
    pub payer_token_a: Pubkey,
    /// creator token b account
    pub payer_token_b: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_a_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_b_program: Pubkey,
    /// Program to create NFT mint/token account and transfer for token22 account
    pub token_2022_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePoolWithDynamicConfigAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_CREATOR: usize = 0;
        const IDX_POSITION_NFT_MINT: usize = 1;
        const IDX_POSITION_NFT_ACCOUNT: usize = 2;
        const IDX_PAYER: usize = 3;
        const IDX_POOL_CREATOR_AUTHORITY: usize = 4;
        const IDX_CONFIG: usize = 5;
        const IDX_POOL_AUTHORITY: usize = 6;
        const IDX_POOL: usize = 7;
        const IDX_POSITION: usize = 8;
        const IDX_TOKEN_A_MINT: usize = 9;
        const IDX_TOKEN_B_MINT: usize = 10;
        const IDX_TOKEN_A_VAULT: usize = 11;
        const IDX_TOKEN_B_VAULT: usize = 12;
        const IDX_PAYER_TOKEN_A: usize = 13;
        const IDX_PAYER_TOKEN_B: usize = 14;
        const IDX_TOKEN_A_PROGRAM: usize = 15;
        const IDX_TOKEN_B_PROGRAM: usize = 16;
        const IDX_TOKEN_2022_PROGRAM: usize = 17;
        const IDX_SYSTEM_PROGRAM: usize = 18;
        const IDX_EVENT_AUTHORITY: usize = 19;
        const IDX_PROGRAM: usize = 20;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            creator: get_req(IDX_CREATOR, "creator")?,
            position_nft_mint: get_req(IDX_POSITION_NFT_MINT, "position_nft_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            payer: get_req(IDX_PAYER, "payer")?,
            pool_creator_authority: get_req(IDX_POOL_CREATOR_AUTHORITY, "pool_creator_authority")?,
            config: get_req(IDX_CONFIG, "config")?,
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            payer_token_a: get_req(IDX_PAYER_TOKEN_A, "payer_token_a")?,
            payer_token_b: get_req(IDX_PAYER_TOKEN_B, "payer_token_b")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            token_2022_program: get_req(IDX_TOKEN_2022_PROGRAM, "token_2022_program")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_initialize_pool_with_dynamic_config_accounts(ix: &InstructionView) -> Result<InitializePoolWithDynamicConfigAccounts, AccountsError> {
    InitializePoolWithDynamicConfigAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// InitializeReward accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeRewardAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub signer: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeRewardAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_REWARD_VAULT: usize = 2;
        const IDX_REWARD_MINT: usize = 3;
        const IDX_SIGNER: usize = 4;
        const IDX_PAYER: usize = 5;
        const IDX_TOKEN_PROGRAM: usize = 6;
        const IDX_SYSTEM_PROGRAM: usize = 7;
        const IDX_EVENT_AUTHORITY: usize = 8;
        const IDX_PROGRAM: usize = 9;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            reward_vault: get_req(IDX_REWARD_VAULT, "reward_vault")?,
            reward_mint: get_req(IDX_REWARD_MINT, "reward_mint")?,
            signer: get_req(IDX_SIGNER, "signer")?,
            payer: get_req(IDX_PAYER, "payer")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_initialize_reward_accounts(ix: &InstructionView) -> Result<InitializeRewardAccounts, AccountsError> {
    InitializeRewardAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// LockPosition accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct LockPositionAccounts {
    pub pool: Pubkey,
    pub position: Pubkey,
    pub vesting: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    /// owner of position
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for LockPositionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_POSITION: usize = 1;
        const IDX_VESTING: usize = 2;
        const IDX_POSITION_NFT_ACCOUNT: usize = 3;
        const IDX_OWNER: usize = 4;
        const IDX_PAYER: usize = 5;
        const IDX_SYSTEM_PROGRAM: usize = 6;
        const IDX_EVENT_AUTHORITY: usize = 7;
        const IDX_PROGRAM: usize = 8;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            vesting: get_req(IDX_VESTING, "vesting")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
            payer: get_req(IDX_PAYER, "payer")?,
            system_program: get_req(IDX_SYSTEM_PROGRAM, "system_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_lock_position_accounts(ix: &InstructionView) -> Result<LockPositionAccounts, AccountsError> {
    LockPositionAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// PermanentLockPosition accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PermanentLockPositionAccounts {
    pub pool: Pubkey,
    pub position: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    /// owner of position
    pub owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for PermanentLockPositionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_POSITION: usize = 1;
        const IDX_POSITION_NFT_ACCOUNT: usize = 2;
        const IDX_OWNER: usize = 3;
        const IDX_EVENT_AUTHORITY: usize = 4;
        const IDX_PROGRAM: usize = 5;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_permanent_lock_position_accounts(ix: &InstructionView) -> Result<PermanentLockPositionAccounts, AccountsError> {
    PermanentLockPositionAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// RefreshVesting accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RefreshVestingAccounts {
    pub pool: Pubkey,
    pub position: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    pub owner: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RefreshVestingAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_POSITION: usize = 1;
        const IDX_POSITION_NFT_ACCOUNT: usize = 2;
        const IDX_OWNER: usize = 3;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
        })
    }
}

pub fn get_refresh_vesting_accounts(ix: &InstructionView) -> Result<RefreshVestingAccounts, AccountsError> {
    RefreshVestingAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// RemoveAllLiquidity accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveAllLiquidityAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    /// The user token a account
    pub token_a_account: Pubkey,
    /// The user token b account
    pub token_b_account: Pubkey,
    /// The vault token account for input token
    pub token_a_vault: Pubkey,
    /// The vault token account for output token
    pub token_b_vault: Pubkey,
    /// The mint of token a
    pub token_a_mint: Pubkey,
    /// The mint of token b
    pub token_b_mint: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    /// owner of position
    pub owner: Pubkey,
    /// Token a program
    pub token_a_program: Pubkey,
    /// Token b program
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveAllLiquidityAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_POSITION: usize = 2;
        const IDX_TOKEN_A_ACCOUNT: usize = 3;
        const IDX_TOKEN_B_ACCOUNT: usize = 4;
        const IDX_TOKEN_A_VAULT: usize = 5;
        const IDX_TOKEN_B_VAULT: usize = 6;
        const IDX_TOKEN_A_MINT: usize = 7;
        const IDX_TOKEN_B_MINT: usize = 8;
        const IDX_POSITION_NFT_ACCOUNT: usize = 9;
        const IDX_OWNER: usize = 10;
        const IDX_TOKEN_A_PROGRAM: usize = 11;
        const IDX_TOKEN_B_PROGRAM: usize = 12;
        const IDX_EVENT_AUTHORITY: usize = 13;
        const IDX_PROGRAM: usize = 14;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            token_a_account: get_req(IDX_TOKEN_A_ACCOUNT, "token_a_account")?,
            token_b_account: get_req(IDX_TOKEN_B_ACCOUNT, "token_b_account")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_remove_all_liquidity_accounts(ix: &InstructionView) -> Result<RemoveAllLiquidityAccounts, AccountsError> {
    RemoveAllLiquidityAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// RemoveLiquidity accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveLiquidityAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    /// The user token a account
    pub token_a_account: Pubkey,
    /// The user token b account
    pub token_b_account: Pubkey,
    /// The vault token account for input token
    pub token_a_vault: Pubkey,
    /// The vault token account for output token
    pub token_b_vault: Pubkey,
    /// The mint of token a
    pub token_a_mint: Pubkey,
    /// The mint of token b
    pub token_b_mint: Pubkey,
    /// The token account for nft
    pub position_nft_account: Pubkey,
    /// owner of position
    pub owner: Pubkey,
    /// Token a program
    pub token_a_program: Pubkey,
    /// Token b program
    pub token_b_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveLiquidityAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_POSITION: usize = 2;
        const IDX_TOKEN_A_ACCOUNT: usize = 3;
        const IDX_TOKEN_B_ACCOUNT: usize = 4;
        const IDX_TOKEN_A_VAULT: usize = 5;
        const IDX_TOKEN_B_VAULT: usize = 6;
        const IDX_TOKEN_A_MINT: usize = 7;
        const IDX_TOKEN_B_MINT: usize = 8;
        const IDX_POSITION_NFT_ACCOUNT: usize = 9;
        const IDX_OWNER: usize = 10;
        const IDX_TOKEN_A_PROGRAM: usize = 11;
        const IDX_TOKEN_B_PROGRAM: usize = 12;
        const IDX_EVENT_AUTHORITY: usize = 13;
        const IDX_PROGRAM: usize = 14;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            position: get_req(IDX_POSITION, "position")?,
            token_a_account: get_req(IDX_TOKEN_A_ACCOUNT, "token_a_account")?,
            token_b_account: get_req(IDX_TOKEN_B_ACCOUNT, "token_b_account")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            position_nft_account: get_req(IDX_POSITION_NFT_ACCOUNT, "position_nft_account")?,
            owner: get_req(IDX_OWNER, "owner")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_remove_liquidity_accounts(ix: &InstructionView) -> Result<RemoveLiquidityAccounts, AccountsError> {
    RemoveLiquidityAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// SetPoolStatus accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetPoolStatusAccounts {
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SetPoolStatusAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_ADMIN: usize = 1;
        const IDX_EVENT_AUTHORITY: usize = 2;
        const IDX_PROGRAM: usize = 3;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            admin: get_req(IDX_ADMIN, "admin")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_set_pool_status_accounts(ix: &InstructionView) -> Result<SetPoolStatusAccounts, AccountsError> {
    SetPoolStatusAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// SplitPosition accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SplitPositionAccounts {
    pub pool: Pubkey,
    /// The first position
    pub first_position: Pubkey,
    /// The token account for position nft
    pub first_position_nft_account: Pubkey,
    /// The second position
    pub second_position: Pubkey,
    /// The token account for position nft
    pub second_position_nft_account: Pubkey,
    /// Owner of first position
    pub first_owner: Pubkey,
    /// Owner of second position
    pub second_owner: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SplitPositionAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_FIRST_POSITION: usize = 1;
        const IDX_FIRST_POSITION_NFT_ACCOUNT: usize = 2;
        const IDX_SECOND_POSITION: usize = 3;
        const IDX_SECOND_POSITION_NFT_ACCOUNT: usize = 4;
        const IDX_FIRST_OWNER: usize = 5;
        const IDX_SECOND_OWNER: usize = 6;
        const IDX_EVENT_AUTHORITY: usize = 7;
        const IDX_PROGRAM: usize = 8;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            first_position: get_req(IDX_FIRST_POSITION, "first_position")?,
            first_position_nft_account: get_req(IDX_FIRST_POSITION_NFT_ACCOUNT, "first_position_nft_account")?,
            second_position: get_req(IDX_SECOND_POSITION, "second_position")?,
            second_position_nft_account: get_req(IDX_SECOND_POSITION_NFT_ACCOUNT, "second_position_nft_account")?,
            first_owner: get_req(IDX_FIRST_OWNER, "first_owner")?,
            second_owner: get_req(IDX_SECOND_OWNER, "second_owner")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_split_position_accounts(ix: &InstructionView) -> Result<SplitPositionAccounts, AccountsError> {
    SplitPositionAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// Swap accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapAccounts {
    pub pool_authority: Pubkey,
    /// Pool account
    pub pool: Pubkey,
    /// The user token account for input token
    pub input_token_account: Pubkey,
    /// The user token account for output token
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    pub token_a_vault: Pubkey,
    /// The vault token account for output token
    pub token_b_vault: Pubkey,
    /// The mint of token a
    pub token_a_mint: Pubkey,
    /// The mint of token b
    pub token_b_mint: Pubkey,
    /// The user performing the swap
    pub payer: Pubkey,
    /// Token a program
    pub token_a_program: Pubkey,
    /// Token b program
    pub token_b_program: Pubkey,
    /// referral token account
    pub referral_token_account: Option<Pubkey>,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_INPUT_TOKEN_ACCOUNT: usize = 2;
        const IDX_OUTPUT_TOKEN_ACCOUNT: usize = 3;
        const IDX_TOKEN_A_VAULT: usize = 4;
        const IDX_TOKEN_B_VAULT: usize = 5;
        const IDX_TOKEN_A_MINT: usize = 6;
        const IDX_TOKEN_B_MINT: usize = 7;
        const IDX_PAYER: usize = 8;
        const IDX_TOKEN_A_PROGRAM: usize = 9;
        const IDX_TOKEN_B_PROGRAM: usize = 10;
        const IDX_REFERRAL_TOKEN_ACCOUNT: usize = 11;
        const IDX_EVENT_AUTHORITY: usize = 12;
        const IDX_PROGRAM: usize = 13;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        let get_opt = |index: usize| -> Option<Pubkey> { accounts.get(index).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            input_token_account: get_req(IDX_INPUT_TOKEN_ACCOUNT, "input_token_account")?,
            output_token_account: get_req(IDX_OUTPUT_TOKEN_ACCOUNT, "output_token_account")?,
            token_a_vault: get_req(IDX_TOKEN_A_VAULT, "token_a_vault")?,
            token_b_vault: get_req(IDX_TOKEN_B_VAULT, "token_b_vault")?,
            token_a_mint: get_req(IDX_TOKEN_A_MINT, "token_a_mint")?,
            token_b_mint: get_req(IDX_TOKEN_B_MINT, "token_b_mint")?,
            payer: get_req(IDX_PAYER, "payer")?,
            token_a_program: get_req(IDX_TOKEN_A_PROGRAM, "token_a_program")?,
            token_b_program: get_req(IDX_TOKEN_B_PROGRAM, "token_b_program")?,
            referral_token_account: get_opt(IDX_REFERRAL_TOKEN_ACCOUNT),
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_swap_accounts(ix: &InstructionView) -> Result<SwapAccounts, AccountsError> {
    SwapAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// UpdateRewardDuration accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateRewardDurationAccounts {
    pub pool: Pubkey,
    pub signer: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for UpdateRewardDurationAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_SIGNER: usize = 1;
        const IDX_EVENT_AUTHORITY: usize = 2;
        const IDX_PROGRAM: usize = 3;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            signer: get_req(IDX_SIGNER, "signer")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_update_reward_duration_accounts(ix: &InstructionView) -> Result<UpdateRewardDurationAccounts, AccountsError> {
    UpdateRewardDurationAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// UpdateRewardFunder accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateRewardFunderAccounts {
    pub pool: Pubkey,
    pub signer: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for UpdateRewardFunderAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL: usize = 0;
        const IDX_SIGNER: usize = 1;
        const IDX_EVENT_AUTHORITY: usize = 2;
        const IDX_PROGRAM: usize = 3;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool: get_req(IDX_POOL, "pool")?,
            signer: get_req(IDX_SIGNER, "signer")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_update_reward_funder_accounts(ix: &InstructionView) -> Result<UpdateRewardFunderAccounts, AccountsError> {
    UpdateRewardFunderAccounts::try_from(ix)
}

// -----------------------------------------------------------------------------
// WithdrawIneligibleReward accounts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawIneligibleRewardAccounts {
    pub pool_authority: Pubkey,
    pub pool: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub funder_token_account: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for WithdrawIneligibleRewardAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        const IDX_POOL_AUTHORITY: usize = 0;
        const IDX_POOL: usize = 1;
        const IDX_REWARD_VAULT: usize = 2;
        const IDX_REWARD_MINT: usize = 3;
        const IDX_FUNDER_TOKEN_ACCOUNT: usize = 4;
        const IDX_FUNDER: usize = 5;
        const IDX_TOKEN_PROGRAM: usize = 6;
        const IDX_EVENT_AUTHORITY: usize = 7;
        const IDX_PROGRAM: usize = 8;
        let accounts = ix.accounts();
        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::accounts::to_pubkey(name, index, a.0)
        };
        Ok(Self {
            pool_authority: get_req(IDX_POOL_AUTHORITY, "pool_authority")?,
            pool: get_req(IDX_POOL, "pool")?,
            reward_vault: get_req(IDX_REWARD_VAULT, "reward_vault")?,
            reward_mint: get_req(IDX_REWARD_MINT, "reward_mint")?,
            funder_token_account: get_req(IDX_FUNDER_TOKEN_ACCOUNT, "funder_token_account")?,
            funder: get_req(IDX_FUNDER, "funder")?,
            token_program: get_req(IDX_TOKEN_PROGRAM, "token_program")?,
            event_authority: get_req(IDX_EVENT_AUTHORITY, "event_authority")?,
            program: get_req(IDX_PROGRAM, "program")?,
        })
    }
}

pub fn get_withdraw_ineligible_reward_accounts(ix: &InstructionView) -> Result<WithdrawIneligibleRewardAccounts, AccountsError> {
    WithdrawIneligibleRewardAccounts::try_from(ix)
}
