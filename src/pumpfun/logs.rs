use std::error::Error;
use std::fmt::{self, Display};

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use substreams::Hex;
// use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use substreams_solana::base58;

use crate::pumpfun::pubkey::Pubkey;

#[derive(Debug, Serialize, Deserialize)]
pub enum PumpFunEvent {
    Create(CreateEvent),
    Trade(TradeEvent),
    Complete(CompleteEvent),
    SetParams(SetParamsEvent),
}

/// Event emitted when a new token is created
///
/// This event contains information about a newly created token, including its
/// metadata, mint address, bonding curve address, and the accounts involved.
#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
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

/// Event emitted when a token is bought or sold
///
/// This event contains details about a trade transaction, including the amounts
/// exchanged, the type of trade (buy/sell), and the updated bonding curve state.
#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
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

/// Event emitted when global parameters are updated
///
/// This event contains information about updates to the global program parameters,
/// including fee settings and initial bonding curve configuration values.
#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
pub struct SetParamsEvent {
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
}

/// Event emitted when a bonding curve operation completes
///
/// This event signals the completion of a bonding curve operation,
/// providing information about the involved accounts.
#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
}

/// Parses base64-encoded program log data into a structured PumpFunEvent
///
/// This function decodes the base64 data from program logs, identifies the event type
/// using the discriminator (first 8 bytes), and deserializes the remaining data into
/// the appropriate event structure.
///
/// # Arguments
///
/// * `signature` - Transaction signature associated with the event
/// * `data` - Base64-encoded event data from program logs
///
/// # Returns
///
/// Returns a parsed PumpFunEvent if successful, or an error if parsing fails
pub fn parse_event(data: &[u8]) -> Result<PumpFunEvent, Box<dyn Error>> {
    // Get event type from the first 8 bytes
    if data.len() < 8 {
        return Err(format!("Data too short to contain discriminator: {:?}", data).into());
    }

    let discriminator = &data[..8];
    match discriminator {
        // CreateEvent
        [27, 114, 169, 77, 222, 235, 99, 118] => Ok(PumpFunEvent::Create(
            CreateEvent::try_from_slice(&data[8..]).map_err(|e| format!("Failed to decode CreateEvent: {}", e))?,
        )),
        // TradeEvent
        [228, 69, 165, 46, 81, 203, 154, 29] => Ok(PumpFunEvent::Trade(
            TradeEvent::try_from_slice(&data[8..]).map_err(|e| format!("Failed to decode TradeEvent: {}", e))?,
        )),
        // CompleteEvent
        [95, 114, 97, 156, 212, 46, 152, 8] => Ok(PumpFunEvent::Complete(
            CompleteEvent::try_from_slice(&data[8..]).map_err(|e| format!("Failed to decode CompleteEvent: {}", e))?,
        )),
        // SetParamsEvent
        [223, 195, 159, 246, 62, 48, 143, 131] => Ok(PumpFunEvent::SetParams(
            SetParamsEvent::try_from_slice(&data[8..]).map_err(|e| format!("Failed to decode SetParamsEvent: {}", e))?,
        )),
        _ => Err(format!("Unknown discriminator: {} {:?}", Hex::encode(discriminator), discriminator).into()),
    }
}
