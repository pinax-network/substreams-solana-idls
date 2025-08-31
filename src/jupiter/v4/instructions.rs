//! Jupiter Aggregator v4 on-chain instructions.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SplitLegDeeper {
    pub percent: u8,
    pub swap_leg: SwapLegSwap,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SplitLeg {
    pub percent: u8,
    pub swap_leg: SwapLegDeeper,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum SwapLegSwap {
    PlaceholderOne,
    PlaceholderTwo,
    Swap { swap: Swap },
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum SwapLegDeeper {
    Chain { swap_legs: Vec<SwapLegSwap> },
    Split { split_legs: Vec<SplitLegDeeper> },
    Swap { swap: Swap },
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum SwapLeg {
    Chain { swap_legs: Vec<SwapLegDeeper> },
    Split { split_legs: Vec<SplitLeg> },
    Swap { swap: Swap },
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
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
pub const WHIRLPOOLSWAPEXACTOUTPUT: [u8; 8] = [202, 223, 84, 36, 215, 74, 85, 49];
pub const RAYDIUMSWAPEXACTOUTPUT: [u8; 8] = [57, 218, 51, 159, 206, 80, 125, 49];
pub const RAYDIUMCLMMSWAPEXACTOUTPUT: [u8; 8] = [169, 250, 54, 119, 22, 40, 144, 233];
pub const CREATEOPENORDERS: [u8; 8] = [74, 221, 179, 211, 73, 19, 243, 196];
pub const CREATEOPENORDERV2: [u8; 8] = [196, 152, 190, 145, 174, 166, 228, 213];
pub const MERCURIALSWAP: [u8; 8] = [149, 185, 29, 185, 85, 17, 177, 46];
pub const CYKURASWAP: [u8; 8] = [146, 141, 97, 210, 219, 131, 156, 229];
pub const SERUMSWAP: [u8; 8] = [214, 156, 12, 138, 245, 125, 105, 5];
pub const SABERSWAP: [u8; 8] = [123, 129, 154, 92, 165, 1, 191, 127];
pub const SABERADDDECIMALS: [u8; 8] = [243, 0, 129, 241, 94, 15, 19, 183];
pub const TOKENSWAP: [u8; 8] = [220, 69, 2, 104, 103, 131, 45, 62];
pub const SENCHASWAP: [u8; 8] = [135, 233, 214, 130, 150, 6, 109, 189];
pub const STEPSWAP: [u8; 8] = [153, 234, 44, 115, 254, 247, 114, 208];
pub const CROPPERSWAP: [u8; 8] = [83, 253, 54, 152, 17, 98, 19, 240];
pub const RAYDIUMSWAP: [u8; 8] = [112, 235, 170, 105, 219, 12, 140, 83];
pub const CREMASWAP: [u8; 8] = [34, 127, 167, 7, 75, 249, 243, 237];
pub const LIFINITYSWAP: [u8; 8] = [183, 166, 76, 79, 88, 200, 51, 22];
pub const MARINADEDEPOSIT: [u8; 8] = [179, 217, 105, 11, 111, 105, 175, 19];
pub const MARINADEUNSTAKE: [u8; 8] = [11, 163, 97, 69, 194, 144, 45, 148];
pub const ALDRINSWAP: [u8; 8] = [47, 218, 175, 129, 237, 6, 110, 159];
pub const ALDRINV2SWAP: [u8; 8] = [128, 98, 252, 246, 215, 95, 234, 146];
pub const WHIRLPOOLSWAP: [u8; 8] = [77, 229, 199, 187, 92, 145, 7, 134];
pub const INVARIANTSWAP: [u8; 8] = [7, 30, 184, 54, 247, 107, 242, 223];
pub const METEORASWAP: [u8; 8] = [154, 10, 145, 107, 182, 195, 218, 225];
pub const GOOSEFXSWAP: [u8; 8] = [233, 190, 87, 77, 91, 160, 139, 197];
pub const DELTAFISWAP: [u8; 8] = [204, 130, 88, 121, 173, 173, 90, 38];
pub const BALANSOLSWAP: [u8; 8] = [143, 149, 77, 232, 51, 84, 103, 65];
pub const MARCOPOLOSWAP: [u8; 8] = [165, 90, 4, 22, 15, 105, 34, 114];
pub const DRADEXSWAP: [u8; 8] = [138, 209, 181, 213, 183, 223, 202, 187];
pub const LIFINITYV2SWAP: [u8; 8] = [15, 168, 80, 238, 128, 245, 155, 117];
pub const RAYDIUMCLMMSWAP: [u8; 8] = [186, 235, 91, 26, 238, 128, 195, 125];
pub const PHOENIXSWAP: [u8; 8] = [203, 194, 12, 192, 232, 82, 28, 146];
pub const CLAIMBONK: [u8; 8] = [120, 40, 0, 162, 21, 239, 195, 164];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RouteInstruction {
    pub swap_leg: SwapLeg,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WhirlpoolswapexactoutputInstruction {
    pub out_amount: u64,
    pub in_amount_with_slippage: AmountWithSlippage,
    pub a_to_b: bool,
    pub platform_fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RaydiumswapexactoutputInstruction {
    pub out_amount: u64,
    pub in_amount_with_slippage: AmountWithSlippage,
    pub platform_fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
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
            WHIRLPOOLSWAPEXACTOUTPUT => Self::Whirlpoolswapexactoutput(WhirlpoolswapexactoutputInstruction::try_from_slice(payload)?),
            RAYDIUMSWAPEXACTOUTPUT => Self::Raydiumswapexactoutput(RaydiumswapexactoutputInstruction::try_from_slice(payload)?),
            RAYDIUMCLMMSWAPEXACTOUTPUT => Self::Raydiumclmmswapexactoutput(RaydiumclmmswapexactoutputInstruction::try_from_slice(payload)?),
            CREATEOPENORDERS => Self::Createopenorders,
            CREATEOPENORDERV2 => Self::Createopenorderv2,
            MERCURIALSWAP => Self::Mercurialswap,
            CYKURASWAP => Self::Cykuraswap,
            SERUMSWAP => Self::Serumswap,
            SABERSWAP => Self::Saberswap,
            SABERADDDECIMALS => Self::Saberadddecimals,
            TOKENSWAP => Self::Tokenswap,
            SENCHASWAP => Self::Senchaswap,
            STEPSWAP => Self::Stepswap,
            CROPPERSWAP => Self::Cropperswap,
            RAYDIUMSWAP => Self::Raydiumswap,
            CREMASWAP => Self::Cremaswap,
            LIFINITYSWAP => Self::Lifinityswap,
            MARINADEDEPOSIT => Self::Marinadedeposit,
            MARINADEUNSTAKE => Self::Marinadeunstake,
            ALDRINSWAP => Self::Aldrinswap,
            ALDRINV2SWAP => Self::Aldrinv2swap,
            WHIRLPOOLSWAP => Self::Whirlpoolswap,
            INVARIANTSWAP => Self::Invariantswap,
            METEORASWAP => Self::Meteoraswap,
            GOOSEFXSWAP => Self::Goosefxswap,
            DELTAFISWAP => Self::Deltafiswap,
            BALANSOLSWAP => Self::Balansolswap,
            MARCOPOLOSWAP => Self::Marcopoloswap,
            DRADEXSWAP => Self::Dradexswap,
            LIFINITYV2SWAP => Self::Lifinityv2swap,
            RAYDIUMCLMMSWAP => Self::Raydiumclmmswap,
            PHOENIXSWAP => Self::Phoenixswap,
            CLAIMBONK => Self::Claimbonk,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<JupiterV4Instruction, ParseError> {
    JupiterV4Instruction::try_from(data)
}
