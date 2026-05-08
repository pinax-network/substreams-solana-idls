//! PumpSwap AMM on-chain account types.

use crate::common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;

use crate::common::accounts::AccountsError;

// Account discriminators (from IDL spec 0.1.0)
pub const BONDING_CURVE_DISC: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];
pub const FEE_CONFIG_DISC: [u8; 8] = [143, 52, 146, 187, 219, 123, 76, 155];
pub const GLOBAL_CONFIG_DISC: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];
pub const GLOBAL_VOLUME_ACCUMULATOR_DISC: [u8; 8] = [202, 42, 246, 43, 142, 190, 30, 255];
pub const POOL_DISC: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
pub const USER_VOLUME_ACCUMULATOR_DISC: [u8; 8] = [86, 255, 112, 14, 102, 53, 154, 250];

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Fees {
    pub lp_fee_bps: u64,
    pub protocol_fee_bps: u64,
    pub creator_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FeeTier {
    pub market_cap_lamports_threshold: u128,
    pub fees: Fees,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct FeeConfig {
    pub bump: u8,
    pub admin: Pubkey,
    pub flat_fees: Fees,
    pub fee_tiers: Vec<FeeTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
    pub whitelist_pda: Pubkey,
    pub reserved_fee_recipient: Pubkey,
    pub mayhem_mode_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct GlobalVolumeAccumulator {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub mint: Pubkey,
    pub total_token_supply: [u64; 30],
    pub sol_volumes: [u64; 30],
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Pool {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub lp_supply: u64,
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct UserVolumeAccumulator {
    pub user: Pubkey,
    pub needs_claim: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub has_total_claimed_tokens: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PumpSwapAccount {
    BondingCurve(BondingCurve),
    FeeConfig(FeeConfig),
    GlobalConfig(GlobalConfig),
    GlobalVolumeAccumulator(GlobalVolumeAccumulator),
    Pool(Pool),
    UserVolumeAccumulator(UserVolumeAccumulator),
}

// -----------------------------------------------------------------------------
// Trade-instruction account layout (Buy / Sell / BuyExactQuoteIn)
// -----------------------------------------------------------------------------
//
// Source of truth: `pumpswap/idl.json` — the `buy`, `sell`, and
// `buy_exact_quote_in` instructions all share the same first 11 accounts in
// the same order. The trailing PDAs (event_authority, program, accumulators,
// fee_config, …) vary by IDL version. Only the V2 `coin_creator_vault_*`
// accounts at indices 17/18 are exposed because they are the only trailing
// fields downstream consumers need.
//
// IMPORTANT: do not hand-index `instruction.accounts().get(N)` against this
// layout in adapter code. Use `TradeAccounts::try_from(ix)` so a future IDL
// re-ordering is caught at compile time / decode time, not silently mis-mapped.
pub const IDX_POOL: usize = 0;
pub const IDX_USER: usize = 1;
pub const IDX_GLOBAL_CONFIG: usize = 2;
pub const IDX_BASE_MINT: usize = 3;
pub const IDX_QUOTE_MINT: usize = 4;
pub const IDX_USER_BASE_TOKEN_ACCOUNT: usize = 5;
pub const IDX_USER_QUOTE_TOKEN_ACCOUNT: usize = 6;
pub const IDX_POOL_BASE_TOKEN_ACCOUNT: usize = 7;
pub const IDX_POOL_QUOTE_TOKEN_ACCOUNT: usize = 8;
pub const IDX_PROTOCOL_FEE_RECIPIENT: usize = 9;
pub const IDX_PROTOCOL_FEE_RECIPIENT_TOKEN_ACCOUNT: usize = 10;
// V2 trailing optional accounts.
pub const IDX_COIN_CREATOR_VAULT_ATA: usize = 17;
pub const IDX_COIN_CREATOR_VAULT_AUTHORITY: usize = 18;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct TradeAccounts {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub global_config: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator_vault_ata: Option<Pubkey>,
    pub coin_creator_vault_authority: Option<Pubkey>,
}

impl<'ix> TryFrom<&InstructionView<'ix>> for TradeAccounts {
    type Error = AccountsError;

    fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
        let accounts = ix.accounts();

        let get_req = |index: usize, name: &'static str| -> Result<Pubkey, AccountsError> {
            let a = accounts.get(index).ok_or(AccountsError::Missing { name, index })?;
            crate::common::accounts::to_pubkey(name, index, a.0)
        };

        let get_opt = |index: usize| -> Option<Pubkey> {
            accounts
                .get(index)
                .and_then(|a| a.0.as_slice().try_into().ok())
                .map(Pubkey::new_from_array)
        };

        Ok(TradeAccounts {
            pool: get_req(IDX_POOL, "pool")?,
            user: get_req(IDX_USER, "user")?,
            global_config: get_req(IDX_GLOBAL_CONFIG, "global_config")?,
            base_mint: get_req(IDX_BASE_MINT, "base_mint")?,
            quote_mint: get_req(IDX_QUOTE_MINT, "quote_mint")?,
            user_base_token_account: get_req(IDX_USER_BASE_TOKEN_ACCOUNT, "user_base_token_account")?,
            user_quote_token_account: get_req(IDX_USER_QUOTE_TOKEN_ACCOUNT, "user_quote_token_account")?,
            pool_base_token_account: get_req(IDX_POOL_BASE_TOKEN_ACCOUNT, "pool_base_token_account")?,
            pool_quote_token_account: get_req(IDX_POOL_QUOTE_TOKEN_ACCOUNT, "pool_quote_token_account")?,
            protocol_fee_recipient: get_req(IDX_PROTOCOL_FEE_RECIPIENT, "protocol_fee_recipient")?,
            protocol_fee_recipient_token_account: get_req(
                IDX_PROTOCOL_FEE_RECIPIENT_TOKEN_ACCOUNT,
                "protocol_fee_recipient_token_account",
            )?,
            coin_creator_vault_ata: get_opt(IDX_COIN_CREATOR_VAULT_ATA),
            coin_creator_vault_authority: get_opt(IDX_COIN_CREATOR_VAULT_AUTHORITY),
        })
    }
}

/// Resolve the typed account list for a `buy` instruction.
pub fn get_buy_accounts(ix: &InstructionView) -> Result<TradeAccounts, AccountsError> {
    TradeAccounts::try_from(ix)
}

/// Resolve the typed account list for a `sell` instruction.
pub fn get_sell_accounts(ix: &InstructionView) -> Result<TradeAccounts, AccountsError> {
    TradeAccounts::try_from(ix)
}

/// Resolve the typed account list for a `buy_exact_quote_in` instruction.
pub fn get_buy_exact_quote_in_accounts(ix: &InstructionView) -> Result<TradeAccounts, AccountsError> {
    TradeAccounts::try_from(ix)
}

pub fn unpack_account(data: &[u8]) -> Result<PumpSwapAccount, ParseError> {
    if data.len() < 8 {
        return Err(ParseError::TooShort(data.len()));
    }
    let (disc, rest) = data.split_at(8);
    let disc: [u8; 8] = disc.try_into().unwrap();
    Ok(match disc {
        BONDING_CURVE_DISC => PumpSwapAccount::BondingCurve(BondingCurve::try_from_slice(rest)?),
        FEE_CONFIG_DISC => PumpSwapAccount::FeeConfig(FeeConfig::try_from_slice(rest)?),
        GLOBAL_CONFIG_DISC => PumpSwapAccount::GlobalConfig(GlobalConfig::try_from_slice(rest)?),
        GLOBAL_VOLUME_ACCUMULATOR_DISC => PumpSwapAccount::GlobalVolumeAccumulator(GlobalVolumeAccumulator::try_from_slice(rest)?),
        POOL_DISC => PumpSwapAccount::Pool(Pool::try_from_slice(rest)?),
        USER_VOLUME_ACCUMULATOR_DISC => PumpSwapAccount::UserVolumeAccumulator(UserVolumeAccumulator::try_from_slice(rest)?),
        _ => return Err(ParseError::Unknown(disc)),
    })
}
