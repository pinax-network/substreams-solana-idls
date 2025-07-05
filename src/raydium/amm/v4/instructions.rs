//! on-chain instruction definitions and Borsh deserialisation helpers.
// https://github.com/raydium-io/raydium-amm/blob/master/program/src/instruction.rs
use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
// use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the instruction data)
// -----------------------------------------------------------------------------
const SWAP_BASE_IN: u8 = 9;
const SWAP_BASE_OUT: u8 = 11;

// The canonical SwapBaseIn payload is 17 bytes.
// Extra bytes (the 0x40 you saw) are legal and ignored by the contract.
// The discriminator is 1 byte, followed by 16 bytes of payload.
//
// The following transaction includes 18 bytes of data, but the first 17 are the
// canonical payload. The 18th byte is ignored by the contract.
// https://solscan.io/tx/3JzyAsW5DZiX6B4CHp4TLkim3ksqoWhRHj2my7qNaGuPrDakoAoUqCa51tWjETTHsVZneFQPjmkRNAEkrn9Q1665
const SWAP_LEN: usize = 17; // Length of the swap instructions (including discriminator)

const _INITIALIZE: u8 = 0; // Placeholder for future use
const _INITIALIZE2: u8 = 0; // Placeholder for future use
const _MONITOR_STEP: u8 = 0; // Placeholder for future use
const _DEPOSIT: u8 = 0; // Placeholder for future use
const _WITHDRAW: u8 = 0; // Placeholder for future use
const _MIGRATE_TO_OPENBOOK: u8 = 0; // Placeholder for future use
const _SET_PARAMS: u8 = 0; // Placeholder for future use
const _WITHDRAW_PNL: u8 = 0; // Placeholder for future use
const _WITHDRAW_SRM: u8 = 0; // Placeholder for future use
const _PRE_INITIALIZE: u8 = 0; // Placeholder for future use
const _SIMULATE_INFO: u8 = 0; // Placeholder for future use
const _ADMIN_CANCEL_ORDERS: u8 = 0; // Placeholder for future use
const _CREATE_CONFIG_ACCOUNT: u8 = 0; // Placeholder for future use
const _UPDATE_CONFIG_ACCOUNT: u8 = 0; // Placeholder for future use

