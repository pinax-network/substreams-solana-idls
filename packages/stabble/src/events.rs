//! Stabble swap events.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const POOL_BALANCE_UPDATED_EVENT: [u8; 8] = [172, 82, 114, 207, 27, 103, 211, 4];
pub const POOL_UPDATED_EVENT: [u8; 8] = [128, 39, 94, 221, 230, 222, 127, 141];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StabbleEvent {
    PoolBalanceUpdatedEvent(PoolBalanceUpdatedEvent),
    PoolUpdatedEvent(PoolUpdatedEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolBalanceUpdatedEvent {
    pub pubkey: Pubkey,
    pub data: PoolBalanceUpdatedData,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolBalanceUpdatedData {
    pub balances: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolUpdatedEvent {
    pub pubkey: Pubkey,
    pub data: PoolUpdatedData,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolUpdatedData {
    pub is_active: bool,
    pub swap_fee: u64,
    pub max_supply: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for StabbleEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            POOL_BALANCE_UPDATED_EVENT => Self::PoolBalanceUpdatedEvent(PoolBalanceUpdatedEvent::try_from_slice(payload)?),
            POOL_UPDATED_EVENT => Self::PoolUpdatedEvent(PoolUpdatedEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<StabbleEvent, ParseError> {
    StabbleEvent::try_from(data)
}
