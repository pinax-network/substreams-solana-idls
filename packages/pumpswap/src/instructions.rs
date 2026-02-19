//! PumpSwap on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

// Discriminators (from IDL)
pub const ADMIN_SET_COIN_CREATOR: [u8; 8] = [242, 40, 117, 145, 73, 96, 105, 104];
pub const ADMIN_UPDATE_TOKEN_INCENTIVES: [u8; 8] = [209, 11, 115, 87, 213, 23, 124, 204];
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const BUY_EXACT_QUOTE_IN: [u8; 8] = [198, 46, 21, 82, 180, 217, 232, 112];
pub const CLAIM_TOKEN_INCENTIVES: [u8; 8] = [16, 4, 71, 28, 204, 1, 40, 27];
pub const CLOSE_USER_VOLUME_ACCUMULATOR: [u8; 8] = [249, 69, 164, 218, 150, 103, 84, 138];
pub const COLLECT_COIN_CREATOR_FEE: [u8; 8] = [160, 57, 89, 42, 181, 139, 43, 66];
pub const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
pub const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const DISABLE: [u8; 8] = [185, 173, 187, 90, 216, 15, 238, 233];
pub const EXTEND_ACCOUNT: [u8; 8] = [234, 102, 194, 203, 150, 72, 62, 229];
pub const INIT_USER_VOLUME_ACCUMULATOR: [u8; 8] = [94, 6, 202, 115, 255, 96, 232, 183];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const SET_COIN_CREATOR: [u8; 8] = [210, 149, 128, 45, 188, 58, 78, 175];
pub const SET_RESERVED_FEE_RECIPIENT: [u8; 8] = [207, 189, 178, 71, 167, 122, 68, 180];
pub const SYNC_USER_VOLUME_ACCUMULATOR: [u8; 8] = [86, 31, 192, 87, 163, 87, 79, 238];
pub const TOGGLE_MAYHEM_MODE: [u8; 8] = [1, 9, 111, 208, 100, 31, 255, 163];
pub const UPDATE_ADMIN: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];
pub const UPDATE_FEE_CONFIG: [u8; 8] = [104, 184, 103, 242, 88, 151, 107, 20];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

#[derive(Debug, Clone, PartialEq)]
pub enum PumpSwapInstruction {
    AdminSetCoinCreator(AdminSetCoinCreatorInstruction),
    AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesInstruction),
    Buy(BuyInstruction),
    BuyExactQuoteIn(BuyExactQuoteInInstruction),
    ClaimTokenIncentives,
    CloseUserVolumeAccumulator,
    CollectCoinCreatorFee,
    CreateConfig(CreateConfigInstruction),
    CreatePool(CreatePoolInstruction),
    Deposit(DepositInstruction),
    Disable(DisableInstruction),
    ExtendAccount,
    InitUserVolumeAccumulator,
    Sell(SellInstruction),
    SetCoinCreator,
    SetReservedFeeRecipient(SetReservedFeeRecipientInstruction),
    SyncUserVolumeAccumulator,
    ToggleMayhemMode(ToggleMayhemModeInstruction),
    UpdateAdmin,
    UpdateFeeConfig(UpdateFeeConfigInstruction),
    Withdraw(WithdrawInstruction),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AdminSetCoinCreatorInstruction {
    pub coin_creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AdminUpdateTokenIncentivesInstruction {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub day_number: u64,
    pub token_supply_per_day: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyInstruction {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyExactQuoteInInstruction {
    pub spendable_quote_in: u64,
    pub min_base_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateConfigInstruction {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolInstruction {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DisableInstruction {
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellInstruction {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SetReservedFeeRecipientInstruction {
    pub reserved_fee_recipient: Pubkey,
    pub whitelist_pda: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ToggleMayhemModeInstruction {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdateFeeConfigInstruction {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInstruction {
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
}

impl<'a> TryFrom<&'a [u8]> for PumpSwapInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            ADMIN_SET_COIN_CREATOR => Self::AdminSetCoinCreator(AdminSetCoinCreatorInstruction::try_from_slice(payload)?),
            ADMIN_UPDATE_TOKEN_INCENTIVES => Self::AdminUpdateTokenIncentives(AdminUpdateTokenIncentivesInstruction::try_from_slice(payload)?),
            BUY => Self::Buy(BuyInstruction::try_from_slice(payload)?),
            BUY_EXACT_QUOTE_IN => Self::BuyExactQuoteIn(BuyExactQuoteInInstruction::try_from_slice(payload)?),
            CLAIM_TOKEN_INCENTIVES => Self::ClaimTokenIncentives,
            CLOSE_USER_VOLUME_ACCUMULATOR => Self::CloseUserVolumeAccumulator,
            COLLECT_COIN_CREATOR_FEE => Self::CollectCoinCreatorFee,
            CREATE_CONFIG => Self::CreateConfig(CreateConfigInstruction::try_from_slice(payload)?),
            CREATE_POOL => Self::CreatePool(CreatePoolInstruction::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            DISABLE => Self::Disable(DisableInstruction::try_from_slice(payload)?),
            EXTEND_ACCOUNT => Self::ExtendAccount,
            INIT_USER_VOLUME_ACCUMULATOR => Self::InitUserVolumeAccumulator,
            SELL => Self::Sell(SellInstruction::try_from_slice(payload)?),
            SET_COIN_CREATOR => Self::SetCoinCreator,
            SET_RESERVED_FEE_RECIPIENT => Self::SetReservedFeeRecipient(SetReservedFeeRecipientInstruction::try_from_slice(payload)?),
            SYNC_USER_VOLUME_ACCUMULATOR => Self::SyncUserVolumeAccumulator,
            TOGGLE_MAYHEM_MODE => Self::ToggleMayhemMode(ToggleMayhemModeInstruction::try_from_slice(payload)?),
            UPDATE_ADMIN => Self::UpdateAdmin,
            UPDATE_FEE_CONFIG => Self::UpdateFeeConfig(UpdateFeeConfigInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<PumpSwapInstruction, ParseError> {
    PumpSwapInstruction::try_from(data)
}