// ──────────────────────────────────────────────────────────────────────────────
// High-level enum
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum RaydiumV4Instruction {
    /// Swaps **base** tokens (the pool’s *coin* side) for **quote** tokens
    /// (`SwapBaseInInstruction`).
    ///
    /// ### What happens
    /// * Debits `amount_in` **base** tokens from the user’s **source** ATA.
    /// * Credits at least `min_amount_out` **quote** tokens to the user’s **destination** ATA.
    /// * Transfers the tokens through the Raydium AMM curve & OpenBook order-book,
    ///   updating open-orders / target-orders as needed.
    /// * Emits a `SwapBaseIn` event.
    ///
    /// ### Accounts
    /// | #  | Modifier(s)              | Purpose                                                   |
    /// |----|--------------------------|-----------------------------------------------------------|
    /// | 0  | `[]`                     | SPL-Token program                                         |
    /// | 1  | `[writable]`             | AMM pool account (Raydium V4 liquidity-state)            |
    /// | 2  | `[]`                     | AMM authority PDA                                         |
    /// | 3  | `[writable]`             | AMM open-orders                                           |
    /// | 4  | `[writable]`             | AMM target-orders                                         |
    /// | 5  | `[writable]`             | AMM **coin** vault (base-token vault)                     |
    /// | 6  | `[writable]`             | AMM **pc** vault (quote-token vault)                      |
    /// | 7  | `[]`                     | OpenBook (or Serum) DEX program                           |
    /// | 8  | `[writable]`             | Market account                                            |
    /// | 9  | `[writable]`             | Market bids slab                                          |
    /// | 10 | `[writable]`             | Market asks slab                                          |
    /// | 11 | `[writable]`             | Market event queue                                        |
    /// | 12 | `[writable]`             | Market **coin** vault (base)                              |
    /// | 13 | `[writable]`             | Market **pc** vault (quote)                               |
    /// | 14 | `[]`                     | Market vault-signer PDA                                   |
    /// | 15 | `[writable]`             | User **source** ATA (base token)                          |
    /// | 16 | `[writable]`             | User **destination** ATA (quote token)                    |
    /// | 17 | `[writable, signer]`     | User wallet (authority & fee-payer)                       |
    /// | 18 | `[]`                     | _System program not needed for this CPI path_¹            |
    ///
    /// ¹ Raydium’s CPI doesn’t call the System program directly; include it only if
    ///   your CPI wrapper does.
    ///
    /// See [`SwapBaseInInstruction`].
    SwapBaseIn(SwapBaseInInstruction),

    // ---------------------------------------------------------------------------
    /// Swaps **quote** tokens for a precise `amount_out` of **base** tokens
    /// (`SwapBaseOutInstruction`).
    ///
    /// ### What happens
    /// * Calculates the necessary **quote** tokens (`max_amount_in`) and debits them
    ///   from the user’s **source** ATA.
    /// * Credits exactly `amount_out` **base** tokens to the user’s **destination** ATA.
    /// * Runs the same AMM-curve & OpenBook settlement logic as `SwapBaseIn`.
    /// * Emits a `SwapBaseOut` event.
    ///
    /// ### Accounts
    /// | #  | Modifier(s)              | Purpose                                                   |
    /// |----|--------------------------|-----------------------------------------------------------|
    /// | 0  | `[]`                     | SPL-Token program                                         |
    /// | 1  | `[writable]`             | AMM pool account (Raydium V4 liquidity-state)            |
    /// | 2  | `[]`                     | AMM authority PDA                                         |
    /// | 3  | `[writable]`             | AMM open-orders                                           |
    /// | 4  | `[writable]`             | AMM target-orders                                         |
    /// | 5  | `[writable]`             | AMM **coin** vault (base-token vault)                     |
    /// | 6  | `[writable]`             | AMM **pc** vault (quote-token vault)                      |
    /// | 7  | `[]`                     | OpenBook (or Serum) DEX program                           |
    /// | 8  | `[writable]`             | Market account                                            |
    /// | 9  | `[writable]`             | Market bids slab                                          |
    /// | 10 | `[writable]`             | Market asks slab                                          |
    /// | 11 | `[writable]`             | Market event queue                                        |
    /// | 12 | `[writable]`             | Market **coin** vault (base)                              |
    /// | 13 | `[writable]`             | Market **pc** vault (quote)                               |
    /// | 14 | `[]`                     | Market vault-signer PDA                                   |
    /// | 15 | `[writable]`             | User **source** ATA (quote token)                         |
    /// | 16 | `[writable]`             | User **destination** ATA (base token)                     |
    /// | 17 | `[writable, signer]`     | User wallet (authority & fee-payer)                       |
    /// | 18 | `[]`                     | _System program not needed for this CPI path_¹            |
    ///
    /// See [`SwapBaseOutInstruction`].
    SwapBaseOut(SwapBaseOutInstruction),

    /// Discriminator did not match any known instruction.
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseInInstruction {
    pub discriminator: u8,

    /// Amount of base token to swap in.
    pub amount_in: u64,

    /// Minimum amount of output token to receive.
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SwapBaseOutInstruction {
    pub discriminator: u8,

    /// Maximum amount of base token to swap in.
    pub max_amount_in: u64,

    /// Amount of output token to receive.
    pub amount_out: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumV4Instruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        // First byte is Raydium’s log_type discriminator
        let discriminator = data[0];
        let payload = data; // include the discriminator – Raydium structs need it

        Ok(match discriminator {
            SWAP_BASE_IN => Self::SwapBaseIn(SwapBaseInInstruction::try_from_slice(&payload[0..SWAP_LEN])?),
            SWAP_BASE_OUT => Self::SwapBaseOut(SwapBaseOutInstruction::try_from_slice(&payload[0..SWAP_LEN])?),
            other => return Err(ParseError::RaydiumUnknown(other)),
        })
    }
}

// Convenience function retaining the old name; forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumV4Instruction, ParseError> {
    RaydiumV4Instruction::try_from(data)
}
