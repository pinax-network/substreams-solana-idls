//! Meteora AMM on-chain instructions.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Multiplier for the pool token. Used to normalized token with different decimal into the same precision.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TokenMultiplier {
    /// Multiplier for token A of the pool.
    pub token_a_multiplier: u64,
    /// Multiplier for token B of the pool.
    pub token_b_multiplier: u64,
    /// Record the highest token decimal in the pool. For example, Token A is 6 decimal, token B is 9 decimal. This will save value of 9.
    pub precision_factor: u8,
}

/// Information regarding fee charges
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Depeg {
    /// The virtual price of staking / interest bearing token
    pub base_virtual_price: u64,
    /// The last time base_virtual_price is updated
    pub base_cache_updated: u64,
    /// Type of the depeg pool
    pub depeg_type: DepegType,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
    pub padding: [u8; 90],
}

/// Padding for future pool fields
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Padding {
    /// Padding 0
    pub padding0: [u8; 6],
    /// Padding 1
    pub padding1: [u64; 21],
    /// Padding 2
    pub padding2: [u64; 21],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PartnerInfo {
    pub fee_numerator: u64,
    pub partner_authority: Pubkey,
    pub pending_fee_a: u64,
    pub pending_fee_b: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum ActivationType {
    Slot,
    Timestamp,
}

/// Rounding direction
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum RoundDirection {
    Floor,
    Ceiling,
}

/// Trade (swap) direction
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum TradeDirection {
    AtoB,
    BtoA,
}

/// Type of the swap curve
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum DepegType {
    None,
    Marinade,
    Lido,
    SplStake,
}

/// Round up, down
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Rounding {
    Up,
    Down,
}

/// Pool type
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum PoolType {
    Permissioned,
    Permissionless,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const INITIALIZE_PERMISSIONED_POOL: [u8; 8] = [228, 5, 176, 43, 24, 227, 14, 223];
pub const INITIALIZE_PERMISSIONLESS_POOL: [u8; 8] = [76, 220, 118, 106, 102, 109, 153, 49];
pub const INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER: [u8; 8] = [94, 117, 139, 171, 48, 253, 193, 43];
pub const ENABLE_OR_DISABLE_POOL: [u8; 8] = [46, 119, 222, 229, 144, 207, 255, 126];
pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
pub const REMOVE_LIQUIDITY_SINGLE_SIDE: [u8; 8] = [186, 59, 62, 74, 79, 177, 167, 202];
pub const ADD_IMBALANCE_LIQUIDITY: [u8; 8] = [184, 127, 170, 13, 9, 241, 22, 34];
pub const REMOVE_BALANCE_LIQUIDITY: [u8; 8] = [58, 212, 85, 140, 4, 91, 103, 31];
pub const ADD_BALANCE_LIQUIDITY: [u8; 8] = [197, 201, 162, 116, 118, 76, 197, 52];
pub const SET_POOL_FEES: [u8; 8] = [145, 138, 174, 71, 38, 187, 76, 61];
pub const OVERRIDE_CURVE_PARAM: [u8; 8] = [169, 210, 127, 241, 63, 80, 152, 93];
pub const GET_POOL_INFO: [u8; 8] = [206, 123, 182, 199, 8, 237, 134, 231];
pub const BOOTSTRAP_LIQUIDITY: [u8; 8] = [9, 159, 30, 158, 36, 216, 53, 191];
pub const CREATE_MINT_METADATA: [u8; 8] = [166, 36, 75, 165, 140, 151, 89, 0];
pub const CREATE_LOCK_ESCROW: [u8; 8] = [208, 153, 175, 90, 248, 246, 37, 38];
pub const LOCK: [u8; 8] = [21, 19, 208, 43, 237, 62, 255, 87];
pub const CLAIM_FEE: [u8; 8] = [96, 222, 96, 162, 109, 168, 75, 80];
pub const CREATE_CONFIG: [u8; 8] = [216, 151, 219, 59, 152, 235, 155, 234];
pub const CLOSE_CONFIG: [u8; 8] = [180, 88, 124, 46, 245, 187, 221, 214];
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG: [u8; 8] = [34, 128, 121, 45, 171, 62, 210, 126];
pub const INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2: [u8; 8] = [71, 208, 43, 98, 147, 202, 246, 30];
pub const INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL: [u8; 8] = [119, 240, 111, 39, 145, 106, 180, 83];
pub const UPDATE_ACTIVATION_POINT: [u8; 8] = [186, 178, 111, 80, 9, 246, 167, 101];
pub const WITHDRAW_PROTOCOL_FEES: [u8; 8] = [40, 126, 172, 205, 67, 145, 97, 185];
pub const SET_WHITELISTED_VAULT: [u8; 8] = [190, 213, 253, 33, 48, 146, 248, 76];
pub const PARTNER_CLAIM_FEE: [u8; 8] = [62, 78, 142, 207, 84, 95, 7, 227];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmmInstruction {
    InitializePermissionedPool(InitializePermissionedPoolInstruction),
    InitializePermissionlessPool(InitializePermissionlessPoolInstruction),
    InitializePermissionlessPoolWithFeeTier(InitializePermissionlessPoolWithFeeTierInstruction),
    EnableOrDisablePool(EnableOrDisablePoolInstruction),
    Swap(SwapInstruction),
    RemoveLiquiditySingleSide(RemoveLiquiditySingleSideInstruction),
    AddImbalanceLiquidity(AddImbalanceLiquidityInstruction),
    RemoveBalanceLiquidity(RemoveBalanceLiquidityInstruction),
    AddBalanceLiquidity(AddBalanceLiquidityInstruction),
    SetPoolFees(SetPoolFeesInstruction),
    OverrideCurveParam(OverrideCurveParamInstruction),
    GetPoolInfo,
    BootstrapLiquidity(BootstrapLiquidityInstruction),
    CreateMintMetadata,
    CreateLockEscrow,
    Lock(LockInstruction),
    ClaimFee(ClaimFeeInstruction),
    CreateConfig(CreateConfigInstruction),
    CloseConfig,
    InitializePermissionlessConstantProductPoolWithConfig(InitializePermissionlessConstantProductPoolWithConfigInstruction),
    InitializePermissionlessConstantProductPoolWithConfig2(InitializePermissionlessConstantProductPoolWithConfig2Instruction),
    InitializeCustomizablePermissionlessConstantProductPool(InitializeCustomizablePermissionlessConstantProductPoolInstruction),
    UpdateActivationPoint(UpdateActivationPointInstruction),
    WithdrawProtocolFees,
    SetWhitelistedVault(SetWhitelistedVaultInstruction),
    PartnerClaimFee(PartnerClaimFeeInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
/// Initialize a new permissioned pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePermissionedPoolInstruction {
    pub curve_type: CurveType,
}

/// Initialize a new permissionless pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePermissionlessPoolInstruction {
    pub curve_type: CurveType,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Initialize a new permissionless pool with customized fee tier
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePermissionlessPoolWithFeeTierInstruction {
    pub curve_type: CurveType,
    pub trade_fee_bps: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Enable or disable a pool. A disabled pool allow only remove balanced liquidity operation.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EnableOrDisablePoolInstruction {
    pub enable: bool,
}

/// Swap token A to B, or vice versa. An amount of trading fee will be charged for liquidity provider, and the admin of the pool.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub in_amount: u64,
    pub minimum_out_amount: u64,
}

/// Withdraw only single token from the pool. Only supported by pool with stable swap curve.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveLiquiditySingleSideInstruction {
    pub pool_token_amount: u64,
    pub minimum_out_amount: u64,
}

/// Deposit tokens to the pool in an imbalance ratio. Only supported by pool with stable swap curve.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddImbalanceLiquidityInstruction {
    pub minimum_pool_token_amount: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Withdraw tokens from the pool in a balanced ratio. User will still able to withdraw from pool even the pool is disabled. This allow user to exit their liquidity when there's some unforeseen event happen.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct RemoveBalanceLiquidityInstruction {
    pub pool_token_amount: u64,
    pub minimum_a_token_out: u64,
    pub minimum_b_token_out: u64,
}

/// Deposit tokens to the pool in a balanced ratio.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AddBalanceLiquidityInstruction {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}

/// Update trading fee charged for liquidity provider, and admin.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetPoolFeesInstruction {
    pub fees: PoolFees,
    pub new_partner_fee_numerator: u64,
}

/// Update swap curve parameters. This function do not allow update of curve type. For example: stable swap curve to constant product curve. Only supported by pool with stable swap curve.
/// Only amp is allowed to be override. The other attributes of stable swap curve will be ignored.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OverrideCurveParamInstruction {
    pub curve_type: CurveType,
}

