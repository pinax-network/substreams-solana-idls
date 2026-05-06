//! on-chain **events** and their Borsh-deserialisation helpers.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
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
                        expected: *[TRADE_LEN_V0, TRADE_LEN_V1, TRADE_LEN_V2, TRADE_LEN_V3].iter().max().unwrap(),
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

// -----------------------------------------------------------------------------
// Permissive trade-event decoder
// -----------------------------------------------------------------------------

/// Minimal trade-event view used by downstream sinks.
///
/// Only includes the leading-layout fields that have remained at fixed byte
/// offsets across every known on-chain schema bump (V0 → V1 → V2 → V3 and the
/// post-2025-10-17 variant that introduced `ix_name` / cashback / extra volume
/// fields). Everything after `user` is intentionally ignored so the decoder
/// stays correct as the program continues to append fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradeEventMinimal {
    /// SPL mint of the bonding-curve token.
    pub mint: [u8; 32],
    /// Lamports moved on the SOL side.
    pub sol_amount: u64,
    /// Token amount moved on the SPL side.
    pub token_amount: u64,
    /// `true` for buys (SOL → SPL), `false` for sells.
    pub is_buy: bool,
    /// Trader wallet (Pubkey).
    pub user: [u8; 32],
}

/// Anchor self-CPI envelope tag prepended by `emit_cpi!` to event-carrying
/// inner-instruction data: `[ANCHOR_SELF_CPI_TAG][event_disc][payload]`.
const ANCHOR_SELF_CPI_TAG: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];

// Stable byte offsets within the TradeEvent payload (after the 16-byte
// preamble has been stripped). Layout:
//   [mint: pubkey (32B)] [sol_amount: u64] [token_amount: u64]
//   [is_buy: bool] [user: pubkey (32B)] ...
const MINT_OFFSET: usize = 0;
const SOL_AMOUNT_OFFSET: usize = 32;
const TOKEN_AMOUNT_OFFSET: usize = 40;
const IS_BUY_OFFSET: usize = 48;
const USER_OFFSET: usize = 49;
const TRADE_PAYLOAD_MIN_LEN: usize = USER_OFFSET + 32; // 81 bytes

/// Decode a pump.fun bonding-curve TradeEvent into the stable subset of
/// fields that downstream sinks need (mint, sol/token amount, direction,
/// user), reading at fixed byte offsets and ignoring whatever follows.
///
/// Tolerates append-only schema growth: works for V0..V3 and any variant
/// that appends fields after the leading layout.
///
/// Expects the inner-instruction data of an `emit_cpi!`-style event:
/// `[ANCHOR_SELF_CPI_TAG (8B)][TRADE (8B)][payload]`.
///
/// Returns `Ok(None)` for envelope-shaped data with a non-trade discriminator
/// (e.g. Create, Complete, SetParams) so callers can ignore those without
/// treating them as errors.
pub fn unpack_trade_event_minimal(data: &[u8]) -> Result<Option<TradeEventMinimal>, ParseError> {
    if data.len() < 16 {
        return Err(ParseError::TooShort(data.len()));
    }
    if data[..8] != ANCHOR_SELF_CPI_TAG {
        return Err(ParseError::Unknown(data[..8].try_into().expect("len 8")));
    }
    let event_disc: [u8; 8] = data[8..16].try_into().expect("len 8");
    if event_disc != TRADE {
        return Ok(None);
    }
    let payload = &data[16..];
    if payload.len() < TRADE_PAYLOAD_MIN_LEN {
        return Err(ParseError::TooShort(payload.len()));
    }
    let mint: [u8; 32] = payload[MINT_OFFSET..MINT_OFFSET + 32]
        .try_into()
        .expect("len 32");
    let sol_amount = u64::from_le_bytes(
        payload[SOL_AMOUNT_OFFSET..SOL_AMOUNT_OFFSET + 8]
            .try_into()
            .expect("len 8"),
    );
    let token_amount = u64::from_le_bytes(
        payload[TOKEN_AMOUNT_OFFSET..TOKEN_AMOUNT_OFFSET + 8]
            .try_into()
            .expect("len 8"),
    );
    let is_buy = match payload[IS_BUY_OFFSET] {
        0 => false,
        1 => true,
        _ => {
            return Err(ParseError::InvalidLength {
                expected: 1,
                got: payload[IS_BUY_OFFSET] as usize,
            })
        }
    };
    let user: [u8; 32] = payload[USER_OFFSET..USER_OFFSET + 32]
        .try_into()
        .expect("len 32");
    Ok(Some(TradeEventMinimal {
        mint,
        sol_amount,
        token_amount,
        is_buy,
        user,
    }))
}

