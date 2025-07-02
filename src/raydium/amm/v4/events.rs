//! on-chain **events** and their Borsh-deserialisation helpers.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

/// ------------------------------------------------------------------------
/// Discriminators
/// ------------------------------------------------------------------------
/// Raydium encodes `log_type` as the first **single** byte of the
/// bincode-serialised struct, so we keep simple `u8` constants here.
/// (No 8-byte Anchor discriminators needed.)
pub const INIT: u8 = 0;
pub const DEPOSIT: u8 = 1;
pub const WITHDRAW: u8 = 2;
pub const SWAP_IN: u8 = 3;
pub const SWAP_OUT: u8 = 4;

// -----------------------------------------------------------------------------
// High-level event enum (concise; rich docs live in each struct)
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumV4Event {
    /// New pool initialised. See [`InitLog`].
    Init(InitLog),

    /// Liquidity added. See [`DepositLog`].
    Deposit(DepositLog),

    /// Liquidity removed. See [`WithdrawLog`].
    Withdraw(WithdrawLog),

    /// Swap where the **base** token is _input_. See [`SwapBaseInLog`].
    SwapBaseIn(SwapBaseInLog),

    /// Swap where the **base** token is _output_. See [`SwapBaseOutLog`].
    SwapBaseOut(SwapBaseOutLog),

    /// Unknown `log_type` – forward-compatible.
    Unknown(u8),
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct InitLog {
    pub log_type: u8,
    pub time: u64,
    pub pc_decimals: u8,
    pub coin_decimals: u8,
    pub pc_lot_size: u64,
    pub coin_lot_size: u64,
    pub pc_amount: u64,
    pub coin_amount: u64,
    pub market: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositLog {
    pub log_type: u8,
    // input
    pub max_coin: u64,
    pub max_pc: u64,
    pub base: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    // calc result
    pub deduct_coin: u64,
    pub deduct_pc: u64,
    pub mint_lp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawLog {
    pub log_type: u8,
    // input
    pub withdraw_lp: u64,
    // user info
    pub user_lp: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    // calc result
    pub out_coin: u64,
    pub out_pc: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseInLog {
    pub log_type: u8,
    // input
    pub amount_in: u64,
    pub minimum_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub out_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseOutLog {
    pub log_type: u8,
    // input
    pub max_in: u64,
    pub amount_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub deduct_in: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumV4Event {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(0));
        }

        // First byte is Raydium’s log_type discriminator
        let disc = data[0];
        let payload = data; // include the discriminator – Raydium structs need it

        Ok(match disc {
            SWAP_IN => Self::SwapBaseIn(SwapBaseInLog::try_from_slice(payload)?),
            SWAP_OUT => Self::SwapBaseOut(SwapBaseOutLog::try_from_slice(payload)?),
            INIT => Self::Init(InitLog::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositLog::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawLog::try_from_slice(payload)?),
            other => return Err(ParseError::RaydiumUnknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumV4Event, ParseError> {
    RaydiumV4Event::try_from(data)
}
