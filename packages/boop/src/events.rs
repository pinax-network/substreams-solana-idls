//! Boop on-chain events.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// Event discriminators
pub const AUTHORITY_TRANSFER_CANCELLED_EVENT_EVENT: [u8; 8] = [192, 121, 140, 224, 229, 96, 13, 143];
pub const AUTHORITY_TRANSFER_COMPLETED_EVENT_EVENT: [u8; 8] = [163, 132, 217, 128, 243, 92, 90, 249];
pub const AUTHORITY_TRANSFER_INITIATED_EVENT_EVENT: [u8; 8] = [121, 246, 95, 155, 229, 109, 148, 205];
pub const BONDING_CURVE_DEPLOYED_EVENT_EVENT: [u8; 8] = [225, 80, 178, 34, 217, 39, 184, 148];
pub const BONDING_CURVE_DEPLOYED_FALLBACK_EVENT_EVENT: [u8; 8] = [106, 252, 243, 115, 199, 159, 247, 31];
pub const BONDING_CURVE_VAULT_CLOSED_EVENT_EVENT: [u8; 8] = [185, 36, 156, 82, 189, 164, 207, 79];
pub const CONFIG_UPDATED_EVENT_EVENT: [u8; 8] = [245, 158, 129, 99, 60, 100, 214, 220];
pub const LIQUIDITY_DEPOSITED_INTO_RAYDIUM_EVENT_EVENT: [u8; 8] = [236, 50, 97, 27, 198, 101, 248, 20];
pub const OPERATORS_ADDED_EVENT_EVENT: [u8; 8] = [247, 58, 112, 56, 203, 186, 112, 152];
pub const OPERATORS_REMOVED_EVENT_EVENT: [u8; 8] = [44, 72, 75, 70, 151, 42, 53, 89];
pub const PAUSED_TOGGLED_EVENT_EVENT: [u8; 8] = [143, 222, 228, 224, 6, 230, 64, 176];
pub const POST_GRADUATION_TRADING_FEES_SPLIT_EVENT_EVENT: [u8; 8] = [34, 231, 16, 81, 36, 203, 158, 196];
pub const RAYDIUM_LIQUIDITY_LOCKED_EVENT_EVENT: [u8; 8] = [172, 189, 8, 241, 137, 175, 59, 100];
pub const RAYDIUM_POOL_CREATED_EVENT_EVENT: [u8; 8] = [170, 178, 21, 215, 84, 222, 34, 101];
pub const RAYDIUM_RANDOM_POOL_CREATED_EVENT_EVENT: [u8; 8] = [152, 251, 128, 152, 158, 235, 83, 53];
pub const SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_EVENT_EVENT: [u8; 8] = [247, 1, 8, 166, 221, 116, 113, 98];
pub const SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_EVENT_EVENT: [u8; 8] = [76, 249, 221, 162, 65, 70, 118, 32];
pub const TOKEN_BOUGHT_EVENT_EVENT: [u8; 8] = [71, 89, 222, 124, 215, 192, 230, 138];
pub const TOKEN_CREATED_EVENT_EVENT: [u8; 8] = [96, 122, 113, 138, 50, 227, 149, 57];
pub const TOKEN_CREATED_FALLBACK_EVENT_EVENT: [u8; 8] = [157, 202, 35, 92, 165, 163, 39, 56];
pub const TOKEN_GRADUATED_EVENT_EVENT: [u8; 8] = [73, 116, 111, 26, 92, 217, 146, 141];
pub const TOKEN_SOLD_EVENT_EVENT: [u8; 8] = [204, 239, 182, 77, 241, 51, 77, 66];
pub const TRADING_FEES_COLLECTED_EVENT_EVENT: [u8; 8] = [225, 63, 26, 55, 134, 243, 210, 203];
pub const TRADING_FEES_COLLECTED_V2_EVENT_EVENT: [u8; 8] = [23, 246, 130, 250, 11, 49, 240, 179];
pub const TRADING_FEES_SPLIT_EVENT_EVENT: [u8; 8] = [113, 60, 159, 17, 253, 174, 135, 122];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AuthorityTransferCompletedEvent {
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AuthorityTransferInitiatedEvent {
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BondingCurveDeployedEvent {
    pub mint: Pubkey,
    pub creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BondingCurveDeployedFallbackEvent {
    pub mint: Pubkey,
    pub creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BondingCurveVaultClosedEvent {
    pub mint: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConfigUpdatedEvent {
    pub protocol_fee_recipient: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub damping_term: u8,
    pub swap_fee_basis_points: u8,
    pub token_for_stakers_basis_points: u16,
    pub token_amount_for_raydium_liquidity: u64,
    pub max_graduation_price_deviation_basis_points: u16,
    pub max_swap_amount_for_pool_price_correction_basis_points: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct LiquidityDepositedIntoRaydiumEvent {
    pub pool_state: Pubkey,
    pub mint: Pubkey,
    pub lp_token_amount: u64,
    pub tokens_deposited: u64,
    pub wsol_deposited: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OperatorsAddedEvent {
    pub operators: Vec<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OperatorsRemovedEvent {
    pub operators: Vec<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PausedToggledEvent {
    pub is_paused: bool,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PostGraduationTradingFeesSplitEvent {
    pub amount: u64,
    pub creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RaydiumLiquidityLockedEvent {
    pub pool_state: Pubkey,
    pub mint: Pubkey,
    pub lp_amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RaydiumPoolCreatedEvent {
    pub pool_state: Pubkey,
    pub mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RaydiumRandomPoolCreatedEvent {
    pub pool_state: Pubkey,
    pub mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapSolForTokensOnRaydiumEvent {
    pub mint: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SwapTokensForSolOnRaydiumEvent {
    pub mint: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TokenBoughtEvent {
    pub mint: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_fee: u64,
    pub buyer: Pubkey,
    pub recipient: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TokenCreatedEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TokenCreatedFallbackEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TokenGraduatedEvent {
    pub mint: Pubkey,
    pub sol_for_liquidity: u64,
    pub graduation_fee: u64,
    pub token_for_distributor: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TokenSoldEvent {
    pub mint: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_fee: u64,
    pub seller: Pubkey,
    pub recipient: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TradingFeesCollectedEvent {
    pub pool_state: Pubkey,
    pub mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TradingFeesCollectedV2Event {
    pub pool_state: Pubkey,
    pub mint: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TradingFeesSplitEvent {
    pub amount: u64,
    pub creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoopEvent {
    AuthorityTransferCancelledEvent,
    AuthorityTransferCompletedEvent(AuthorityTransferCompletedEvent),
    AuthorityTransferInitiatedEvent(AuthorityTransferInitiatedEvent),
    BondingCurveDeployedEvent(BondingCurveDeployedEvent),
    BondingCurveDeployedFallbackEvent(BondingCurveDeployedFallbackEvent),
    BondingCurveVaultClosedEvent(BondingCurveVaultClosedEvent),
    ConfigUpdatedEvent(ConfigUpdatedEvent),
    LiquidityDepositedIntoRaydiumEvent(LiquidityDepositedIntoRaydiumEvent),
    OperatorsAddedEvent(OperatorsAddedEvent),
    OperatorsRemovedEvent(OperatorsRemovedEvent),
    PausedToggledEvent(PausedToggledEvent),
    PostGraduationTradingFeesSplitEvent(PostGraduationTradingFeesSplitEvent),
    RaydiumLiquidityLockedEvent(RaydiumLiquidityLockedEvent),
    RaydiumPoolCreatedEvent(RaydiumPoolCreatedEvent),
    RaydiumRandomPoolCreatedEvent(RaydiumRandomPoolCreatedEvent),
    SwapSolForTokensOnRaydiumEvent(SwapSolForTokensOnRaydiumEvent),
    SwapTokensForSolOnRaydiumEvent(SwapTokensForSolOnRaydiumEvent),
    TokenBoughtEvent(TokenBoughtEvent),
    TokenCreatedEvent(TokenCreatedEvent),
    TokenCreatedFallbackEvent(TokenCreatedFallbackEvent),
    TokenGraduatedEvent(TokenGraduatedEvent),
    TokenSoldEvent(TokenSoldEvent),
    TradingFeesCollectedEvent(TradingFeesCollectedEvent),
    TradingFeesCollectedV2Event(TradingFeesCollectedV2Event),
    TradingFeesSplitEvent(TradingFeesSplitEvent),
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for BoopEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            AUTHORITY_TRANSFER_CANCELLED_EVENT_EVENT => Self::AuthorityTransferCancelledEvent,
            AUTHORITY_TRANSFER_COMPLETED_EVENT_EVENT => Self::AuthorityTransferCompletedEvent(AuthorityTransferCompletedEvent::try_from_slice(payload)?),
            AUTHORITY_TRANSFER_INITIATED_EVENT_EVENT => Self::AuthorityTransferInitiatedEvent(AuthorityTransferInitiatedEvent::try_from_slice(payload)?),
            BONDING_CURVE_DEPLOYED_EVENT_EVENT => Self::BondingCurveDeployedEvent(BondingCurveDeployedEvent::try_from_slice(payload)?),
            BONDING_CURVE_DEPLOYED_FALLBACK_EVENT_EVENT => Self::BondingCurveDeployedFallbackEvent(BondingCurveDeployedFallbackEvent::try_from_slice(payload)?),
            BONDING_CURVE_VAULT_CLOSED_EVENT_EVENT => Self::BondingCurveVaultClosedEvent(BondingCurveVaultClosedEvent::try_from_slice(payload)?),
            CONFIG_UPDATED_EVENT_EVENT => Self::ConfigUpdatedEvent(ConfigUpdatedEvent::try_from_slice(payload)?),
            LIQUIDITY_DEPOSITED_INTO_RAYDIUM_EVENT_EVENT => {
                Self::LiquidityDepositedIntoRaydiumEvent(LiquidityDepositedIntoRaydiumEvent::try_from_slice(payload)?)
            }
            OPERATORS_ADDED_EVENT_EVENT => Self::OperatorsAddedEvent(OperatorsAddedEvent::try_from_slice(payload)?),
            OPERATORS_REMOVED_EVENT_EVENT => Self::OperatorsRemovedEvent(OperatorsRemovedEvent::try_from_slice(payload)?),
            PAUSED_TOGGLED_EVENT_EVENT => Self::PausedToggledEvent(PausedToggledEvent::try_from_slice(payload)?),
            POST_GRADUATION_TRADING_FEES_SPLIT_EVENT_EVENT => {
                Self::PostGraduationTradingFeesSplitEvent(PostGraduationTradingFeesSplitEvent::try_from_slice(payload)?)
            }
            RAYDIUM_LIQUIDITY_LOCKED_EVENT_EVENT => Self::RaydiumLiquidityLockedEvent(RaydiumLiquidityLockedEvent::try_from_slice(payload)?),
            RAYDIUM_POOL_CREATED_EVENT_EVENT => Self::RaydiumPoolCreatedEvent(RaydiumPoolCreatedEvent::try_from_slice(payload)?),
            RAYDIUM_RANDOM_POOL_CREATED_EVENT_EVENT => Self::RaydiumRandomPoolCreatedEvent(RaydiumRandomPoolCreatedEvent::try_from_slice(payload)?),
            SWAP_SOL_FOR_TOKENS_ON_RAYDIUM_EVENT_EVENT => Self::SwapSolForTokensOnRaydiumEvent(SwapSolForTokensOnRaydiumEvent::try_from_slice(payload)?),
            SWAP_TOKENS_FOR_SOL_ON_RAYDIUM_EVENT_EVENT => Self::SwapTokensForSolOnRaydiumEvent(SwapTokensForSolOnRaydiumEvent::try_from_slice(payload)?),
            TOKEN_BOUGHT_EVENT_EVENT => Self::TokenBoughtEvent(TokenBoughtEvent::try_from_slice(payload)?),
            TOKEN_CREATED_EVENT_EVENT => Self::TokenCreatedEvent(TokenCreatedEvent::try_from_slice(payload)?),
            TOKEN_CREATED_FALLBACK_EVENT_EVENT => Self::TokenCreatedFallbackEvent(TokenCreatedFallbackEvent::try_from_slice(payload)?),
            TOKEN_GRADUATED_EVENT_EVENT => Self::TokenGraduatedEvent(TokenGraduatedEvent::try_from_slice(payload)?),
            TOKEN_SOLD_EVENT_EVENT => Self::TokenSoldEvent(TokenSoldEvent::try_from_slice(payload)?),
            TRADING_FEES_COLLECTED_EVENT_EVENT => Self::TradingFeesCollectedEvent(TradingFeesCollectedEvent::try_from_slice(payload)?),
            TRADING_FEES_COLLECTED_V2_EVENT_EVENT => Self::TradingFeesCollectedV2Event(TradingFeesCollectedV2Event::try_from_slice(payload)?),
            TRADING_FEES_SPLIT_EVENT_EVENT => Self::TradingFeesSplitEvent(TradingFeesSplitEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack_event(data: &[u8]) -> Result<BoopEvent, ParseError> {
    BoopEvent::try_from(data)
}
