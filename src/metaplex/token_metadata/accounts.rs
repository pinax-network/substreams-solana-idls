//! Metaplex Token Metadata on-chain accounts.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::common::ParseError;

pub use mpl_token_metadata::accounts::{
    CollectionAuthorityRecord, DeprecatedMasterEditionV1, Edition, EditionMarker, EditionMarkerV2, HolderDelegateRecord,
    MasterEdition, Metadata, MetadataDelegateRecord, TokenOwnedEscrow, TokenRecord, UseAuthorityRecord,
};
pub use mpl_token_metadata::types::{EscrowAuthority, TokenDelegateRole, TokenStandard, TokenState};

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

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct Reservation {
    pub address: Pubkey,
    pub spots_remaining: u64,
    pub total_spots: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct ReservationV1 {
    pub address: Pubkey,
    pub spots_remaining: u8,
    pub total_spots: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct ReservationListV1 {
    pub key: u8,
    pub master_edition: Pubkey,
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<ReservationV1>,
}

impl ReservationListV1 {
    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct ReservationListV2 {
    pub key: u8,
    pub master_edition: Pubkey,
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<Reservation>,
    pub total_reservation_spots: u64,
    pub current_reservation_spots: u64,
}

impl ReservationListV2 {
    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenMetadataAccount {
    Metadata(Metadata),
    MasterEditionV1(DeprecatedMasterEditionV1),
    MasterEditionV2(MasterEdition),
    ReservationListV1(ReservationListV1),
    ReservationListV2(ReservationListV2),
    Edition(Edition),
    EditionMarker(EditionMarker),
    UseAuthorityRecord(UseAuthorityRecord),
    CollectionAuthorityRecord(CollectionAuthorityRecord),
    TokenOwnedEscrow(TokenOwnedEscrow),
    TokenRecord(TokenRecord),
    MetadataDelegateRecord(MetadataDelegateRecord),
    EditionMarkerV2(EditionMarkerV2),
    HolderDelegateRecord(HolderDelegateRecord),
}

pub fn unpack(data: &[u8]) -> Result<TokenMetadataAccount, ParseError> {
    let Some(discriminator) = data.first().copied() else {
        return Err(ParseError::TooShort(data.len()));
    };

    match discriminator {
        METADATA_ACCOUNT => Ok(TokenMetadataAccount::Metadata(Metadata::from_bytes(data)?)),
        MASTER_EDITION_V1_ACCOUNT => Ok(TokenMetadataAccount::MasterEditionV1(DeprecatedMasterEditionV1::from_bytes(data)?)),
        MASTER_EDITION_V2_ACCOUNT => Ok(TokenMetadataAccount::MasterEditionV2(MasterEdition::from_bytes(data)?)),
        RESERVATION_LIST_V1_ACCOUNT => Ok(TokenMetadataAccount::ReservationListV1(ReservationListV1::from_bytes(data)?)),
        RESERVATION_LIST_V2_ACCOUNT => Ok(TokenMetadataAccount::ReservationListV2(ReservationListV2::from_bytes(data)?)),
        EDITION_ACCOUNT => Ok(TokenMetadataAccount::Edition(Edition::from_bytes(data)?)),
        EDITION_MARKER_ACCOUNT => Ok(TokenMetadataAccount::EditionMarker(EditionMarker::from_bytes(data)?)),
        USE_AUTHORITY_RECORD_ACCOUNT => Ok(TokenMetadataAccount::UseAuthorityRecord(UseAuthorityRecord::from_bytes(data)?)),
        COLLECTION_AUTHORITY_RECORD_ACCOUNT => {
            Ok(TokenMetadataAccount::CollectionAuthorityRecord(CollectionAuthorityRecord::from_bytes(data)?))
        }
        TOKEN_OWNED_ESCROW_ACCOUNT => Ok(TokenMetadataAccount::TokenOwnedEscrow(TokenOwnedEscrow::from_bytes(data)?)),
        TOKEN_RECORD_ACCOUNT => Ok(TokenMetadataAccount::TokenRecord(TokenRecord::from_bytes(data)?)),
        METADATA_DELEGATE_RECORD_ACCOUNT => {
            Ok(TokenMetadataAccount::MetadataDelegateRecord(MetadataDelegateRecord::from_bytes(data)?))
        }
        EDITION_MARKER_V2_ACCOUNT => Ok(TokenMetadataAccount::EditionMarkerV2(EditionMarkerV2::from_bytes(data)?)),
        HOLDER_DELEGATE_RECORD_ACCOUNT => Ok(TokenMetadataAccount::HolderDelegateRecord(HolderDelegateRecord::from_bytes(data)?)),
        other => Err(ParseError::TokenMetadataUnknown(other)),
    }
}
