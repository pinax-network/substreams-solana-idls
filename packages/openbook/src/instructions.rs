//! Openbook V2 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (Anchor: sha256("global:<name>")[..8])
// -----------------------------------------------------------------------------
pub const CREATE_MARKET: [u8; 8] = [144, 108, 36, 253, 0, 66, 198, 135];
pub const CLOSE_MARKET: [u8; 8] = [85, 110, 58, 54, 235, 116, 186, 243];
pub const CREATE_OPEN_ORDERS_INDEXER: [u8; 8] = [171, 126, 215, 182, 24, 164, 11, 140];
pub const CLOSE_OPEN_ORDERS_INDEXER: [u8; 8] = [79, 83, 5, 0, 139, 203, 21, 23];
pub const CREATE_OPEN_ORDERS_ACCOUNT: [u8; 8] = [185, 23, 9, 63, 203, 16, 188, 67];
pub const CLOSE_OPEN_ORDERS_ACCOUNT: [u8; 8] = [152, 4, 209, 100, 199, 99, 135, 129];
pub const PLACE_ORDER: [u8; 8] = [192, 5, 135, 164, 64, 29, 23, 31];
pub const EDIT_ORDER: [u8; 8] = [187, 200, 109, 244, 227, 74, 100, 155];
pub const EDIT_ORDER_PEGGED: [u8; 8] = [218, 241, 60, 47, 224, 251, 141, 21];
pub const PLACE_ORDERS: [u8; 8] = [3, 24, 247, 100, 246, 85, 228, 254];
pub const CANCEL_ALL_AND_PLACE_ORDERS: [u8; 8] = [244, 157, 228, 190, 85, 189, 142, 239];
pub const PLACE_ORDER_PEGGED: [u8; 8] = [231, 37, 53, 131, 114, 242, 99, 84];
pub const PLACE_TAKE_ORDER: [u8; 8] = [32, 218, 6, 147, 212, 239, 41, 180];
pub const CONSUME_EVENTS: [u8; 8] = [112, 146, 45, 234, 126, 207, 226, 190];
pub const CONSUME_GIVEN_EVENTS: [u8; 8] = [38, 255, 63, 126, 142, 175, 207, 120];
pub const CANCEL_ORDER: [u8; 8] = [27, 94, 14, 102, 222, 90, 12, 129];
pub const CANCEL_ORDER_BY_CLIENT_ORDER_ID: [u8; 8] = [5, 121, 85, 222, 156, 133, 34, 197];
pub const CANCEL_ALL_ORDERS: [u8; 8] = [26, 137, 116, 143, 50, 139, 137, 112];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const REFILL: [u8; 8] = [128, 207, 142, 11, 54, 232, 38, 201];
pub const SETTLE_FUNDS: [u8; 8] = [144, 202, 213, 95, 205, 207, 31, 253];
pub const SETTLE_FUNDS_EXPIRED: [u8; 8] = [175, 169, 244, 232, 30, 119, 128, 98];
pub const SWEEP_FEES: [u8; 8] = [48, 202, 163, 161, 57, 34, 13, 130];
pub const SET_DELEGATE: [u8; 8] = [29, 112, 86, 251, 3, 147, 189, 253];
pub const SET_MARKET_EXPIRED: [u8; 8] = [133, 229, 242, 104, 144, 114, 92, 140];
pub const PRUNE_ORDERS: [u8; 8] = [174, 95, 2, 216, 23, 93, 193, 10];
pub const STUB_ORACLE_CREATE: [u8; 8] = [179, 92, 120, 24, 71, 213, 37, 65];
pub const STUB_ORACLE_CLOSE: [u8; 8] = [167, 46, 104, 62, 153, 195, 38, 10];
pub const STUB_ORACLE_SET: [u8; 8] = [225, 125, 105, 50, 91, 169, 177, 152];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum OpenbookInstruction {
    CreateMarket,
    CloseMarket,
    CreateOpenOrdersIndexer,
    CloseOpenOrdersIndexer,
    CreateOpenOrdersAccount,
    CloseOpenOrdersAccount,
    PlaceOrder,
    EditOrder,
    EditOrderPegged,
    PlaceOrders,
    CancelAllAndPlaceOrders,
    PlaceOrderPegged,
    PlaceTakeOrder,
    ConsumeEvents,
    ConsumeGivenEvents,
    CancelOrder,
    CancelOrderByClientOrderId,
    CancelAllOrders,
    Deposit(DepositInstruction),
    Refill,
    SettleFunds,
    SettleFundsExpired,
    SweepFees,
    SetDelegate,
    SetMarketExpired,
    PruneOrders,
    StubOracleCreate(StubOracleCreateInstruction),
    StubOracleClose,
    StubOracleSet(StubOracleSetInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub base_amount: u64,
    pub quote_amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StubOracleCreateInstruction {
    pub price: f64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StubOracleSetInstruction {
    pub price: f64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for OpenbookInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            CREATE_MARKET => Self::CreateMarket,
            CLOSE_MARKET => Self::CloseMarket,
            CREATE_OPEN_ORDERS_INDEXER => Self::CreateOpenOrdersIndexer,
            CLOSE_OPEN_ORDERS_INDEXER => Self::CloseOpenOrdersIndexer,
            CREATE_OPEN_ORDERS_ACCOUNT => Self::CreateOpenOrdersAccount,
            CLOSE_OPEN_ORDERS_ACCOUNT => Self::CloseOpenOrdersAccount,
            PLACE_ORDER => Self::PlaceOrder,
            EDIT_ORDER => Self::EditOrder,
            EDIT_ORDER_PEGGED => Self::EditOrderPegged,
            PLACE_ORDERS => Self::PlaceOrders,
            CANCEL_ALL_AND_PLACE_ORDERS => Self::CancelAllAndPlaceOrders,
            PLACE_ORDER_PEGGED => Self::PlaceOrderPegged,
            PLACE_TAKE_ORDER => Self::PlaceTakeOrder,
            CONSUME_EVENTS => Self::ConsumeEvents,
            CONSUME_GIVEN_EVENTS => Self::ConsumeGivenEvents,
            CANCEL_ORDER => Self::CancelOrder,
            CANCEL_ORDER_BY_CLIENT_ORDER_ID => Self::CancelOrderByClientOrderId,
            CANCEL_ALL_ORDERS => Self::CancelAllOrders,
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            REFILL => Self::Refill,
            SETTLE_FUNDS => Self::SettleFunds,
            SETTLE_FUNDS_EXPIRED => Self::SettleFundsExpired,
            SWEEP_FEES => Self::SweepFees,
            SET_DELEGATE => Self::SetDelegate,
            SET_MARKET_EXPIRED => Self::SetMarketExpired,
            PRUNE_ORDERS => Self::PruneOrders,
            STUB_ORACLE_CREATE => Self::StubOracleCreate(StubOracleCreateInstruction::try_from_slice(payload)?),
            STUB_ORACLE_CLOSE => Self::StubOracleClose,
            STUB_ORACLE_SET => Self::StubOracleSet(StubOracleSetInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<OpenbookInstruction, ParseError> {
    OpenbookInstruction::try_from(data)
}
