//! BonkSwap on-chain instructions.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct FixedPoint {
    pub v: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Token {
    pub v: u64,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const CREATE_POOL: [u8; 8] = [244, 236, 117, 4, 18, 0, 62, 88];
pub const CREATE_PROVIDER: [u8; 8] = [229, 222, 145, 200, 251, 64, 186, 242];
pub const CREATE_STATE: [u8; 8] = [96, 136, 104, 83, 116, 63, 225, 58];
pub const ADD_TOKENS: [u8; 8] = [235, 5, 251, 241, 34, 5, 87, 148];
pub const WITHDRAW_BUYBACK: [u8; 8] = [165, 47, 140, 185, 166, 81, 67, 193];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const WITHDRAW_SHARES: [u8; 8] = [154, 125, 104, 220, 109, 76, 190, 75];
pub const WITHDRAW_LP_FEE: [u8; 8] = [228, 128, 27, 71, 99, 91, 176, 245];
pub const WITHDRAW_PROJECT_FEE: [u8; 8] = [217, 56, 100, 164, 87, 185, 120, 122];
pub const CREATE_FARM: [u8; 8] = [132, 184, 152, 144, 36, 178, 106, 65];
pub const CREATE_DUAL_FARM: [u8; 8] = [116, 188, 85, 98, 87, 133, 44, 198];
pub const CREATE_TRIPLE_FARM: [u8; 8] = [47, 47, 162, 177, 193, 92, 246, 43];
pub const WITHDRAW_REWARDS: [u8; 8] = [12, 208, 61, 169, 41, 19, 58, 161];
pub const CLOSE_POOL: [u8; 8] = [78, 253, 165, 27, 159, 185, 29, 236];
pub const WITHDRAW_MERCANTI_FEE: [u8; 8] = [9, 63, 114, 166, 104, 156, 8, 156];
pub const ADD_SUPPLY: [u8; 8] = [208, 22, 199, 241, 3, 135, 81, 50];
pub const UPDATE_FEES: [u8; 8] = [1, 7, 208, 119, 69, 167, 97, 219];
pub const RESET_FARM: [u8; 8] = [245, 17, 148, 220, 55, 229, 71, 86];
pub const UPDATE_REWARD_TOKENS: [u8; 8] = [156, 68, 147, 116, 29, 57, 159, 114];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BonkSwapInstruction {
    CreatePool(CreatePoolInstruction),
    CreateProvider(CreateProviderInstruction),
    CreateState(CreateStateInstruction),
    AddTokens(AddTokensInstruction),
    WithdrawBuyback,
    Swap(SwapInstruction),
    WithdrawShares(WithdrawSharesInstruction),
    WithdrawLpFee,
    WithdrawProjectFee,
    CreateFarm(CreateFarmInstruction),
    CreateDualFarm(CreateDualFarmInstruction),
    CreateTripleFarm(CreateTripleFarmInstruction),
    WithdrawRewards,
    ClosePool,
    WithdrawMercantiFee,
    AddSupply(AddSupplyInstruction),
    UpdateFees(UpdateFeesInstruction),
    ResetFarm,
    UpdateRewardTokens,
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreatePoolInstruction {
    pub lp_fee: FixedPoint,
    pub buyback_fee: FixedPoint,
    pub project_fee: FixedPoint,
    pub mercanti_fee: FixedPoint,
    pub initial_token_x: Token,
    pub initial_token_y: Token,
    pub bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateProviderInstruction {
    pub token_x_amount: Token,
    pub token_y_amount: Token,
    pub bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateStateInstruction {
    pub nonce: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddTokensInstruction {
    pub delta_x: Token,
    pub delta_y: Token,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapInstruction {
    pub delta_in: Token,
    pub price_limit: FixedPoint,
    pub x_to_y: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawSharesInstruction {
    pub shares: Token,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateFarmInstruction {
    pub supply: Token,
    pub duration: u64,
    pub bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateDualFarmInstruction {
    pub supply_marco: Token,
    pub supply_project_first: Token,
    pub duration: u64,
    pub bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateTripleFarmInstruction {
    pub supply_marco: Token,
    pub supply_project_first: Token,
    pub supply_project_second: Token,
    pub duration: u64,
    pub bump: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddSupplyInstruction {
    pub supply_marco: Token,
    pub supply_project_first: Token,
    pub supply_project_second: Token,
    pub duration: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateFeesInstruction {
    pub new_buyback_fee: FixedPoint,
    pub new_project_fee: FixedPoint,
    pub new_provider_fee: FixedPoint,
    pub new_mercanti_fee: FixedPoint,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for BonkSwapInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            CREATE_POOL => Self::CreatePool(CreatePoolInstruction::try_from_slice(payload)?),
            CREATE_PROVIDER => Self::CreateProvider(CreateProviderInstruction::try_from_slice(payload)?),
            CREATE_STATE => Self::CreateState(CreateStateInstruction::try_from_slice(payload)?),
            ADD_TOKENS => Self::AddTokens(AddTokensInstruction::try_from_slice(payload)?),
            WITHDRAW_BUYBACK => Self::WithdrawBuyback,
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            WITHDRAW_SHARES => Self::WithdrawShares(WithdrawSharesInstruction::try_from_slice(payload)?),
            WITHDRAW_LP_FEE => Self::WithdrawLpFee,
            WITHDRAW_PROJECT_FEE => Self::WithdrawProjectFee,
            CREATE_FARM => Self::CreateFarm(CreateFarmInstruction::try_from_slice(payload)?),
            CREATE_DUAL_FARM => Self::CreateDualFarm(CreateDualFarmInstruction::try_from_slice(payload)?),
            CREATE_TRIPLE_FARM => Self::CreateTripleFarm(CreateTripleFarmInstruction::try_from_slice(payload)?),
            WITHDRAW_REWARDS => Self::WithdrawRewards,
            CLOSE_POOL => Self::ClosePool,
            WITHDRAW_MERCANTI_FEE => Self::WithdrawMercantiFee,
            ADD_SUPPLY => Self::AddSupply(AddSupplyInstruction::try_from_slice(payload)?),
            UPDATE_FEES => Self::UpdateFees(UpdateFeesInstruction::try_from_slice(payload)?),
            RESET_FARM => Self::ResetFarm,
            UPDATE_REWARD_TOKENS => Self::UpdateRewardTokens,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<BonkSwapInstruction, ParseError> {
    BonkSwapInstruction::try_from(data)
}
