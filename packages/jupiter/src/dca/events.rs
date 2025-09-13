//! Jupiter DCA on-chain events.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes)
// -----------------------------------------------------------------------------
const COLLECTED_FEE: [u8; 8] = [42, 136, 216, 116, 181, 209, 109, 181];
const FILLED: [u8; 8] = [134, 4, 17, 63, 221, 45, 177, 173];
const OPENED: [u8; 8] = [166, 172, 97, 9, 77, 76, 189, 109];
const CLOSED: [u8; 8] = [50, 31, 87, 155, 135, 220, 195, 239];
const WITHDRAW: [u8; 8] = [192, 241, 201, 217, 70, 150, 90, 247];
const DEPOSIT: [u8; 8] = [62, 205, 242, 175, 244, 169, 136, 52];

// -----------------------------------------------------------------------------
// High-level event enum
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JupiterDcaEvent {
    CollectedFee(CollectedFeeEvent),
    Filled(FilledEvent),
    Opened(OpenedEvent),
    Closed(ClosedEvent),
    Withdraw(WithdrawEvent),
    Deposit(DepositEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CollectedFeeEvent {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FilledEvent {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub fee_mint: Pubkey,
    pub fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OpenedEvent {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub in_deposited: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub cycle_frequency: i64,
    pub in_amount_per_cycle: u64,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClosedEvent {
    pub user_key: Pubkey,
    pub dca_key: Pubkey,
    pub in_deposited: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub cycle_frequency: i64,
    pub in_amount_per_cycle: u64,
    pub created_at: i64,
    pub total_in_withdrawn: u64,
    pub total_out_withdrawn: u64,
    pub unfilled_amount: u64,
    pub user_closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawEvent {
    pub dca_key: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub user_withdraw: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositEvent {
    pub dca_key: Pubkey,
    pub amount: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for JupiterDcaEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            COLLECTED_FEE => Self::CollectedFee(CollectedFeeEvent::try_from_slice(payload)?),
            FILLED => Self::Filled(FilledEvent::try_from_slice(payload)?),
            OPENED => Self::Opened(OpenedEvent::try_from_slice(payload)?),
            CLOSED => Self::Closed(ClosedEvent::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawEvent::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositEvent::try_from_slice(payload)?),
            _ => Self::Unknown,
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<JupiterDcaEvent, ParseError> {
    JupiterDcaEvent::try_from(data)
}
