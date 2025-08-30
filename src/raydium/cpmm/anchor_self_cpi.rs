//! Raydium CPMM events emitted via `emit!`.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const SWAP_EVENT: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
pub const LP_CHANGE_EVENT: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumCpmmEvent {
    SwapEvent(SwapEvent),
    LpChangeEvent(LpChangeEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapEvent {
    pub pool_id: Pubkey,
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub trade_fee: u64,
    pub creator_fee: u64,
    pub creator_fee_on_input: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct LpChangeEvent {
    pub pool_id: Pubkey,
    pub lp_amount_before: u64,
    pub token_0_vault_before: u64,
    pub token_1_vault_before: u64,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u32,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumCpmmEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[8..];

        Ok(match disc {
            SWAP_EVENT => Self::SwapEvent(SwapEvent::try_from_slice(payload)?),
            LP_CHANGE_EVENT => Self::LpChangeEvent(LpChangeEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumCpmmEvent, ParseError> {
    RaydiumCpmmEvent::try_from(data)
}
