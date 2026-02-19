//! Tensor Marketplace on-chain instruction definitions.
//!
//! Supports all NFT standards: cNFT, pNFT (legacy), Token-2022, WNS, and Metaplex Core.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (Anchor: sha256("global:<name>")[..8])
// -----------------------------------------------------------------------------

// Core operations (cNFT / Metaplex metadata)
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const BUY_SPL: [u8; 8] = [65, 136, 254, 255, 59, 130, 234, 174];
pub const LIST: [u8; 8] = [54, 174, 193, 67, 17, 41, 132, 38];
pub const DELIST: [u8; 8] = [55, 136, 205, 107, 107, 173, 4, 31];
pub const EDIT: [u8; 8] = [15, 183, 33, 86, 87, 28, 151, 145];
pub const BID: [u8; 8] = [199, 56, 85, 38, 146, 243, 37, 158];
pub const CANCEL_BID: [u8; 8] = [40, 243, 190, 217, 208, 253, 86, 206];
pub const CLOSE_EXPIRED_BID: [u8; 8] = [83, 20, 105, 67, 248, 68, 104, 190];
pub const CLOSE_EXPIRED_LISTING: [u8; 8] = [150, 70, 13, 135, 9, 204, 75, 4];
pub const TAKE_BID_META_HASH: [u8; 8] = [85, 227, 202, 70, 45, 215, 10, 193];
pub const TAKE_BID_FULL_META: [u8; 8] = [242, 194, 203, 225, 234, 53, 10, 96];
pub const TCOMP_NOOP: [u8; 8] = [106, 162, 10, 226, 132, 68, 223, 21];

// Legacy (pNFT) variants
pub const BUY_LEGACY: [u8; 8] = [68, 127, 43, 8, 212, 31, 249, 114];
pub const BUY_LEGACY_SPL: [u8; 8] = [134, 94, 125, 229, 24, 157, 194, 199];
pub const CLOSE_EXPIRED_LISTING_LEGACY: [u8; 8] = [56, 16, 96, 188, 55, 68, 250, 58];
pub const DELIST_LEGACY: [u8; 8] = [88, 35, 231, 184, 110, 218, 149, 23];
pub const LIST_LEGACY: [u8; 8] = [6, 110, 255, 18, 16, 36, 8, 30];
pub const TAKE_BID_LEGACY: [u8; 8] = [188, 35, 116, 108, 0, 233, 237, 201];

// Token-2022 variants
pub const BUY_T22: [u8; 8] = [81, 98, 227, 171, 201, 105, 180, 216];
pub const BUY_T22_SPL: [u8; 8] = [102, 21, 163, 39, 94, 39, 122, 94];
pub const CLOSE_EXPIRED_LISTING_T22: [u8; 8] = [69, 2, 190, 122, 144, 119, 122, 220];
pub const DELIST_T22: [u8; 8] = [216, 72, 73, 18, 204, 82, 123, 26];
pub const LIST_T22: [u8; 8] = [9, 117, 93, 230, 221, 4, 199, 212];
pub const TAKE_BID_T22: [u8; 8] = [18, 250, 113, 242, 31, 244, 19, 150];

// WNS variants
pub const BUY_WNS: [u8; 8] = [168, 43, 179, 217, 44, 59, 35, 244];
pub const BUY_WNS_SPL: [u8; 8] = [113, 137, 57, 23, 186, 196, 217, 210];
pub const CLOSE_EXPIRED_LISTING_WNS: [u8; 8] = [222, 31, 183, 134, 230, 207, 7, 132];
pub const DELIST_WNS: [u8; 8] = [172, 171, 57, 16, 74, 158, 32, 57];
pub const LIST_WNS: [u8; 8] = [23, 202, 102, 138, 255, 190, 39, 196];
pub const TAKE_BID_WNS: [u8; 8] = [88, 5, 122, 88, 250, 139, 35, 216];

