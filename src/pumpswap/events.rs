//! PumpSwap AMM events.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Event discriminators (from IDL spec 0.1.0)
pub const ADMIN_SET_COIN_CREATOR_EVENT: [u8; 8] = [45, 220, 93, 24, 25, 97, 172, 104];
pub const ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT: [u8; 8] = [147, 250, 108, 120, 247, 29, 67, 222];
pub const BUY_EVENT: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119];
pub const CLAIM_TOKEN_INCENTIVES_EVENT: [u8; 8] = [79, 172, 246, 49, 205, 91, 206, 232];
pub const CLOSE_USER_VOLUME_ACCUMULATOR_EVENT: [u8; 8] = [146, 159, 189, 172, 146, 88, 56, 244];
pub const COLLECT_COIN_CREATOR_FEE_EVENT: [u8; 8] = [232, 245, 194, 238, 234, 218, 58, 89];
pub const CREATE_CONFIG_EVENT: [u8; 8] = [107, 52, 89, 129, 55, 226, 81, 22];
pub const CREATE_POOL_EVENT: [u8; 8] = [177, 49, 12, 210, 160, 118, 167, 116];
pub const DEPOSIT_EVENT: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];
pub const DISABLE_EVENT: [u8; 8] = [107, 253, 193, 76, 228, 202, 27, 104];
pub const EXTEND_ACCOUNT_EVENT: [u8; 8] = [97, 97, 215, 144, 93, 146, 22, 124];
pub const INIT_USER_VOLUME_ACCUMULATOR_EVENT: [u8; 8] = [134, 36, 13, 72, 232, 101, 130, 216];
pub const SELL_EVENT: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42];
pub const SET_BONDING_CURVE_COIN_CREATOR_EVENT: [u8; 8] = [242, 231, 235, 102, 65, 99, 189, 211];
pub const SET_METAPLEX_COIN_CREATOR_EVENT: [u8; 8] = [150, 107, 199, 123, 124, 207, 102, 228];
pub const SYNC_USER_VOLUME_ACCUMULATOR_EVENT: [u8; 8] = [197, 122, 167, 124, 116, 81, 91, 255];
pub const UPDATE_ADMIN_EVENT: [u8; 8] = [225, 152, 171, 87, 246, 63, 66, 234];
pub const UPDATE_FEE_CONFIG_EVENT: [u8; 8] = [90, 23, 65, 35, 62, 244, 188, 208];
pub const WITHDRAW_EVENT: [u8; 8] = [22, 9, 133, 26, 160, 44, 71, 192];

