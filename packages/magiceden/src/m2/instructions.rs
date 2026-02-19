//! Magic Eden M2 on-chain instructions.

use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MIP1ExecuteSaleV2Args {
    pub price: u64,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MIP1SellArgs {
    pub price: u64,
    pub expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OCPExecuteSaleV2Args {
    pub price: u64,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OCPSellArgs {
    pub price: u64,
    pub expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawByMMMArgs {
    pub wallet: Pubkey,
    pub auction_house: Pubkey,
    pub amount: u64,
    pub mmm_pool_uuid: Pubkey,
}

// Discriminators
pub const WITHDRAW_FROM_TREASURY: [u8; 8] = [0, 164, 86, 76, 56, 72, 12, 170];
pub const UPDATE_AUCTION_HOUSE: [u8; 8] = [84, 215, 2, 172, 241, 0, 245, 219];
pub const CREATE_AUCTION_HOUSE: [u8; 8] = [221, 66, 242, 159, 249, 206, 134, 241];
pub const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
pub const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
pub const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
pub const CANCEL_SELL: [u8; 8] = [198, 198, 130, 203, 163, 95, 175, 75];
pub const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const BUY_V2: [u8; 8] = [184, 23, 238, 97, 103, 197, 211, 61];
pub const CANCEL_BUY: [u8; 8] = [238, 76, 36, 218, 132, 177, 224, 233];
pub const OCP_SELL: [u8; 8] = [22, 41, 217, 220, 21, 104, 61, 99];
pub const OCP_CANCEL_SELL: [u8; 8] = [73, 4, 55, 246, 37, 155, 2, 166];
pub const OCP_EXECUTE_SALE_V2: [u8; 8] = [200, 83, 31, 82, 156, 156, 20, 97];
pub const EXECUTE_SALE_V2: [u8; 8] = [91, 220, 49, 223, 204, 129, 53, 193];
pub const MIP1_SELL: [u8; 8] = [58, 50, 172, 111, 166, 151, 22, 94];
pub const MIP1_EXECUTE_SALE_V2: [u8; 8] = [236, 163, 204, 173, 71, 144, 235, 118];
pub const MIP1_CANCEL_SELL: [u8; 8] = [74, 190, 185, 225, 88, 105, 209, 156];
pub const WITHDRAW_BY_MMM: [u8; 8] = [35, 73, 133, 139, 32, 55, 213, 140];

#[derive(Debug, Clone, PartialEq)]
pub enum MagicEdenInstruction {
    WithdrawFromTreasury(WithdrawFromTreasuryInstruction),
    UpdateAuctionHouse(UpdateAuctionHouseInstruction),
    CreateAuctionHouse(CreateAuctionHouseInstruction),
    Withdraw(WithdrawInstruction),
    Deposit(DepositInstruction),
    Sell(SellInstruction),
    CancelSell(CancelSellInstruction),
    Buy(BuyInstruction),
    BuyV2(BuyV2Instruction),
    CancelBuy(CancelBuyInstruction),
    OcpSell(OcpSellInstruction),
    OcpCancelSell,
    OcpExecuteSaleV2(OcpExecuteSaleV2Instruction),
    ExecuteSaleV2(ExecuteSaleV2Instruction),
    Mip1Sell(Mip1SellInstruction),
    Mip1ExecuteSaleV2(Mip1ExecuteSaleV2Instruction),
    Mip1CancelSell,
    WithdrawByMmm(WithdrawByMmmInstruction),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawFromTreasuryInstruction {
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UpdateAuctionHouseInstruction {
    pub seller_fee_basis_points: Option<u16>,
    pub buyer_referral_bp: Option<u16>,
    pub seller_referral_bp: Option<u16>,
    pub requires_notary: Option<bool>,
    pub nprob: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateAuctionHouseInstruction {
    pub bump: u8,
    pub treasury_bump: u8,
    pub seller_fee_basis_points: u16,
    pub buyer_referral_bp: u16,
    pub seller_referral_bp: u16,
    pub requires_notary: bool,
    pub create_auction_house_nonce: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInstruction {
    pub escrow_payment_bump: u8,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub escrow_payment_bump: u8,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SellInstruction {
    pub seller_state_bump: u8,
    pub program_as_signer_bump: u8,
    pub buyer_price: u64,
    pub token_size: u64,
    pub seller_state_expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CancelSellInstruction {
    pub buyer_price: u64,
    pub token_size: u64,
    pub seller_state_expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyInstruction {
    pub buyer_state_bump: u8,
    pub escrow_payment_bump: u8,
    pub buyer_price: u64,
    pub token_size: u64,
    pub buyer_state_expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BuyV2Instruction {
    pub buyer_price: u64,
    pub token_size: u64,
    pub buyer_state_expiry: i64,
    pub buyer_creator_royalty_bp: u16,
    pub extra_args: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CancelBuyInstruction {
    pub buyer_price: u64,
    pub token_size: u64,
    pub buyer_state_expiry: i64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OcpSellInstruction {
    pub args: OCPSellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OcpExecuteSaleV2Instruction {
    pub args: OCPExecuteSaleV2Args,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ExecuteSaleV2Instruction {
    pub escrow_payment_bump: u8,
    pub program_as_signer_bump: u8,
    pub buyer_price: u64,
    pub token_size: u64,
    pub buyer_state_expiry: i64,
    pub seller_state_expiry: i64,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: u16,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Mip1SellInstruction {
    pub args: MIP1SellArgs,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Mip1ExecuteSaleV2Instruction {
    pub args: MIP1ExecuteSaleV2Args,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawByMmmInstruction {
    pub args: WithdrawByMMMArgs,
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
            WITHDRAW_FROM_TREASURY => Self::WithdrawFromTreasury(WithdrawFromTreasuryInstruction::try_from_slice(payload)?),
            UPDATE_AUCTION_HOUSE => Self::UpdateAuctionHouse(UpdateAuctionHouseInstruction::try_from_slice(payload)?),
            CREATE_AUCTION_HOUSE => Self::CreateAuctionHouse(CreateAuctionHouseInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            SELL => Self::Sell(SellInstruction::try_from_slice(payload)?),
            CANCEL_SELL => Self::CancelSell(CancelSellInstruction::try_from_slice(payload)?),
            BUY => Self::Buy(BuyInstruction::try_from_slice(payload)?),
            BUY_V2 => Self::BuyV2(BuyV2Instruction::try_from_slice(payload)?),
            CANCEL_BUY => Self::CancelBuy(CancelBuyInstruction::try_from_slice(payload)?),
            OCP_SELL => Self::OcpSell(OcpSellInstruction::try_from_slice(payload)?),
            OCP_CANCEL_SELL => Self::OcpCancelSell,
            OCP_EXECUTE_SALE_V2 => Self::OcpExecuteSaleV2(OcpExecuteSaleV2Instruction::try_from_slice(payload)?),
            EXECUTE_SALE_V2 => Self::ExecuteSaleV2(ExecuteSaleV2Instruction::try_from_slice(payload)?),
            MIP1_SELL => Self::Mip1Sell(Mip1SellInstruction::try_from_slice(payload)?),
            MIP1_EXECUTE_SALE_V2 => Self::Mip1ExecuteSaleV2(Mip1ExecuteSaleV2Instruction::try_from_slice(payload)?),
            MIP1_CANCEL_SELL => Self::Mip1CancelSell,
            WITHDRAW_BY_MMM => Self::WithdrawByMmm(WithdrawByMmmInstruction::try_from_slice(payload)?),
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

pub fn unpack(data: &[u8]) -> Result<MagicEdenInstruction, ParseError> {
    MagicEdenInstruction::try_from(data)
}
