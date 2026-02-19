//! Aldrin on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum OrderType {
    Limit,
    ImmediateOrCancel,
    PostOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum SelfTradeBehavior {
    DecrementTake,
    CancelProvide,
    AbortTransaction,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const INITIALIZE_MARKET: [u8; 8] = [35, 35, 189, 193, 155, 48, 170, 203];
pub const NEW_ORDER: [u8; 8] = [153, 0, 116, 34, 241, 46, 40, 139];
pub const MATCH_ORDERS: [u8; 8] = [17, 1, 201, 93, 7, 51, 251, 134];
pub const CONSUME_EVENTS: [u8; 8] = [221, 145, 177, 52, 31, 47, 63, 201];
pub const CANCEL_ORDER: [u8; 8] = [95, 129, 237, 240, 8, 49, 223, 132];
pub const SETTLE_FUNDS: [u8; 8] = [238, 64, 163, 96, 75, 171, 16, 33];
pub const CANCEL_ORDER_BY_CLIENT_ID: [u8; 8] = [223, 248, 134, 155, 106, 111, 152, 171];
pub const DISABLE_MARKET: [u8; 8] = [118, 212, 158, 116, 48, 136, 131, 43];
pub const SWEEP_FEES: [u8; 8] = [175, 225, 98, 71, 118, 66, 34, 148];
pub const NEW_ORDER_V2: [u8; 8] = [113, 151, 157, 25, 61, 181, 5, 246];
pub const NEW_ORDER_V3: [u8; 8] = [70, 208, 211, 251, 217, 236, 66, 174];
pub const CANCEL_ORDER_V2: [u8; 8] = [110, 83, 130, 136, 146, 136, 66, 67];
pub const CANCEL_ORDER_BY_CLIENT_ID_V2: [u8; 8] = [233, 134, 190, 180, 125, 59, 19, 171];
pub const SEND_TAKE: [u8; 8] = [248, 142, 177, 108, 46, 157, 229, 83];
pub const CLOSE_OPEN_ORDERS: [u8; 8] = [200, 216, 63, 239, 7, 230, 255, 20];
pub const INIT_OPEN_ORDERS: [u8; 8] = [230, 167, 76, 177, 168, 44, 155, 13];
pub const PRUNE: [u8; 8] = [136, 19, 42, 184, 152, 204, 244, 94];
pub const CONSUME_EVENTS_PERMISSIONED: [u8; 8] = [133, 69, 70, 199, 52, 0, 39, 191];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum AldrinInstruction {
    InitializeMarket(InitializeMarketInstruction),
    NewOrder(NewOrderInstructionV1),
    MatchOrders(MatchOrdersInstruction),
    ConsumeEvents(ConsumeEventsInstruction),
    CancelOrder(CancelOrderInstructionV2),
    SettleFunds,
    CancelOrderByClientId(CancelOrderByClientIdInstruction),
    DisableMarket,
    SweepFees,
    NewOrderV2(NewOrderInstructionV2),
    NewOrderV3(NewOrderInstructionV3),
    CancelOrderV2(CancelOrderInstructionV2),
    CancelOrderByClientIdV2(CancelOrderByClientIdV2Instruction),
    SendTake(SendTakeInstruction),
    CloseOpenOrders,
    InitOpenOrders,
    Prune(PruneInstruction),
    ConsumeEventsPermissioned(ConsumeEventsPermissionedInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct InitializeMarketInstruction {
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub fee_rate_bps: u16,
    pub vault_signer_nonce: u64,
    pub pc_dust_threshold: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct NewOrderInstructionV1 {
    pub side: Side,
    pub limit_price: u64,
    pub max_qty: u64,
    pub order_type: OrderType,
    pub client_id: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CancelOrderInstructionV2 {
    pub side: Side,
    pub order_id: u128,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct NewOrderInstructionV2 {
    pub side: Side,
    pub limit_price: u64,
    pub max_qty: u64,
    pub order_type: OrderType,
    pub client_id: u64,
    pub self_trade_behavior: SelfTradeBehavior,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct NewOrderInstructionV3 {
    pub side: Side,
    pub limit_price: u64,
    pub max_coin_qty: u64,
    pub max_native_pc_qty_including_fees: u64,
    pub self_trade_behavior: SelfTradeBehavior,
    pub order_type: OrderType,
    pub client_order_id: u64,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SendTakeInstruction {
    pub side: Side,
    pub limit_price: u64,
    pub max_coin_qty: u64,
    pub max_native_pc_qty_including_fees: u64,
    pub min_coin_qty: u64,
    pub min_native_pc_qty: u64,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MatchOrdersInstruction {
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConsumeEventsInstruction {
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CancelOrderByClientIdInstruction {
    pub client_order_id: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CancelOrderByClientIdV2Instruction {
    pub client_order_id: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PruneInstruction {
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConsumeEventsPermissionedInstruction {
    pub limit: u16,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for AldrinInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            INITIALIZE_MARKET => Self::InitializeMarket(InitializeMarketInstruction::try_from_slice(payload)?),
            NEW_ORDER => Self::NewOrder(NewOrderInstructionV1::try_from_slice(payload)?),
            MATCH_ORDERS => Self::MatchOrders(MatchOrdersInstruction::try_from_slice(payload)?),
            CONSUME_EVENTS => Self::ConsumeEvents(ConsumeEventsInstruction::try_from_slice(payload)?),
            CANCEL_ORDER => Self::CancelOrder(CancelOrderInstructionV2::try_from_slice(payload)?),
            SETTLE_FUNDS => Self::SettleFunds,
            CANCEL_ORDER_BY_CLIENT_ID => Self::CancelOrderByClientId(CancelOrderByClientIdInstruction::try_from_slice(payload)?),
            DISABLE_MARKET => Self::DisableMarket,
            SWEEP_FEES => Self::SweepFees,
            NEW_ORDER_V2 => Self::NewOrderV2(NewOrderInstructionV2::try_from_slice(payload)?),
            NEW_ORDER_V3 => Self::NewOrderV3(NewOrderInstructionV3::try_from_slice(payload)?),
            CANCEL_ORDER_V2 => Self::CancelOrderV2(CancelOrderInstructionV2::try_from_slice(payload)?),
            CANCEL_ORDER_BY_CLIENT_ID_V2 => Self::CancelOrderByClientIdV2(CancelOrderByClientIdV2Instruction::try_from_slice(payload)?),
            SEND_TAKE => Self::SendTake(SendTakeInstruction::try_from_slice(payload)?),
            CLOSE_OPEN_ORDERS => Self::CloseOpenOrders,
            INIT_OPEN_ORDERS => Self::InitOpenOrders,
            PRUNE => Self::Prune(PruneInstruction::try_from_slice(payload)?),
            CONSUME_EVENTS_PERMISSIONED => Self::ConsumeEventsPermissioned(ConsumeEventsPermissionedInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<AldrinInstruction, ParseError> {
    AldrinInstruction::try_from(data)
}
