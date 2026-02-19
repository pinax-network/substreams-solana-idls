//! Metaplex Token Metadata on-chain instructions.
//!
//! Non-Anchor program using sequential u8 index discriminators (first byte of instruction data).

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// ── Discriminator constants (u8 index) ──────────────────────────────────
pub const CREATE_METADATA_ACCOUNT: u8 = 0;
pub const UPDATE_METADATA_ACCOUNT: u8 = 1;
pub const DEPRECATED_CREATE_MASTER_EDITION: u8 = 2;
pub const DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN: u8 = 3;
pub const UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN: u8 = 4;
pub const DEPRECATED_SET_RESERVATION_LIST: u8 = 5;
pub const DEPRECATED_CREATE_RESERVATION_LIST: u8 = 6;
pub const SIGN_METADATA: u8 = 7;
pub const DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN: u8 = 8;
pub const DEPRECATED_MINT_PRINTING_TOKENS: u8 = 9;
pub const CREATE_MASTER_EDITION: u8 = 10;
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN: u8 = 11;
pub const CONVERT_MASTER_EDITION_V1_TO_V2: u8 = 12;
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY: u8 = 13;
pub const PUFF_METADATA: u8 = 14;
pub const UPDATE_METADATA_ACCOUNT_V2: u8 = 15;
pub const CREATE_METADATA_ACCOUNT_V2: u8 = 16;
pub const CREATE_MASTER_EDITION_V3: u8 = 17;
pub const VERIFY_COLLECTION: u8 = 18;
pub const UTILIZE: u8 = 19;
pub const APPROVE_USE_AUTHORITY: u8 = 20;
pub const REVOKE_USE_AUTHORITY: u8 = 21;
pub const UNVERIFY_COLLECTION: u8 = 22;
pub const APPROVE_COLLECTION_AUTHORITY: u8 = 23;
pub const REVOKE_COLLECTION_AUTHORITY: u8 = 24;
pub const SET_AND_VERIFY_COLLECTION: u8 = 25;
pub const FREEZE_DELEGATED_ACCOUNT: u8 = 26;
pub const THAW_DELEGATED_ACCOUNT: u8 = 27;
pub const REMOVE_CREATOR_VERIFICATION: u8 = 28;
pub const BURN_NFT: u8 = 29;
pub const VERIFY_SIZED_COLLECTION_ITEM: u8 = 30;
pub const UNVERIFY_SIZED_COLLECTION_ITEM: u8 = 31;
pub const SET_AND_VERIFY_SIZED_COLLECTION_ITEM: u8 = 32;
pub const CREATE_METADATA_ACCOUNT_V3: u8 = 33;
pub const SET_COLLECTION_SIZE: u8 = 34;
pub const SET_TOKEN_STANDARD: u8 = 35;
pub const BUBBLEGUM_SET_COLLECTION_SIZE: u8 = 36;
pub const BURN_EDITION_NFT: u8 = 37;
pub const CREATE_ESCROW_ACCOUNT: u8 = 38;
pub const CLOSE_ESCROW_ACCOUNT: u8 = 39;
pub const TRANSFER_OUT_OF_ESCROW: u8 = 40;
pub const BURN: u8 = 41;
pub const CREATE: u8 = 42;
pub const MINT: u8 = 43;
pub const DELEGATE: u8 = 44;
pub const REVOKE: u8 = 45;
pub const LOCK: u8 = 46;
pub const UNLOCK: u8 = 47;
pub const MIGRATE: u8 = 48;
pub const TRANSFER: u8 = 49;
pub const UPDATE: u8 = 50;
pub const USE: u8 = 51;
pub const VERIFY: u8 = 52;
pub const UNVERIFY: u8 = 53;
pub const COLLECT: u8 = 54;
pub const PRINT: u8 = 55;

