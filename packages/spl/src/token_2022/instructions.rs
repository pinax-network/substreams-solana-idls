//! SPL Token 2022 on-chain instructions.

use common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (single-byte index)
// -----------------------------------------------------------------------------
pub const INITIALIZE_MINT: u8 = 0;
pub const INITIALIZE_ACCOUNT: u8 = 1;
pub const INITIALIZE_MULTISIG: u8 = 2;
pub const TRANSFER: u8 = 3;
pub const APPROVE: u8 = 4;
pub const REVOKE: u8 = 5;
pub const SET_AUTHORITY: u8 = 6;
pub const MINT_TO: u8 = 7;
pub const BURN: u8 = 8;
pub const CLOSE_ACCOUNT: u8 = 9;
pub const FREEZE_ACCOUNT: u8 = 10;
pub const THAW_ACCOUNT: u8 = 11;
pub const TRANSFER_CHECKED: u8 = 12;
pub const APPROVE_CHECKED: u8 = 13;
pub const MINT_TO_CHECKED: u8 = 14;
pub const BURN_CHECKED: u8 = 15;
pub const INITIALIZE_ACCOUNT2: u8 = 16;
pub const SYNC_NATIVE: u8 = 17;
pub const INITIALIZE_ACCOUNT3: u8 = 18;
pub const INITIALIZE_MULTISIG2: u8 = 19;
pub const INITIALIZE_MINT2: u8 = 20;
pub const GET_ACCOUNT_DATA_SIZE: u8 = 21;
pub const INITIALIZE_IMMUTABLE_OWNER: u8 = 22;
pub const AMOUNT_TO_UI_AMOUNT: u8 = 23;
pub const UI_AMOUNT_TO_AMOUNT: u8 = 24;
pub const INITIALIZE_MINT_CLOSE_AUTHORITY: u8 = 25;
pub const TRANSFER_FEE_EXTENSION: u8 = 26;
pub const CONFIDENTIAL_TRANSFER_EXTENSION: u8 = 27;
pub const DEFAULT_ACCOUNT_STATE_EXTENSION: u8 = 28;
pub const REALLOCATE: u8 = 29;
pub const MEMO_TRANSFER_EXTENSION: u8 = 30;
pub const CREATE_NATIVE_MINT: u8 = 31;
pub const INITIALIZE_NON_TRANSFERABLE_MINT: u8 = 32;
pub const INTEREST_BEARING_MINT_EXTENSION: u8 = 33;
pub const CPI_GUARD_EXTENSION: u8 = 34;
pub const INITIALIZE_PERMANENT_DELEGATE: u8 = 35;
pub const TRANSFER_HOOK_EXTENSION: u8 = 36;
pub const CONFIDENTIAL_TRANSFER_FEE_EXTENSION: u8 = 37;
pub const WITHDRAWAL_EXCESS_LAMPORTS: u8 = 38;
pub const METADATA_POINTER_EXTENSION: u8 = 39;
pub const GROUP_POINTER_EXTENSION: u8 = 40;
pub const GROUP_MEMBER_POINTER_EXTENSION: u8 = 41;
pub const TOKEN_METADATA_EXTENSION: u8 = 42;

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum Token2022Instruction {
    InitializeMint,
    InitializeAccount,
    InitializeMultisig,
    Transfer { amount: u64 },
    Approve { amount: u64 },
    Revoke,
    SetAuthority,
    MintTo { amount: u64 },
    Burn { amount: u64 },
    CloseAccount,
    FreezeAccount,
    ThawAccount,
    TransferChecked { amount: u64, decimals: u8 },
    ApproveChecked { amount: u64, decimals: u8 },
    MintToChecked { amount: u64, decimals: u8 },
    BurnChecked { amount: u64, decimals: u8 },
    InitializeAccount2,
    SyncNative,
    InitializeAccount3,
    InitializeMultisig2,
    InitializeMint2,
    GetAccountDataSize,
    InitializeImmutableOwner,
    AmountToUiAmount,
    UiAmountToAmount,
    InitializeMintCloseAuthority,
    TransferFeeExtension,
    ConfidentialTransferExtension,
    DefaultAccountStateExtension,
    Reallocate,
    MemoTransferExtension,
    CreateNativeMint,
    InitializeNonTransferableMint,
    InterestBearingMintExtension,
    CpiGuardExtension,
    InitializePermanentDelegate,
    TransferHookExtension,
    ConfidentialTransferFeeExtension,
    WithdrawalExcessLamports,
    MetadataPointerExtension,
    GroupPointerExtension,
    GroupMemberPointerExtension,
    TokenMetadataExtension,
}

