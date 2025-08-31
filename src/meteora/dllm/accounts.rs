use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::accounts::AccountsError;

/// Accounts for the `add_liquidity` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            bin_array_lower: get_req(9, "bin_array_lower")?,
            bin_array_upper: get_req(10, "bin_array_upper")?,
            sender: get_req(11, "sender")?,
            token_x_program: get_req(12, "token_x_program")?,
            token_y_program: get_req(13, "token_y_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_add_liquidity_accounts(ix: &InstructionView) -> Result<AddLiquidityAccounts, AccountsError> {
    AddLiquidityAccounts::try_from(ix)
}

/// Accounts for the `add_liquidity2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidity2Accounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidity2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidity2Accounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            sender: get_req(9, "sender")?,
            token_x_program: get_req(10, "token_x_program")?,
            token_y_program: get_req(11, "token_y_program")?,
            event_authority: get_req(12, "event_authority")?,
            program: get_req(13, "program")?,
        })
    }
}

pub fn get_add_liquidity2_accounts(ix: &InstructionView) -> Result<AddLiquidity2Accounts, AccountsError> {
    AddLiquidity2Accounts::try_from(ix)
}

/// Accounts for the `add_liquidity_by_strategy` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityByStrategyAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityByStrategyAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityByStrategyAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            bin_array_lower: get_req(9, "bin_array_lower")?,
            bin_array_upper: get_req(10, "bin_array_upper")?,
            sender: get_req(11, "sender")?,
            token_x_program: get_req(12, "token_x_program")?,
            token_y_program: get_req(13, "token_y_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_add_liquidity_by_strategy_accounts(ix: &InstructionView) -> Result<AddLiquidityByStrategyAccounts, AccountsError> {
    AddLiquidityByStrategyAccounts::try_from(ix)
}

/// Accounts for the `add_liquidity_by_strategy2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityByStrategy2Accounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityByStrategy2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityByStrategy2Accounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            sender: get_req(9, "sender")?,
            token_x_program: get_req(10, "token_x_program")?,
            token_y_program: get_req(11, "token_y_program")?,
            event_authority: get_req(12, "event_authority")?,
            program: get_req(13, "program")?,
        })
    }
}

pub fn get_add_liquidity_by_strategy2_accounts(ix: &InstructionView) -> Result<AddLiquidityByStrategy2Accounts, AccountsError> {
    AddLiquidityByStrategy2Accounts::try_from(ix)
}

/// Accounts for the `add_liquidity_by_strategy_one_side` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityByStrategyOneSideAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityByStrategyOneSideAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityByStrategyOneSideAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token: get_req(3, "user_token")?,
            reserve: get_req(4, "reserve")?,
            token_mint: get_req(5, "token_mint")?,
            bin_array_lower: get_req(6, "bin_array_lower")?,
            bin_array_upper: get_req(7, "bin_array_upper")?,
            sender: get_req(8, "sender")?,
            token_program: get_req(9, "token_program")?,
            event_authority: get_req(10, "event_authority")?,
            program: get_req(11, "program")?,
        })
    }
}

pub fn get_add_liquidity_by_strategy_one_side_accounts(ix: &InstructionView) -> Result<AddLiquidityByStrategyOneSideAccounts, AccountsError> {
    AddLiquidityByStrategyOneSideAccounts::try_from(ix)
}

/// Accounts for the `add_liquidity_by_weight` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityByWeightAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityByWeightAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityByWeightAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            bin_array_lower: get_req(9, "bin_array_lower")?,
            bin_array_upper: get_req(10, "bin_array_upper")?,
            sender: get_req(11, "sender")?,
            token_x_program: get_req(12, "token_x_program")?,
            token_y_program: get_req(13, "token_y_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_add_liquidity_by_weight_accounts(ix: &InstructionView) -> Result<AddLiquidityByWeightAccounts, AccountsError> {
    AddLiquidityByWeightAccounts::try_from(ix)
}

/// Accounts for the `add_liquidity_one_side` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityOneSideAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityOneSideAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityOneSideAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token: get_req(3, "user_token")?,
            reserve: get_req(4, "reserve")?,
            token_mint: get_req(5, "token_mint")?,
            bin_array_lower: get_req(6, "bin_array_lower")?,
            bin_array_upper: get_req(7, "bin_array_upper")?,
            sender: get_req(8, "sender")?,
            token_program: get_req(9, "token_program")?,
            event_authority: get_req(10, "event_authority")?,
            program: get_req(11, "program")?,
        })
    }
}