/// Bootstrap the pool when liquidity is depleted.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BootstrapLiquidityInstruction {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Lock Lp token
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LockInstruction {
    pub max_amount: u64,
}

/// Claim fee
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimFeeInstruction {
    pub max_amount: u64,
}

/// Create config
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateConfigInstruction {
    pub config_parameters: ConfigParameters,
}

/// Initialize permissionless pool with config
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePermissionlessConstantProductPoolWithConfigInstruction {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

/// Initialize permissionless pool with config 2
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializePermissionlessConstantProductPoolWithConfig2Instruction {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub activation_point: Option<u64>,
}

/// Initialize permissionless pool with customizable params
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeCustomizablePermissionlessConstantProductPoolInstruction {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub params: CustomizableParams,
}

/// Update activation slot
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateActivationPointInstruction {
    pub new_activation_point: u64,
}

/// Set whitelisted vault
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetWhitelistedVaultInstruction {
    pub whitelisted_vault: Pubkey,
}

/// Partner claim fee
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PartnerClaimFeeInstruction {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for AmmInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }
        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");
        Ok(match discriminator {
            INITIALIZE_PERMISSIONED_POOL => Self::InitializePermissionedPool(InitializePermissionedPoolInstruction::try_from_slice(payload)?),
            INITIALIZE_PERMISSIONLESS_POOL => Self::InitializePermissionlessPool(InitializePermissionlessPoolInstruction::try_from_slice(payload)?),
            INITIALIZE_PERMISSIONLESS_POOL_WITH_FEE_TIER => {
                Self::InitializePermissionlessPoolWithFeeTier(InitializePermissionlessPoolWithFeeTierInstruction::try_from_slice(payload)?)
            }
            ENABLE_OR_DISABLE_POOL => Self::EnableOrDisablePool(EnableOrDisablePoolInstruction::try_from_slice(payload)?),
            SWAP => Self::Swap(SwapInstruction::try_from_slice(payload)?),
            REMOVE_LIQUIDITY_SINGLE_SIDE => Self::RemoveLiquiditySingleSide(RemoveLiquiditySingleSideInstruction::try_from_slice(payload)?),
            ADD_IMBALANCE_LIQUIDITY => Self::AddImbalanceLiquidity(AddImbalanceLiquidityInstruction::try_from_slice(payload)?),
            REMOVE_BALANCE_LIQUIDITY => Self::RemoveBalanceLiquidity(RemoveBalanceLiquidityInstruction::try_from_slice(payload)?),
            ADD_BALANCE_LIQUIDITY => Self::AddBalanceLiquidity(AddBalanceLiquidityInstruction::try_from_slice(payload)?),
            SET_POOL_FEES => Self::SetPoolFees(SetPoolFeesInstruction::try_from_slice(payload)?),
            OVERRIDE_CURVE_PARAM => Self::OverrideCurveParam(OverrideCurveParamInstruction::try_from_slice(payload)?),
            GET_POOL_INFO => Self::GetPoolInfo,
            BOOTSTRAP_LIQUIDITY => Self::BootstrapLiquidity(BootstrapLiquidityInstruction::try_from_slice(payload)?),
            CREATE_MINT_METADATA => Self::CreateMintMetadata,
            CREATE_LOCK_ESCROW => Self::CreateLockEscrow,
            LOCK => Self::Lock(LockInstruction::try_from_slice(payload)?),
            CLAIM_FEE => Self::ClaimFee(ClaimFeeInstruction::try_from_slice(payload)?),
            CREATE_CONFIG => Self::CreateConfig(CreateConfigInstruction::try_from_slice(payload)?),
            CLOSE_CONFIG => Self::CloseConfig,
            INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG => Self::InitializePermissionlessConstantProductPoolWithConfig(
                InitializePermissionlessConstantProductPoolWithConfigInstruction::try_from_slice(payload)?,
            ),
            INITIALIZE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL_WITH_CONFIG2 => Self::InitializePermissionlessConstantProductPoolWithConfig2(
                InitializePermissionlessConstantProductPoolWithConfig2Instruction::try_from_slice(payload)?,
            ),
            INITIALIZE_CUSTOMIZABLE_PERMISSIONLESS_CONSTANT_PRODUCT_POOL => Self::InitializeCustomizablePermissionlessConstantProductPool(
                InitializeCustomizablePermissionlessConstantProductPoolInstruction::try_from_slice(payload)?,
            ),
            UPDATE_ACTIVATION_POINT => Self::UpdateActivationPoint(UpdateActivationPointInstruction::try_from_slice(payload)?),
            WITHDRAW_PROTOCOL_FEES => Self::WithdrawProtocolFees,
            SET_WHITELISTED_VAULT => Self::SetWhitelistedVault(SetWhitelistedVaultInstruction::try_from_slice(payload)?),
            PARTNER_CLAIM_FEE => Self::PartnerClaimFee(PartnerClaimFeeInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<AmmInstruction, ParseError> {
    AmmInstruction::try_from(data)
}
