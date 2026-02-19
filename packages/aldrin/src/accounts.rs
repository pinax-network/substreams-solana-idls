//! Aldrin on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Account discriminators
pub const MARKET_STATE_V2_ACCOUNT: [u8; 8] = [181, 11, 35, 91, 85, 209, 1, 51];
pub const MARKET_STATE_ACCOUNT: [u8; 8] = [0, 125, 123, 215, 95, 96, 164, 194];
pub const OPEN_ORDERS_ACCOUNT: [u8; 8] = [139, 166, 123, 206, 111, 2, 116, 33];
pub const REQUEST_QUEUE_HEADER_ACCOUNT: [u8; 8] = [28, 165, 120, 78, 225, 191, 157, 182];
pub const REQUEST_ACCOUNT: [u8; 8] = [125, 172, 150, 161, 162, 115, 39, 71];
pub const REQUEST_VIEW_ACCOUNT: [u8; 8] = [206, 80, 233, 109, 31, 54, 182, 125];
pub const EVENT_QUEUE_HEADER_ACCOUNT: [u8; 8] = [63, 228, 117, 106, 79, 9, 172, 71];
pub const EVENT_ACCOUNT: [u8; 8] = [125, 192, 125, 158, 9, 115, 152, 233];
pub const EVENT_VIEW_ACCOUNT: [u8; 8] = [207, 149, 151, 29, 47, 44, 215, 175];

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MarketStateV2 {
    pub inner: MarketState,
    pub open_orders_authority: Pubkey,
    pub prune_authority: Pubkey,
    pub consume_events_authority: Pubkey,
    pub padding: [u8; 992],
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MarketState {
    pub account_flags: u64,
    pub own_address: [u64; 4],
    pub vault_signer_nonce: u64,
    pub coin_mint: [u64; 4],
    pub pc_mint: [u64; 4],
    pub coin_vault: [u64; 4],
    pub coin_deposits_total: u64,
    pub coin_fees_accrued: u64,
    pub pc_vault: [u64; 4],
    pub pc_deposits_total: u64,
    pub pc_fees_accrued: u64,
    pub pc_dust_threshold: u64,
    pub req_q: [u64; 4],
    pub event_q: [u64; 4],
    pub bids: [u64; 4],
    pub asks: [u64; 4],
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub fee_rate_bps: u64,
    pub referrer_rebates_accrued: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OpenOrders {
    pub account_flags: u64,
    pub market: [u64; 4],
    pub owner: [u64; 4],
    pub native_coin_free: u64,
    pub native_coin_total: u64,
    pub native_pc_free: u64,
    pub native_pc_total: u64,
    pub free_slot_bits: u128,
    pub is_bid_bits: u128,
    pub orders: [u128; 128],
    pub client_order_ids: [u64; 128],
    pub referrer_rebates_accrued: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RequestQueueHeader {
    pub account_flags: u64,
    pub head: u64,
    pub count: u64,
    pub next_seq_num: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Request {
    pub request_flags: u8,
    pub owner_slot: u8,
    pub fee_tier: u8,
    pub self_trade_behavior: u8,
    pub padding: [u8; 4],
    pub max_coin_qty_or_cancel_id: u64,
    pub native_pc_qty_locked: u64,
    pub order_id: u128,
    pub owner: [u64; 4],
    pub client_order_id: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct EventQueueHeader {
    pub account_flags: u64,
    pub head: u64,
    pub count: u64,
    pub seq_num: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Event {
    pub event_flags: u8,
    pub owner_slot: u8,
    pub fee_tier: u8,
    pub _padding: [u8; 5],
    pub native_qty_released: u64,
    pub native_qty_paid: u64,
    pub native_fee_or_rebate: u64,
    pub order_id: u128,
    pub owner: [u64; 4],
    pub client_order_id: u64,
}
