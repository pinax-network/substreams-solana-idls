//! Pump.fun on-chain instruction definitions and Borsh deserialisation helpers.
use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the instruction data)
// -----------------------------------------------------------------------------
const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234]; // 33e685a4017f83ad
const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173]; // 66063d1201daebea
const INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
const SET_PARAMS: [u8; 8] = [165, 31, 134, 53, 189, 180, 130, 255];
const CREATE_INSTRUCTION: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// ──────────────────────────────────────────────────────────────────────────────
// High-level enum
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum PumpFunInstruction {
    /// Initialise global state (no payload).
    Initialize,

    /// Updates protocol parameters on an existing bonding curve.
    ///
    /// ### What happens
    /// * Writes new virtual reserve figures to the curve config.
    /// * Changes the global `fee_recipient`.
    /// * Emits a `ParamsUpdated` event.
    ///
    /// ### Accounts
    /// | # | Modifier(s)        | Purpose                          |
    /// |---|--------------------|----------------------------------|
    /// | 0 | `[writable]`       | Bonding-curve configuration      |
    /// | 1 | `[signer]`         | Admin authority (curve owner)    |
    /// | 2 | `[]`               | Global state                     |
    /// | 3 | `[]`               | Pump.fun program ID              |
    ///
    /// See [`SetParamsInstruction`].
    SetParams(SetParamsInstruction),

    /// Creates a new SPL-Token ↔ SOL bonding pool on Pump.fun.
    ///
    /// ### What happens
    /// * Initialises (or configures) the SPL-Token mint under Pump.fun’s mint authority.
    /// * Creates the Metaplex metadata account with the given `name`, `symbol`, and `uri`.
    /// * Derives & initialises bonding-curve config and curve accounts.
    /// * Registers the pool in global state under the `creator`.
    /// * Emits a `PoolCreated` event.
    ///
    /// ### Accounts
    /// | #  | Modifier(s)          | Purpose                                                        |
    /// |----|----------------------|----------------------------------------------------------------|
    /// | 0  | `[writable, signer]` | SPL-Token mint to initialise/configure                         |
    /// | 1  | `[]`                 | Pump.fun Token Mint Authority                                  |
    /// | 2  | `[writable]`         | Bonding-curve configuration account                            |
    /// | 3  | `[writable]`         | Associated bonding-curve account                               |
    /// | 4  | `[]`                 | Global state                                                   |
    /// | 5  | `[]`                 | Metaplex Token Metadata program                                |
    /// | 6  | `[writable]`         | Metadata account for name/symbol/URI                           |
    /// | 7  | `[writable, signer]` | User wallet (fee payer & pool creator)                         |
    /// | 8  | `[]`                 | System program                                                 |
    /// | 9  | `[]`                 | SPL-Token program                                              |
    /// | 10 | `[]`                 | Associated-Token-Account program                               |
    /// | 11 | `[]`                 | Rent sysvar                                                    |
    /// | 12 | `[]`                 | Event authority                                                |
    /// | 13 | `[]`                 | Pump.fun program ID                                            |
    ///
    /// See [`CreateInstruction`].
    Create(CreateInstruction),

    /// Buys tokens from a Pump.fun bonding curve (SOL → SPL).
    ///
    /// ### What happens
    /// * Transfers up to `max_sol_cost` lamports from the buyer.
    /// * Deposits SOL into the curve’s vault.
    /// * Mints `amount` tokens to the buyer’s destination ATA (creating it if needed).
    /// * Pays protocol & creator fees.
    /// * Emits a `Buy` event.
    ///
    /// ### Accounts
    /// | #  | Modifier(s)          | Purpose                                           |
    /// |----|----------------------|---------------------------------------------------|
    /// | 0  | `[]`                 | Global state                                      |
    /// | 1  | `[writable]`         | Fee recipient                                     |
    /// | 2  | `[]`                 | Token mint                                        |
    /// | 3  | `[writable]`         | Bonding-curve configuration                       |
    /// | 4  | `[writable]`         | Vault holding the curve’s **token** reserve       |
    /// | 5  | `[writable]`         | User state (per-user data)                        |
    /// | 6  | `[writable, signer]` | Buyer wallet (fee payer)                          |
    /// | 7  | `[]`                 | System program                                    |
    /// | 8  | `[]`                 | SPL-Token program                                 |
    /// | 9  | `[writable]`         | Creator vault (creator fees)                      |
    /// | 10 | `[]`                 | Event authority                                   |
    /// | 11 | `[]`                 | Pump.fun program ID                               |
    ///
    /// See [`BuyInstruction`].
    Buy(BuyInstruction),

    /// Sells tokens to a Pump.fun bonding curve (SPL → SOL).
    ///
    /// ### What happens
    /// * Burns `amount` tokens from the seller’s account.
    /// * Withdraws SOL from the curve’s SOL vault.
    /// * Deducts protocol & creator fees.
    /// * Sends **at least** `min_sol_output` lamports to the seller.
    /// * Emits a `Sell` event.
    ///
    /// ### Accounts
    /// | #  | Modifier(s)          | Purpose                                           |
    /// |----|----------------------|---------------------------------------------------|
    /// | 0  | `[]`                 | Global state                                      |
    /// | 1  | `[writable]`         | Fee recipient                                     |
    /// | 2  | `[]`                 | Token mint (e.g. CWOIN / TOLY)                    |
    /// | 3  | `[writable]`         | Bonding-curve configuration                       |
    /// | 4  | `[writable]`         | Vault holding the curve’s **token** reserve       |
    /// | 5  | `[writable]`         | User state (per-user data)                        |
    /// | 6  | `[writable, signer]` | Seller wallet (fee payer)                         |
    /// | 7  | `[]`                 | System program                                    |
    /// | 8  | `[writable]`         | Creator vault (creator-fee withdrawals)           |
    /// | 9  | `[]`                 | SPL-Token program                                 |
    /// | 10 | `[]`                 | Event authority                                   |
    /// | 11 | `[]`                 | Pump.fun program ID                               |
    ///
    /// See [`SellInstruction`].
    Sell(SellInstruction),

    /// Withdraw accumulated protocol fees.
    Withdraw,

    /// Discriminator did not match any known instruction.
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------

/// Instruction data for [`PumpFunInstruction::SetParams`]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetParamsInstruction {
    /// Account that will collect protocol fees going forward.
    pub fee_recipient: Pubkey,
    /// Virtual token reserve used in price calculation.
    pub initial_virtual_token_reserves: u64,
    /// Virtual SOL reserve used in price calculation.
    pub initial_virtual_sol_reserves: u64,
    /// Real SPL-Token balance present at pool creation (for reference).
    pub initial_real_token_reserves: u64,
    /// Total supply of the SPL-Token.
    pub token_total_supply: u64,
    /// Protocol fee charged on each trade (basis points, i.e. 1 bp = 0.01 %).
    pub fee_basis_points: u64,
}
/// Instruction data for [`PumpFunInstruction::Create`]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateInstruction {
    /// Display name for the token pool (UTF-8).
    pub name: String,
    /// Ticker symbol (≤ 10 UTF-8 bytes).
    pub symbol: String,
    /// URI pointing to off-chain JSON metadata.
    pub uri: String,
    /// Pool creator (receives creator fees).
    pub creator: Pubkey,
}

/// Instruction data for [`PumpFunInstruction::Buy`].
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyInstruction {
    /// Amount of tokens to purchase.
    pub amount: u64,
    /// Maximum lamports the buyer will pay (slippage guard).
    pub max_sol_cost: u64,
}

/// Instruction data for [`PumpFunInstruction::Sell`].
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellInstruction {
    /// Number of tokens to sell.
    pub amount: u64,
    /// Minimum lamports the seller expects (slippage guard).
    pub min_sol_output: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PumpFunInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SET_PARAMS => Self::SetParams(SetParamsInstruction::try_from_slice(payload)?),
            CREATE_INSTRUCTION => Self::Create(CreateInstruction::try_from_slice(payload)?),
            BUY => Self::Buy(BuyInstruction::try_from_slice(payload)?),
            SELL => Self::Sell(SellInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw,
            INITIALIZE => Self::Initialize,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

// Convenience function retaining the old name; forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PumpFunInstruction, ParseError> {
    PumpFunInstruction::try_from(data)
}