pub fn get_add_liquidity_one_side_accounts(ix: &InstructionView) -> Result<AddLiquidityOneSideAccounts, AccountsError> {
    AddLiquidityOneSideAccounts::try_from(ix)
}

/// Accounts for the `add_liquidity_one_side_precise` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityOneSidePreciseAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityOneSidePreciseAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityOneSidePreciseAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token: get_req(3, "user_token")?,
            reserve: get_req(4, "reserve")?,
            token_mint: get_req(5, "token_mint")?,
            bin_array_lower: get_req(6, "bin_array_lower")?,
            bin_array_upper: get_req(7, "bin_array_upper")?,
            sender: get_req(8, "sender")?,
            token_program: get_req(9, "token_program")?,
            event_authority: get_req(10, "event_authority")?,
            program: get_req(11, "program")?,
        })
    }
}

pub fn get_add_liquidity_one_side_precise_accounts(ix: &InstructionView) -> Result<AddLiquidityOneSidePreciseAccounts, AccountsError> {
    AddLiquidityOneSidePreciseAccounts::try_from(ix)
}

/// Accounts for the `add_liquidity_one_side_precise2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidityOneSidePrecise2Accounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token: Pubkey,
    pub reserve: Pubkey,
    pub token_mint: Pubkey,
    pub sender: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for AddLiquidityOneSidePrecise2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(AddLiquidityOneSidePrecise2Accounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token: get_req(3, "user_token")?,
            reserve: get_req(4, "reserve")?,
            token_mint: get_req(5, "token_mint")?,
            sender: get_req(6, "sender")?,
            token_program: get_req(7, "token_program")?,
            event_authority: get_req(8, "event_authority")?,
            program: get_req(9, "program")?,
        })
    }
}

pub fn get_add_liquidity_one_side_precise2_accounts(ix: &InstructionView) -> Result<AddLiquidityOneSidePrecise2Accounts, AccountsError> {
    AddLiquidityOneSidePrecise2Accounts::try_from(ix)
}

accounts!(
    ClaimFeeAccounts,
    get_claim_fee_accounts,
    {
        lb_pair,
        position,
        bin_array_lower,
        bin_array_upper,
        sender,
        reserve_x,
        reserve_y,
        user_token_x,
        user_token_y,
        token_x_mint,
        token_y_mint,
        token_program,
        event_authority,
        program,
    }
);

accounts!(
    ClaimFee2Accounts,
    get_claim_fee2_accounts,
    {
        lb_pair,
        position,
        sender,
        reserve_x,
        reserve_y,
        user_token_x,
        user_token_y,
        token_x_mint,
        token_y_mint,
        token_program_x,
        token_program_y,
        memo_program,
        event_authority,
        program,
    }
);

accounts!(
    ClaimRewardAccounts,
    get_claim_reward_accounts,
    {
        lb_pair,
        position,
        bin_array_lower,
        bin_array_upper,
        sender,
        reward_vault,
        reward_mint,
        user_token_account,
        token_program,
        event_authority,
        program,
    }
);

accounts!(
    ClaimReward2Accounts,
    get_claim_reward2_accounts,
    {
        lb_pair,
        position,
        sender,
        reward_vault,
        reward_mint,
        user_token_account,
        token_program,
        memo_program,
        event_authority,
        program,
    }
);

accounts!(
    CloseClaimProtocolFeeOperatorAccounts,
    get_close_claim_protocol_fee_operator_accounts,
    {
        claim_fee_operator,
        rent_receiver,
        admin,
    }
);

accounts!(
    ClosePositionAccounts,
    get_close_position_accounts,
    {
        position,
        lb_pair,
        bin_array_lower,
        bin_array_upper,
        sender,
        rent_receiver,
        event_authority,
        program,
    }
);

accounts!(
    ClosePosition2Accounts,
    get_close_position2_accounts,
    {
        position,
        sender,
        rent_receiver,
        event_authority,
        program,
    }
);

accounts!(
    ClosePositionIfEmptyAccounts,
    get_close_position_if_empty_accounts,
    {
        position,
        sender,
        rent_receiver,
        event_authority,
        program,
    }
);

