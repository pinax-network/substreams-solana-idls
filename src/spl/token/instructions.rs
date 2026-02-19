//! SPL Token on-chain instructions.

use crate::common::ParseError;

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

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum TokenInstruction {
    InitializeMint { decimals: u8, freeze_authority: bool },
    InitializeAccount,
    InitializeMultisig { m: u8 },
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
    InitializeMultisig2 { m: u8 },
    InitializeMint2 { decimals: u8, freeze_authority: bool },
}

// -----------------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------------
impl TryFrom<&[u8]> for TokenInstruction {
    type Error = ParseError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(0));
        }
        let rest = &data[1..];
        match data[0] {
            INITIALIZE_MINT => {
                if rest.len() < 2 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::InitializeMint {
                    decimals: rest[0],
                    freeze_authority: rest.len() > 34,
                })
            }
            INITIALIZE_ACCOUNT => Ok(Self::InitializeAccount),
            INITIALIZE_MULTISIG => {
                if rest.is_empty() {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::InitializeMultisig { m: rest[0] })
            }
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
            INITIALIZE_MULTISIG2 => {
                if rest.is_empty() {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::InitializeMultisig2 { m: rest[0] })
            }
            INITIALIZE_MINT2 => {
                if rest.len() < 2 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::InitializeMint2 {
                    decimals: rest[0],
                    freeze_authority: rest.len() > 34,
                })
            }
            other => Err(ParseError::SplUnknown(other)),
        }
    }
}

pub fn unpack(data: &[u8]) -> Result<TokenInstruction, ParseError> {
    TokenInstruction::try_from(data)
}
