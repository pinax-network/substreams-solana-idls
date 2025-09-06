//! Meteora AMM on-chain events.

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use solana_program::pubkey::Pubkey;

/// Multiplier for the pool token. Used to normalized token with different decimal into the same precision.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TokenMultiplier {
    /// Multiplier for token A of the pool.
    pub token_a_multiplier: u64,
    /// Multiplier for token B of the pool.
    pub token_b_multiplier: u64,
    /// Record the highest token decimal in the pool. For example, Token A is 6 decimal, token B is 9 decimal. This will save value of 9.
    pub precision_factor: u8,
}

/// Information regarding fee charges
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolFees {
    /// Trade fees are extra token amounts that are held inside the token
    /// accounts during a trade, making the value of liquidity tokens rise.
    /// Trade fee numerator
    pub trade_fee_numerator: u64,
    /// Trade fee denominator
    pub trade_fee_denominator: u64,
    /// Protocol trading fees are extra token amounts that are held inside the token
    /// accounts during a trade, with the equivalent in pool tokens minted to
    /// the protocol of the program.
    /// Protocol trade fee numerator
    pub protocol_trade_fee_numerator: u64,
    /// Protocol trade fee denominator
    pub protocol_trade_fee_denominator: u64,
}

/// Contains information for depeg pool
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Depeg {
    /// The virtual price of staking / interest bearing token
    pub base_virtual_price: u64,
    /// The last time base_virtual_price is updated
    pub base_cache_updated: u64,
    /// Type of the depeg pool
    pub depeg_type: DepegType,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ConfigParameters {
    pub trade_fee_numerator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub activation_duration: u64,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub index: u64,
    pub partner_fee_numerator: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CustomizableParams {
    /// Trading fee.
    pub trade_fee_numerator: u32,
    /// The pool start trading.
    pub activation_point: Option<u64>,
    /// Whether the pool support alpha vault
    pub has_alpha_vault: bool,
    /// Activation type
    pub activation_type: u8,
    /// Padding
    #[serde(with = "BigArray")]
    pub padding: [u8; 90],
}

/// Padding for future pool fields
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Padding {
    /// Padding 0
    pub padding0: [u8; 6],
    /// Padding 1
    pub padding1: [u64; 21],
    /// Padding 2
    pub padding2: [u64; 21],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PartnerInfo {
    pub fee_numerator: u64,
    pub partner_authority: Pubkey,
    pub pending_fee_a: u64,
    pub pending_fee_b: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Bootstrapping {
    /// Activation point, can be slot or timestamp
    pub activation_point: u64,
    /// Whitelisted vault to be able to buy pool before activation_point
    pub whitelisted_vault: Pubkey,
    /// Need to store pool creator in lauch pool, so they can modify liquidity before activation_point
    pub pool_creator: Pubkey,
    /// Activation type, 0 means by slot, 1 means by timestamp
    pub activation_type: u8,
}

/// Type of the activation
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum ActivationType {
    Slot,
    Timestamp,
}

/// Rounding direction
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum RoundDirection {
    Floor,
    Ceiling,
}

/// Trade (swap) direction
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TradeDirection {
    AtoB,
    BtoA,
}

/// Type of the swap curve
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum NewCurveType {
    ConstantProduct,
    Stable {
        /// Amplification coefficient
        amp: u64,
        /// Multiplier for the pool token. Used to normalized token with different decimal into the same precision.
        token_multiplier: TokenMultiplier,
        /// Depeg pool information. Contains functions to allow token amount to be repeg using stake / interest bearing token virtual price
        depeg: Depeg,
        /// The last amp updated timestamp. Used to prevent update_curve_info called infinitely many times within a short period
        last_amp_updated_timestamp: u64,
    },
    NewCurve {
        field_one: u64,
        field_two: u64,
    },
}

/// Type of the swap curve
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum CurveType {
    ConstantProduct,
    Stable {
        /// Amplification coefficient
        amp: u64,
        /// Multiplier for the pool token. Used to normalized token with different decimal into the same precision.
        token_multiplier: TokenMultiplier,
        /// Depeg pool information. Contains functions to allow token amount to be repeg using stake / interest bearing token virtual price
        depeg: Depeg,
        /// The last amp updated timestamp. Used to prevent update_curve_info called infinitely many times within a short period
        last_amp_updated_timestamp: u64,
    },
}

/// Type of depeg pool
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum DepegType {
    None,
    Marinade,
    Lido,
    SplStake,
}

/// Round up, down
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum Rounding {
    Up,
    Down,
}

/// Pool type
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum PoolType {
    Permissioned,
    Permissionless,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
const ADD_LIQUIDITY: [u8; 8] = [31, 94, 125, 90, 227, 52, 61, 186];
const REMOVE_LIQUIDITY: [u8; 8] = [116, 244, 97, 232, 103, 31, 152, 58];
const BOOTSTRAP_LIQUIDITY: [u8; 8] = [121, 127, 38, 136, 92, 55, 14, 247];
const SWAP: [u8; 8] = [81, 108, 227, 190, 205, 208, 10, 196];
const SET_POOL_FEES: [u8; 8] = [245, 26, 198, 164, 88, 18, 75, 9];
const POOL_INFO: [u8; 8] = [207, 20, 87, 97, 251, 212, 234, 45];
const TRANSFER_ADMIN: [u8; 8] = [228, 169, 131, 244, 61, 56, 65, 254];
const OVERRIDE_CURVE_PARAM: [u8; 8] = [247, 20, 165, 248, 75, 5, 54, 246];
const POOL_CREATED: [u8; 8] = [202, 44, 41, 88, 104, 220, 157, 82];
const POOL_ENABLED: [u8; 8] = [2, 151, 18, 83, 204, 134, 92, 191];
const MIGRATE_FEE_ACCOUNT: [u8; 8] = [223, 234, 232, 26, 252, 105, 180, 125];
const CREATE_LOCK_ESCROW: [u8; 8] = [74, 94, 106, 141, 49, 17, 98, 109];
const LOCK: [u8; 8] = [220, 183, 67, 215, 153, 207, 56, 234];
const CLAIM_FEE: [u8; 8] = [75, 122, 154, 48, 140, 74, 123, 163];
const CREATE_CONFIG: [u8; 8] = [199, 152, 10, 19, 39, 39, 157, 104];
const CLOSE_CONFIG: [u8; 8] = [249, 181, 108, 89, 4, 150, 90, 174];
const WITHDRAW_PROTOCOL_FEES: [u8; 8] = [30, 240, 207, 196, 139, 239, 79, 28];
const PARTNER_CLAIM_FEES: [u8; 8] = [135, 131, 10, 94, 119, 209, 202, 48];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AmmEvent {
    AddLiquidity(AddLiquidity),
    RemoveLiquidity(RemoveLiquidity),
    BootstrapLiquidity(BootstrapLiquidity),
    Swap(Swap),
    SetPoolFees(SetPoolFees),
    PoolInfo(PoolInfo),
    TransferAdmin(TransferAdmin),
    OverrideCurveParam(OverrideCurveParam),
    PoolCreated(PoolCreated),
    PoolEnabled(PoolEnabled),
    MigrateFeeAccount(MigrateFeeAccount),
    CreateLockEscrow(CreateLockEscrow),
    Lock(Lock),
    ClaimFee(ClaimFee),
    CreateConfig(CreateConfig),
    CloseConfig(CloseConfig),
    WithdrawProtocolFees(WithdrawProtocolFees),
    PartnerClaimFees(PartnerClaimFees),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AddLiquidity {
    pub lp_mint_amount: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct RemoveLiquidity {
    pub lp_unmint_amount: u64,
    pub token_a_out_amount: u64,
    pub token_b_out_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct BootstrapLiquidity {
    pub lp_mint_amount: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub pool: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Swap {
    pub in_amount: u64,
    pub out_amount: u64,
    pub trade_fee: u64,
    pub protocol_fee: u64,
    pub host_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetPoolFees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub protocol_trade_fee_denominator: u64,
    pub pool: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolInfo {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub virtual_price: f64,
    pub current_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TransferAdmin {
    pub admin: Pubkey,
    pub new_admin: Pubkey,
    pub pool: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct OverrideCurveParam {
    pub new_amp: u64,
    pub updated_timestamp: u64,
    pub pool: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolCreated {
    pub lp_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub pool_type: PoolType,
    pub pool: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PoolEnabled {
    pub pool: Pubkey,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MigrateFeeAccount {
    pub pool: Pubkey,
    pub new_admin_token_a_fee: Pubkey,
    pub new_admin_token_b_fee: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateLockEscrow {
    pub pool: Pubkey,
    pub owner: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Lock {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ClaimFee {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateConfig {
    pub trade_fee_numerator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub config: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CloseConfig {
    pub config: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawProtocolFees {
    pub pool: Pubkey,
    pub protocol_a_fee: u64,
    pub protocol_b_fee: u64,
    pub protocol_a_fee_owner: Pubkey,
    pub protocol_b_fee_owner: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PartnerClaimFees {
    pub pool: Pubkey,
    pub fee_a: u64,
    pub fee_b: u64,
    pub partner: Pubkey,
}

impl<'a> TryFrom<&'a [u8]> for AmmEvent {
    type Error = ParseError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            ADD_LIQUIDITY => Self::AddLiquidity(AddLiquidity::try_from_slice(payload)?),
            REMOVE_LIQUIDITY => Self::RemoveLiquidity(RemoveLiquidity::try_from_slice(payload)?),
            BOOTSTRAP_LIQUIDITY => Self::BootstrapLiquidity(BootstrapLiquidity::try_from_slice(payload)?),
            SWAP => Self::Swap(Swap::try_from_slice(payload)?),
            SET_POOL_FEES => Self::SetPoolFees(SetPoolFees::try_from_slice(payload)?),
            POOL_INFO => Self::PoolInfo(PoolInfo::try_from_slice(payload)?),
            TRANSFER_ADMIN => Self::TransferAdmin(TransferAdmin::try_from_slice(payload)?),
            OVERRIDE_CURVE_PARAM => Self::OverrideCurveParam(OverrideCurveParam::try_from_slice(payload)?),
            POOL_CREATED => Self::PoolCreated(PoolCreated::try_from_slice(payload)?),
            POOL_ENABLED => Self::PoolEnabled(PoolEnabled::try_from_slice(payload)?),
            MIGRATE_FEE_ACCOUNT => Self::MigrateFeeAccount(MigrateFeeAccount::try_from_slice(payload)?),
            CREATE_LOCK_ESCROW => Self::CreateLockEscrow(CreateLockEscrow::try_from_slice(payload)?),
            LOCK => Self::Lock(Lock::try_from_slice(payload)?),
            CLAIM_FEE => Self::ClaimFee(ClaimFee::try_from_slice(payload)?),
            CREATE_CONFIG => Self::CreateConfig(CreateConfig::try_from_slice(payload)?),
            CLOSE_CONFIG => Self::CloseConfig(CloseConfig::try_from_slice(payload)?),
            WITHDRAW_PROTOCOL_FEES => Self::WithdrawProtocolFees(WithdrawProtocolFees::try_from_slice(payload)?),
            PARTNER_CLAIM_FEES => Self::PartnerClaimFees(PartnerClaimFees::try_from_slice(payload)?),
            _ => Self::Unknown,
        })
    }
}
pub fn parse_event(data: &[u8]) -> Result<AmmEvent, ParseError> {
    AmmEvent::try_from(data)
}
