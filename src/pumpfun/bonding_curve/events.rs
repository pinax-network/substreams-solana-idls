//! on-chain **events** and their Borsh-deserialisation helpers.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the emitted log’s data)
// -----------------------------------------------------------------------------
const CREATE: [u8; 8] = [27, 114, 169, 77, 222, 235, 99, 118];
const COMPLETE: [u8; 8] = [95, 114, 97, 156, 212, 46, 152, 8];
const SET_PARAMS: [u8; 8] = [223, 195, 159, 246, 62, 48, 143, 131];
const TRADE: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
const TRADE_LEN_V1: usize = 121;
const TRADE_LEN_V2: usize = 217;

// -----------------------------------------------------------------------------
// High-level event enum (concise; rich docs live in each struct)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PumpFunEvent {
    /// Pool created. See [`CreateEvent`].
    Create(CreateEvent),

    /// Trade executed (buy or sell). See [`TradeEvent`].
    TradeV1(TradeEventV1),
    TradeV2(TradeEventV2),

    /// Pool completed / closed. See [`CompleteEvent`].
    Complete(CompleteEvent),

    /// Pool parameters updated. See [`SetParamsEvent`].
    SetParams(SetParamsEvent),

    /// Discriminator did not match any known event.
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs (inline field comments instead of tables)
// -----------------------------------------------------------------------------

/// Emitted once when a new bonding-curve pool is created.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    /// SPL-Token mint address for the pool.
    pub mint: Pubkey,
    /// PDA of the curve configuration account.
    pub bonding_curve: Pubkey,
    /// Wallet that paid the creation fee.
    pub user: Pubkey,
    /// Wallet that will earn creator fees.
    pub creator: Pubkey,
    /// Unix-epoch seconds when the pool was created.
    pub timestamp: i64,
    /// Virtual SOL reserves **after** creation.
    pub virtual_sol_reserves: u64,
    /// Virtual token reserves **after** creation.
    pub virtual_token_reserves: u64,
    /// Real SOL balance in the vault.
    pub real_sol_reserves: u64,
    /// Real token balance in the vault.
    pub real_token_reserves: u64,
}

/// Emitted on every buy or sell.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TradeEventV1 {
    pub mint: Pubkey,
    /// Lamports moved (positive on buys, negative on sells).
    pub sol_amount: u64,
    /// Token amount moved (positive on buys, negative on sells).
    pub token_amount: u64,
    /// `true` = buy (SOL→SPL), `false` = sell.
    pub is_buy: bool,
    /// Trader’s wallet.
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
}

/// Emitted on every buy or sell.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TradeEventV2 {
    pub mint: Pubkey,
    /// Lamports moved (positive on buys, negative on sells).
    pub sol_amount: u64,
    /// Token amount moved (positive on buys, negative on sells).
    pub token_amount: u64,
    /// `true` = buy (SOL→SPL), `false` = sell.
    pub is_buy: bool,
    /// Trader’s wallet.
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    /// Protocol-fee recipient at the time of the trade.
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u64,
    /// Protocol fee paid (lamports).
    pub fee: u64,
    /// Pool creator wallet.
    pub creator: Pubkey,
    pub creator_fee_basis_points: u64,
    /// Creator fee paid (lamports).
    pub creator_fee: u64,
}

/// Emitted whenever pool parameters change.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetParamsEvent {
    /// New protocol-fee recipient.
    pub fee_recipient: Pubkey,
    /// Updated virtual token reserve.
    pub initial_virtual_token_reserves: u64,
    /// Updated virtual SOL reserve.
    pub initial_virtual_sol_reserves: u64,
    /// Real token balance at creation (constant reference).
    pub initial_real_token_reserves: u64,
    /// Total token supply at the time of the update.
    pub token_total_supply: u64,
    /// New protocol fee (basis points).
    pub fee_basis_points: u64,
}

/// Emitted when a pool is closed / liquidity exhausted.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CompleteEvent {
    /// Wallet that triggered completion (last trade).
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PumpFunEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 16 {
            // 8 bytes Pump.fun discriminator + 8 bytes Anchor discriminator
            return Err(ParseError::TooShort(data.len()));
        }

        let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let payload = &data[16..]; // skip both discriminators

        Ok(match disc {
            CREATE => Self::Create(CreateEvent::try_from_slice(payload)?),
            COMPLETE => Self::Complete(CompleteEvent::try_from_slice(payload)?),
            SET_PARAMS => Self::SetParams(SetParamsEvent::try_from_slice(payload)?),
            TRADE => match payload.len() {
                TRADE_LEN_V1 => Self::TradeV1(TradeEventV1::try_from_slice(payload)?),
                TRADE_LEN_V2 => Self::TradeV2(TradeEventV2::try_from_slice(payload)?),
                other => {
                    return Err(ParseError::InvalidLength {
                        expected: TRADE_LEN_V1.max(TRADE_LEN_V2),
                        got: other,
                    })
                }
            },
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PumpFunEvent, ParseError> {
    PumpFunEvent::try_from(data)
}
