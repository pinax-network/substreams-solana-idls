//! Moonshot on-chain events.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// Event discriminators (sha256("event:<Name>")[..8])
pub const TRADE_EVENT: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
pub const MIGRATION_EVENT: [u8; 8] = [255, 202, 76, 147, 91, 231, 73, 22];

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum TradeType {
    Buy,
    Sell,
}

// -----------------------------------------------------------------------------
// Event structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TradeEvent {
    pub amount: u64,
    pub collateral_amount: u64,
    pub dex_fee: u64,
    pub helio_fee: u64,
    pub allocation: u64,
    pub curve: Pubkey,
    pub cost_token: Pubkey,
    pub sender: Pubkey,
    pub trade_type: TradeType,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MigrationEvent {
    pub tokens_migrated: u64,
    pub tokens_burned: u64,
    pub collateral_migrated: u64,
    pub fee: u64,
    pub label: String,
}

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoonshotEvent {
    TradeEvent(TradeEvent),
    MigrationEvent(MigrationEvent),
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for MoonshotEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            TRADE_EVENT => Self::TradeEvent(TradeEvent::try_from_slice(payload)?),
            MIGRATION_EVENT => Self::MigrationEvent(MigrationEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack_event(data: &[u8]) -> Result<MoonshotEvent, ParseError> {
    MoonshotEvent::try_from(data)
}
