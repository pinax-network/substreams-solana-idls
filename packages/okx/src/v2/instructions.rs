//! OKX Dex v2 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// -----------------------------------------------------------------------------
// Custom types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapArgs {
    pub amount_in: u64,
    pub expect_amount_out: u64,
    pub min_return: u64,
    pub amounts: Vec<u64>,
    pub routes: Vec<Vec<Route>>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Route {
    pub dexes: Vec<Dex>,
    pub weights: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Dex {
    SplTokenSwap,
    StableSwap,
    Whirlpool,
    MeteoraDynamicpool,
    RaydiumSwap,
    RaydiumStableSwap,
    RaydiumClmmSwap,
    AldrinExchangeV1,
    AldrinExchangeV2,
    LifinityV1,
    LifinityV2,
    RaydiumClmmSwapV2,
    FluxBeam,
    MeteoraDlmm,
    RaydiumCpmmSwap,
    OpenBookV2,
    WhirlpoolV2,
    Phoenix,
    ObricV2,
    SanctumAddLiq,
    SanctumRemoveLiq,
    SanctumNonWsolSwap,
    SanctumWsolSwap,
    PumpfunBuy,
    PumpfunSell,
    StabbleSwap,
    SanctumRouter,
    MeteoraVaultDeposit,
    MeteoraVaultWithdraw,
    Saros,
    MeteoraLst,
    Solfi,
    QualiaSwap,
    Zerofi,
    PumpfunammBuy,
    PumpfunammSell,
    Virtuals,
    VertigoBuy,
    VertigoSell,
    PerpetualsAddLiq,
    PerpetualsRemoveLiq,
    PerpetualsSwap,
    RaydiumLaunchpad,
    LetsBonkFun,
    Woofi,
    MeteoraDbc,
    MeteoraDlmmSwap2,
    MeteoraDAMMV2,
    Gavel,
    BoopfunBuy,
    BoopfunSell,
    MeteoraDbc2,
    GooseFX,
    Dooar,
    Numeraire,
    SaberDecimalWrapperDeposit,
    SaberDecimalWrapperWithdraw,
    SarosDlmm,
    OneDexSwap,
    Manifest,
    ByrealClmm,
    PancakeSwapV3Swap,
    PancakeSwapV3SwapV2,
    Tessera,
    SolRfq,
    PumpfunBuy2,
    PumpfunammBuy2,
    Humidifi,
    HeavenBuy,
    HeavenSell,
    SolfiV2,
    PumpfunBuy3,
    PumpfunSell3,
    PumpfunammBuy3,
    PumpfunammSell3,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const SWAP_V3: [u8; 8] = [240, 224, 38, 33, 176, 31, 241, 175];

// -----------------------------------------------------------------------------
// Instruction payloads
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapV3Instruction {
    pub args: SwapArgs,
    pub commission_info: u32,
    pub platform_fee_rate: u16,
    pub order_id: u64,
}

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OkxV2Instruction {
    SwapV3(SwapV3Instruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for OkxV2Instruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            SWAP_V3 => Self::SwapV3(SwapV3Instruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<OkxV2Instruction, ParseError> {
    OkxV2Instruction::try_from(data)
}