#[derive(Debug, Clone, PartialEq)]
pub enum PumpSwapEvent {
    AdminSetCoinCreator(AdminSetCoinCreatorEvent),
    AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesEvent),
    Buy(BuyEvent),
    ClaimTokenIncentives(ClaimTokenIncentivesEvent),
    CloseUserVolumeAccumulator(CloseUserVolumeAccumulatorEvent),
    CollectCoinCreatorFee(CollectCoinCreatorFeeEvent),
    CreateConfig(CreateConfigEvent),
    CreatePool(CreatePoolEvent),
    Deposit(DepositEvent),
    Disable(DisableEvent),
    ExtendAccount(ExtendAccountEvent),
    InitUserVolumeAccumulator(InitUserVolumeAccumulatorEvent),
    Sell(SellEvent),
    SetBondingCurveCoinCreator(SetBondingCurveCoinCreatorEvent),
    SetMetaplexCoinCreator(SetMetaplexCoinCreatorEvent),
    SyncUserVolumeAccumulator(SyncUserVolumeAccumulatorEvent),
    UpdateAdmin(UpdateAdminEvent),
    UpdateFeeConfig(UpdateFeeConfigEvent),
    Withdraw(WithdrawEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AdminSetCoinCreatorEvent {
    pub timestamp: i64,
    pub admin_set_coin_creator_authority: Pubkey,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub old_coin_creator: Pubkey,
    pub new_coin_creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AdminUpdateTokenIncentivesEvent {
    pub start_time: i64,
    pub end_time: i64,
    pub day_number: u64,
    pub token_supply_per_day: u64,
    pub mint: Pubkey,
    pub seconds_in_a_day: i64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
    pub track_volume: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub min_base_amount_out: u64,
    pub ix_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimTokenIncentivesEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CloseUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub timestamp: i64,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CollectCoinCreatorFeeEvent {
    pub timestamp: i64,
    pub coin_creator: Pubkey,
    pub coin_creator_fee: u64,
    pub coin_creator_vault_ata: Pubkey,
    pub coin_creator_token_account: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositEvent {
    pub timestamp: i64,
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DisableEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: Pubkey,
    pub user: Pubkey,
    pub current_size: u64,
    pub new_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitUserVolumeAccumulatorEvent {
    pub payer: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetBondingCurveCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub bonding_curve: Pubkey,
    pub coin_creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetMetaplexCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Pubkey,
    pub pool: Pubkey,
    pub metadata: Pubkey,
    pub coin_creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SyncUserVolumeAccumulatorEvent {
    pub user: Pubkey,
    pub total_claimed_tokens_before: u64,
    pub total_claimed_tokens_after: u64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateAdminEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub new_admin: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateFeeConfigEvent {
    pub timestamp: i64,
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawEvent {
    pub timestamp: i64,
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_out: u64,
    pub quote_amount_out: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}

pub fn unpack_event(data: &[u8]) -> Result<PumpSwapEvent, ParseError> {
    if data.len() < 8 {
        return Err(ParseError::TooShort(data.len()));
    }
    let (disc, rest) = data.split_at(8);
    let disc: [u8; 8] = disc.try_into().unwrap();
    Ok(match disc {
        ADMIN_SET_COIN_CREATOR_EVENT => PumpSwapEvent::AdminSetCoinCreator(AdminSetCoinCreatorEvent::try_from_slice(rest)?),
        ADMIN_UPDATE_TOKEN_INCENTIVES_EVENT => PumpSwapEvent::AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesEvent::try_from_slice(rest)?),
        BUY_EVENT => PumpSwapEvent::Buy(BuyEvent::try_from_slice(rest)?),
        CLAIM_TOKEN_INCENTIVES_EVENT => PumpSwapEvent::ClaimTokenIncentives(ClaimTokenIncentivesEvent::try_from_slice(rest)?),
        CLOSE_USER_VOLUME_ACCUMULATOR_EVENT => PumpSwapEvent::CloseUserVolumeAccumulator(CloseUserVolumeAccumulatorEvent::try_from_slice(rest)?),
        COLLECT_COIN_CREATOR_FEE_EVENT => PumpSwapEvent::CollectCoinCreatorFee(CollectCoinCreatorFeeEvent::try_from_slice(rest)?),
        CREATE_CONFIG_EVENT => PumpSwapEvent::CreateConfig(CreateConfigEvent::try_from_slice(rest)?),
        CREATE_POOL_EVENT => PumpSwapEvent::CreatePool(CreatePoolEvent::try_from_slice(rest)?),
        DEPOSIT_EVENT => PumpSwapEvent::Deposit(DepositEvent::try_from_slice(rest)?),
        DISABLE_EVENT => PumpSwapEvent::Disable(DisableEvent::try_from_slice(rest)?),
        EXTEND_ACCOUNT_EVENT => PumpSwapEvent::ExtendAccount(ExtendAccountEvent::try_from_slice(rest)?),
        INIT_USER_VOLUME_ACCUMULATOR_EVENT => PumpSwapEvent::InitUserVolumeAccumulator(InitUserVolumeAccumulatorEvent::try_from_slice(rest)?),
        SELL_EVENT => PumpSwapEvent::Sell(SellEvent::try_from_slice(rest)?),
        SET_BONDING_CURVE_COIN_CREATOR_EVENT => PumpSwapEvent::SetBondingCurveCoinCreator(SetBondingCurveCoinCreatorEvent::try_from_slice(rest)?),
        SET_METAPLEX_COIN_CREATOR_EVENT => PumpSwapEvent::SetMetaplexCoinCreator(SetMetaplexCoinCreatorEvent::try_from_slice(rest)?),
        SYNC_USER_VOLUME_ACCUMULATOR_EVENT => PumpSwapEvent::SyncUserVolumeAccumulator(SyncUserVolumeAccumulatorEvent::try_from_slice(rest)?),
        UPDATE_ADMIN_EVENT => PumpSwapEvent::UpdateAdmin(UpdateAdminEvent::try_from_slice(rest)?),
        UPDATE_FEE_CONFIG_EVENT => PumpSwapEvent::UpdateFeeConfig(UpdateFeeConfigEvent::try_from_slice(rest)?),
        WITHDRAW_EVENT => PumpSwapEvent::Withdraw(WithdrawEvent::try_from_slice(rest)?),
        _ => return Err(ParseError::Unknown(disc)),
    })
}

// -----------------------------------------------------------------------------
// Permissive trade-event decoder
// -----------------------------------------------------------------------------

/// Direction of a pump.fun AMM trade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeDirection {
    Buy,
    Sell,
}

/// Minimal trade-event view used by downstream sinks.
///
/// Only includes the leading-layout fields that have remained at fixed byte
/// offsets across every known on-chain schema bump (V1, V2, the post-2026-02-12
/// PumpSwap rename, and the cashback fields shipped 2026-02-17). Everything
/// after `user` is intentionally ignored so the decoder stays correct as the
/// program continues to append fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradeEventMinimal {
    pub direction: TradeDirection,
    /// Trader wallet (Pubkey).
    pub user: [u8; 32],
    /// `base_amount_out` for buys, `base_amount_in` for sells (lamports of base mint).
    pub base_amount: u64,
    /// `quote_amount_in` for buys, `quote_amount_out` for sells (lamports of quote mint).
    pub quote_amount: u64,
}

/// Anchor self-CPI envelope tag prepended by `emit_cpi!` to event-carrying
/// inner-instruction data: `[ANCHOR_SELF_CPI_TAG][event_disc][payload]`.
const ANCHOR_SELF_CPI_TAG: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];

// Stable byte offsets within the BuyEvent / SellEvent payload (after the
// 16-byte preamble). Both events share an identical leading layout:
//   [i64 timestamp (8B)] [13 × u64 (104B)] [pool: pubkey (32B)] [user: pubkey (32B)] ...
// `base_amount_(out|in)` is the 2nd numeric field; `quote_amount_(in|out)` is
// the 8th. `user` is the second pubkey, after `pool`.
const BASE_AMOUNT_OFFSET: usize = 8;
const QUOTE_AMOUNT_OFFSET: usize = 56;
const USER_OFFSET: usize = 144;
const TRADE_PAYLOAD_MIN_LEN: usize = USER_OFFSET + 32; // 176 bytes

/// Decode a pump.fun AMM trade event into the stable subset of fields that
/// downstream sinks need (direction, user, base/quote amount), reading at
/// fixed byte offsets and ignoring whatever follows.
///
/// Tolerates append-only schema growth: works for V1, V2, PumpSwap-with-cashback,
/// and any future variant that appends fields after the leading layout.
///
/// Expects the inner-instruction data of an `emit_cpi!`-style event:
/// `[ANCHOR_SELF_CPI_TAG (8B)][event_disc (8B)][payload]`.
///
/// Returns `Ok(None)` for envelope-shaped data with a non-trade discriminator
/// (e.g. CreateConfig, Deposit) so callers can ignore those without treating
/// them as errors.
pub fn unpack_trade_event_minimal(data: &[u8]) -> Result<Option<TradeEventMinimal>, ParseError> {
    if data.len() < 16 {
        return Err(ParseError::TooShort(data.len()));
    }
    if data[..8] != ANCHOR_SELF_CPI_TAG {
        return Err(ParseError::Unknown(data[..8].try_into().expect("len 8")));
    }
    let event_disc: [u8; 8] = data[8..16].try_into().expect("len 8");
    let direction = match event_disc {
        BUY_EVENT => TradeDirection::Buy,
        SELL_EVENT => TradeDirection::Sell,
        _ => return Ok(None),
    };
    let payload = &data[16..];
    if payload.len() < TRADE_PAYLOAD_MIN_LEN {
        return Err(ParseError::TooShort(payload.len()));
    }
    let base_amount = u64::from_le_bytes(
        payload[BASE_AMOUNT_OFFSET..BASE_AMOUNT_OFFSET + 8]
            .try_into()
            .expect("len 8"),
    );
    let quote_amount = u64::from_le_bytes(
        payload[QUOTE_AMOUNT_OFFSET..QUOTE_AMOUNT_OFFSET + 8]
            .try_into()
            .expect("len 8"),
    );
    let user: [u8; 32] = payload[USER_OFFSET..USER_OFFSET + 32]
        .try_into()
        .expect("len 32");
    Ok(Some(TradeEventMinimal {
        direction,
        user,
        base_amount,
        quote_amount,
    }))
}

#[cfg(test)]
mod minimal_tests {
    use super::*;

    /// Build a synthetic emit_cpi! event payload: anchor tag + event disc +
    /// the stable leading layout. `trailing` simulates whatever the program
    /// has tacked on in the current/future versions.
    fn make_event(disc: [u8; 8], base: u64, quote: u64, user: [u8; 32], trailing: &[u8]) -> Vec<u8> {
        let mut data = Vec::with_capacity(16 + TRADE_PAYLOAD_MIN_LEN + trailing.len());
        data.extend_from_slice(&ANCHOR_SELF_CPI_TAG);
        data.extend_from_slice(&disc);
        data.extend_from_slice(&0i64.to_le_bytes()); // timestamp
        data.extend_from_slice(&base.to_le_bytes()); // 2nd field
        for _ in 0..5 {
            data.extend_from_slice(&0u64.to_le_bytes());
        } // 3rd-7th
        data.extend_from_slice(&quote.to_le_bytes()); // 8th
        for _ in 0..6 {
            data.extend_from_slice(&0u64.to_le_bytes());
        } // 9th-14th
        data.extend_from_slice(&[0u8; 32]); // pool
        data.extend_from_slice(&user); // user
        data.extend_from_slice(trailing);
        data
    }

    fn pk(b: u8) -> [u8; 32] {
        [b; 32]
    }

    #[test]
    fn buy_event_minimal_payload() {
        let data = make_event(BUY_EVENT, 1_000, 999, pk(0xab), &[]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert_eq!(ev.direction, TradeDirection::Buy);
        assert_eq!(ev.base_amount, 1_000);
        assert_eq!(ev.quote_amount, 999);
        assert_eq!(ev.user, pk(0xab));
    }

    #[test]
    fn sell_event_minimal_payload() {
        let data = make_event(SELL_EVENT, 7, 13, pk(0xcd), &[]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert_eq!(ev.direction, TradeDirection::Sell);
        assert_eq!(ev.base_amount, 7);
        assert_eq!(ev.quote_amount, 13);
    }

    #[test]
    fn ignores_v2_trailing_bytes() {
        // V2 appended coin_creator + 2 fee u64s = 48 bytes after the leading 176.
        let data = make_event(BUY_EVENT, 100, 50, pk(0x01), &vec![0u8; 48]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert_eq!(ev.base_amount, 100);
        assert_eq!(ev.quote_amount, 50);
    }

    #[test]
    fn ignores_pumpswap_trailing_bytes() {
        // Post-2026-02-12 PumpSwap appended track_volume bool + 5 u64s + ix_name + cashback.
        // Exact size doesn't matter — decoder ignores whatever follows.
        let data = make_event(BUY_EVENT, 42, 84, pk(0x02), &vec![0u8; 224]);
        let ev = unpack_trade_event_minimal(&data).unwrap().unwrap();
        assert_eq!(ev.base_amount, 42);
        assert_eq!(ev.quote_amount, 84);
    }

    #[test]
    fn returns_ok_none_for_non_trade_event() {
        // CreateConfig / Deposit / etc. share the envelope; the caller should
        // be able to skip them without treating them as decode failures.
        let create_config = CREATE_CONFIG_EVENT;
        let mut data = Vec::new();
        data.extend_from_slice(&ANCHOR_SELF_CPI_TAG);
        data.extend_from_slice(&create_config);
        data.extend_from_slice(&[0u8; TRADE_PAYLOAD_MIN_LEN]);
        assert!(unpack_trade_event_minimal(&data).unwrap().is_none());
    }

    #[test]
    fn errors_on_data_without_anchor_cpi_tag() {
        let mut data = make_event(BUY_EVENT, 1, 1, pk(0xff), &[]);
        data[..8].fill(0);
        assert!(matches!(
            unpack_trade_event_minimal(&data),
            Err(ParseError::Unknown(_))
        ));
    }

    #[test]
    fn errors_on_payload_shorter_than_leading_layout() {
        let mut data = Vec::new();
        data.extend_from_slice(&ANCHOR_SELF_CPI_TAG);
        data.extend_from_slice(&BUY_EVENT);
        data.extend_from_slice(&[0u8; 100]); // < 176-byte minimum
        assert!(matches!(
            unpack_trade_event_minimal(&data),
            Err(ParseError::TooShort(_))
        ));
    }

    #[test]
    fn errors_on_data_too_short_for_envelope() {
        assert!(matches!(
            unpack_trade_event_minimal(&[]),
            Err(ParseError::TooShort(_))
        ));
        assert!(matches!(
            unpack_trade_event_minimal(&ANCHOR_SELF_CPI_TAG),
            Err(ParseError::TooShort(_))
        ));
    }
}
