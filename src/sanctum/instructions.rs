//! Sanctum Router on-chain instruction definitions.
//!
//! Single-byte discriminator followed by a u64 amount (little-endian) for most instructions.

use crate::common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (sequential u8 indices)
// -----------------------------------------------------------------------------
pub const STAKE_WRAPPED_SOL: u8 = 0;
pub const SWAP_VIA_STAKE: u8 = 1;
pub const CREATE_FEE_TOKEN_ACCOUNT: u8 = 2;
pub const CLOSE_FEE_TOKEN_ACCOUNT: u8 = 3;
pub const WITHDRAW_FEES: u8 = 4;
pub const DEPOSIT_STAKE: u8 = 5;
pub const PREFUND_WITHDRAW_STAKE: u8 = 6;
pub const PREFUND_SWAP_VIA_STAKE: u8 = 7;
pub const WITHDRAW_WRAPPED_SOL: u8 = 8;

// ──────────────────────────────────────────────────────────────────────────────
// Instruction enum
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SanctumInstruction {
    /// Stake wSOL → LST (wrap SOL, stake via pool, receive LST).
    StakeWrappedSol {
        amount: u64,
    },

    /// Swap one LST for another via an intermediate stake account.
    SwapViaStake {
        amount: u64,
    },

    /// Create a fee token account for a given mint.
    CreateFeeTokenAccount,

    /// Close a fee token account.
    CloseFeeTokenAccount,

    /// Withdraw accumulated fees.
    WithdrawFees,

    /// Deposit a stake account → LST.
    DepositStake,

    /// Pre-fund a withdraw-stake operation.
    PrefundWithdrawStake {
        amount: u64,
    },

    /// Pre-fund a swap-via-stake operation.
    PrefundSwapViaStake {
        amount: u64,
    },

    /// Withdraw wSOL (unwrap LST → wSOL).
    WithdrawWrappedSol {
        amount: u64,
    },

    Unknown,
}

// ──────────────────────────────────────────────────────────────────────────────
// Deserialisation
// ──────────────────────────────────────────────────────────────────────────────

fn read_u64(data: &[u8]) -> Result<u64, ParseError> {
    if data.len() < 8 {
        return Err(ParseError::TooShort(data.len()));
    }
    Ok(u64::from_le_bytes(data[..8].try_into().unwrap()))
}

impl<'a> TryFrom<&'a [u8]> for SanctumInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(0));
        }

        let disc = data[0];
        let payload = &data[1..];

        Ok(match disc {
            STAKE_WRAPPED_SOL => Self::StakeWrappedSol { amount: read_u64(payload)? },
            SWAP_VIA_STAKE => Self::SwapViaStake { amount: read_u64(payload)? },
            CREATE_FEE_TOKEN_ACCOUNT => Self::CreateFeeTokenAccount,
            CLOSE_FEE_TOKEN_ACCOUNT => Self::CloseFeeTokenAccount,
            WITHDRAW_FEES => Self::WithdrawFees,
            DEPOSIT_STAKE => Self::DepositStake,
            PREFUND_WITHDRAW_STAKE => Self::PrefundWithdrawStake { amount: read_u64(payload)? },
            PREFUND_SWAP_VIA_STAKE => Self::PrefundSwapViaStake { amount: read_u64(payload)? },
            WITHDRAW_WRAPPED_SOL => Self::WithdrawWrappedSol { amount: read_u64(payload)? },
            other => return Err(ParseError::Unknown([other, 0, 0, 0, 0, 0, 0, 0])),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<SanctumInstruction, ParseError> {
    SanctumInstruction::try_from(data)
}
