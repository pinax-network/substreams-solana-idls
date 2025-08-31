//! Raydium CPMM instructions.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const CLOSE_PERMISSION_PDA: [u8; 8] = [156, 84, 32, 118, 69, 135, 70, 123];
pub const COLLECT_CREATOR_FEE: [u8; 8] = [20, 22, 86, 123, 198, 28, 219, 132];
pub const COLLECT_FUND_FEE: [u8; 8] = [167, 138, 78, 149, 223, 194, 6, 126];
pub const COLLECT_PROTOCOL_FEE: [u8; 8] = [136, 136, 252, 221, 194, 66, 126, 89];
pub const CREATE_AMM_CONFIG: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
pub const CREATE_PERMISSION_PDA: [u8; 8] = [135, 136, 2, 216, 137, 169, 181, 202];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIALIZE_WITH_PERMISSION: [u8; 8] = [63, 55, 254, 65, 49, 178, 89, 121];
pub const SWAP_BASE_INPUT: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];
pub const SWAP_BASE_OUTPUT: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];
pub const UPDATE_AMM_CONFIG: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];
pub const UPDATE_POOL_STATUS: [u8; 8] = [130, 87, 108, 6, 46, 224, 117, 123];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumCpmmInstruction {
    ClosePermissionPda,
    CollectCreatorFee,
    CollectFundFee(CollectFundFeeInstruction),
    CollectProtocolFee(CollectProtocolFeeInstruction),
    CreateAmmConfig(CreateAmmConfigInstruction),
    CreatePermissionPda,
    Deposit(DepositInstruction),
    Initialize(InitializeInstruction),
    InitializeWithPermission(InitializeWithPermissionInstruction),
    SwapBaseInput(SwapBaseInputInstruction),
    SwapBaseOutput(SwapBaseOutputInstruction),
    UpdateAmmConfig(UpdateAmmConfigInstruction),
    UpdatePoolStatus(UpdatePoolStatusInstruction),
    Withdraw(WithdrawInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Collect the fund fee accrued to the pool
/// 
/// # Arguments
/// 
/// * `ctx` - The context of accounts
/// * `amount_0_requested` - The maximum amount of token_0 to send, can be 0 to collect fees in only token_1
/// * `amount_1_requested` - The maximum amount of token_1 to send, can be 0 to collect fees in only token_0
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectFundFeeInstruction {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// Collect the protocol fee accrued to the pool
/// 
/// # Arguments
/// 
/// * `ctx` - The context of accounts
/// * `amount_0_requested` - The maximum amount of token_0 to send, can be 0 to collect fees in only token_1
/// * `amount_1_requested` - The maximum amount of token_1 to send, can be 0 to collect fees in only token_0
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CollectProtocolFeeInstruction {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

/// # Arguments
/// 
/// * `ctx`- The accounts needed by instruction.
/// * `index` - The index of amm config, there may be multiple config.
/// * `trade_fee_rate` - Trade fee rate, can be changed.
/// * `protocol_fee_rate` - The rate of protocol fee within trade fee.
/// * `fund_fee_rate` - The rate of fund fee within trade fee.
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateAmmConfigInstruction {
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub creator_fee_rate: u64,
}

/// Deposit lp token to the pool
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `lp_token_amount` - Increased number of LPs
/// * `maximum_token_0_amount` -  Maximum token 0 amount to deposit, prevents excessive slippage
/// * `maximum_token_1_amount` - Maximum token 1 amount to deposit, prevents excessive slippage
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositInstruction {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

/// Creates a pool for the given token pair and the initial price
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `init_amount_0` - the initial amount_0 to deposit
/// * `init_amount_1` - the initial amount_1 to deposit
/// * `open_time` - the timestamp allowed for swap
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeInstruction {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
}

/// Create a pool with permission
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `init_amount_0` - the initial amount_0 to deposit
/// * `init_amount_1` - the initial amount_1 to deposit
/// * `open_time` - the timestamp allowed for swap
/// * `creator_fee_on` - creator fee model, 0：both token0 and token1 (depends on the input), 1: only token0, 2: only token1
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitializeWithPermissionInstruction {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
    pub creator_fee_on: CreatorFeeOn,
}

/// Swap the tokens in the pool base input amount
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `amount_in` -  input amount to transfer, output to DESTINATION is based on the exchange rate
/// * `minimum_amount_out` -  Minimum amount of output token, prevents excessive slippage
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseInputInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

/// Swap the tokens in the pool base output amount
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `max_amount_in` -  input amount prevents excessive slippage
/// * `amount_out` -  amount of output token
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseOutputInstruction {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

/// Updates the owner of the amm config
/// Must be called by the current owner or admin
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `trade_fee_rate`- The new trade fee rate of amm config, be set when `param` is 0
/// * `protocol_fee_rate`- The new protocol fee rate of amm config, be set when `param` is 1
/// * `fund_fee_rate`- The new fund fee rate of amm config, be set when `param` is 2
/// * `new_owner`- The config's new owner, be set when `param` is 3
/// * `new_fund_owner`- The config's new fund owner, be set when `param` is 4
/// * `param`- The value can be 0 | 1 | 2 | 3 | 4, otherwise will report a error
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateAmmConfigInstruction {
    pub param: u8,
    pub value: u64,
}

/// Update pool status for given value
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `status` - The value of status
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdatePoolStatusInstruction {
    pub status: u8,
}

/// Withdraw lp for token0 and token1
/// 
/// # Arguments
/// 
/// * `ctx`- The context of accounts
/// * `lp_token_amount` - Amount of pool tokens to burn. User receives an output of token a and b based on the percentage of the pool tokens that are returned.
/// * `minimum_token_0_amount` -  Minimum amount of token 0 to receive, prevents excessive slippage
/// * `minimum_token_1_amount` -  Minimum amount of token 1 to receive, prevents excessive slippage
/// 
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawInstruction {
    pub lp_token_amount: u64,
    pub minimum_token_0_amount: u64,
    pub minimum_token_1_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum CreatorFeeOn {
    BothToken,
    OnlyToken0,
    OnlyToken1,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumCpmmInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            CLOSE_PERMISSION_PDA => Self::ClosePermissionPda,
            COLLECT_CREATOR_FEE => Self::CollectCreatorFee,
            COLLECT_FUND_FEE => Self::CollectFundFee(CollectFundFeeInstruction::try_from_slice(payload)?),
            COLLECT_PROTOCOL_FEE => Self::CollectProtocolFee(CollectProtocolFeeInstruction::try_from_slice(payload)?),
            CREATE_AMM_CONFIG => Self::CreateAmmConfig(CreateAmmConfigInstruction::try_from_slice(payload)?),
            CREATE_PERMISSION_PDA => Self::CreatePermissionPda,
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            INITIALIZE => Self::Initialize(InitializeInstruction::try_from_slice(payload)?),
            INITIALIZE_WITH_PERMISSION => Self::InitializeWithPermission(InitializeWithPermissionInstruction::try_from_slice(payload)?),
            SWAP_BASE_INPUT => Self::SwapBaseInput(SwapBaseInputInstruction::try_from_slice(payload)?),
            SWAP_BASE_OUTPUT => Self::SwapBaseOutput(SwapBaseOutputInstruction::try_from_slice(payload)?),
            UPDATE_AMM_CONFIG => Self::UpdateAmmConfig(UpdateAmmConfigInstruction::try_from_slice(payload)?),
            UPDATE_POOL_STATUS => Self::UpdatePoolStatus(UpdatePoolStatusInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumCpmmInstruction, ParseError> {
    RaydiumCpmmInstruction::try_from(data)
}