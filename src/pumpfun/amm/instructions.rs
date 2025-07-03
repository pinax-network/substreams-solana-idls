//! Pump.fun AMM on-chain instructions (auto-generated from IDL).

use crate::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

// -------------------------------------------------------------------------
// Discriminators
// -------------------------------------------------------------------------
const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234]; // 33e685a4017f83ad
const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173]; // 66063d1201daebea
const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188]; // e992d18ecf6840bc
const _CREATE_POOL_V1_SIZE: usize = 0;
const _CREATE_POOL_V2_SIZE: usize = 341;
const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
const DISABLE: [u8; 8] = [185, 173, 187, 90, 216, 15, 238, 233];
const EXTEND_ACCOUNT: [u8; 8] = [234, 102, 194, 203, 150, 72, 62, 229];
const UPDATE_ADMIN: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];
const UPDATE_FEE_CONFIG: [u8; 8] = [104, 184, 103, 242, 88, 151, 107, 20];
const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// -------------------------------------------------------------------------
// Instruction enumeration
// -------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum PumpFunAmmInstruction {
    Buy(BuyInstruction),
    CreateConfig(CreateConfigInstruction),
    CreatePoolV1(CreatePoolInstructionV1),
    CreatePoolV2(CreatePoolInstructionV2),
    Deposit(DepositInstruction),
    Disable(DisableInstruction),
    ExtendAccount,
    Sell(SellInstruction),
    UpdateAdmin,
    UpdateFeeConfig(UpdateFeeConfigInstruction),
    Withdraw(WithdrawInstruction),
    Unknown,
}

// -------------------------------------------------------------------------
// Payload structs
// -------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct BuyInstruction {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreateConfigInstruction {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreatePoolInstructionV1 {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CreatePoolInstructionV2 {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub coin_creator: Pubkey,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositInstruction {
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DisableInstruction {
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SellInstruction {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UpdateFeeConfigInstruction {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawInstruction {
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for PumpFunAmmInstruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            BUY => Self::Buy(BuyInstruction::try_from_slice(payload)?),
            SELL => Self::Sell(SellInstruction::try_from_slice(payload)?),
            CREATE_CONFIG => Self::CreateConfig(CreateConfigInstruction::try_from_slice(payload)?),
            CREATE_POOL => Self::CreatePoolV2(CreatePoolInstructionV2::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            DISABLE => Self::Disable(DisableInstruction::try_from_slice(payload)?),
            EXTEND_ACCOUNT => Self::ExtendAccount,
            UPDATE_ADMIN => Self::UpdateAdmin,
            UPDATE_FEE_CONFIG => Self::UpdateFeeConfig(UpdateFeeConfigInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            // If the discriminator does not match any known instruction, return Unknown.
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

// Convenience function retaining the old name; forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<PumpFunAmmInstruction, ParseError> {
    PumpFunAmmInstruction::try_from(data)
}