accounts!(
    ClosePresetParameterAccounts,
    get_close_preset_parameter_accounts,
    {
        preset_parameter,
        admin,
        rent_receiver,
    }
);

accounts!(
    ClosePresetParameter2Accounts,
    get_close_preset_parameter2_accounts,
    {
        preset_parameter,
        admin,
        rent_receiver,
    }
);

accounts!(
    CreateClaimProtocolFeeOperatorAccounts,
    get_create_claim_protocol_fee_operator_accounts,
    {
        claim_fee_operator,
        operator,
        admin,
        system_program,
    }
);

accounts!(
    DecreasePositionLengthAccounts,
    get_decrease_position_length_accounts,
    {
        rent_receiver,
        position,
        owner,
        system_program,
        event_authority,
        program,
    }
);

accounts!(
    ForIdlTypeGenerationDoNotCallAccounts,
    get_for_idl_type_generation_do_not_call_accounts,
    {
        dummy_zc_account,
    }
);

accounts!(
    FundRewardAccounts,
    get_fund_reward_accounts,
    {
        lb_pair,
        reward_vault,
        reward_mint,
        funder_token_account,
        funder,
        bin_array,
        token_program,
        event_authority,
        program,
    }
);

/// Accounts for the `go_to_a_bin` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct GoToABinAccounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub from_bin_array: Option<Pubkey>,
    pub to_bin_array: Option<Pubkey>,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for GoToABinAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(GoToABinAccounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            from_bin_array: get_opt(2),
            to_bin_array: get_opt(3),
            event_authority: get_req(4, "event_authority")?,
            program: get_req(5, "program")?,
        })
    }
}

pub fn get_go_to_a_bin_accounts(ix: &InstructionView) -> Result<GoToABinAccounts, AccountsError> {
    GoToABinAccounts::try_from(ix)
}

accounts!(
    IncreaseOracleLengthAccounts,
    get_increase_oracle_length_accounts,
    {
        oracle,
        funder,
        system_program,
        event_authority,
        program,
    }
);

accounts!(
    IncreasePositionLengthAccounts,
    get_increase_position_length_accounts,
    {
        funder,
        lb_pair,
        position,
        owner,
        system_program,
        event_authority,
        program,
    }
);

accounts!(
    InitializeBinArrayAccounts,
    get_initialize_bin_array_accounts,
    {
        lb_pair,
        bin_array,
        funder,
        system_program,
    }
);

accounts!(
    InitializeBinArrayBitmapExtensionAccounts,
    get_initialize_bin_array_bitmap_extension_accounts,
    {
        lb_pair,
        /// Initialize an account to store if a bin array is initialized.
        bin_array_bitmap_extension,
        funder,
        system_program,
        rent,
    }
);

/// Accounts for the `initialize_customizable_permissionless_lb_pair` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeCustomizablePermissionlessLbPairAccounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub user_token_x: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub user_token_y: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeCustomizablePermissionlessLbPairAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(InitializeCustomizablePermissionlessLbPairAccounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            token_mint_x: get_req(2, "token_mint_x")?,
            token_mint_y: get_req(3, "token_mint_y")?,
            reserve_x: get_req(4, "reserve_x")?,
            reserve_y: get_req(5, "reserve_y")?,
            oracle: get_req(6, "oracle")?,
            user_token_x: get_req(7, "user_token_x")?,
            funder: get_req(8, "funder")?,
            token_program: get_req(9, "token_program")?,
            system_program: get_req(10, "system_program")?,
            user_token_y: get_req(11, "user_token_y")?,
            event_authority: get_req(12, "event_authority")?,
            program: get_req(13, "program")?,
        })
    }
}

pub fn get_initialize_customizable_permissionless_lb_pair_accounts(
    ix: &InstructionView,
) -> Result<InitializeCustomizablePermissionlessLbPairAccounts, AccountsError> {
    InitializeCustomizablePermissionlessLbPairAccounts::try_from(ix)
}

