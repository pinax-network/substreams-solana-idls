//! Raydium CPMM events.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const LP_CHANGE_EVENT: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];
pub const SWAP_EVENT: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
const SWAP_EVENT_V1_PAYLOAD_LEN: usize = 32 + 8 * 6 + 1;
const SWAP_EVENT_V2_PAYLOAD_LEN: usize = SWAP_EVENT_V1_PAYLOAD_LEN + 32 + 32 + 8 + 8 + 1;

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaydiumCpmmEvent {
    LpChangeEvent(LpChangeEvent),
    SwapEventV1(SwapEventV1),
    SwapEventV2(SwapEventV2),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Emitted when deposit and withdraw
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LpChangeEvent {
    pub pool_id: Pubkey,
    pub lp_amount_before: u64,
    /// pool vault sub trade fees
    pub token_0_vault_before: u64,
    /// pool vault sub trade fees
    pub token_1_vault_before: u64,
    /// calculate result without transfer fee
    pub token_0_amount: u64,
    /// calculate result without transfer fee
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u8,
}
/// Emitted when swap
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapEventV1 {
    pub pool_id: Pubkey,
    /// pool vault sub trade fees
    pub input_vault_before: u64,
    /// pool vault sub trade fees
    pub output_vault_before: u64,
    /// calculate result without transfer fee
    pub input_amount: u64,
    /// calculate result without transfer fee
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    /// `true` if the swap was performed with the base token as input.
    pub base_input: bool,
}

/// Emitted when swap
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapEventV2 {
    pub pool_id: Pubkey,
    /// pool vault sub trade fees
    pub input_vault_before: u64,
    /// pool vault sub trade fees
    pub output_vault_before: u64,
    /// calculate result without transfer fee
    pub input_amount: u64,
    /// calculate result without transfer fee
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub trade_fee: u64,
    /// Amount of fee tokens going to creator
    pub creator_fee: u64,
    pub creator_fee_on_input: bool,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumCpmmEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            LP_CHANGE_EVENT => Self::LpChangeEvent(LpChangeEvent::try_from_slice(payload)?),
            SWAP_EVENT => match payload.len() {
                SWAP_EVENT_V1_PAYLOAD_LEN => Self::SwapEventV1(SwapEventV1::try_from_slice(payload)?),
                SWAP_EVENT_V2_PAYLOAD_LEN => Self::SwapEventV2(SwapEventV2::try_from_slice(payload)?),
                _ => Self::Unknown,
            },
            _ => Self::Unknown,
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumCpmmEvent, ParseError> {
    RaydiumCpmmEvent::try_from(data)
}
