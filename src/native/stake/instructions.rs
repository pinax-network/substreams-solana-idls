//! Stake Program instructions.
//!
//! Uses a sequential little-endian u32 discriminator (first 4 bytes).

use crate::common::ParseError;

// Discriminators (little-endian u32)
pub const INITIALIZE: [u8; 4] = [0, 0, 0, 0];
pub const AUTHORIZE: [u8; 4] = [1, 0, 0, 0];
pub const DELEGATE_STAKE: [u8; 4] = [2, 0, 0, 0];
pub const SPLIT: [u8; 4] = [3, 0, 0, 0];
pub const WITHDRAW: [u8; 4] = [4, 0, 0, 0];
pub const DEACTIVATE: [u8; 4] = [5, 0, 0, 0];
pub const SET_LOCKUP: [u8; 4] = [6, 0, 0, 0];
pub const MERGE: [u8; 4] = [7, 0, 0, 0];
pub const AUTHORIZE_WITH_SEED: [u8; 4] = [8, 0, 0, 0];
pub const INITIALIZE_CHECKED: [u8; 4] = [9, 0, 0, 0];
pub const AUTHORIZE_CHECKED: [u8; 4] = [10, 0, 0, 0];
pub const AUTHORIZE_CHECKED_WITH_SEED: [u8; 4] = [11, 0, 0, 0];
pub const SET_LOCKUP_CHECKED: [u8; 4] = [12, 0, 0, 0];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StakeInstruction {
    Initialize(RawPayload),
    Authorize(RawPayload),
    DelegateStake(RawPayload),
    Split(RawPayload),
    Withdraw(RawPayload),
    Deactivate(RawPayload),
    SetLockup(RawPayload),
    Merge(RawPayload),
    AuthorizeWithSeed(RawPayload),
    InitializeChecked(RawPayload),
    AuthorizeChecked(RawPayload),
    AuthorizeCheckedWithSeed(RawPayload),
    SetLockupChecked(RawPayload),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawPayload {
    pub data: Vec<u8>,
}

impl<'a> TryFrom<&'a [u8]> for StakeInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 4 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(4);
        let disc: [u8; 4] = disc.try_into().unwrap();
        let raw = RawPayload { data: payload.to_vec() };
        Ok(match disc {
            INITIALIZE => Self::Initialize(raw),
            AUTHORIZE => Self::Authorize(raw),
            DELEGATE_STAKE => Self::DelegateStake(raw),
            SPLIT => Self::Split(raw),
            WITHDRAW => Self::Withdraw(raw),
            DEACTIVATE => Self::Deactivate(raw),
            SET_LOCKUP => Self::SetLockup(raw),
            MERGE => Self::Merge(raw),
            AUTHORIZE_WITH_SEED => Self::AuthorizeWithSeed(raw),
            INITIALIZE_CHECKED => Self::InitializeChecked(raw),
            AUTHORIZE_CHECKED => Self::AuthorizeChecked(raw),
            AUTHORIZE_CHECKED_WITH_SEED => Self::AuthorizeCheckedWithSeed(raw),
            SET_LOCKUP_CHECKED => Self::SetLockupChecked(raw),
            _ => return Err(ParseError::Unknown([disc[0], disc[1], disc[2], disc[3], 0, 0, 0, 0])),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<StakeInstruction, ParseError> {
    StakeInstruction::try_from(data)
}