/// Accounts for the `initialize_customizable_permissionless_lb_pair2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeCustomizablePermissionlessLbPair2Accounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub user_token_x: Pubkey,
    pub funder: Pubkey,
    pub token_badge_x: Option<Pubkey>,
    pub token_badge_y: Option<Pubkey>,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub system_program: Pubkey,
    pub user_token_y: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeCustomizablePermissionlessLbPair2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(InitializeCustomizablePermissionlessLbPair2Accounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            token_mint_x: get_req(2, "token_mint_x")?,
            token_mint_y: get_req(3, "token_mint_y")?,
            reserve_x: get_req(4, "reserve_x")?,
            reserve_y: get_req(5, "reserve_y")?,
            oracle: get_req(6, "oracle")?,
            user_token_x: get_req(7, "user_token_x")?,
            funder: get_req(8, "funder")?,
            token_badge_x: get_opt(9),
            token_badge_y: get_opt(10),
            token_program_x: get_req(11, "token_program_x")?,
            token_program_y: get_req(12, "token_program_y")?,
            system_program: get_req(13, "system_program")?,
            user_token_y: get_req(14, "user_token_y")?,
            event_authority: get_req(15, "event_authority")?,
            program: get_req(16, "program")?,
        })
    }
}

pub fn get_initialize_customizable_permissionless_lb_pair2_accounts(
    ix: &InstructionView,
) -> Result<InitializeCustomizablePermissionlessLbPair2Accounts, AccountsError> {
    InitializeCustomizablePermissionlessLbPair2Accounts::try_from(ix)
}

/// Accounts for the `initialize_lb_pair` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeLbPairAccounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub preset_parameter: Pubkey,
    pub funder: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeLbPairAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(InitializeLbPairAccounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            token_mint_x: get_req(2, "token_mint_x")?,
            token_mint_y: get_req(3, "token_mint_y")?,
            reserve_x: get_req(4, "reserve_x")?,
            reserve_y: get_req(5, "reserve_y")?,
            oracle: get_req(6, "oracle")?,
            preset_parameter: get_req(7, "preset_parameter")?,
            funder: get_req(8, "funder")?,
            token_program: get_req(9, "token_program")?,
            system_program: get_req(10, "system_program")?,
            rent: get_req(11, "rent")?,
            event_authority: get_req(12, "event_authority")?,
            program: get_req(13, "program")?,
        })
    }
}

pub fn get_initialize_lb_pair_accounts(ix: &InstructionView) -> Result<InitializeLbPairAccounts, AccountsError> {
    InitializeLbPairAccounts::try_from(ix)
}

/// Accounts for the `initialize_lb_pair2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeLbPair2Accounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub preset_parameter: Pubkey,
    pub funder: Pubkey,
    pub token_badge_x: Option<Pubkey>,
    pub token_badge_y: Option<Pubkey>,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeLbPair2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(InitializeLbPair2Accounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            token_mint_x: get_req(2, "token_mint_x")?,
            token_mint_y: get_req(3, "token_mint_y")?,
            reserve_x: get_req(4, "reserve_x")?,
            reserve_y: get_req(5, "reserve_y")?,
            oracle: get_req(6, "oracle")?,
            preset_parameter: get_req(7, "preset_parameter")?,
            funder: get_req(8, "funder")?,
            token_badge_x: get_opt(9),
            token_badge_y: get_opt(10),
            token_program_x: get_req(11, "token_program_x")?,
            token_program_y: get_req(12, "token_program_y")?,
            system_program: get_req(13, "system_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_initialize_lb_pair2_accounts(ix: &InstructionView) -> Result<InitializeLbPair2Accounts, AccountsError> {
    InitializeLbPair2Accounts::try_from(ix)
}

/// Accounts for the `initialize_permission_lb_pair` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializePermissionLbPairAccounts {
    pub base: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
    pub token_badge_x: Option<Pubkey>,
    pub token_badge_y: Option<Pubkey>,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializePermissionLbPairAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(InitializePermissionLbPairAccounts {
            base: get_req(0, "base")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            token_mint_x: get_req(3, "token_mint_x")?,
            token_mint_y: get_req(4, "token_mint_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            oracle: get_req(7, "oracle")?,
            admin: get_req(8, "admin")?,
            token_badge_x: get_opt(9),
            token_badge_y: get_opt(10),
            token_program_x: get_req(11, "token_program_x")?,
            token_program_y: get_req(12, "token_program_y")?,
            system_program: get_req(13, "system_program")?,
            rent: get_req(14, "rent")?,
            event_authority: get_req(15, "event_authority")?,
            program: get_req(16, "program")?,
        })
    }
}

