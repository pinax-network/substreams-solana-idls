use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
const SET_PARAMS: [u8; 8] = [165, 31, 134, 53, 189, 180, 130, 255];
const CREATE_INSTRUCTION: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// -----------------------------------------------------------------------------
// Event data structures
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PumpFunInstruction {
    Initialize,
    SetParams(SetParamsInstruction),
    Create(CreateInstruction),
    Buy(BuyInstruction),
    Sell(SellInstruction),
    Withdraw,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetParamsInstruction {
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateInstruction {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
/// Buys tokens from a Pump.fun bonding curve, swapping SOL for the specified SPL token.
///
/// This instruction transfers up to `max_sol_cost` lamports from the buyer’s wallet,
/// sends any protocol fees to the fee recipient, deposits the SOL into the curve’s vault,
/// mints `amount` tokens to the buyer’s token account (creating it if needed),
/// and emits a purchase event.
///
/// ### Accounts expected by this instruction:
///
/// 0. `[]` Global state account.
/// 1. `[writable]` Fee recipient account.
/// 2. `[]` Token mint (e.g. CWOIN).
/// 3. `[writable]` Bonding curve configuration account.
/// 4. `[writable]` Vault account holding the curve’s token reserve.
/// 5. `[writable]` User state account (tracks per-user data).
/// 6. `[writable, signer]` Buyer’s wallet (fee payer).
/// 7. `[]` System program.
/// 8. `[]` SPL Token program.
/// 9. `[writable]` Creator vault account (for creator-fee withdrawals).
/// 10. `[]` Event authority account (used to record purchase events).
/// 11. `[]` Pump.fun program ID.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct BuyInstruction {
    /// Amount of tokens to buy (in token smallest units)
    pub amount: u64, // foo
    /// Maximum acceptable SOL cost for the purchase (slippage protection)
    pub max_sol_cost: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SellInstruction {
    pub amount: u64,
    pub min_sol_output: u64,
}

// -----------------------------------------------------------------------------
// Parsing implementation
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PumpFunInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 16 {
            return Err(ParseError::TooShort(data.len()));
        }

        let discriminator: [u8; 8] = data[0..8].try_into().expect("slice with length 8");
        let payload = &data[8..];

        match discriminator {
            SET_PARAMS => Ok(Self::SetParams(SetParamsInstruction::try_from_slice(payload)?)),
            CREATE_INSTRUCTION => Ok(Self::Create(CreateInstruction::try_from_slice(payload)?)),
            BUY => Ok(Self::Buy(BuyInstruction::try_from_slice(payload)?)),
            SELL => Ok(Self::Sell(SellInstruction::try_from_slice(payload)?)),
            WITHDRAW => Ok(Self::Withdraw),
            INITIALIZE => Ok(Self::Initialize),
            other => Err(ParseError::Unknown(other)),
        }
    }
}

// Convenience function retaining the old name; forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PumpFunInstruction, ParseError> {
    PumpFunInstruction::try_from(data)
}
