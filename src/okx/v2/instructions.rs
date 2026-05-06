//! OKX Dex v2 on-chain instructions.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;

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

#[derive(Debug, Clone, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SolRfqArgs {
    pub rfq_id: u64,
    pub expected_maker_amount: u64,
    pub expected_taker_amount: u64,
    pub maker_send_amount: u64,
    pub taker_send_amount: u64,
    pub expiry: u64,
    pub maker_use_native_sol: bool,
    pub taker_use_native_sol: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SugarMoneyArgs {
    pub bonding_curve_bump: u8,
    pub bonding_curve_sol_associated_account_bump: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct HumidifiSwap2Args {
    pub swap_id: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ScorchArgs {
    pub id: u128,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SanctumPrefundSwapViaStakeArgs {
    pub swap_id: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WhalestreetV2Args {
    pub auth_amount_in: u64,
    pub auth: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SolfiV2WithSigArgs {
    pub unix_timestamp: u64,
    pub msg_amount_in: u64,
    pub expect_amount_out: u64,
    pub slippage: u16,
    pub user_request_slot: u64,
    pub signature: [u8; 64],
    pub recovery_id: u8,
}

impl Default for SolfiV2WithSigArgs {
    fn default() -> Self {
        Self {
            unix_timestamp: 0,
            msg_amount_in: 0,
            expect_amount_out: 0,
            slippage: 0,
            user_request_slot: 0,
            signature: [0; 64],
            recovery_id: 0,
        }
    }
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
    SolRfq(SolRfqArgs),
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
    Goonfi,
    MoonitBuy,
    MoonitSell,
    RaydiumSwapV2,
    Whalestreet,
    SugarMoneyBuy(SugarMoneyArgs),
    SugarMoneySell(SugarMoneyArgs),
    MeteoraDAMMV2Swap2,
    AlphaQ,
    FutarchyAmm,
    PumpfunSell2,
    HumidifiSwap2(HumidifiSwap2Args),
    Scorch(ScorchArgs),
    JupiterLendDeposit,
    JupiterLendRedeem,
    TaurusFi,
    BisonFi,
    GoonfiV2,
    Quantum,
    BoopfunBuy2,
    BoopfunSell2,
    ByrealClmm2,
    Dooar2,
    HeavenBuy2,
    HeavenSell2,
    MoonitBuy2,
    MoonitSell2,
    SaberDecimalWrapperDeposit2,
    SaberDecimalWrapperWithdraw2,
    ByrealPropAmm,
    SanctumPrefundSwapViaStake(SanctumPrefundSwapViaStakeArgs),
    AbyssAmm,
    Aquifer,
    WhalestreetV2(WhalestreetV2Args),
    SolfiV2WithSig(SolfiV2WithSigArgs),
}

pub const DEX_VARIANT_NAMES: &[&str] = &[
    "SplTokenSwap",
    "StableSwap",
    "Whirlpool",
    "MeteoraDynamicpool",
    "RaydiumSwap",
    "RaydiumStableSwap",
    "RaydiumClmmSwap",
    "AldrinExchangeV1",
    "AldrinExchangeV2",
    "LifinityV1",
    "LifinityV2",
    "RaydiumClmmSwapV2",
    "FluxBeam",
    "MeteoraDlmm",
    "RaydiumCpmmSwap",
    "OpenBookV2",
    "WhirlpoolV2",
    "Phoenix",
    "ObricV2",
    "SanctumAddLiq",
    "SanctumRemoveLiq",
    "SanctumNonWsolSwap",
    "SanctumWsolSwap",
    "PumpfunBuy",
    "PumpfunSell",
    "StabbleSwap",
    "SanctumRouter",
    "MeteoraVaultDeposit",
    "MeteoraVaultWithdraw",
    "Saros",
    "MeteoraLst",
    "Solfi",
    "QualiaSwap",
    "Zerofi",
    "PumpfunammBuy",
    "PumpfunammSell",
    "Virtuals",
    "VertigoBuy",
    "VertigoSell",
    "PerpetualsAddLiq",
    "PerpetualsRemoveLiq",
    "PerpetualsSwap",
    "RaydiumLaunchpad",
    "LetsBonkFun",
    "Woofi",
    "MeteoraDbc",
    "MeteoraDlmmSwap2",
    "MeteoraDAMMV2",
    "Gavel",
    "BoopfunBuy",
    "BoopfunSell",
    "MeteoraDbc2",
    "GooseFX",
    "Dooar",
    "Numeraire",
    "SaberDecimalWrapperDeposit",
    "SaberDecimalWrapperWithdraw",
    "SarosDlmm",
    "OneDexSwap",
    "Manifest",
    "ByrealClmm",
    "PancakeSwapV3Swap",
    "PancakeSwapV3SwapV2",
    "Tessera",
    "SolRfq",
    "PumpfunBuy2",
    "PumpfunammBuy2",
    "Humidifi",
    "HeavenBuy",
    "HeavenSell",
    "SolfiV2",
    "PumpfunBuy3",
    "PumpfunSell3",
    "PumpfunammBuy3",
    "PumpfunammSell3",
    "Goonfi",
    "MoonitBuy",
    "MoonitSell",
    "RaydiumSwapV2",
    "Whalestreet",
    "SugarMoneyBuy",
    "SugarMoneySell",
    "MeteoraDAMMV2Swap2",
    "AlphaQ",
    "FutarchyAmm",
    "PumpfunSell2",
    "HumidifiSwap2",
    "Scorch",
    "JupiterLendDeposit",
    "JupiterLendRedeem",
    "TaurusFi",
    "BisonFi",
    "GoonfiV2",
    "Quantum",
    "BoopfunBuy2",
    "BoopfunSell2",
    "ByrealClmm2",
    "Dooar2",
    "HeavenBuy2",
    "HeavenSell2",
    "MoonitBuy2",
    "MoonitSell2",
    "SaberDecimalWrapperDeposit2",
    "SaberDecimalWrapperWithdraw2",
    "ByrealPropAmm",
    "SanctumPrefundSwapViaStake",
    "AbyssAmm",
    "Aquifer",
    "WhalestreetV2",
    "SolfiV2WithSig",
];

impl FromStr for Dex {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let variant = value.split_once(" {").map(|(name, _)| name).unwrap_or(value);
        Ok(match variant {
            "SplTokenSwap" => Self::SplTokenSwap,
            "StableSwap" => Self::StableSwap,
            "Whirlpool" => Self::Whirlpool,
            "MeteoraDynamicpool" => Self::MeteoraDynamicpool,
            "RaydiumSwap" => Self::RaydiumSwap,
            "RaydiumStableSwap" => Self::RaydiumStableSwap,
            "RaydiumClmmSwap" => Self::RaydiumClmmSwap,
            "AldrinExchangeV1" => Self::AldrinExchangeV1,
            "AldrinExchangeV2" => Self::AldrinExchangeV2,
            "LifinityV1" => Self::LifinityV1,
            "LifinityV2" => Self::LifinityV2,
            "RaydiumClmmSwapV2" => Self::RaydiumClmmSwapV2,
            "FluxBeam" => Self::FluxBeam,
            "MeteoraDlmm" => Self::MeteoraDlmm,
            "RaydiumCpmmSwap" => Self::RaydiumCpmmSwap,
            "OpenBookV2" => Self::OpenBookV2,
            "WhirlpoolV2" => Self::WhirlpoolV2,
            "Phoenix" => Self::Phoenix,
            "ObricV2" => Self::ObricV2,
            "SanctumAddLiq" => Self::SanctumAddLiq,
            "SanctumRemoveLiq" => Self::SanctumRemoveLiq,
            "SanctumNonWsolSwap" => Self::SanctumNonWsolSwap,
            "SanctumWsolSwap" => Self::SanctumWsolSwap,
            "PumpfunBuy" => Self::PumpfunBuy,
            "PumpfunSell" => Self::PumpfunSell,
            "StabbleSwap" => Self::StabbleSwap,
            "SanctumRouter" => Self::SanctumRouter,
            "MeteoraVaultDeposit" => Self::MeteoraVaultDeposit,
            "MeteoraVaultWithdraw" => Self::MeteoraVaultWithdraw,
            "Saros" => Self::Saros,
            "MeteoraLst" => Self::MeteoraLst,
            "Solfi" => Self::Solfi,
            "QualiaSwap" => Self::QualiaSwap,
            "Zerofi" => Self::Zerofi,
            "PumpfunammBuy" => Self::PumpfunammBuy,
            "PumpfunammSell" => Self::PumpfunammSell,
            "Virtuals" => Self::Virtuals,
            "VertigoBuy" => Self::VertigoBuy,
            "VertigoSell" => Self::VertigoSell,
            "PerpetualsAddLiq" => Self::PerpetualsAddLiq,
            "PerpetualsRemoveLiq" => Self::PerpetualsRemoveLiq,
            "PerpetualsSwap" => Self::PerpetualsSwap,
            "RaydiumLaunchpad" => Self::RaydiumLaunchpad,
            "LetsBonkFun" => Self::LetsBonkFun,
            "Woofi" => Self::Woofi,
            "MeteoraDbc" => Self::MeteoraDbc,
            "MeteoraDlmmSwap2" => Self::MeteoraDlmmSwap2,
            "MeteoraDAMMV2" => Self::MeteoraDAMMV2,
            "Gavel" => Self::Gavel,
            "BoopfunBuy" => Self::BoopfunBuy,
            "BoopfunSell" => Self::BoopfunSell,
            "MeteoraDbc2" => Self::MeteoraDbc2,
            "GooseFX" => Self::GooseFX,
            "Dooar" => Self::Dooar,
            "Numeraire" => Self::Numeraire,
            "SaberDecimalWrapperDeposit" => Self::SaberDecimalWrapperDeposit,
            "SaberDecimalWrapperWithdraw" => Self::SaberDecimalWrapperWithdraw,
            "SarosDlmm" => Self::SarosDlmm,
            "OneDexSwap" => Self::OneDexSwap,
            "Manifest" => Self::Manifest,
            "ByrealClmm" => Self::ByrealClmm,
            "PancakeSwapV3Swap" => Self::PancakeSwapV3Swap,
            "PancakeSwapV3SwapV2" => Self::PancakeSwapV3SwapV2,
            "Tessera" => Self::Tessera,
            "SolRfq" => Self::SolRfq(SolRfqArgs::default()),
            "PumpfunBuy2" => Self::PumpfunBuy2,
            "PumpfunammBuy2" => Self::PumpfunammBuy2,
            "Humidifi" => Self::Humidifi,
            "HeavenBuy" => Self::HeavenBuy,
            "HeavenSell" => Self::HeavenSell,
            "SolfiV2" => Self::SolfiV2,
            "PumpfunBuy3" => Self::PumpfunBuy3,
            "PumpfunSell3" => Self::PumpfunSell3,
            "PumpfunammBuy3" => Self::PumpfunammBuy3,
            "PumpfunammSell3" => Self::PumpfunammSell3,
            "Goonfi" => Self::Goonfi,
            "MoonitBuy" => Self::MoonitBuy,
            "MoonitSell" => Self::MoonitSell,
            "RaydiumSwapV2" => Self::RaydiumSwapV2,
            "Whalestreet" => Self::Whalestreet,
            "SugarMoneyBuy" => Self::SugarMoneyBuy(SugarMoneyArgs::default()),
            "SugarMoneySell" => Self::SugarMoneySell(SugarMoneyArgs::default()),
            "MeteoraDAMMV2Swap2" => Self::MeteoraDAMMV2Swap2,
            "AlphaQ" => Self::AlphaQ,
            "FutarchyAmm" => Self::FutarchyAmm,
            "PumpfunSell2" => Self::PumpfunSell2,
            "HumidifiSwap2" => Self::HumidifiSwap2(HumidifiSwap2Args::default()),
            "Scorch" => Self::Scorch(ScorchArgs::default()),
            "JupiterLendDeposit" => Self::JupiterLendDeposit,
            "JupiterLendRedeem" => Self::JupiterLendRedeem,
            "TaurusFi" => Self::TaurusFi,
            "BisonFi" => Self::BisonFi,
            "GoonfiV2" => Self::GoonfiV2,
            "Quantum" => Self::Quantum,
            "BoopfunBuy2" => Self::BoopfunBuy2,
            "BoopfunSell2" => Self::BoopfunSell2,
            "ByrealClmm2" => Self::ByrealClmm2,
            "Dooar2" => Self::Dooar2,
            "HeavenBuy2" => Self::HeavenBuy2,
            "HeavenSell2" => Self::HeavenSell2,
            "MoonitBuy2" => Self::MoonitBuy2,
            "MoonitSell2" => Self::MoonitSell2,
            "SaberDecimalWrapperDeposit2" => Self::SaberDecimalWrapperDeposit2,
            "SaberDecimalWrapperWithdraw2" => Self::SaberDecimalWrapperWithdraw2,
            "ByrealPropAmm" => Self::ByrealPropAmm,
            "SanctumPrefundSwapViaStake" => Self::SanctumPrefundSwapViaStake(SanctumPrefundSwapViaStakeArgs::default()),
            "AbyssAmm" => Self::AbyssAmm,
            "Aquifer" => Self::Aquifer,
            "WhalestreetV2" => Self::WhalestreetV2(WhalestreetV2Args::default()),
            "SolfiV2WithSig" => Self::SolfiV2WithSig(SolfiV2WithSigArgs::default()),
            "SanctumSwapWithoutWsol" => Self::SanctumNonWsolSwap,
            _ => return Err(ParseError::InvalidLength { expected: 0, got: value.len() }),
        })
    }
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

impl SwapV3Instruction {
    fn unpack(payload: &[u8]) -> Result<Self, ParseError> {
        let mut reader = payload;
        let instruction = Self::deserialize(&mut reader)?;

        // Mainnet `swap_v3` payloads have been observed with a trailing zero
        // word after the IDL-defined fields. Keep non-zero trailing bytes strict.
        if reader.iter().any(|byte| *byte != 0) {
            return Err(ParseError::InvalidLength {
                expected: payload.len() - reader.len(),
                got: payload.len(),
            });
        }

        Ok(instruction)
    }
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
            SWAP_V3 => Self::SwapV3(SwapV3Instruction::unpack(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<OkxV2Instruction, ParseError> {
    OkxV2Instruction::try_from(data)
}