pub fn get_initialize_permission_lb_pair_accounts(ix: &InstructionView) -> Result<InitializePermissionLbPairAccounts, AccountsError> {
    InitializePermissionLbPairAccounts::try_from(ix)
}

accounts!(
    InitializePositionAccounts,
    get_initialize_position_accounts,
    {
        payer,
        position,
        lb_pair,
        owner,
        system_program,
        rent,
        event_authority,
        program,
    }
);

accounts!(
    InitializePositionByOperatorAccounts,
    get_initialize_position_by_operator_accounts,
    {
        payer,
        base,
        position,
        lb_pair,
        owner,
        /// operator
        operator,
        operator_token_x,
        owner_token_x,
        system_program,
        event_authority,
        program,
    }
);

accounts!(
    InitializePositionPdaAccounts,
    get_initialize_position_pda_accounts,
    {
        payer,
        base,
        position,
        lb_pair,
        /// owner
        owner,
        system_program,
        rent,
        event_authority,
        program,
    }
);

accounts!(
    InitializePresetParameterAccounts,
    get_initialize_preset_parameter_accounts,
    {
        preset_parameter,
        admin,
        system_program,
        rent,
    }
);

accounts!(
    InitializePresetParameter2Accounts,
    get_initialize_preset_parameter2_accounts,
    {
        preset_parameter,
        admin,
        system_program,
    }
);

/// Accounts for the `initialize_reward` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeRewardAccounts {
    pub lb_pair: Pubkey,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub token_badge: Option<Pubkey>,
    pub admin: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for InitializeRewardAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(InitializeRewardAccounts {
            lb_pair: get_req(0, "lb_pair")?,
            reward_vault: get_req(1, "reward_vault")?,
            reward_mint: get_req(2, "reward_mint")?,
            token_badge: get_opt(3),
            admin: get_req(4, "admin")?,
            token_program: get_req(5, "token_program")?,
            system_program: get_req(6, "system_program")?,
            rent: get_req(7, "rent")?,
            event_authority: get_req(8, "event_authority")?,
            program: get_req(9, "program")?,
        })
    }
}

pub fn get_initialize_reward_accounts(ix: &InstructionView) -> Result<InitializeRewardAccounts, AccountsError> {
    InitializeRewardAccounts::try_from(ix)
}

accounts!(
    InitializeTokenBadgeAccounts,
    get_initialize_token_badge_accounts,
    {
        token_mint,
        token_badge,
        admin,
        system_program,
    }
);

accounts!(
    MigrateBinArrayAccounts,
    get_migrate_bin_array_accounts,
    {
        lb_pair,
    }
);

accounts!(
    MigratePositionAccounts,
    get_migrate_position_accounts,
    {
        position_v2,
        position_v1,
        lb_pair,
        bin_array_lower,
        bin_array_upper,
        owner,
        system_program,
        rent_receiver,
        event_authority,
        program,
    }
);

/// Accounts for the `rebalance_liquidity` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RebalanceLiquidityAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub owner: Pubkey,
    pub rent_payer: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RebalanceLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RebalanceLiquidityAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            owner: get_req(9, "owner")?,
            rent_payer: get_req(10, "rent_payer")?,
            token_x_program: get_req(11, "token_x_program")?,
            token_y_program: get_req(12, "token_y_program")?,
            memo_program: get_req(13, "memo_program")?,
            system_program: get_req(14, "system_program")?,
            event_authority: get_req(15, "event_authority")?,
            program: get_req(16, "program")?,
        })
    }
}

pub fn get_rebalance_liquidity_accounts(ix: &InstructionView) -> Result<RebalanceLiquidityAccounts, AccountsError> {
    RebalanceLiquidityAccounts::try_from(ix)
}

/// Accounts for the `remove_all_liquidity` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveAllLiquidityAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveAllLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RemoveAllLiquidityAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            bin_array_lower: get_req(9, "bin_array_lower")?,
            bin_array_upper: get_req(10, "bin_array_upper")?,
            sender: get_req(11, "sender")?,
            token_x_program: get_req(12, "token_x_program")?,
            token_y_program: get_req(13, "token_y_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_remove_all_liquidity_accounts(ix: &InstructionView) -> Result<RemoveAllLiquidityAccounts, AccountsError> {
    RemoveAllLiquidityAccounts::try_from(ix)
}

