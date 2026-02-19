//! Tensor Marketplace events.
//!
//! Events are emitted via the `tcomp_noop` CPI instruction with a serialized
//! `TcompEvent` enum as the instruction data (after the 8-byte discriminator).

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use super::instructions::{Field, Target};

// The tcomp_noop discriminator (same as the instruction)
pub const TCOMP_NOOP: [u8; 8] = [106, 162, 10, 226, 132, 68, 223, 21];

// ──────────────────────────────────────────────────────────────────────────────
// Event enum
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TensorEvent {
    Maker(MakeEvent),
    Taker(TakeEvent),
    Unknown,
}

// ──────────────────────────────────────────────────────────────────────────────
// Event structs
// ──────────────────────────────────────────────────────────────────────────────

/// Emitted when a listing or bid is created/edited.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MakeEvent {
    pub maker: Pubkey,
    pub bid_id: Option<Pubkey>,
    pub target: Target,
    /// Whitelist address or asset ID depending on target.
    pub target_id: Pubkey,
    pub field: Option<Field>,
    pub field_id: Option<Pubkey>,
    pub amount: u64,
    pub quantity: u32,
    pub currency: Option<Pubkey>,
    pub expiry: i64,
    pub private_taker: Option<Pubkey>,
    pub asset_id: Option<Pubkey>,
}

/// Emitted when a listing is bought or a bid is taken.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TakeEvent {
    pub taker: Pubkey,
    pub bid_id: Option<Pubkey>,
    pub target: Target,
    /// Whitelist address or asset ID depending on target.
    pub target_id: Pubkey,
    pub field: Option<Field>,
    pub field_id: Option<Pubkey>,
    pub amount: u64,
    pub quantity: u32,
    pub tcomp_fee: u64,
    pub taker_broker_fee: u64,
    pub maker_broker_fee: u64,
    pub creator_fee: u64,
    pub currency: Option<Pubkey>,
    /// The NFT that was traded (useful for collection-wide bids).
    pub asset_id: Option<Pubkey>,
}

// ──────────────────────────────────────────────────────────────────────────────
// Borsh deserialisation
// ──────────────────────────────────────────────────────────────────────────────

// Tensor events are serialized as an enum with a leading byte (0 = Maker, 1 = Taker),
// passed as the data of the tcomp_noop instruction (after the 8-byte discriminator).
impl<'a> TryFrom<&'a [u8]> for TensorEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 9 {
            return Err(ParseError::TooShort(data.len()));
        }

        // Skip 8-byte anchor discriminator
        let payload = &data[8..];

        // First byte is enum variant
        match payload[0] {
            0 => Ok(Self::Maker(MakeEvent::try_from_slice(&payload[1..])?)),
            1 => Ok(Self::Taker(TakeEvent::try_from_slice(&payload[1..])?)),
            _ => Ok(Self::Unknown),
        }
    }
}

pub fn unpack(data: &[u8]) -> Result<TensorEvent, ParseError> {
    TensorEvent::try_from(data)
}
