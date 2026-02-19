//! Serum DEX V3 instructions.
//!
//! Serum uses a sequential little-endian u32 discriminator (first 4 bytes of
//! instruction data) rather than Anchor's 8-byte sha256 discriminators.

use crate::common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (little-endian u32)
// -----------------------------------------------------------------------------
pub const INITIALIZE_MARKET: [u8; 4] = [0, 0, 0, 0];
pub const NEW_ORDER: [u8; 4] = [1, 0, 0, 0];
pub const MATCH_ORDERS: [u8; 4] = [2, 0, 0, 0];
pub const CONSUME_EVENTS: [u8; 4] = [3, 0, 0, 0];
pub const CANCEL_ORDER: [u8; 4] = [4, 0, 0, 0];
pub const SETTLE_FUNDS: [u8; 4] = [5, 0, 0, 0];
pub const CANCEL_ORDER_BY_CLIENT_ID: [u8; 4] = [6, 0, 0, 0];
pub const DISABLE_MARKET: [u8; 4] = [7, 0, 0, 0];
pub const SWEEP_FEES: [u8; 4] = [8, 0, 0, 0];
pub const NEW_ORDER_V2: [u8; 4] = [9, 0, 0, 0];
pub const NEW_ORDER_V3: [u8; 4] = [10, 0, 0, 0];
pub const CANCEL_ORDER_V2: [u8; 4] = [11, 0, 0, 0];
pub const CANCEL_ORDER_BY_CLIENT_ID_V2: [u8; 4] = [12, 0, 0, 0];
pub const SEND_TAKE: [u8; 4] = [13, 0, 0, 0];
pub const CLOSE_OPEN_ORDERS: [u8; 4] = [14, 0, 0, 0];
pub const INIT_OPEN_ORDERS: [u8; 4] = [15, 0, 0, 0];
pub const PRUNE: [u8; 4] = [16, 0, 0, 0];
pub const CONSUME_EVENTS_PERMISSIONED: [u8; 4] = [17, 0, 0, 0];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerumInstruction {
    InitializeMarket(RawPayload),
    NewOrder(RawPayload),
    MatchOrders(RawPayload),
    ConsumeEvents(RawPayload),
    CancelOrder(RawPayload),
    SettleFunds(RawPayload),
    CancelOrderByClientId(RawPayload),
    DisableMarket(RawPayload),
    SweepFees(RawPayload),
    NewOrderV2(RawPayload),
    NewOrderV3(RawPayload),
    CancelOrderV2(RawPayload),
    CancelOrderByClientIdV2(RawPayload),
    SendTake(RawPayload),
    CloseOpenOrders(RawPayload),
    InitOpenOrders(RawPayload),
    Prune(RawPayload),
    ConsumeEventsPermissioned(RawPayload),
}

// -----------------------------------------------------------------------------
// Payload struct
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawPayload {
    pub data: Vec<u8>,
}

// -----------------------------------------------------------------------------
// Deserialisation
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for SerumInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 4 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(4);
        let disc: [u8; 4] = disc.try_into().unwrap();
        let raw = RawPayload { data: payload.to_vec() };
        Ok(match disc {
            INITIALIZE_MARKET => Self::InitializeMarket(raw),
            NEW_ORDER => Self::NewOrder(raw),
            MATCH_ORDERS => Self::MatchOrders(raw),
            CONSUME_EVENTS => Self::ConsumeEvents(raw),
            CANCEL_ORDER => Self::CancelOrder(raw),
            SETTLE_FUNDS => Self::SettleFunds(raw),
            CANCEL_ORDER_BY_CLIENT_ID => Self::CancelOrderByClientId(raw),
            DISABLE_MARKET => Self::DisableMarket(raw),
            SWEEP_FEES => Self::SweepFees(raw),
            NEW_ORDER_V2 => Self::NewOrderV2(raw),
            NEW_ORDER_V3 => Self::NewOrderV3(raw),
            CANCEL_ORDER_V2 => Self::CancelOrderV2(raw),
            CANCEL_ORDER_BY_CLIENT_ID_V2 => Self::CancelOrderByClientIdV2(raw),
            SEND_TAKE => Self::SendTake(raw),
            CLOSE_OPEN_ORDERS => Self::CloseOpenOrders(raw),
            INIT_OPEN_ORDERS => Self::InitOpenOrders(raw),
            PRUNE => Self::Prune(raw),
            CONSUME_EVENTS_PERMISSIONED => Self::ConsumeEventsPermissioned(raw),
            _ => return Err(ParseError::Unknown([disc[0], disc[1], disc[2], disc[3], 0, 0, 0, 0])),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<SerumInstruction, ParseError> {
    SerumInstruction::try_from(data)
}
