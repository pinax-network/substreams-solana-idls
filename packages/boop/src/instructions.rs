//! Boop on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// Discriminators
pub const ADD_OPERATORS: [u8; 8] = [165, 199, 62, 214, 81, 54, 4, 150];
pub const BUY_TOKEN: [u8; 8] = [138, 127, 14, 91, 38, 87, 115, 105];
pub const CANCEL_AUTHORITY_TRANSFER: [u8; 8] = [94, 131, 125, 184, 183, 24, 125, 229];
pub const CLOSE_BONDING_CURVE_VAULT: [u8; 8] = [189, 71, 189, 239, 113, 66, 59, 189];
pub const COLLECT_METEORA_TRADING_FEES: [u8; 8] = [249, 95, 126, 91, 81, 162, 83, 250];
pub const COLLECT_METEORA_TRADING_FEES_V2: [u8; 8] = [96, 39, 109, 46, 5, 161, 15, 57];
pub const COLLECT_TRADING_FEES: [u8; 8] = [189, 38, 205, 234, 81, 77, 25, 1];
pub const COLLECT_TRADING_FEES_V2: [u8; 8] = [180, 138, 160, 155, 243, 88, 168, 7];
pub const COMPLETE_AUTHORITY_TRANSFER: [u8; 8] = [81, 233, 91, 132, 175, 31, 151, 141];
pub const CREATE_METEORA_POOL: [u8; 8] = [246, 254, 33, 37, 225, 176, 41, 232];
pub const CREATE_RAYDIUM_POOL: [u8; 8] = [65, 45, 119, 77, 204, 178, 84, 2];
pub const CREATE_RAYDIUM_RANDOM_POOL: [u8; 8] = [78, 44, 173, 29, 132, 180, 4, 172];
pub const CREATE_TOKEN: [u8; 8] = [84, 52, 204, 228, 24, 140, 234, 75];
pub const CREATE_TOKEN_FALLBACK: [u8; 8] = [253, 184, 126, 199, 235, 232, 172, 162];
pub const DEPLOY_BONDING_CURVE: [u8; 8] = [180, 89, 199, 76, 168, 236, 217, 138];
pub const DEPLOY_BONDING_CURVE_FALLBACK: [u8; 8] = [53, 230, 172, 84, 77, 174, 22, 61];
pub const DEPOSIT_INTO_RAYDIUM: [u8; 8] = [168, 89, 99, 30, 117, 49, 88, 224];
pub const GRADUATE: [u8; 8] = [45, 235, 225, 181, 17, 218, 64, 130];
pub const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
pub const INITIATE_AUTHORITY_TRANSFER: [u8; 8] = [210, 43, 101, 215, 119, 140, 106, 218];
pub const LOCK_RAYDIUM_LIQUIDITY: [u8; 8] = [173, 255, 148, 6, 122, 99, 140, 22];
pub const REMOVE_OPERATORS: [u8; 8] = [42, 20, 89, 83, 222, 37, 4, 109];
pub const SELL_TOKEN: [u8; 8] = [109, 61, 40, 187, 230, 176, 135, 174];
pub const SPLIT_POST_GRADUATION_TRADING_FEES: [u8; 8] = [241, 178, 177, 69, 38, 187, 58, 176];
pub const SPLIT_TRADING_FEES: [u8; 8] = [96, 126, 225, 47, 185, 213, 50, 58];
pub const SWAP_SOL_FOR_TOKENS_ON_RAYDIUM: [u8; 8] = [107, 248, 131, 239, 152, 234, 54, 35];
pub const SWAP_TOKENS_FOR_SOL_ON_RAYDIUM: [u8; 8] = [216, 172, 130, 148, 34, 98, 215, 163];
pub const TOGGLE_PAUSED: [u8; 8] = [54, 83, 147, 198, 123, 97, 218, 72];
pub const UPDATE_CONFIG: [u8; 8] = [29, 158, 252, 191, 10, 83, 219, 99];