// Metaplex Core variants
pub const BUY_CORE: [u8; 8] = [169, 227, 87, 255, 76, 86, 255, 25];
pub const BUY_CORE_SPL: [u8; 8] = [234, 28, 37, 122, 114, 239, 233, 208];
pub const CLOSE_EXPIRED_LISTING_CORE: [u8; 8] = [89, 171, 78, 80, 74, 188, 63, 58];
pub const DELIST_CORE: [u8; 8] = [56, 24, 231, 2, 227, 19, 14, 68];
pub const LIST_CORE: [u8; 8] = [173, 76, 167, 125, 118, 71, 1, 153];
pub const TAKE_BID_CORE: [u8; 8] = [250, 41, 248, 20, 61, 161, 27, 141];

// ──────────────────────────────────────────────────────────────────────────────
// Shared types
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Target {
    AssetId = 0,
    Whitelist = 1,
}

impl borsh::BorshSerialize for Target {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[*self as u8])
    }
}

impl borsh::BorshDeserialize for Target {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        match buf[0] {
            0 => Ok(Self::AssetId),
            1 => Ok(Self::Whitelist),
            v => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid Target: {v}"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Field {
    Name = 0,
}

impl borsh::BorshSerialize for Field {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[*self as u8])
    }
}

impl borsh::BorshDeserialize for Field {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        match buf[0] {
            0 => Ok(Self::Name),
            v => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid Field: {v}"))),
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Instruction enum
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TensorInstruction {
    // Listings
    List(ListInstruction),
    Delist,
    Edit(EditInstruction),
    Buy(BuyInstruction),
    BuySpl(BuyInstruction),
    CloseExpiredListing,

    // Bids
    Bid(BidInstruction),
    CancelBid,
    CloseExpiredBid,
    TakeBidMetaHash(TakeBidMetaHashInstruction),
    TakeBidFullMeta(TakeBidFullMetaInstruction),

    // Legacy (pNFT)
    ListLegacy(ListInstruction),
    DelistLegacy,
    BuyLegacy(BuyInstruction),
    BuyLegacySpl(BuyInstruction),
    CloseExpiredListingLegacy,
    TakeBidLegacy(TakeBidMetaHashInstruction),

    // Token-2022
    ListT22(ListInstruction),
    DelistT22,
    BuyT22(BuyInstruction),
    BuyT22Spl(BuyInstruction),
    CloseExpiredListingT22,
    TakeBidT22(TakeBidMetaHashInstruction),

    // WNS
    ListWns(ListInstruction),
    DelistWns,
    BuyWns(BuyInstruction),
    BuyWnsSpl(BuyInstruction),
    CloseExpiredListingWns,
    TakeBidWns(TakeBidMetaHashInstruction),

    // Metaplex Core
    ListCore(ListInstruction),
    DelistCore,
    BuyCore(BuyInstruction),
    BuyCoreSpl(BuyInstruction),
    CloseExpiredListingCore,
    TakeBidCore(TakeBidMetaHashInstruction),

    // Admin
    TcompNoop,

    Unknown,
}

// ──────────────────────────────────────────────────────────────────────────────
// Payload structs
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ListInstruction {
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<Pubkey>,
    pub private_taker: Option<Pubkey>,
    pub maker_broker: Option<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EditInstruction {
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<Pubkey>,
    pub private_taker: Option<Pubkey>,
    pub maker_broker: Option<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyInstruction {
    pub max_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BidInstruction {
    pub bid_id: Pubkey,
    pub target: Target,
    pub target_id: Pubkey,
    pub field: Option<Field>,
    pub field_id: Option<Pubkey>,
    pub amount: u64,
    pub quantity: u32,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<Pubkey>,
    pub private_taker: Option<Pubkey>,
    pub maker_broker: Option<Pubkey>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TakeBidMetaHashInstruction {
    pub min_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TakeBidFullMetaInstruction {
    pub min_amount: u64,
    pub creators: Vec<Creator>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

// ──────────────────────────────────────────────────────────────────────────────
// Borsh deserialisation
// ──────────────────────────────────────────────────────────────────────────────
impl<'a> TryFrom<&'a [u8]> for TensorInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            // Listings
            LIST => Self::List(ListInstruction::try_from_slice(payload)?),
            DELIST => Self::Delist,
            EDIT => Self::Edit(EditInstruction::try_from_slice(payload)?),
            BUY => Self::Buy(BuyInstruction::try_from_slice(payload)?),
            BUY_SPL => Self::BuySpl(BuyInstruction::try_from_slice(payload)?),
            CLOSE_EXPIRED_LISTING => Self::CloseExpiredListing,

            // Bids
            BID => Self::Bid(BidInstruction::try_from_slice(payload)?),
            CANCEL_BID => Self::CancelBid,
            CLOSE_EXPIRED_BID => Self::CloseExpiredBid,
            TAKE_BID_META_HASH => Self::TakeBidMetaHash(TakeBidMetaHashInstruction::try_from_slice(payload)?),
            TAKE_BID_FULL_META => Self::TakeBidFullMeta(TakeBidFullMetaInstruction::try_from_slice(payload)?),

            // Legacy
            LIST_LEGACY => Self::ListLegacy(ListInstruction::try_from_slice(payload)?),
            DELIST_LEGACY => Self::DelistLegacy,
            BUY_LEGACY => Self::BuyLegacy(BuyInstruction::try_from_slice(payload)?),
            BUY_LEGACY_SPL => Self::BuyLegacySpl(BuyInstruction::try_from_slice(payload)?),
            CLOSE_EXPIRED_LISTING_LEGACY => Self::CloseExpiredListingLegacy,
            TAKE_BID_LEGACY => Self::TakeBidLegacy(TakeBidMetaHashInstruction::try_from_slice(payload)?),

            // Token-2022
            LIST_T22 => Self::ListT22(ListInstruction::try_from_slice(payload)?),
            DELIST_T22 => Self::DelistT22,
            BUY_T22 => Self::BuyT22(BuyInstruction::try_from_slice(payload)?),
            BUY_T22_SPL => Self::BuyT22Spl(BuyInstruction::try_from_slice(payload)?),
            CLOSE_EXPIRED_LISTING_T22 => Self::CloseExpiredListingT22,
            TAKE_BID_T22 => Self::TakeBidT22(TakeBidMetaHashInstruction::try_from_slice(payload)?),

            // WNS
            LIST_WNS => Self::ListWns(ListInstruction::try_from_slice(payload)?),
            DELIST_WNS => Self::DelistWns,
            BUY_WNS => Self::BuyWns(BuyInstruction::try_from_slice(payload)?),
            BUY_WNS_SPL => Self::BuyWnsSpl(BuyInstruction::try_from_slice(payload)?),
            CLOSE_EXPIRED_LISTING_WNS => Self::CloseExpiredListingWns,
            TAKE_BID_WNS => Self::TakeBidWns(TakeBidMetaHashInstruction::try_from_slice(payload)?),

            // Core
            LIST_CORE => Self::ListCore(ListInstruction::try_from_slice(payload)?),
            DELIST_CORE => Self::DelistCore,
            BUY_CORE => Self::BuyCore(BuyInstruction::try_from_slice(payload)?),
            BUY_CORE_SPL => Self::BuyCoreSpl(BuyInstruction::try_from_slice(payload)?),
            CLOSE_EXPIRED_LISTING_CORE => Self::CloseExpiredListingCore,
            TAKE_BID_CORE => Self::TakeBidCore(TakeBidMetaHashInstruction::try_from_slice(payload)?),

            // Admin
            TCOMP_NOOP => Self::TcompNoop,

            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<TensorInstruction, ParseError> {
    TensorInstruction::try_from(data)
}
