//! on-chain **events** and their Borsh-deserialisation helpers.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the emitted log’s data)
// -----------------------------------------------------------------------------
pub const CREATE: [u8; 8] = [27, 114, 169, 77, 222, 235, 99, 118];
pub const COMPLETE: [u8; 8] = [95, 114, 97, 156, 212, 46, 152, 8];
pub const SET_PARAMS: [u8; 8] = [223, 195, 159, 246, 62, 48, 143, 131];
pub const TRADE: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
pub const TRADE_LEN_V0: usize = 121 - 16;
pub const TRADE_LEN_V1: usize = 137 - 16;
pub const TRADE_LEN_V2: usize = 233 - 16;
pub const TRADE_LEN_V3: usize = 266 - 16;

// -----------------------------------------------------------------------------
// High-level event enum (concise; rich docs live in each struct)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PumpFunEvent {
    /// Pool created. See [`CreateEvent`].
    Create(CreateEvent),

    /// Trade executed (buy or sell). See [`TradeEvent`].
    TradeV0(TradeEventV0),
    TradeV1(TradeEventV1),
    TradeV2(TradeEventV2),
    TradeV3(TradeEventV3),

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
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateEvent {
    /// Name of the pool, e.g., "Pump.fun".
    pub name: String,

    /// Symbol for the pool, e.g., "PUMP".
    pub symbol: String,
    /// URI to the pool metadata (e.g., JSON file).
    /// This is not a Solana URI, but a generic URL.
    /// It can point to any location, such as IPFS or a web server.
    /// The URI should be a valid UTF-8 string.
    /// It is recommended to use a content-addressed storage solution like IPFS.
    /// Example: `https://ipfs.io/ipfs/bafkreidp5sbto4mvutr6tcdkq5tv2b5zp3orzpbddmvz2bxbyttc3kii2m`
    ///
    /// Note: The URI is not validated for correctness, but it should be a valid URL.
    /// It is the responsibility of the creator to ensure that the URI points to a valid resource.
    /// If the URI is invalid or points to a non-existent resource, it may lead to issues when users
    /// try to access the pool metadata.
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
    /// Virtual token reserves **after** creation.
    pub virtual_token_reserves: u64,
    /// Virtual SOL reserves **after** creation.
    pub virtual_sol_reserves: u64,
    /// Real token balance in the vault.
    pub real_token_reserves: u64,
    /// Total token supply at the time of creation.
    pub token_total_supply: u64,
}

/// Emitted on every buy or sell.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TradeEventV0 {
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
}

/// Emitted on every buy or sell.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
///
/// https://github.com/pump-fun/pump-public-docs
/// On every trade the original creator of the coin receives 0.05 % of all trade fees.
/// This is applicable for all coins that were present on the bonding curve or PumpSwap from the date of May 13 2025.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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

/// Emitted on every buy or sell.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TradeEventV3 {
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
    /// Whether volume tracking is enabled for this pool.
    pub track_volume: bool,
    /// Total unclaimed tokens at the time of the trade.
    pub total_unclaimed_tokens: u64,
    /// Total claimed tokens at the time of the trade.
    pub total_claimed_tokens: u64,
    /// Current SOL volume at the time of the trade.
    pub current_sol_volume: u64,
    /// Last timestamp when volume was updated.
    pub last_update_timestamp: i64,
}

/// Emitted whenever pool parameters change.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
            // 8 bytes discriminator + 8 bytes Anchor discriminator
            return Err(ParseError::TooShort(data.len()));
        }

        let _disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
        let anchor_disc: [u8; 8] = data[8..16].try_into().expect("slice len 8");
        let payload = &data[16..]; // skip both discriminators

        Ok(match anchor_disc {
            CREATE => Self::Create(CreateEvent::try_from_slice(payload)?),
            COMPLETE => Self::Complete(CompleteEvent::try_from_slice(payload)?),
            SET_PARAMS => Self::SetParams(SetParamsEvent::try_from_slice(payload)?),
            TRADE => match payload.len() {
                TRADE_LEN_V0 => Self::TradeV0(TradeEventV0::try_from_slice(payload)?),
                TRADE_LEN_V1 => Self::TradeV1(TradeEventV1::try_from_slice(payload)?),
                TRADE_LEN_V2 => Self::TradeV2(TradeEventV2::try_from_slice(payload)?),
                TRADE_LEN_V3 => Self::TradeV3(TradeEventV3::try_from_slice(payload)?),
                other => {
                    return Err(ParseError::InvalidLength {
                        expected: *[TRADE_LEN_V0, TRADE_LEN_V1, TRADE_LEN_V2, TRADE_LEN_V3]
                            .iter()
                            .max()
                            .unwrap(),
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
