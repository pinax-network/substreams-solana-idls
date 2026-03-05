//! Moonshot on-chain instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators  (sha256("global:<snake_case>")[..8])
// -----------------------------------------------------------------------------
pub const TOKEN_MINT: [u8; 8] = [3, 44, 164, 184, 123, 13, 245, 179];
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const MIGRATE_FUNDS: [u8; 8] = [42, 229, 10, 231, 189, 62, 193, 174];
pub const CONFIG_INIT: [u8; 8] = [13, 236, 164, 173, 106, 253, 164, 185];
pub const CONFIG_UPDATE: [u8; 8] = [80, 37, 109, 136, 82, 135, 89, 241];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoonshotInstruction {
    TokenMint(TokenMintInstruction),
    Buy(BuyInstruction),
    Sell(SellInstruction),
    MigrateFunds,
    ConfigInit(ConfigInitInstruction),
    ConfigUpdate(ConfigUpdateInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TokenMintInstruction {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub collateral_currency: u8,
    pub amount: u64,
    pub curve_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyInstruction {
    pub amount: u64,
    pub collateral_amount: u64,
    pub slippage_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellInstruction {
    pub amount: u64,
    pub collateral_amount: u64,
    pub slippage_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ConfigInitInstruction {
    pub migration_authority: Option<Pubkey>,
    pub backend_authority: Option<Pubkey>,
    pub config_authority: Option<Pubkey>,
    pub helio_fee: Option<Pubkey>,
    pub dex_fee: Option<Pubkey>,
    pub fee_bps: Option<u16>,
    pub dex_fee_share: Option<u8>,
    pub migration_fee: Option<u64>,
    pub marketcap_threshold: Option<u64>,
    pub marketcap_currency: Option<u8>,
    pub min_supported_decimal_places: Option<u8>,
    pub max_supported_decimal_places: Option<u8>,
    pub min_supported_token_supply: Option<u64>,
    pub max_supported_token_supply: Option<u64>,
    pub coef_b: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ConfigUpdateInstruction {
    pub migration_authority: Option<Pubkey>,
    pub backend_authority: Option<Pubkey>,
    pub config_authority: Option<Pubkey>,
    pub helio_fee: Option<Pubkey>,
    pub dex_fee: Option<Pubkey>,
    pub fee_bps: Option<u16>,
    pub dex_fee_share: Option<u8>,
    pub migration_fee: Option<u64>,
    pub marketcap_threshold: Option<u64>,
    pub marketcap_currency: Option<u8>,
    pub min_supported_decimal_places: Option<u8>,
    pub max_supported_decimal_places: Option<u8>,
    pub min_supported_token_supply: Option<u64>,
    pub max_supported_token_supply: Option<u64>,
    pub coef_b: Option<u32>,
}

// -----------------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for MoonshotInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            TOKEN_MINT => Self::TokenMint(TokenMintInstruction::try_from_slice(payload)?),
            BUY => Self::Buy(BuyInstruction::try_from_slice(payload)?),
            SELL => Self::Sell(SellInstruction::try_from_slice(payload)?),
            MIGRATE_FUNDS => Self::MigrateFunds,
            CONFIG_INIT => Self::ConfigInit(ConfigInitInstruction::try_from_slice(payload)?),
            CONFIG_UPDATE => Self::ConfigUpdate(ConfigUpdateInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<MoonshotInstruction, ParseError> {
    MoonshotInstruction::try_from(data)
}