#[derive(Debug, Clone, PartialEq)]
pub enum BoopInstruction {
    AddOperators(AddOperatorsInstruction),
    BuyToken(BuyTokenInstruction),
    CancelAuthorityTransfer,
    CloseBondingCurveVault,
    CollectMeteoraTradingFees,
    CollectMeteoraTradingFeesV2,
    CollectTradingFees,
    CollectTradingFeesV2,
    CompleteAuthorityTransfer,
    CreateMeteoraPool,
    CreateRaydiumPool,
    CreateRaydiumRandomPool,
    CreateToken(CreateTokenInstruction),
    CreateTokenFallback(CreateTokenFallbackInstruction),
    DeployBondingCurve(DeployBondingCurveInstruction),
    DeployBondingCurveFallback(DeployBondingCurveFallbackInstruction),
    DepositIntoRaydium(DepositIntoRaydiumInstruction),
    Graduate,
    Initialize(InitializeInstruction),
    InitiateAuthorityTransfer(InitiateAuthorityTransferInstruction),
    LockRaydiumLiquidity,
    RemoveOperators(RemoveOperatorsInstruction),
    SellToken(SellTokenInstruction),
    SplitPostGraduationTradingFees,
    SplitTradingFees,
    SwapSolForTokensOnRaydium(SwapSolForTokensOnRaydiumInstruction),
    SwapTokensForSolOnRaydium(SwapTokensForSolOnRaydiumInstruction),
    TogglePaused,
    UpdateConfig(UpdateConfigInstruction),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AddOperatorsInstruction {
    pub operators: Vec<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyTokenInstruction {
    pub buy_amount: u64,
    pub amount_out_min: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateTokenInstruction {
    pub salt: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateTokenFallbackInstruction {
    pub salt: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DeployBondingCurveInstruction {
    pub creator: Pubkey,
    pub salt: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DeployBondingCurveFallbackInstruction {
    pub creator: Pubkey,
    pub salt: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DepositIntoRaydiumInstruction {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct InitializeInstruction {
    pub protocol_fee_recipient: Pubkey,
    pub token_distributor: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct InitiateAuthorityTransferInstruction {
    pub new_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RemoveOperatorsInstruction {
    pub operators: Vec<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellTokenInstruction {
    pub sell_amount: u64,
    pub amount_out_min: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapSolForTokensOnRaydiumInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapTokensForSolOnRaydiumInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdateConfigInstruction {
    pub new_protocol_fee_recipient: Pubkey,
    pub new_virtual_sol_reserves: u64,
    pub new_virtual_token_reserves: u64,
    pub new_graduation_target: u64,
    pub new_graduation_fee: u64,
    pub new_damping_term: u8,
    pub new_swap_fee_basis_points: u8,
    pub new_token_for_stakers_basis_points: u16,
    pub new_token_amount_for_raydium_liquidity: u64,
    pub new_max_graduation_price_deviation_basis_points: u16,
    pub new_max_swap_amount_for_pool_price_correction_basis_points: u16,
}

impl<'a> TryFrom<&'a [u8]> for BoopInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            ADD_OPERATORS => Self::AddOperators(AddOperatorsInstruction::try_from_slice(payload)?),
            BUY_TOKEN => Self::BuyToken(BuyTokenInstruction::try_from_slice(payload)?),
            CANCEL_AUTHORITY_TRANSFER => Self::CancelAuthorityTransfer,
            CLOSE_BONDING_CURVE_VAULT => Self::CloseBondingCurveVault,
            COLLECT_METEORA_TRADING_FEES => Self::CollectMeteoraTradingFees,
            COLLECT_METEORA_TRADING_FEES_V2 => Self::CollectMeteoraTradingFeesV2,
            COLLECT_TRADING_FEES => Self::CollectTradingFees,
            COLLECT_TRADING_FEES_V2 => Self::CollectTradingFeesV2,
            COMPLETE_AUTHORITY_TRANSFER => Self::CompleteAuthorityTransfer,
            CREATE_METEORA_POOL => Self::CreateMeteoraPool,
            CREATE_RAYDIUM_POOL => Self::CreateRaydiumPool,
            CREATE_RAYDIUM_RANDOM_POOL => Self::CreateRaydiumRandomPool,
            CREATE_TOKEN => Self::CreateToken(CreateTokenInstruction::try_from_slice(payload)?),
            CREATE_TOKEN_FALLBACK => Self::CreateTokenFallback(CreateTokenFallbackInstruction::try_from_slice(payload)?),
            DEPLOY_BONDING_CURVE => Self::DeployBondingCurve(DeployBondingCurveInstruction::try_from_slice(payload)?),
            DEPLOY_BONDING_CURVE_FALLBACK => Self::DeployBondingCurveFallback(DeployBondingCurveFallbackInstruction::try_from_slice(payload)?),
            DEPOSIT_INTO_RAYDIUM => Self::DepositIntoRaydium(DepositIntoRaydiumInstruction::try_from_slice(payload)?),
            GRADUATE => Self::Graduate,
            INITIALIZE => Self::Initialize(InitializeInstruction::try_from_slice(payload)?),
            INITIATE_AUTHORITY_TRANSFER => Self::InitiateAuthorityTransfer(InitiateAuthorityTransferInstruction::try_from_slice(payload)?),
            LOCK_RAYDIUM_LIQUIDITY => Self::LockRaydiumLiquidity,
            REMOVE_OPERATORS => Self::RemoveOperators(RemoveOperatorsInstruction::try_from_slice(payload)?),
            SELL_TOKEN => Self::SellToken(SellTokenInstruction::try_from_slice(payload)?),
            SPLIT_POST_GRADUATION_TRADING_FEES => Self::SplitPostGraduationTradingFees,
            SPLIT_TRADING_FEES => Self::SplitTradingFees,
            SWAP_SOL_FOR_TOKENS_ON_RAYDIUM => Self::SwapSolForTokensOnRaydium(SwapSolForTokensOnRaydiumInstruction::try_from_slice(payload)?),
            SWAP_TOKENS_FOR_SOL_ON_RAYDIUM => Self::SwapTokensForSolOnRaydium(SwapTokensForSolOnRaydiumInstruction::try_from_slice(payload)?),
            TOGGLE_PAUSED => Self::TogglePaused,
            UPDATE_CONFIG => Self::UpdateConfig(UpdateConfigInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<BoopInstruction, ParseError> {
    BoopInstruction::try_from(data)
}
