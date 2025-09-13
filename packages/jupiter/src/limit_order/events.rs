//! Jupiter Limit Order on-chain **events** and their Borsh-deserialisation helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the emitted log’s data)
// -----------------------------------------------------------------------------
const CANCEL_ORDER_EVENT: [u8; 8] = [174, 66, 141, 17, 4, 224, 162, 77]; // ae428d1104e0a24d
const CREATE_ORDER_EVENT: [u8; 8] = [49, 142, 72, 166, 230, 29, 84, 84]; // 318e48a6e61d5454

// -----------------------------------------------------------------------------
// High-level event enum (concise; rich docs live in each struct)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JupiterLimitOrderEvent {
    /// Cancel order. See [`CancelOrderEvent`].
    CancelOrder(CancelOrderEvent),
    /// Create order. See [`CreateOrderEvent`].
    CreateOrder(CreateOrderEvent),
    /// Discriminator did not match any known event.
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs (inline field comments instead of tables)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CancelOrderEvent {
    pub order_key: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateOrderEvent {
    pub order_key: Pubkey,
    pub maker: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub expired_at: Option<i64>,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for JupiterLimitOrderEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 16 {
            return Err(ParseError::TooShort(data.len()));
        }
        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[16..];
        Ok(match disc {
            CANCEL_ORDER_EVENT => Self::CancelOrder(CancelOrderEvent::try_from_slice(payload)?),
            CREATE_ORDER_EVENT => Self::CreateOrder(CreateOrderEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<JupiterLimitOrderEvent, ParseError> {
    JupiterLimitOrderEvent::try_from(data)
}
