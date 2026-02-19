//! Dumpfun on-chain instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RampingLimit {
    pub start_sec: i64,
    pub end_sec: i64,
    pub limit_bps: u16,
}

// Discriminators
pub const BUY_EXACT_TOKENS: [u8; 8] = [129, 145, 209, 75, 88, 169, 142, 8];
pub const BUY_TOKENS_WITH_EXACT_SOL: [u8; 8] = [107, 198, 250, 226, 105, 48, 89, 135];
pub const DRAIN_POOL_SURPLUS: [u8; 8] = [172, 123, 233, 0, 98, 159, 144, 191];
pub const MINT: [u8; 8] = [51, 57, 225, 47, 182, 146, 137, 166];
pub const SELL_EXACT_TOKENS: [u8; 8] = [34, 198, 103, 105, 28, 0, 138, 92];

#[derive(Debug, Clone, PartialEq)]
pub enum DumpfunInstruction {
    BuyExactTokens(BuyExactTokensInstruction),
    BuyTokensWithExactSol(BuyTokensWithExactSolInstruction),
    DrainPoolSurplus,
    Mint(MintInstruction),
    SellExactTokens(SellExactTokensInstruction),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyExactTokensInstruction {
    pub token_out: u64,
    pub max_sol_in: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyTokensWithExactSolInstruction {
    pub sol_in: u64,
    pub min_token_out: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MintInstruction {
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub sell_lock_period: i64,
    pub virtual_sol_reserve: u64,
    pub ramping_limits: Vec<RampingLimit>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellExactTokensInstruction {
    pub token_in: u64,
    pub min_sol_out: u64,
}

impl<'a> TryFrom<&'a [u8]> for DumpfunInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            BUY_EXACT_TOKENS => Self::BuyExactTokens(BuyExactTokensInstruction::try_from_slice(payload)?),
            BUY_TOKENS_WITH_EXACT_SOL => Self::BuyTokensWithExactSol(BuyTokensWithExactSolInstruction::try_from_slice(payload)?),
            DRAIN_POOL_SURPLUS => Self::DrainPoolSurplus,
            MINT => Self::Mint(MintInstruction::try_from_slice(payload)?),
            SELL_EXACT_TOKENS => Self::SellExactTokens(SellExactTokensInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<DumpfunInstruction, ParseError> {
    DumpfunInstruction::try_from(data)
}
