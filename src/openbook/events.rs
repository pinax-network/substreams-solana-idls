//! Openbook V2 on-chain events.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (Anchor: sha256("event:<EventName>")[..8])
// -----------------------------------------------------------------------------
pub const DEPOSIT_LOG: [u8; 8] = [141, 186, 168, 252, 108, 141, 72, 94];
pub const FILL_LOG: [u8; 8] = [150, 23, 41, 148, 152, 162, 215, 64];
pub const MARKET_META_DATA_LOG: [u8; 8] = [209, 87, 212, 236, 164, 58, 60, 117];
pub const TOTAL_ORDER_FILL_EVENT: [u8; 8] = [8, 235, 48, 58, 174, 76, 156, 105];
pub const SET_DELEGATE_LOG: [u8; 8] = [53, 130, 151, 92, 109, 57, 145, 112];
pub const SETTLE_FUNDS_LOG: [u8; 8] = [10, 50, 240, 117, 237, 67, 230, 233];
pub const SWEEP_FEES_LOG: [u8; 8] = [210, 242, 26, 77, 94, 48, 255, 61];
pub const OPEN_ORDERS_POSITION_LOG: [u8; 8] = [196, 249, 148, 33, 168, 228, 73, 6];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum OpenbookEvent {
    DepositLog(DepositLogEvent),
    FillLog(FillLogEvent),
    MarketMetaDataLog(MarketMetaDataLogEvent),
    TotalOrderFillEvent(TotalOrderFillEventData),
    SetDelegateLog(SetDelegateLogEvent),
    SettleFundsLog(SettleFundsLogEvent),
    SweepFeesLog(SweepFeesLogEvent),
    OpenOrdersPositionLog(OpenOrdersPositionLogEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Event structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositLogEvent {
    pub open_orders_account: Pubkey,
    pub signer: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FillLogEvent {
    pub market: Pubkey,
    pub taker_side: u8,
    pub maker_slot: u8,
    pub maker_out: bool,
    pub timestamp: u64,
    pub seq_num: u64,
    pub maker: Pubkey,
    pub maker_client_order_id: u64,
    pub maker_fee: u64,
    pub maker_timestamp: u64,
    pub taker: Pubkey,
    pub taker_client_order_id: u64,
    pub taker_fee_ceil: u64,
    pub price: i64,
    pub quantity: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MarketMetaDataLogEvent {
    pub market: Pubkey,
    pub name: String,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub base_lot_size: i64,
    pub quote_lot_size: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TotalOrderFillEventData {
    pub side: u8,
    pub taker: Pubkey,
    pub total_quantity_paid: u64,
    pub total_quantity_received: u64,
    pub fees: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetDelegateLogEvent {
    pub open_orders_account: Pubkey,
    pub delegate: Option<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SettleFundsLogEvent {
    pub open_orders_account: Pubkey,
    pub base_native: u64,
    pub quote_native: u64,
    pub referrer_rebate: u64,
    pub referrer: Option<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SweepFeesLogEvent {
    pub market: Pubkey,
    pub amount: u64,
    pub receiver: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OpenOrdersPositionLogEvent {
    pub owner: Pubkey,
    pub open_orders_account_num: u32,
    pub market: Pubkey,
    pub bids_base_lots: i64,
    pub bids_quote_lots: i64,
    pub asks_base_lots: i64,
    pub base_free_native: u64,
    pub quote_free_native: u64,
    pub locked_maker_fees: u64,
    pub referrer_rebates_available: u64,
    pub maker_volume: u128,
    pub taker_volume: u128,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for OpenbookEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            DEPOSIT_LOG => Self::DepositLog(DepositLogEvent::try_from_slice(payload)?),
            FILL_LOG => Self::FillLog(FillLogEvent::try_from_slice(payload)?),
            MARKET_META_DATA_LOG => Self::MarketMetaDataLog(MarketMetaDataLogEvent::try_from_slice(payload)?),
            TOTAL_ORDER_FILL_EVENT => Self::TotalOrderFillEvent(TotalOrderFillEventData::try_from_slice(payload)?),
            SET_DELEGATE_LOG => Self::SetDelegateLog(SetDelegateLogEvent::try_from_slice(payload)?),
            SETTLE_FUNDS_LOG => Self::SettleFundsLog(SettleFundsLogEvent::try_from_slice(payload)?),
            SWEEP_FEES_LOG => Self::SweepFeesLog(SweepFeesLogEvent::try_from_slice(payload)?),
            OPEN_ORDERS_POSITION_LOG => Self::OpenOrdersPositionLog(OpenOrdersPositionLogEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<OpenbookEvent, ParseError> {
    OpenbookEvent::try_from(data)
}