#[cfg(test)]
mod minimal_tests {
    use super::*;

    fn make_event(is_buy: bool, mint: [u8; 32], user: [u8; 32], sol: u64, token: u64, trailing: &[u8]) -> Vec<u8> {
        let mut data = Vec::with_capacity(16 + TRADE_PAYLOAD_MIN_LEN + trailing.len());
        data.extend_from_slice(&ANCHOR_SELF_CPI_TAG);
        data.extend_from_slice(&TRADE);
        data.extend_from_slice(&mint); // 32
        data.extend_from_slice(&sol.to_le_bytes()); // 8
        data.extend_from_slice(&token.to_le_bytes()); // 8
        data.push(if is_buy { 1 } else { 0 }); // 1
        data.extend_from_slice(&user); // 32
        data.extend_from_slice(trailing);
        data
    }

    fn pk(b: u8) -> [u8; 32] {
        [b; 32]
    }

    #[test]
    fn buy_with_v0_payload() {
        let data = make_event(true, pk(0xa1), pk(0xa2), 1_000_000_000, 50_000_000, &[]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert!(ev.is_buy);
        assert_eq!(ev.sol_amount, 1_000_000_000);
        assert_eq!(ev.token_amount, 50_000_000);
        assert_eq!(ev.mint, pk(0xa1));
        assert_eq!(ev.user, pk(0xa2));
    }

    #[test]
    fn sell_with_v0_payload() {
        let data = make_event(false, pk(0xb1), pk(0xb2), 7, 13, &[]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert!(!ev.is_buy);
        assert_eq!(ev.sol_amount, 7);
        assert_eq!(ev.token_amount, 13);
    }

    #[test]
    fn ignores_v3_trailing_fields() {
        // V3 - V0 = 250 - 81 = 169 bytes of appended fields.
        let data = make_event(true, pk(0xc1), pk(0xc2), 100, 200, &vec![0u8; 169]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert_eq!(ev.sol_amount, 100);
        assert_eq!(ev.token_amount, 200);
    }

    #[test]
    fn ignores_post_v3_trailing_fields() {
        // The post-2025-10-17 variant adds ix_name (String) + cashback fields.
        let data = make_event(true, pk(0xd1), pk(0xd2), 42, 84, &vec![0u8; 250]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert_eq!(ev.sol_amount, 42);
        assert_eq!(ev.token_amount, 84);
    }

    #[test]
    fn returns_ok_none_for_non_trade_event() {
        let mut data = Vec::new();
        data.extend_from_slice(&ANCHOR_SELF_CPI_TAG);
        data.extend_from_slice(&CREATE);
        data.extend_from_slice(&[0u8; TRADE_PAYLOAD_MIN_LEN]);
        assert!(unpack_trade_event_minimal(&data).unwrap().is_none());
    }

    #[test]
    fn errors_on_data_without_anchor_cpi_tag() {
        let mut data = make_event(true, pk(0), pk(0), 1, 1, &[]);
        data[..8].fill(0);
        assert!(matches!(
            unpack_trade_event_minimal(&data),
            Err(ParseError::Unknown(_))
        ));
    }

    #[test]
    fn errors_on_short_payload() {
        let mut data = Vec::new();
        data.extend_from_slice(&ANCHOR_SELF_CPI_TAG);
        data.extend_from_slice(&TRADE);
        data.extend_from_slice(&[0u8; 50]); // < 81-byte minimum
        assert!(matches!(
            unpack_trade_event_minimal(&data),
            Err(ParseError::TooShort(_))
        ));
    }

    #[test]
    fn errors_on_invalid_is_buy_byte() {
        let mut data = make_event(true, pk(0), pk(0), 1, 1, &[]);
        // is_buy lives at offset 16 (preamble) + 48 (within payload) = 64
        data[16 + IS_BUY_OFFSET] = 2;
        assert!(matches!(
            unpack_trade_event_minimal(&data),
            Err(ParseError::InvalidLength { .. })
        ));
    }
}
