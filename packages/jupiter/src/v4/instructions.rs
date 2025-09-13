//! Jupiter Aggregator v4 on-chain instructions.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SplitLegDeeper {
    pub percent: u8,
    pub swap_leg: SwapLegSwap,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SplitLeg {
    pub percent: u8,
    pub swap_leg: SwapLegDeeper,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum SwapLegSwap {
    PlaceholderOne,
    PlaceholderTwo,
    Swap { swap: Swap },
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum SwapLegDeeper {
    Chain { swap_legs: Vec<SwapLegSwap> },
    Split { split_legs: Vec<SplitLegDeeper> },
    Swap { swap: Swap },
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum SwapLeg {
    Chain { swap_legs: Vec<SwapLegDeeper> },
    Split { split_legs: Vec<SplitLeg> },
    Swap { swap: Swap },
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema { a_to_b: bool },
    Lifinity,
    Mercurial,
    Cykura,
    Serum { side: Side },
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin { side: Side },
    AldrinV2 { side: Side },
    Whirlpool { a_to_b: bool },
    Invariant { x_to_y: bool },
    Meteora,
    GooseFX,
    DeltaFi { stable: bool },
    Balansol,
    MarcoPolo { x_to_y: bool },
    Dradex { side: Side },
    LifinityV2,
    RaydiumClmm,
    Openbook { side: Side },
    Phoenix { side: Side },
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const ROUTE: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JupiterV4Instruction {
    Route(RouteInstruction),
    Whirlpoolswapexactoutput(WhirlpoolswapexactoutputInstruction),
    Raydiumswapexactoutput(RaydiumswapexactoutputInstruction),
    Raydiumclmmswapexactoutput(RaydiumclmmswapexactoutputInstruction),
    Createopenorders,
    Createopenorderv2,
    Mercurialswap,
    Cykuraswap,
    Serumswap,
    Saberswap,
    Saberadddecimals,
    Tokenswap,
    Senchaswap,
    Stepswap,
    Cropperswap,
    Raydiumswap,
    Cremaswap,
    Lifinityswap,
    Marinadedeposit,
    Marinadeunstake,
    Aldrinswap,
    Aldrinv2swap,
    Whirlpoolswap,
    Invariantswap,
    Meteoraswap,
    Goosefxswap,
    Deltafiswap,
    Balansolswap,
    Marcopoloswap,
    Dradexswap,
    Lifinityv2swap,
    Raydiumclmmswap,
    Phoenixswap,
    Claimbonk,
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RouteInstruction {
    pub swap_leg: SwapLeg,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WhirlpoolswapexactoutputInstruction {
    pub out_amount: u64,
    pub in_amount_with_slippage: AmountWithSlippage,
    pub a_to_b: bool,
    pub platform_fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RaydiumswapexactoutputInstruction {
    pub out_amount: u64,
    pub in_amount_with_slippage: AmountWithSlippage,
    pub platform_fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RaydiumclmmswapexactoutputInstruction {
    pub out_amount: u64,
    pub in_amount_with_slippage: AmountWithSlippage,
    pub platform_fee_bps: u8,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for JupiterV4Instruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            ROUTE => Self::Route(RouteInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<JupiterV4Instruction, ParseError> {
    JupiterV4Instruction::try_from(data)
}