/// Accounts for the `remove_liquidity` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveLiquidityAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveLiquidityAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RemoveLiquidityAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            bin_array_lower: get_req(9, "bin_array_lower")?,
            bin_array_upper: get_req(10, "bin_array_upper")?,
            sender: get_req(11, "sender")?,
            token_x_program: get_req(12, "token_x_program")?,
            token_y_program: get_req(13, "token_y_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_remove_liquidity_accounts(ix: &InstructionView) -> Result<RemoveLiquidityAccounts, AccountsError> {
    RemoveLiquidityAccounts::try_from(ix)
}

/// Accounts for the `remove_liquidity2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveLiquidity2Accounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveLiquidity2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RemoveLiquidity2Accounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            sender: get_req(9, "sender")?,
            token_x_program: get_req(10, "token_x_program")?,
            token_y_program: get_req(11, "token_y_program")?,
            memo_program: get_req(12, "memo_program")?,
            event_authority: get_req(13, "event_authority")?,
            program: get_req(14, "program")?,
        })
    }
}

pub fn get_remove_liquidity2_accounts(ix: &InstructionView) -> Result<RemoveLiquidity2Accounts, AccountsError> {
    RemoveLiquidity2Accounts::try_from(ix)
}

/// Accounts for the `remove_liquidity_by_range` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveLiquidityByRangeAccounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveLiquidityByRangeAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RemoveLiquidityByRangeAccounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            bin_array_lower: get_req(9, "bin_array_lower")?,
            bin_array_upper: get_req(10, "bin_array_upper")?,
            sender: get_req(11, "sender")?,
            token_x_program: get_req(12, "token_x_program")?,
            token_y_program: get_req(13, "token_y_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_remove_liquidity_by_range_accounts(ix: &InstructionView) -> Result<RemoveLiquidityByRangeAccounts, AccountsError> {
    RemoveLiquidityByRangeAccounts::try_from(ix)
}

/// Accounts for the `remove_liquidity_by_range2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveLiquidityByRange2Accounts {
    pub position: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub user_token_x: Pubkey,
    pub user_token_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub sender: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for RemoveLiquidityByRange2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(RemoveLiquidityByRange2Accounts {
            position: get_req(0, "position")?,
            lb_pair: get_req(1, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(2),
            user_token_x: get_req(3, "user_token_x")?,
            user_token_y: get_req(4, "user_token_y")?,
            reserve_x: get_req(5, "reserve_x")?,
            reserve_y: get_req(6, "reserve_y")?,
            token_x_mint: get_req(7, "token_x_mint")?,
            token_y_mint: get_req(8, "token_y_mint")?,
            sender: get_req(9, "sender")?,
            token_x_program: get_req(10, "token_x_program")?,
            token_y_program: get_req(11, "token_y_program")?,
            memo_program: get_req(12, "memo_program")?,
            event_authority: get_req(13, "event_authority")?,
            program: get_req(14, "program")?,
        })
    }
}

pub fn get_remove_liquidity_by_range2_accounts(ix: &InstructionView) -> Result<RemoveLiquidityByRange2Accounts, AccountsError> {
    RemoveLiquidityByRange2Accounts::try_from(ix)
}

accounts!(
    SetActivationPointAccounts,
    get_set_activation_point_accounts,
    {
        lb_pair,
        admin,
    }
);

accounts!(
    SetPairStatusAccounts,
    get_set_pair_status_accounts,
    {
        lb_pair,
        admin,
    }
);

accounts!(
    SetPairStatusPermissionlessAccounts,
    get_set_pair_status_permissionless_accounts,
    {
        lb_pair,
        creator,
    }
);

accounts!(
    SetPreActivationDurationAccounts,
    get_set_pre_activation_duration_accounts,
    {
        lb_pair,
        creator,
    }
);

accounts!(
    SetPreActivationSwapAddressAccounts,
    get_set_pre_activation_swap_address_accounts,
    {
        lb_pair,
        creator,
    }
);

