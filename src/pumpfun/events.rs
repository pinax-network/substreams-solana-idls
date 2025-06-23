use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
const CREATE: [u8; 8] = [27, 114, 169, 77, 222, 235, 99, 118];
const TRADE: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
const COMPLETE: [u8; 8] = [95, 114, 97, 156, 212, 46, 152, 8];
const SETPARMS: [u8; 8] = [223, 195, 159, 246, 62, 48, 143, 131];

// -----------------------------------------------------------------------------
// Event data structures
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PumpFunEvent {
    Create(CreateEvent),
    Trade(TradeEvent),
    Complete(CompleteEvent),
    SetParams(SetParamsEvent),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TradeEvent {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u64,
    pub fee: u64,
    pub creator: Pubkey,
    pub creator_fee_basis_points: u64,
    pub creator_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetParamsEvent {
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
}

// -----------------------------------------------------------------------------
// Parsing implementation
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PumpFunEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 16 {
            return Err(ParseError::TooShort(data.len()));
        }

        let discriminator: [u8; 8] = data[0..8].try_into().expect("slice with length 8");
        let payload = &data[16..]; // skip pump.fun + Anchor discriminators

        match discriminator {
            CREATE => Ok(Self::Create(CreateEvent::try_from_slice(payload)?)),
            TRADE => Ok(Self::Trade(TradeEvent::try_from_slice(payload)?)),
            COMPLETE => Ok(Self::Complete(CompleteEvent::try_from_slice(payload)?)),
            SETPARMS => Ok(Self::SetParams(SetParamsEvent::try_from_slice(payload)?)),
            other => Err(ParseError::Unknown(other)),
        }
    }
}

// Convenience function retaining the old name; forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PumpFunEvent, ParseError> {
    PumpFunEvent::try_from(data)
}