// -----------------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------------
impl TryFrom<&[u8]> for Token2022Instruction {
    type Error = ParseError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(0));
        }
        let rest = &data[1..];
        match data[0] {
            INITIALIZE_MINT => Ok(Self::InitializeMint),
            INITIALIZE_ACCOUNT => Ok(Self::InitializeAccount),
            INITIALIZE_MULTISIG => Ok(Self::InitializeMultisig),
            TRANSFER => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::Transfer {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            APPROVE => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::Approve {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            REVOKE => Ok(Self::Revoke),
            SET_AUTHORITY => Ok(Self::SetAuthority),
            MINT_TO => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::MintTo {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            BURN => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::Burn {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            CLOSE_ACCOUNT => Ok(Self::CloseAccount),
            FREEZE_ACCOUNT => Ok(Self::FreezeAccount),
            THAW_ACCOUNT => Ok(Self::ThawAccount),
            TRANSFER_CHECKED => {
                if rest.len() < 9 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::TransferChecked {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    decimals: rest[8],
                })
            }
            APPROVE_CHECKED => {
                if rest.len() < 9 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::ApproveChecked {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    decimals: rest[8],
                })
            }
            MINT_TO_CHECKED => {
                if rest.len() < 9 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::MintToChecked {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    decimals: rest[8],
                })
            }
            BURN_CHECKED => {
                if rest.len() < 9 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::BurnChecked {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                    decimals: rest[8],
                })
            }
            INITIALIZE_ACCOUNT2 => Ok(Self::InitializeAccount2),
            SYNC_NATIVE => Ok(Self::SyncNative),
            INITIALIZE_ACCOUNT3 => Ok(Self::InitializeAccount3),
            INITIALIZE_MULTISIG2 => Ok(Self::InitializeMultisig2),
            INITIALIZE_MINT2 => Ok(Self::InitializeMint2),
            GET_ACCOUNT_DATA_SIZE => Ok(Self::GetAccountDataSize),
            INITIALIZE_IMMUTABLE_OWNER => Ok(Self::InitializeImmutableOwner),
            AMOUNT_TO_UI_AMOUNT => Ok(Self::AmountToUiAmount),
            UI_AMOUNT_TO_AMOUNT => Ok(Self::UiAmountToAmount),
            INITIALIZE_MINT_CLOSE_AUTHORITY => Ok(Self::InitializeMintCloseAuthority),
            TRANSFER_FEE_EXTENSION => Ok(Self::TransferFeeExtension),
            CONFIDENTIAL_TRANSFER_EXTENSION => Ok(Self::ConfidentialTransferExtension),
            DEFAULT_ACCOUNT_STATE_EXTENSION => Ok(Self::DefaultAccountStateExtension),
            REALLOCATE => Ok(Self::Reallocate),
            MEMO_TRANSFER_EXTENSION => Ok(Self::MemoTransferExtension),
            CREATE_NATIVE_MINT => Ok(Self::CreateNativeMint),
            INITIALIZE_NON_TRANSFERABLE_MINT => Ok(Self::InitializeNonTransferableMint),
            INTEREST_BEARING_MINT_EXTENSION => Ok(Self::InterestBearingMintExtension),
            CPI_GUARD_EXTENSION => Ok(Self::CpiGuardExtension),
            INITIALIZE_PERMANENT_DELEGATE => Ok(Self::InitializePermanentDelegate),
            TRANSFER_HOOK_EXTENSION => Ok(Self::TransferHookExtension),
            CONFIDENTIAL_TRANSFER_FEE_EXTENSION => Ok(Self::ConfidentialTransferFeeExtension),
            WITHDRAWAL_EXCESS_LAMPORTS => Ok(Self::WithdrawalExcessLamports),
            METADATA_POINTER_EXTENSION => Ok(Self::MetadataPointerExtension),
            GROUP_POINTER_EXTENSION => Ok(Self::GroupPointerExtension),
            GROUP_MEMBER_POINTER_EXTENSION => Ok(Self::GroupMemberPointerExtension),
            TOKEN_METADATA_EXTENSION => Ok(Self::TokenMetadataExtension),
            other => Err(ParseError::SplUnknown(other)),
        }
    }
}

pub fn unpack(data: &[u8]) -> Result<Token2022Instruction, ParseError> {
    Token2022Instruction::try_from(data)
}
