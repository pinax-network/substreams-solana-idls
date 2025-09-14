//! DFlow Aggregator v4 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OrchestratorFlags {
    pub flags: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MeteoraDlmmSwapOptions {
    pub amount: u64,
    pub num_bin_arrays: u8,
    pub orchestrator_flags: OrchestratorFlags,
}

// Placeholder type for variants we don't explicitly support.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Empty;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Action {
    WhirlpoolsSwap(Empty),
    ClearpoolsSwap(Empty),
    RaydiumAmmSwap(Empty),
    LifinityV2Swap(Empty),
    MeteoraDlmmSwap(MeteoraDlmmSwapOptions),
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapParams {
    pub actions: Vec<Action>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub params: SwapParams,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DflowV4Instruction {
    Swap(SwapInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for DflowV4Instruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            _ => Self::Unknown,
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<DflowV4Instruction, ParseError> {
    DflowV4Instruction::try_from(data)
}
