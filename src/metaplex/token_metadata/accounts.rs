//! Metaplex Token Metadata on-chain accounts.

use crate::common::ParseError;

pub use mpl_token_metadata::accounts::{
    DeprecatedMasterEditionV1, Edition, EditionMarker, EditionMarkerV2, MasterEdition, Metadata,
};
pub use mpl_token_metadata::types::TokenStandard;

// Account kind values come from the Metaplex `Key` enum stored as the first byte.
pub const UNINITIALIZED_ACCOUNT: u8 = 0;
pub const EDITION_ACCOUNT: u8 = 1;
pub const MASTER_EDITION_V1_ACCOUNT: u8 = 2;
pub const RESERVATION_LIST_V1_ACCOUNT: u8 = 3;
pub const METADATA_ACCOUNT: u8 = 4;
pub const RESERVATION_LIST_V2_ACCOUNT: u8 = 5;
pub const MASTER_EDITION_V2_ACCOUNT: u8 = 6;
pub const EDITION_MARKER_ACCOUNT: u8 = 7;
pub const USE_AUTHORITY_RECORD_ACCOUNT: u8 = 8;
pub const COLLECTION_AUTHORITY_RECORD_ACCOUNT: u8 = 9;
pub const TOKEN_OWNED_ESCROW_ACCOUNT: u8 = 10;
pub const TOKEN_RECORD_ACCOUNT: u8 = 11;
pub const METADATA_DELEGATE_RECORD_ACCOUNT: u8 = 12;
pub const EDITION_MARKER_V2_ACCOUNT: u8 = 13;
pub const HOLDER_DELEGATE_RECORD_ACCOUNT: u8 = 14;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenMetadataAccount {
    Metadata(Metadata),
    MasterEditionV1(DeprecatedMasterEditionV1),
    MasterEditionV2(MasterEdition),
    Edition(Edition),
    EditionMarker(EditionMarker),
    EditionMarkerV2(EditionMarkerV2),
}

pub fn unpack(data: &[u8]) -> Result<TokenMetadataAccount, ParseError> {
    let Some(discriminator) = data.first().copied() else {
        return Err(ParseError::TooShort(0));
    };

    match discriminator {
        METADATA_ACCOUNT => Ok(TokenMetadataAccount::Metadata(Metadata::from_bytes(data)?)),
        MASTER_EDITION_V1_ACCOUNT => Ok(TokenMetadataAccount::MasterEditionV1(
            DeprecatedMasterEditionV1::from_bytes(data)?,
        )),
        MASTER_EDITION_V2_ACCOUNT => Ok(TokenMetadataAccount::MasterEditionV2(MasterEdition::from_bytes(data)?)),
        EDITION_ACCOUNT => Ok(TokenMetadataAccount::Edition(Edition::from_bytes(data)?)),
        EDITION_MARKER_ACCOUNT => Ok(TokenMetadataAccount::EditionMarker(EditionMarker::from_bytes(data)?)),
        EDITION_MARKER_V2_ACCOUNT => Ok(TokenMetadataAccount::EditionMarkerV2(EditionMarkerV2::from_bytes(data)?)),
        other => Err(ParseError::TokenMetadataUnknown(other)),
    }
}