/// Accounts for the `swap` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapAccounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Option<Pubkey>,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SwapAccounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            reserve_x: get_req(2, "reserve_x")?,
            reserve_y: get_req(3, "reserve_y")?,
            user_token_in: get_req(4, "user_token_in")?,
            user_token_out: get_req(5, "user_token_out")?,
            token_x_mint: get_req(6, "token_x_mint")?,
            token_y_mint: get_req(7, "token_y_mint")?,
            oracle: get_req(8, "oracle")?,
            host_fee_in: get_opt(9),
            user: get_req(10, "user")?,
            token_x_program: get_req(11, "token_x_program")?,
            token_y_program: get_req(12, "token_y_program")?,
            event_authority: get_req(13, "event_authority")?,
            program: get_req(14, "program")?,
        })
    }
}

pub fn get_swap_accounts(ix: &InstructionView) -> Result<SwapAccounts, AccountsError> {
    SwapAccounts::try_from(ix)
}

/// Accounts for the `swap2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Swap2Accounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Option<Pubkey>,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for Swap2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(Swap2Accounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            reserve_x: get_req(2, "reserve_x")?,
            reserve_y: get_req(3, "reserve_y")?,
            user_token_in: get_req(4, "user_token_in")?,
            user_token_out: get_req(5, "user_token_out")?,
            token_x_mint: get_req(6, "token_x_mint")?,
            token_y_mint: get_req(7, "token_y_mint")?,
            oracle: get_req(8, "oracle")?,
            host_fee_in: get_opt(9),
            user: get_req(10, "user")?,
            token_x_program: get_req(11, "token_x_program")?,
            token_y_program: get_req(12, "token_y_program")?,
            memo_program: get_req(13, "memo_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_swap2_accounts(ix: &InstructionView) -> Result<Swap2Accounts, AccountsError> {
    Swap2Accounts::try_from(ix)
}

/// Accounts for the `swap_exact_out` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapExactOutAccounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Option<Pubkey>,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapExactOutAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SwapExactOutAccounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            reserve_x: get_req(2, "reserve_x")?,
            reserve_y: get_req(3, "reserve_y")?,
            user_token_in: get_req(4, "user_token_in")?,
            user_token_out: get_req(5, "user_token_out")?,
            token_x_mint: get_req(6, "token_x_mint")?,
            token_y_mint: get_req(7, "token_y_mint")?,
            oracle: get_req(8, "oracle")?,
            host_fee_in: get_opt(9),
            user: get_req(10, "user")?,
            token_x_program: get_req(11, "token_x_program")?,
            token_y_program: get_req(12, "token_y_program")?,
            event_authority: get_req(13, "event_authority")?,
            program: get_req(14, "program")?,
        })
    }
}

pub fn get_swap_exact_out_accounts(ix: &InstructionView) -> Result<SwapExactOutAccounts, AccountsError> {
    SwapExactOutAccounts::try_from(ix)
}

/// Accounts for the `swap_exact_out2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapExactOut2Accounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Option<Pubkey>,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapExactOut2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SwapExactOut2Accounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            reserve_x: get_req(2, "reserve_x")?,
            reserve_y: get_req(3, "reserve_y")?,
            user_token_in: get_req(4, "user_token_in")?,
            user_token_out: get_req(5, "user_token_out")?,
            token_x_mint: get_req(6, "token_x_mint")?,
            token_y_mint: get_req(7, "token_y_mint")?,
            oracle: get_req(8, "oracle")?,
            host_fee_in: get_opt(9),
            user: get_req(10, "user")?,
            token_x_program: get_req(11, "token_x_program")?,
            token_y_program: get_req(12, "token_y_program")?,
            memo_program: get_req(13, "memo_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_swap_exact_out2_accounts(ix: &InstructionView) -> Result<SwapExactOut2Accounts, AccountsError> {
    SwapExactOut2Accounts::try_from(ix)
}

/// Accounts for the `swap_with_price_impact` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapWithPriceImpactAccounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Option<Pubkey>,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapWithPriceImpactAccounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SwapWithPriceImpactAccounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            reserve_x: get_req(2, "reserve_x")?,
            reserve_y: get_req(3, "reserve_y")?,
            user_token_in: get_req(4, "user_token_in")?,
            user_token_out: get_req(5, "user_token_out")?,
            token_x_mint: get_req(6, "token_x_mint")?,
            token_y_mint: get_req(7, "token_y_mint")?,
            oracle: get_req(8, "oracle")?,
            host_fee_in: get_opt(9),
            user: get_req(10, "user")?,
            token_x_program: get_req(11, "token_x_program")?,
            token_y_program: get_req(12, "token_y_program")?,
            event_authority: get_req(13, "event_authority")?,
            program: get_req(14, "program")?,
        })
    }
}