// ── Parsed instruction args (selected) ─────────────────────────────────

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Uses {
    pub use_method: u8,
    pub remaining: u64,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CollectionDetails {
    pub discriminator: u8,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DataV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateMetadataAccountV3Args {
    pub data: DataV2,
    pub is_mutable: bool,
    pub collection_details: Option<CollectionDetails>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateMasterEditionV3Args {
    pub max_supply: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdateMetadataAccountV2Args {
    pub data: Option<DataV2>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

// ── Instruction enum ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TokenMetadataInstruction {
    CreateMetadataAccount,
    UpdateMetadataAccount,
    DeprecatedCreateMasterEdition,
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
    UpdatePrimarySaleHappenedViaToken,
    DeprecatedSetReservationList,
    DeprecatedCreateReservationList,
    SignMetadata,
    DeprecatedMintPrintingTokensViaToken,
    DeprecatedMintPrintingTokens,
    CreateMasterEdition,
    MintNewEditionFromMasterEditionViaToken,
    ConvertMasterEditionV1ToV2,
    MintNewEditionFromMasterEditionViaVaultProxy,
    PuffMetadata,
    UpdateMetadataAccountV2(UpdateMetadataAccountV2Args),
    CreateMetadataAccountV2,
    CreateMasterEditionV3(CreateMasterEditionV3Args),
    VerifyCollection,
    Utilize,
    ApproveUseAuthority,
    RevokeUseAuthority,
    UnverifyCollection,
    ApproveCollectionAuthority,
    RevokeCollectionAuthority,
    SetAndVerifyCollection,
    FreezeDelegatedAccount,
    ThawDelegatedAccount,
    RemoveCreatorVerification,
    BurnNft,
    VerifySizedCollectionItem,
    UnverifySizedCollectionItem,
    SetAndVerifySizedCollectionItem,
    CreateMetadataAccountV3(CreateMetadataAccountV3Args),
    SetCollectionSize,
    SetTokenStandard,
    BubblegumSetCollectionSize,
    BurnEditionNft,
    CreateEscrowAccount,
    CloseEscrowAccount,
    TransferOutOfEscrow,
    Burn,
    Create,
    Mint,
    Delegate,
    Revoke,
    Lock,
    Unlock,
    Migrate,
    Transfer,
    Update,
    Use,
    Verify,
    Unverify,
    Collect,
    Print,
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for TokenMetadataInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(0));
        }

        let discriminator = data[0];
        let payload = &data[1..];

        Ok(match discriminator {
            CREATE_METADATA_ACCOUNT => Self::CreateMetadataAccount,
            UPDATE_METADATA_ACCOUNT => Self::UpdateMetadataAccount,
            DEPRECATED_CREATE_MASTER_EDITION => Self::DeprecatedCreateMasterEdition,
            DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN => Self::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
            UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN => Self::UpdatePrimarySaleHappenedViaToken,
            DEPRECATED_SET_RESERVATION_LIST => Self::DeprecatedSetReservationList,
            DEPRECATED_CREATE_RESERVATION_LIST => Self::DeprecatedCreateReservationList,
            SIGN_METADATA => Self::SignMetadata,
            DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN => Self::DeprecatedMintPrintingTokensViaToken,
            DEPRECATED_MINT_PRINTING_TOKENS => Self::DeprecatedMintPrintingTokens,
            CREATE_MASTER_EDITION => Self::CreateMasterEdition,
            MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN => Self::MintNewEditionFromMasterEditionViaToken,
            CONVERT_MASTER_EDITION_V1_TO_V2 => Self::ConvertMasterEditionV1ToV2,
            MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY => Self::MintNewEditionFromMasterEditionViaVaultProxy,
            PUFF_METADATA => Self::PuffMetadata,
            UPDATE_METADATA_ACCOUNT_V2 => Self::UpdateMetadataAccountV2(UpdateMetadataAccountV2Args::try_from_slice(payload)?),
            CREATE_METADATA_ACCOUNT_V2 => Self::CreateMetadataAccountV2,
            CREATE_MASTER_EDITION_V3 => Self::CreateMasterEditionV3(CreateMasterEditionV3Args::try_from_slice(payload)?),
            VERIFY_COLLECTION => Self::VerifyCollection,
            UTILIZE => Self::Utilize,
            APPROVE_USE_AUTHORITY => Self::ApproveUseAuthority,
            REVOKE_USE_AUTHORITY => Self::RevokeUseAuthority,
            UNVERIFY_COLLECTION => Self::UnverifyCollection,
            APPROVE_COLLECTION_AUTHORITY => Self::ApproveCollectionAuthority,
            REVOKE_COLLECTION_AUTHORITY => Self::RevokeCollectionAuthority,
            SET_AND_VERIFY_COLLECTION => Self::SetAndVerifyCollection,
            FREEZE_DELEGATED_ACCOUNT => Self::FreezeDelegatedAccount,
            THAW_DELEGATED_ACCOUNT => Self::ThawDelegatedAccount,
            REMOVE_CREATOR_VERIFICATION => Self::RemoveCreatorVerification,
            BURN_NFT => Self::BurnNft,
            VERIFY_SIZED_COLLECTION_ITEM => Self::VerifySizedCollectionItem,
            UNVERIFY_SIZED_COLLECTION_ITEM => Self::UnverifySizedCollectionItem,
            SET_AND_VERIFY_SIZED_COLLECTION_ITEM => Self::SetAndVerifySizedCollectionItem,
            CREATE_METADATA_ACCOUNT_V3 => Self::CreateMetadataAccountV3(CreateMetadataAccountV3Args::try_from_slice(payload)?),
            SET_COLLECTION_SIZE => Self::SetCollectionSize,
            SET_TOKEN_STANDARD => Self::SetTokenStandard,
            BUBBLEGUM_SET_COLLECTION_SIZE => Self::BubblegumSetCollectionSize,
            BURN_EDITION_NFT => Self::BurnEditionNft,
            CREATE_ESCROW_ACCOUNT => Self::CreateEscrowAccount,
            CLOSE_ESCROW_ACCOUNT => Self::CloseEscrowAccount,
            TRANSFER_OUT_OF_ESCROW => Self::TransferOutOfEscrow,
            BURN => Self::Burn,
            CREATE => Self::Create,
            MINT => Self::Mint,
            DELEGATE => Self::Delegate,
            REVOKE => Self::Revoke,
            LOCK => Self::Lock,
            UNLOCK => Self::Unlock,
            MIGRATE => Self::Migrate,
            TRANSFER => Self::Transfer,
            UPDATE => Self::Update,
            USE => Self::Use,
            VERIFY => Self::Verify,
            UNVERIFY => Self::Unverify,
            COLLECT => Self::Collect,
            PRINT => Self::Print,
            _ => return Err(ParseError::TokenMetadataUnknown(discriminator)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<TokenMetadataInstruction, ParseError> {
    TokenMetadataInstruction::try_from(data)
}
