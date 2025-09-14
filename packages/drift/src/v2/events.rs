//! Drift v2 on-chain events.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const SPOT_INTEREST_RECORD: [u8; 8] = [183, 186, 203, 186, 225, 187, 95, 130];
pub const SWAP_RECORD: [u8; 8] = [162, 187, 123, 194, 138, 56, 250, 241];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftEvent {
    SpotInterestRecord(SpotInterestRecord),
    SwapRecord(SwapRecord),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SpotInterestRecord {
    pub ts: i64,
    pub market_index: u16,
    pub deposit_balance: u128,
    pub cumulative_deposit_interest: u128,
    pub borrow_balance: u128,
    pub cumulative_borrow_interest: u128,
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapRecord {
    pub ts: i64,
    pub user: Pubkey,
    pub amount_out: u64,
    pub amount_in: u64,
    pub out_market_index: u16,
    pub in_market_index: u16,
    pub out_oracle_price: i64,
    pub in_oracle_price: i64,
    pub fee: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for DriftEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SPOT_INTEREST_RECORD => Self::SpotInterestRecord(SpotInterestRecord::try_from_slice(payload)?),
            SWAP_RECORD => Self::SwapRecord(SwapRecord::try_from_slice(payload)?),
            _ => Self::Unknown,
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<DriftEvent, ParseError> {
    DriftEvent::try_from(data)
}
