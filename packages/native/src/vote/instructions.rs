//! Vote Program instructions.
//!
//! Uses a sequential little-endian u32 discriminator (first 4 bytes).

use common::ParseError;

// Discriminators (little-endian u32)
pub const INITIALIZE: [u8; 4] = [0, 0, 0, 0];
pub const AUTHORIZE: [u8; 4] = [1, 0, 0, 0];
pub const VOTE: [u8; 4] = [2, 0, 0, 0];
pub const WITHDRAW: [u8; 4] = [3, 0, 0, 0];
pub const UPDATE_VALIDATOR_IDENTITY: [u8; 4] = [4, 0, 0, 0];
pub const UPDATE_COMMISSION: [u8; 4] = [5, 0, 0, 0];
pub const VOTE_SWITCH: [u8; 4] = [6, 0, 0, 0];
pub const AUTHORIZE_CHECKED: [u8; 4] = [7, 0, 0, 0];
pub const UPDATE_VOTE_STATE: [u8; 4] = [8, 0, 0, 0];
pub const UPDATE_VOTE_STATE_SWITCH: [u8; 4] = [9, 0, 0, 0];
pub const AUTHORIZE_WITH_SEED: [u8; 4] = [10, 0, 0, 0];
pub const AUTHORIZE_CHECKED_WITH_SEED: [u8; 4] = [11, 0, 0, 0];
pub const COMPACT_UPDATE_VOTE_STATE: [u8; 4] = [12, 0, 0, 0];
pub const COMPACT_UPDATE_VOTE_STATE_SWITCH: [u8; 4] = [13, 0, 0, 0];
pub const TOWER_SYNC: [u8; 4] = [14, 0, 0, 0];
pub const TOWER_SYNC_SWITCH: [u8; 4] = [15, 0, 0, 0];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoteInstruction {
    Initialize(RawPayload),
    Authorize(RawPayload),
    Vote(RawPayload),
    Withdraw(RawPayload),
    UpdateValidatorIdentity(RawPayload),
    UpdateCommission(RawPayload),
    VoteSwitch(RawPayload),
    AuthorizeChecked(RawPayload),
    UpdateVoteState(RawPayload),
    UpdateVoteStateSwitch(RawPayload),
    AuthorizeWithSeed(RawPayload),
    AuthorizeCheckedWithSeed(RawPayload),
    CompactUpdateVoteState(RawPayload),
    CompactUpdateVoteStateSwitch(RawPayload),
    TowerSync(RawPayload),
    TowerSyncSwitch(RawPayload),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawPayload {
    pub data: Vec<u8>,
}

impl<'a> TryFrom<&'a [u8]> for VoteInstruction {
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
            VOTE => Self::Vote(raw),
            WITHDRAW => Self::Withdraw(raw),
            UPDATE_VALIDATOR_IDENTITY => Self::UpdateValidatorIdentity(raw),
            UPDATE_COMMISSION => Self::UpdateCommission(raw),
            VOTE_SWITCH => Self::VoteSwitch(raw),
            AUTHORIZE_CHECKED => Self::AuthorizeChecked(raw),
            UPDATE_VOTE_STATE => Self::UpdateVoteState(raw),
            UPDATE_VOTE_STATE_SWITCH => Self::UpdateVoteStateSwitch(raw),
            AUTHORIZE_WITH_SEED => Self::AuthorizeWithSeed(raw),
            AUTHORIZE_CHECKED_WITH_SEED => Self::AuthorizeCheckedWithSeed(raw),
            COMPACT_UPDATE_VOTE_STATE => Self::CompactUpdateVoteState(raw),
            COMPACT_UPDATE_VOTE_STATE_SWITCH => Self::CompactUpdateVoteStateSwitch(raw),
            TOWER_SYNC => Self::TowerSync(raw),
            TOWER_SYNC_SWITCH => Self::TowerSyncSwitch(raw),
            _ => return Err(ParseError::Unknown([disc[0], disc[1], disc[2], disc[3], 0, 0, 0, 0])),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<VoteInstruction, ParseError> {
    VoteInstruction::try_from(data)
}
