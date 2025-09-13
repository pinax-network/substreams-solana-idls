//! Raydium Launchpad events.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const CLAIM_VESTED_EVENT: [u8; 8] = [21, 194, 114, 87, 120, 211, 226, 32];
pub const CREATE_VESTING_EVENT: [u8; 8] = [201, 216, 28, 169, 227, 76, 208, 95];
pub const POOL_CREATE_EVENT: [u8; 8] = [35, 19, 27, 213, 21, 36, 194, 123];
pub const TRADE_EVENT: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
const ANCHOR_DISC: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];

// -----------------------------------------------------------------------------
// Event enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumLaunchpadEvent {
    ClaimVestedEvent(ClaimVestedEvent),
    CreateVestingEvent(CreateVestingEvent),
    PoolCreateEvent(PoolCreateEvent),
    TradeEvent(TradeEvent),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Emitted when vesting token claimed by beneficiary
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClaimVestedEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub claim_amount: u64,
}

/// Emitted when vest_account created
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateVestingEvent {
    pub pool_state: Pubkey,
    pub beneficiary: Pubkey,
    pub share_amount: u64,
}

/// Emitted when pool created
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolCreateEvent {
    pub pool_state: Pubkey,
    pub creator: Pubkey,
    pub config: Pubkey,
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}

/// Emitted when trade process
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TradeEvent {
    pub pool_state: Pubkey,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base_before: u64,
    pub real_quote_before: u64,
    pub real_base_after: u64,
    pub real_quote_after: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub protocol_fee: u64,
    pub platform_fee: u64,
    pub creator_fee: u64,
    pub share_fee: u64,
    pub trade_direction: TradeDirection,
    pub pool_status: PoolStatus,
    pub exact_in: bool,
}

// -----------------------------------------------------------------------------
// Additional types
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MintParams {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum CurveParams {
    Constant { data: ConstantCurve },
    Fixed { data: FixedCurve },
    Linear { data: LinearCurve },
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ConstantCurve {
    pub supply: u64,
    pub total_base_sell: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct FixedCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct LinearCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct VestingParams {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum AmmCreatorFeeOn {
    QuoteToken,
    BothToken,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TradeDirection {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum PoolStatus {
    Fund,
    Migrate,
    Trade,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumLaunchpadEvent {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = if data.len() >= 16 && &data[0..8] == ANCHOR_DISC {
            let disc: [u8; 8] = data[8..16].try_into().expect("slice len 8");
            (disc, &data[16..])
        } else {
            let disc: [u8; 8] = data[0..8].try_into().expect("slice len 8");
            (disc, &data[8..])
        };
        Ok(match disc {
            CLAIM_VESTED_EVENT => Self::ClaimVestedEvent(ClaimVestedEvent::try_from_slice(payload)?),
            CREATE_VESTING_EVENT => Self::CreateVestingEvent(CreateVestingEvent::try_from_slice(payload)?),
            POOL_CREATE_EVENT => Self::PoolCreateEvent(PoolCreateEvent::try_from_slice(payload)?),
            TRADE_EVENT => Self::TradeEvent(TradeEvent::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumLaunchpadEvent, ParseError> {
    RaydiumLaunchpadEvent::try_from(data)
}
