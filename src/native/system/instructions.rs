//! System Program instructions.
//!
//! Uses a sequential little-endian u32 discriminator (first 4 bytes).

use crate::common::ParseError;

// Discriminators (little-endian u32)
pub const CREATE_ACCOUNT: [u8; 4] = [0, 0, 0, 0];
pub const ASSIGN: [u8; 4] = [1, 0, 0, 0];
pub const TRANSFER: [u8; 4] = [2, 0, 0, 0];
pub const CREATE_ACCOUNT_WITH_SEED: [u8; 4] = [3, 0, 0, 0];
pub const ADVANCE_NONCE_ACCOUNT: [u8; 4] = [4, 0, 0, 0];
pub const WITHDRAW_NONCE_ACCOUNT: [u8; 4] = [5, 0, 0, 0];
pub const INITIALIZE_NONCE_ACCOUNT: [u8; 4] = [6, 0, 0, 0];
pub const AUTHORIZE_NONCE_ACCOUNT: [u8; 4] = [7, 0, 0, 0];
pub const ALLOCATE: [u8; 4] = [8, 0, 0, 0];
pub const ALLOCATE_WITH_SEED: [u8; 4] = [9, 0, 0, 0];
pub const ASSIGN_WITH_SEED: [u8; 4] = [10, 0, 0, 0];
pub const TRANSFER_WITH_SEED: [u8; 4] = [11, 0, 0, 0];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemInstruction {
    CreateAccount(RawPayload),
    Assign(RawPayload),
    Transfer(RawPayload),
    CreateAccountWithSeed(RawPayload),
    AdvanceNonceAccount(RawPayload),
    WithdrawNonceAccount(RawPayload),
    InitializeNonceAccount(RawPayload),
    AuthorizeNonceAccount(RawPayload),
    Allocate(RawPayload),
    AllocateWithSeed(RawPayload),
    AssignWithSeed(RawPayload),
    TransferWithSeed(RawPayload),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawPayload {
    pub data: Vec<u8>,
}

impl<'a> TryFrom<&'a [u8]> for SystemInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 4 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(4);
        let disc: [u8; 4] = disc.try_into().unwrap();
        let raw = RawPayload { data: payload.to_vec() };
        Ok(match disc {
            CREATE_ACCOUNT => Self::CreateAccount(raw),
            ASSIGN => Self::Assign(raw),
            TRANSFER => Self::Transfer(raw),
            CREATE_ACCOUNT_WITH_SEED => Self::CreateAccountWithSeed(raw),
            ADVANCE_NONCE_ACCOUNT => Self::AdvanceNonceAccount(raw),
            WITHDRAW_NONCE_ACCOUNT => Self::WithdrawNonceAccount(raw),
            INITIALIZE_NONCE_ACCOUNT => Self::InitializeNonceAccount(raw),
            AUTHORIZE_NONCE_ACCOUNT => Self::AuthorizeNonceAccount(raw),
            ALLOCATE => Self::Allocate(raw),
            ALLOCATE_WITH_SEED => Self::AllocateWithSeed(raw),
            ASSIGN_WITH_SEED => Self::AssignWithSeed(raw),
            TRANSFER_WITH_SEED => Self::TransferWithSeed(raw),
            _ => return Err(ParseError::Unknown([disc[0], disc[1], disc[2], disc[3], 0, 0, 0, 0])),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<SystemInstruction, ParseError> {
    SystemInstruction::try_from(data)
}
