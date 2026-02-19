//! Magic Eden M3 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Allowlist {
    pub kind: u8,
    pub value: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolArgs {
    pub spot_price: u64,
    pub curve_type: u8,
    pub curve_delta: u64,
    pub reinvest_fulfill_buy: bool,
    pub reinvest_fulfill_sell: bool,
    pub expiry: i64,
    pub lp_fee_bp: u16,
    pub referral: Pubkey,
    pub cosigner_annotation: [u8; 32],
    pub buyside_creator_royalty_bp: u16,
    pub uuid: Pubkey,
    pub payment_mint: Pubkey,
    pub allowlists: [Allowlist; 6],
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DepositSellArgs {
    pub asset_amount: u64,
    pub allowlist_aux: Option<String>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SetSharedEscrowArgs {
    pub shared_escrow_count: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolDepositBuyArgs {
    pub payment_amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolFulfillBuyArgs {
    pub asset_amount: u64,
    pub min_payment_amount: u64,
    pub allowlist_aux: Option<String>,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolFulfillSellArgs {
    pub asset_amount: u64,
    pub max_payment_amount: u64,
    pub buyside_creator_royalty_bp: u16,
    pub allowlist_aux: Option<String>,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolMip1FulfillSellArgs {
    pub asset_amount: u64,
    pub max_payment_amount: u64,
    pub allowlist_aux: Option<String>,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolOcpFulfillSellArgs {
    pub asset_amount: u64,
    pub max_payment_amount: u64,
    pub allowlist_aux: Option<String>,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: i16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolWithdrawBuyArgs {
    pub payment_amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdateAllowlistsArgs {
    pub allowlists: [Allowlist; 6],
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdatePoolArgs {
    pub spot_price: u64,
    pub curve_type: u8,
    pub curve_delta: u64,
    pub reinvest_fulfill_buy: bool,
    pub reinvest_fulfill_sell: bool,
    pub expiry: i64,
    pub lp_fee_bp: u16,
    pub referral: Pubkey,
    pub cosigner_annotation: [u8; 32],
    pub buyside_creator_royalty_bp: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawSellArgs {
    pub asset_amount: u64,
    pub allowlist_aux: Option<String>,
}

// Discriminators
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const UPDATE_POOL: [u8; 8] = [239, 214, 170, 78, 36, 35, 30, 34];
pub const UPDATE_ALLOWLISTS: [u8; 8] = [28, 78, 222, 235, 209, 68, 182, 83];
pub const SOL_CLOSE_POOL: [u8; 8] = [248, 125, 232, 20, 117, 78, 234, 234];
pub const SOL_DEPOSIT_BUY: [u8; 8] = [66, 229, 11, 54, 109, 164, 85, 238];
pub const SOL_WITHDRAW_BUY: [u8; 8] = [154, 41, 80, 232, 174, 48, 246, 35];
pub const SOL_FULFILL_BUY: [u8; 8] = [92, 16, 226, 79, 31, 242, 53, 118];
pub const SOL_FULFILL_SELL: [u8; 8] = [164, 180, 96, 192, 103, 225, 105, 232];
pub const WITHDRAW_SELL: [u8; 8] = [47, 42, 156, 228, 249, 163, 254, 185];
pub const DEPOSIT_SELL: [u8; 8] = [144, 131, 44, 156, 197, 10, 197, 43];
pub const OCP_DEPOSIT_SELL: [u8; 8] = [33, 137, 85, 133, 77, 224, 174, 253];
pub const SOL_OCP_FULFILL_BUY: [u8; 8] = [113, 225, 170, 65, 181, 212, 10, 33];
pub const SOL_OCP_FULFILL_SELL: [u8; 8] = [213, 40, 58, 99, 129, 109, 245, 147];
pub const OCP_WITHDRAW_SELL: [u8; 8] = [63, 213, 54, 109, 109, 182, 137, 191];
pub const MIP1_DEPOSIT_SELL: [u8; 8] = [158, 89, 94, 253, 120, 17, 120, 47];
pub const MIP1_WITHDRAW_SELL: [u8; 8] = [177, 206, 154, 141, 3, 248, 128, 16];
pub const SOL_MIP1_FULFILL_SELL: [u8; 8] = [59, 11, 73, 107, 40, 105, 64, 210];
pub const SOL_MIP1_FULFILL_BUY: [u8; 8] = [236, 82, 158, 122, 8, 24, 175, 145];
pub const CLOSE_IF_BALANCE_INVALID: [u8; 8] = [69, 42, 108, 96, 93, 97, 112, 200];
pub const SET_SHARED_ESCROW: [u8; 8] = [201, 167, 60, 154, 7, 24, 237, 247];

#[derive(Debug, Clone, PartialEq)]
pub enum MagicEdenInstruction {
    CreatePool(CreatePoolInstruction),
    UpdatePool(UpdatePoolInstruction),
    UpdateAllowlists(UpdateAllowlistsInstruction),
    SolClosePool,
    SolDepositBuy(SolDepositBuyInstruction),
    SolWithdrawBuy(SolWithdrawBuyInstruction),
    SolFulfillBuy(SolFulfillBuyInstruction),
    SolFulfillSell(SolFulfillSellInstruction),
    WithdrawSell(WithdrawSellInstruction),
    DepositSell(DepositSellInstruction),
    OcpDepositSell(OcpDepositSellInstruction),
    SolOcpFulfillBuy(SolOcpFulfillBuyInstruction),
    SolOcpFulfillSell(SolOcpFulfillSellInstruction),
    OcpWithdrawSell(OcpWithdrawSellInstruction),
    Mip1DepositSell(Mip1DepositSellInstruction),
    Mip1WithdrawSell(Mip1WithdrawSellInstruction),
    SolMip1FulfillSell(SolMip1FulfillSellInstruction),
    SolMip1FulfillBuy(SolMip1FulfillBuyInstruction),
    CloseIfBalanceInvalid,
    SetSharedEscrow(SetSharedEscrowInstruction),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolInstruction {
    pub args: CreatePoolArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdatePoolInstruction {
    pub args: UpdatePoolArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdateAllowlistsInstruction {
    pub args: UpdateAllowlistsArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolDepositBuyInstruction {
    pub args: SolDepositBuyArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolWithdrawBuyInstruction {
    pub args: SolWithdrawBuyArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolFulfillBuyInstruction {
    pub args: SolFulfillBuyArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolFulfillSellInstruction {
    pub args: SolFulfillSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawSellInstruction {
    pub args: WithdrawSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DepositSellInstruction {
    pub args: DepositSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OcpDepositSellInstruction {
    pub args: DepositSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolOcpFulfillBuyInstruction {
    pub args: SolFulfillBuyArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolOcpFulfillSellInstruction {
    pub args: SolOcpFulfillSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OcpWithdrawSellInstruction {
    pub args: WithdrawSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Mip1DepositSellInstruction {
    pub args: DepositSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Mip1WithdrawSellInstruction {
    pub args: WithdrawSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolMip1FulfillSellInstruction {
    pub args: SolMip1FulfillSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SolMip1FulfillBuyInstruction {
    pub args: SolFulfillBuyArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SetSharedEscrowInstruction {
    pub args: SetSharedEscrowArgs,
}

impl<'a> TryFrom<&'a [u8]> for MagicEdenInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            CREATE_POOL => Self::CreatePool(CreatePoolInstruction::try_from_slice(payload)?),
            UPDATE_POOL => Self::UpdatePool(UpdatePoolInstruction::try_from_slice(payload)?),
            UPDATE_ALLOWLISTS => Self::UpdateAllowlists(UpdateAllowlistsInstruction::try_from_slice(payload)?),
            SOL_CLOSE_POOL => Self::SolClosePool,
            SOL_DEPOSIT_BUY => Self::SolDepositBuy(SolDepositBuyInstruction::try_from_slice(payload)?),
            SOL_WITHDRAW_BUY => Self::SolWithdrawBuy(SolWithdrawBuyInstruction::try_from_slice(payload)?),
            SOL_FULFILL_BUY => Self::SolFulfillBuy(SolFulfillBuyInstruction::try_from_slice(payload)?),
            SOL_FULFILL_SELL => Self::SolFulfillSell(SolFulfillSellInstruction::try_from_slice(payload)?),
            WITHDRAW_SELL => Self::WithdrawSell(WithdrawSellInstruction::try_from_slice(payload)?),
            DEPOSIT_SELL => Self::DepositSell(DepositSellInstruction::try_from_slice(payload)?),
            OCP_DEPOSIT_SELL => Self::OcpDepositSell(OcpDepositSellInstruction::try_from_slice(payload)?),
            SOL_OCP_FULFILL_BUY => Self::SolOcpFulfillBuy(SolOcpFulfillBuyInstruction::try_from_slice(payload)?),
            SOL_OCP_FULFILL_SELL => Self::SolOcpFulfillSell(SolOcpFulfillSellInstruction::try_from_slice(payload)?),
            OCP_WITHDRAW_SELL => Self::OcpWithdrawSell(OcpWithdrawSellInstruction::try_from_slice(payload)?),
            MIP1_DEPOSIT_SELL => Self::Mip1DepositSell(Mip1DepositSellInstruction::try_from_slice(payload)?),
            MIP1_WITHDRAW_SELL => Self::Mip1WithdrawSell(Mip1WithdrawSellInstruction::try_from_slice(payload)?),
            SOL_MIP1_FULFILL_SELL => Self::SolMip1FulfillSell(SolMip1FulfillSellInstruction::try_from_slice(payload)?),
            SOL_MIP1_FULFILL_BUY => Self::SolMip1FulfillBuy(SolMip1FulfillBuyInstruction::try_from_slice(payload)?),
            CLOSE_IF_BALANCE_INVALID => Self::CloseIfBalanceInvalid,
            SET_SHARED_ESCROW => Self::SetSharedEscrow(SetSharedEscrowInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<MagicEdenInstruction, ParseError> {
    MagicEdenInstruction::try_from(data)
}
