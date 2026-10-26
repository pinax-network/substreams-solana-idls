//! Phonenix trading instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const INITIALIZE_MARKET: u8 = 100;
pub const SWAP: u8 = 0;
pub const SWAP_WITH_FREE_FUNDS: u8 = 1;

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhonenixInstruction {
    Swap(SwapInstruction),
    SwapWithFreeFunds(SwapWithFreeFundsInstruction),
    Unknown,
    InitializeMarket(InitializeMarketInstruction),
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    /// Raw order packet payload
    pub order_packet: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapWithFreeFundsInstruction {
    /// Raw order packet payload
    pub order_packet: Vec<u8>,
}

/// Exact InitializeParams layout from the Phoenix Shank IDL.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeMarketInstruction {
    pub market_size_params: MarketSizeParams,
    pub num_quote_lots_per_quote_unit: u64,
    pub tick_size_in_quote_lots_per_base_unit: u64,
    pub num_base_lots_per_base_unit: u64,
    pub taker_fee_bps: u16,
    pub fee_collector: Pubkey,
    pub raw_base_units_per_base_unit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MarketSizeParams {
    pub bids_size: u64,
    pub asks_size: u64,
    pub num_seats: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PhonenixInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(1);
        let discriminator = disc[0];
        Ok(match discriminator {
            INITIALIZE_MARKET => Self::InitializeMarket(InitializeMarketInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction {
                order_packet: payload.to_vec(),
            }),
            SWAP_WITH_FREE_FUNDS => Self::SwapWithFreeFunds(SwapWithFreeFundsInstruction {
                order_packet: payload.to_vec(),
            }),
            other => return Err(ParseError::Unknown([other, 0, 0, 0, 0, 0, 0, 0])),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PhonenixInstruction, ParseError> {
    PhonenixInstruction::try_from(data)
}
