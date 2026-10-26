//! Pump.fun AMM on-chain instructions (auto-generated from IDL).

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// -------------------------------------------------------------------------
// Discriminators
// -------------------------------------------------------------------------
const BUY: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234]; // 66063d1201daebea
const BUY_EXACT_QUOTE_IN: [u8; 8] = [198, 46, 21, 82, 180, 217, 232, 112]; // c62e1552b4d9e870
const SELL: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173]; // 33e685a4017f83ad
const CREATE_CONFIG: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
const CREATE_POOL: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188]; // e992d18ecf6840bc
const CREATE_POOL_V3_PAYLOAD_SIZE: usize = 62;
const DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
const DISABLE: [u8; 8] = [185, 173, 187, 90, 216, 15, 238, 233];
const EXTEND_ACCOUNT: [u8; 8] = [234, 102, 194, 203, 150, 72, 62, 229];
const UPDATE_ADMIN: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];
const UPDATE_FEE_CONFIG: [u8; 8] = [104, 184, 103, 242, 88, 151, 107, 20];
const WITHDRAW: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// -------------------------------------------------------------------------
// Instruction enumeration
// -------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum PumpFunAmmInstruction {
    Buy(BuyInstruction),
    BuyExactQuoteIn(BuyExactQuoteInInstruction),
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
    // Append new variants to preserve the existing Borsh enum discriminants.
    CreatePoolV3(CreatePoolInstructionV3),
}

// -------------------------------------------------------------------------
// Payload structs
// -------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyInstruction {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyExactQuoteInInstruction {
    pub spendable_quote_in: u64,
    pub min_base_amount_out: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateConfigInstruction {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolInstructionV1 {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolInstructionV2 {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub coin_creator: Pubkey,
}

/// Current IDL creation arguments. The first fields retain the V2 wire layout.
/// `OptionBool` and `OptionU64` are IDL tuple wrappers, not Rust `Option<T>`;
/// neither adds a Borsh option discriminant. See `docs/pump-amm-creation.md`.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreatePoolInstructionV3 {
    pub base: CreatePoolInstructionV2,
    pub is_mayhem_mode: bool,
    pub is_cashback_coin: OptionBool,
    pub creator_fee_bps: OptionU64,
    pub can_edit_creator_fee: OptionBool,
    pub is_holder_reward: OptionBool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OptionBool(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct OptionU64(pub u64);
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DisableInstruction {
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellInstruction {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UpdateFeeConfigInstruction {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Pubkey; 8],
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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

        // Match the current PumpSwap OptionBool suffix while retaining support
        // for historical amount-only buy arguments in this compatibility module.
        let payload = match discriminator {
            BUY | BUY_EXACT_QUOTE_IN if payload.len() == 17 => {
                bool::try_from_slice(&payload[16..])?;
                &payload[..16]
            }
            _ => payload,
        };

        Ok(match discriminator {
            BUY => Self::Buy(BuyInstruction::try_from_slice(payload)?),
            BUY_EXACT_QUOTE_IN => Self::BuyExactQuoteIn(BuyExactQuoteInInstruction::try_from_slice(payload)?),
            SELL => Self::Sell(SellInstruction::try_from_slice(payload)?),
            CREATE_CONFIG => Self::CreateConfig(CreateConfigInstruction::try_from_slice(payload)?),
            CREATE_POOL if payload.len() == CREATE_POOL_V3_PAYLOAD_SIZE => Self::CreatePoolV3(CreatePoolInstructionV3::try_from_slice(payload)?),
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
