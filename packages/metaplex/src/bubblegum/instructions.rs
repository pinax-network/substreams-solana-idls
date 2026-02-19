//! Metaplex Bubblegum on-chain instructions.
//!
//! Anchor program using sha256("global:<snake_case_name>")[..8] discriminators.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;

// ── Discriminator constants ─────────────────────────────────────────────
pub const BURN: [u8; 8] = [116, 110, 29, 56, 107, 219, 42, 93];
pub const CANCEL_REDEEM: [u8; 8] = [111, 76, 232, 50, 39, 175, 48, 242];
pub const CREATE_TREE: [u8; 8] = [165, 83, 136, 142, 89, 202, 47, 220];
pub const DECOMPRESS_V1: [u8; 8] = [54, 85, 76, 70, 228, 250, 164, 81];
pub const DELEGATE: [u8; 8] = [90, 147, 75, 178, 85, 88, 4, 137];
pub const MINT_TO_COLLECTION_V1: [u8; 8] = [153, 18, 178, 47, 197, 158, 86, 15];
pub const MINT_V1: [u8; 8] = [145, 98, 192, 118, 184, 147, 118, 104];
pub const REDEEM: [u8; 8] = [184, 12, 86, 149, 70, 196, 97, 225];
pub const SET_AND_VERIFY_COLLECTION: [u8; 8] = [235, 242, 121, 216, 158, 234, 180, 234];
pub const SET_DECOMPRESSIBLE_STATE: [u8; 8] = [82, 104, 152, 6, 149, 111, 100, 13];
pub const SET_TREE_DELEGATE: [u8; 8] = [253, 118, 66, 37, 190, 49, 154, 102];
pub const TRANSFER: [u8; 8] = [163, 52, 200, 231, 140, 3, 69, 186];
pub const UNVERIFY_COLLECTION: [u8; 8] = [250, 251, 42, 106, 41, 137, 186, 168];
pub const UNVERIFY_CREATOR: [u8; 8] = [107, 178, 57, 39, 105, 115, 112, 152];
pub const VERIFY_COLLECTION: [u8; 8] = [56, 113, 101, 253, 79, 55, 122, 169];
pub const VERIFY_CREATOR: [u8; 8] = [52, 17, 96, 132, 71, 4, 85, 194];
pub const UPDATE_METADATA: [u8; 8] = [170, 182, 43, 239, 97, 78, 225, 186];

// ── Instruction enum ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum BubblegumInstruction {
    Burn,
    CancelRedeem,
    CreateTree,
    DecompressV1,
    Delegate,
    MintToCollectionV1,
    MintV1,
    Redeem,
    SetAndVerifyCollection,
    SetDecompressibleState,
    SetTreeDelegate,
    Transfer,
    UnverifyCollection,
    UnverifyCreator,
    VerifyCollection,
    VerifyCreator,
    UpdateMetadata,
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for BubblegumInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, _payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            BURN => Self::Burn,
            CANCEL_REDEEM => Self::CancelRedeem,
            CREATE_TREE => Self::CreateTree,
            DECOMPRESS_V1 => Self::DecompressV1,
            DELEGATE => Self::Delegate,
            MINT_TO_COLLECTION_V1 => Self::MintToCollectionV1,
            MINT_V1 => Self::MintV1,
            REDEEM => Self::Redeem,
            SET_AND_VERIFY_COLLECTION => Self::SetAndVerifyCollection,
            SET_DECOMPRESSIBLE_STATE => Self::SetDecompressibleState,
            SET_TREE_DELEGATE => Self::SetTreeDelegate,
            TRANSFER => Self::Transfer,
            UNVERIFY_COLLECTION => Self::UnverifyCollection,
            UNVERIFY_CREATOR => Self::UnverifyCreator,
            VERIFY_COLLECTION => Self::VerifyCollection,
            VERIFY_CREATOR => Self::VerifyCreator,
            UPDATE_METADATA => Self::UpdateMetadata,
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<BubblegumInstruction, ParseError> {
    BubblegumInstruction::try_from(data)
}