pub fn get_swap_with_price_impact_accounts(ix: &InstructionView) -> Result<SwapWithPriceImpactAccounts, AccountsError> {
    SwapWithPriceImpactAccounts::try_from(ix)
}

/// Accounts for the `swap_with_price_impact2` instruction.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapWithPriceImpact2Accounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Option<Pubkey>,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Option<Pubkey>,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for SwapWithPriceImpact2Accounts {
    type Error = AccountsError;
    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();
        let get_req = |i: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(i).ok_or(AccountsError::Missing { name, index: i })?;
            crate::accounts::to_pubkey(name, i, &a.0)
        };
        let get_opt = |i: usize| -> Option<Pubkey> { accounts.get(i).and_then(|a| a.0.as_slice().try_into().ok()).map(Pubkey::new_from_array) };
        Ok(SwapWithPriceImpact2Accounts {
            lb_pair: get_req(0, "lb_pair")?,
            bin_array_bitmap_extension: get_opt(1),
            reserve_x: get_req(2, "reserve_x")?,
            reserve_y: get_req(3, "reserve_y")?,
            user_token_in: get_req(4, "user_token_in")?,
            user_token_out: get_req(5, "user_token_out")?,
            token_x_mint: get_req(6, "token_x_mint")?,
            token_y_mint: get_req(7, "token_y_mint")?,
            oracle: get_req(8, "oracle")?,
            host_fee_in: get_opt(9),
            user: get_req(10, "user")?,
            token_x_program: get_req(11, "token_x_program")?,
            token_y_program: get_req(12, "token_y_program")?,
            memo_program: get_req(13, "memo_program")?,
            event_authority: get_req(14, "event_authority")?,
            program: get_req(15, "program")?,
        })
    }
}

pub fn get_swap_with_price_impact2_accounts(ix: &InstructionView) -> Result<SwapWithPriceImpact2Accounts, AccountsError> {
    SwapWithPriceImpact2Accounts::try_from(ix)
}

accounts!(
    UpdateBaseFeeParametersAccounts,
    get_update_base_fee_parameters_accounts,
    {
        lb_pair,
        admin,
        event_authority,
        program,
    }
);

accounts!(
    UpdateDynamicFeeParametersAccounts,
    get_update_dynamic_fee_parameters_accounts,
    {
        lb_pair,
        admin,
        event_authority,
        program,
    }
);

accounts!(
    UpdateFeesAndReward2Accounts,
    get_update_fees_and_reward2_accounts,
    {
        position,
        lb_pair,
        owner,
    }
);

accounts!(
    UpdateFeesAndRewardsAccounts,
    get_update_fees_and_rewards_accounts,
    {
        position,
        lb_pair,
        bin_array_lower,
        bin_array_upper,
        owner,
    }
);

accounts!(
    UpdatePositionOperatorAccounts,
    get_update_position_operator_accounts,
    {
        position,
        owner,
        event_authority,
        program,
    }
);

accounts!(
    UpdateRewardDurationAccounts,
    get_update_reward_duration_accounts,
    {
        lb_pair,
        admin,
        bin_array,
        event_authority,
        program,
    }
);

accounts!(
    UpdateRewardFunderAccounts,
    get_update_reward_funder_accounts,
    {
        lb_pair,
        admin,
        event_authority,
        program,
    }
);

accounts!(
    WithdrawIneligibleRewardAccounts,
    get_withdraw_ineligible_reward_accounts,
    {
        lb_pair,
        reward_vault,
        reward_mint,
        funder_token_account,
        funder,
        bin_array,
        token_program,
        memo_program,
        event_authority,
        program,
    }
);

accounts!(
    WithdrawProtocolFeeAccounts,
    get_withdraw_protocol_fee_accounts,
    {
        lb_pair,
        reserve_x,
        reserve_y,
        token_x_mint,
        token_y_mint,
        receiver_token_x,
        receiver_token_y,
        claim_fee_operator,
        /// operator
        operator,
        token_x_program,
        token_y_program,
        memo_program,
    }
);
