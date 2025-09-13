#![allow(deprecated)]

//! on-chain instruction definitions and Borsh deserialisation helpers.
// https://github.com/raydium-io/raydium-amm/blob/master/program/src/instruction.rs
use borsh::{BorshDeserialize, BorshSerialize};
use common::ParseError;
// use solana_program::pubkey::Pubkey;

// -----------------------------------------------------------------------------
// Discriminators (first 8 bytes of the instruction data)
// -----------------------------------------------------------------------------
pub const INITIALIZE: u8 = 0;
pub const INITIALIZE2: u8 = 1;
pub const MONITOR_STEP: u8 = 2;
pub const DEPOSIT: u8 = 3;
pub const WITHDRAW: u8 = 4;
pub const MIGRATE_TO_OPENBOOK: u8 = 5;
pub const SET_PARAMS: u8 = 6;
pub const WITHDRAW_PNL: u8 = 7;
pub const WITHDRAW_SRM: u8 = 8;
pub const SWAP_BASE_IN: u8 = 9;
pub const PRE_INITIALIZE: u8 = 10;
pub const SWAP_BASE_OUT: u8 = 11;
pub const SIMULATE_INFO: u8 = 12;
pub const ADMIN_CANCEL_ORDERS: u8 = 13;
pub const CREATE_CONFIG_ACCOUNT: u8 = 14;
pub const UPDATE_CONFIG_ACCOUNT: u8 = 15;

// The canonical SwapBaseIn payload is 17 bytes.
// Extra bytes (the 0x40 you saw) are legal and ignored by the contract.
// The discriminator is 1 byte, followed by 16 bytes of payload.
//
// The following transaction includes 18 bytes of data, but the first 17 are the
// canonical payload. The 18th byte is ignored by the contract.
// https://solscan.io/tx/3JzyAsW5DZiX6B4CHp4TLkim3ksqoWhRHj2my7qNaGuPrDakoAoUqCa51tWjETTHsVZneFQPjmkRNAEkrn9Q1665
const SWAP_LEN: usize = 16; // Length of the swap instructions (excluding discriminator)

// ──────────────────────────────────────────────────────────────────────────────
// High-level enum
// ──────────────────────────────────────────────────────────────────────────────
/// Instructions supported by the Raydium-v4 AMM program.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum RaydiumV4Instruction {
    // ─────────────────────────────────────────────────────────────────────
    // Pool creation
    // ─────────────────────────────────────────────────────────────────────
    /// ### What happens
    /// * Creates all PDA / vault / market accounts for a brand-new pool.
    /// * Seeds the pool with `init_coin_amount` + `init_pc_amount`.
    /// * Mints initial LP tokens to the user.
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose                                                         |
    /// |----|-------------|-----------------------------------------------------------------|
    /// | 0  | `[]`        | SPL-Token program                                               |
    /// | 1  | `[]`        | Associated-Token program                                        |
    /// | 2  | `[]`        | System program                                                  |
    /// | 3  | `[]`        | Rent sysvar                                                    |
    /// | 4  | `[w]`       | **New** AMM account                                             |
    /// | 5  | `[]`        | AMM authority **PDA**                                           |
    /// | 6  | `[w]`       | AMM open-orders                                                 |
    /// | 7  | `[w]`       | AMM LP-mint                                                    |
    /// | 8  | `[]`        | AMM coin-mint                                                  |
    /// | 9  | `[]`        | AMM pc-mint                                                    |
    /// | 10 | `[w]`       | AMM coin-vault (base)                                          |
    /// | 11 | `[w]`       | AMM pc-vault  (quote)                                          |
    /// | 12 | `[w]`       | AMM target-orders                                              |
    /// | 13 | `[]`        | AMM config **PDA**                                             |
    /// | 14 | `[]`        | Pool-creation fee recipient                                    |
    /// | 15 | `[]`        | Market program id (OpenBook / Serum)                           |
    /// | 16 | `[w]`       | Market account                                                 |
    /// | 17 | `[s]`       | User wallet (payer)                                            |
    /// | 18 | `[]`        | User coin ATA (source)                                         |
    /// | 19 | `[]`        | User pc   ATA (source)                                         |
    /// | 20 | `[w]`       | User LP-token ATA (destination)                                |
    Initialize2(Initialize2Instruction),

    /// *Deprecated – kept only for legacy pools.*
    #[deprecated(note = "Not supported; use `Initialize2` instead")]
    Initialize(InitializeInstruction),

    // ─────────────────────────────────────────────────────────────────────
    // Crank / monitoring
    // ─────────────────────────────────────────────────────────────────────
    /// ### What happens
    /// * Places or cancels orders to keep the pool balanced.
    /// * Syncs on-chain inventory ↔ order-book inventory.
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose                                   |
    /// |----|-------------|-------------------------------------------|
    /// | 0  | `[]`        | SPL-Token program                         |
    /// | 1  | `[]`        | Rent sysvar                              |
    /// | 2  | `[]`        | Clock sysvar                             |
    /// | 3  | `[w]`       | AMM account                              |
    /// | 4  | `[]`        | AMM authority **PDA**                     |
    /// | 5  | `[w]`       | AMM open-orders                          |
    /// | 6  | `[w]`       | AMM target-orders                        |
    /// | 7  | `[w]`       | AMM coin-vault                           |
    /// | 8  | `[w]`       | AMM pc-vault                             |
    /// | 9  | `[]`        | Market program id                        |
    /// | 10 | `[w]`       | Market account                           |
    /// | 11 | `[w]`       | Market coin-vault                        |
    /// | 12 | `[w]`       | Market pc-vault                          |
    /// | 13 | `[]`        | Market vault-signer **PDA**              |
    /// | 14 | `[w]`       | Market request-queue                     |
    /// | 15 | `[w]`       | Market event-queue                       |
    /// | 16 | `[w]`       | Market bids slab                         |
    /// | 17 | `[w]`       | Market asks slab                         |
    /// | 18 | `[w]`       | *(opt)* SRM/MSRM discount account        |
    /// | 19 | `[w]`       | *(opt)* Referrer pc account              |
    MonitorStep(MonitorStepInstruction),

    // ─────────────────────────────────────────────────────────────────────
    // Liquidity-provision
    // ─────────────────────────────────────────────────────────────────────
    /// ### What happens
    /// * Transfers `max_coin_amount` + `max_pc_amount` into the pool.
    /// * Mints LP tokens to the user (slippage-checked).
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose                                            |
    /// |----|-------------|----------------------------------------------------|
    /// | 0  | `[]`        | SPL-Token program                                  |
    /// | 1  | `[w]`       | AMM account                                        |
    /// | 2  | `[]`        | AMM authority **PDA**                               |
    /// | 3  | `[]`        | AMM open-orders                                    |
    /// | 4  | `[w]`       | AMM target-orders                                  |
    /// | 5  | `[w]`       | AMM LP-mint                                        |
    /// | 6  | `[w]`       | AMM coin-vault                                     |
    /// | 7  | `[w]`       | AMM pc-vault                                       |
    /// | 8  | `[]`        | Market account                                     |
    /// | 9  | `[w]`       | User coin ATA (source)                             |
    /// | 10 | `[w]`       | User pc   ATA (source)                             |
    /// | 11 | `[w]`       | User LP-token ATA (destination)                    |
    /// | 12 | `[s]`       | User wallet                                        |
    /// | 13 | `[]`        | Market event-queue                                 |
    Deposit(DepositInstruction),

    /// ### What happens
    /// * Burns `amount` LP tokens.
    /// * Sends proportional coin/pc back to the user.
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose                                            |
    /// |----|-------------|----------------------------------------------------|
    /// | 0  | `[]`        | SPL-Token program                                  |
    /// | 1  | `[w]`       | AMM account                                        |
    /// | 2  | `[]`        | AMM authority **PDA**                               |
    /// | 3  | `[w]`       | AMM open-orders                                    |
    /// | 4  | `[w]`       | AMM target-orders                                  |
    /// | 5  | `[w]`       | AMM LP-mint                                        |
    /// | 6  | `[w]`       | AMM coin-vault                                     |
    /// | 7  | `[w]`       | AMM pc-vault                                       |
    /// | 8  | `[]`        | Market program id                                  |
    /// | 9  | `[w]`       | Market account                                     |
    /// | 10 | `[w]`       | Market coin-vault                                  |
    /// | 11 | `[w]`       | Market pc-vault                                    |
    /// | 12 | `[]`        | Market vault-signer **PDA**                        |
    /// | 13 | `[w]`       | User LP-token ATA (source)                         |
    /// | 14 | `[w]`       | User coin ATA (destination)                        |
    /// | 15 | `[w]`       | User pc   ATA (destination)                        |
    /// | 16 | `[s]`       | User wallet                                        |
    /// | 17 | `[w]`       | Market event-queue                                 |
    /// | 18 | `[w]`       | Market bids slab                                   |
    /// | 19 | `[w]`       | Market asks slab                                   |
    Withdraw(WithdrawInstruction),

    // ─────────────────────────────────────────────────────────────────────
    // Swaps
    // ─────────────────────────────────────────────────────────────────────
    /// ### What happens
    /// * Debits `amount_in` **base** tokens from the user.
    /// * Credits ≥`min_amount_out` **quote** tokens to the user.
    /// * Routes through AMM curve + order-book.
    ///
    /// ### Accounts
    /// | #  | Modifier(s)          | Purpose                                                          |
    /// |----|----------------------|------------------------------------------------------------------|
    /// | 0  | `[]`                 | SPL-Token program                                               |
    /// | 1  | `[w]`                | AMM account                                                     |
    /// | 2  | `[]`                 | AMM authority **PDA**                                           |
    /// | 3  | `[w]`                | AMM open-orders                                                 |
    /// | 4  | `[w]`                | *(opt)* AMM target-orders (unused here)                         |
    /// | 5  | `[w]`                | AMM **coin** vault (base)                                       |
    /// | 6  | `[w]`                | AMM **pc**  vault (quote)                                       |
    /// | 7  | `[]`                 | Market program id                                               |
    /// | 8  | `[w]`                | Market account                                                  |
    /// | 9  | `[w]`                | Market bids slab                                                |
    /// | 10 | `[w]`                | Market asks slab                                                |
    /// | 11 | `[w]`                | Market event-queue                                              |
    /// | 12 | `[w]`                | Market **coin** vault (base)                                    |
    /// | 13 | `[w]`                | Market **pc**  vault (quote)                                    |
    /// | 14 | `[]`                 | Market vault-signer **PDA**                                     |
    /// | 15 | `[w]`                | User **source** ATA (base token)                                |
    /// | 16 | `[w]`                | User **destination** ATA (quote token)                          |
    /// | 17 | `[signer, writable]` | User wallet                                                     |
    SwapBaseIn(SwapBaseInInstruction),

    /// *Same account list and behaviour as `SwapBaseIn`, but
    /// the contract ensures an **exact-output / max-input** swap.*
    SwapBaseOut(SwapBaseOutInstruction),

    // ─────────────────────────────────────────────────────────────────────
    // Admin / maintenance
    // ─────────────────────────────────────────────────────────────────────
    /// ### What happens
    /// * Closes the old Serum market, opens a fresh OpenBook market,
    ///   and migrates open-orders.
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose (old → new)                                  |
    /// |----|-------------|-------------------------------------------------------|
    /// | 0  | `[]`        | SPL-Token program                                     |
    /// | 1  | `[]`        | System program                                        |
    /// | 2  | `[]`        | Rent sysvar                                          |
    /// | 3  | `[w]`       | AMM account                                           |
    /// | 4  | `[]`        | AMM authority **PDA**                                 |
    /// | 5  | `[w]`       | AMM open-orders (old)                                 |
    /// | 6  | `[w]`       | AMM coin-vault                                        |
    /// | 7  | `[w]`       | AMM pc-vault                                          |
    /// | 8  | `[w]`       | AMM target-orders                                     |
    /// | 9  | `[]`        | Market program id (old)                               |
    /// | 10 | `[w]`       | Market account (old)                                  |
    /// | 11 | `[w]`       | Market bids  (old)                                    |
    /// | 12 | `[w]`       | Market asks  (old)                                    |
    /// | 13 | `[w]`       | Market event-queue (old)                              |
    /// | 14 | `[w]`       | Market coin-vault (old)                               |
    /// | 15 | `[w]`       | Market pc-vault  (old)                                |
    /// | 16 | `[]`        | Market vault-signer **PDA** (old)                     |
    /// | 17 | `[w]`       | AMM open-orders (new)                                 |
    /// | 18 | `[]`        | Market program id (new)                               |
    /// | 19 | `[]`        | Market account (new)                                  |
    /// | 20 | `[]`        | Admin wallet                                          |
    MigrateToOpenBook,

    /// ### What happens
    /// * Owner tweaks fees, owner address, or replacement open-orders.
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose                                        |
    /// |----|-------------|------------------------------------------------|
    /// | 0  | `[]`        | SPL-Token program                              |
    /// | 1  | `[w]`       | AMM account                                    |
    /// | 2  | `[]`        | AMM authority **PDA**                           |
    /// | 3  | `[w]`       | AMM open-orders                                |
    /// | 4  | `[w]`       | AMM target-orders                              |
    /// | 5  | `[w]`       | AMM coin-vault                                 |
    /// | 6  | `[w]`       | AMM pc-vault                                   |
    /// | 7  | `[]`        | Market program id                              |
    /// | 8  | `[w]`       | Market account                                 |
    /// | 9  | `[w]`       | Market coin-vault                              |
    /// | 10 | `[w]`       | Market pc-vault                                |
    /// | 11 | `[]`        | Market vault-signer **PDA**                    |
    /// | 12 | `[w]`       | Market event-queue                             |
    /// | 13 | `[w]`       | Market bids slab                               |
    /// | 14 | `[w]`       | Market asks slab                               |
    /// | 15 | `[s]`       | Admin wallet                                   |
    /// | 16 | `[]`        | *(opt)* new open-orders                        |
    SetParams(SetParamsInstruction),

    /// ### What happens
    /// * Transfers accrued PnL to the destination accounts.
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose                           |
    /// |----|-------------|-----------------------------------|
    /// | 0  | `[]`        | SPL-Token program                 |
    /// | 1  | `[w]`       | AMM account                       |
    /// | 2  | `[]`        | AMM config **PDA**                |
    /// | 3  | `[]`        | AMM authority **PDA**             |
    /// | 4  | `[w]`       | AMM open-orders                   |
    /// | 5  | `[w]`       | AMM coin-vault                    |
    /// | 6  | `[w]`       | AMM pc-vault                      |
    /// | 7  | `[w]`       | User coin ATA (destination)       |
    /// | 8  | `[w]`       | User pc   ATA (destination)       |
    /// | 9  | `[s]`       | User wallet                       |
    /// | 10 | `[w]`       | AMM target-orders                 |
    /// | 11 | `[]`        | Market program id                 |
    /// | 12 | `[w]`       | Market account                    |
    /// | 13 | `[w]`       | Market event-queue                |
    /// | 14 | `[w]`       | Market coin-vault                 |
    /// | 15 | `[w]`       | Market pc-vault                   |
    /// | 16 | `[]`        | Market vault-signer **PDA**       |
    /// | 17 | `[]`        | *(opt)* referrer pc account       |
    WithdrawPnl,

    /// ### What happens
    /// * Admin pulls SRM / MSRM discount tokens out of the pool.
    ///
    /// ### Accounts
    /// | #  | Modifier(s) | Purpose                                         |
    /// |----|-------------|-------------------------------------------------|
    /// | 0  | `[]`        | SPL-Token program                               |
    /// | 1  | `[]`        | AMM account                                     |
    /// | 2  | `[s]`       | Admin wallet                                    |
    /// | 3  | `[]`        | AMM authority **PDA**                            |
    /// | 4  | `[w]`       | SRM/MSRM source account                         |
    /// | 5  | `[w]`       | SRM/MSRM destination account                    |
    WithdrawSrm(WithdrawSrmInstruction),

    // ─────────────────────────────────────────────────────────────────────
    // Helpers / admin
    // ─────────────────────────────────────────────────────────────────────
    SimulateInfo(SimulateInstruction),
    AdminCancelOrders(AdminCancelOrdersInstruction),
    CreateConfigAccount,
    UpdateConfigAccount(ConfigArgs),

    // ─────────────────────────────────────────────────────────────────────
    // Legacy helper
    // ─────────────────────────────────────────────────────────────────────
    #[deprecated(note = "Only used in the original multi-step pool init")]
    PreInitialize(PreInitializeInstruction),
}

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapBaseInInstruction {
    /// Amount of base token to swap in.
    pub amount_in: u64,

    /// Minimum amount of output token to receive.
    pub minimum_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapBaseOutInstruction {
    /// Maximum amount of base token to swap in.
    pub max_amount_in: u64,

    /// Amount of output token to receive.
    pub amount_out: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InitializeInstruction {
    /// Nonce used to derive the pool-authority PDA
    nonce: u8,
    /// UTC timestamp (seconds) after which the pool is allowed to open
    open_time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Initialize2Instruction {
    /// Nonce used to derive the pool-authority PDA
    nonce: u8,
    /// UTC timestamp (seconds) after which the pool is allowed to open
    open_time: u64,
    /// Quote-token amount that seeds the **pc** vault on creation
    init_pc_amount: u64,
    /// Base-token amount that seeds the **coin** vault on creation
    init_coin_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PreInitializeInstruction {
    /// Nonce used to derive the pool-authority PDA (legacy helper)
    nonce: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MonitorStepInstruction {
    /// Upper bound on the number of *planned* orders to check this crank
    plan_order_limit: u16,
    /// Maximum *new* orders that may be placed in this crank
    place_order_limit: u16,
    /// Maximum open orders that may be *cancelled* in this crank
    cancel_order_limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct DepositInstruction {
    /// Max base-token (**coin**) the user is willing to deposit
    max_coin_amount: u64,
    /// Max quote-token (**pc**) the user is willing to deposit
    max_pc_amount: u64,
    /// Which side acts as “base” when forming the LP ratio (0 = coin, 1 = pc)
    base_side: u64,
    /// Slippage guard: minimum amount of the *other* token that must be deposited
    other_amount_min: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInstruction {
    /// LP-token amount to burn for the withdrawal
    amount: u64,
    /// Min base-token (**coin**) expected; 0 = no limit
    min_coin_amount: Option<u64>,
    /// Min quote-token (**pc**) expected; 0 = no limit
    min_pc_amount: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SetParamsInstruction {
    /// Discriminator selecting which parameter is being updated
    param: u8,
    /// New numeric value (used for simple `u64` params like fees)
    value: Option<u64>,
    /// New public key (used when changing owner / authority)
    new_pubkey: Option<[u8; 32]>,
    /* other Raydium-specific fields (fees, last_order_distance) omitted */
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct WithdrawSrmInstruction {
    /// (M)SRM amount to withdraw from the fee-discount vault
    amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SimulateInstruction {
    /// Discriminator from `SimulateParams` choosing what to simulate
    param: u8,
    /// Payload if simulating a *base-in* swap (absent otherwise)
    swap_base_in_value: Option<SwapBaseInInstruction>,
    /// Payload if simulating a *base-out* swap (absent otherwise)
    swap_base_out_value: Option<SwapBaseOutInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct AdminCancelOrdersInstruction {
    /// Maximum number of orders the admin crank will cancel this call
    limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ConfigArgs {
    /// Field selector (0 = owner, 1 = admin, 2 = create_pool_fee, …)
    param: u8,
    /// New owner/admin pubkey when `param` selects that field
    owner: Option<[u8; 32]>,
    /// New pool-creation fee (lamports) when `param == 2`
    create_pool_fee: Option<u64>,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for RaydiumV4Instruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        // First byte is Raydium’s log_type discriminator
        let discriminator = data[0];
        let payload = &data[1..]; // exclude the discriminator

        Ok(match discriminator {
            INITIALIZE => Self::Initialize(InitializeInstruction::try_from_slice(payload)?),
            INITIALIZE2 => Self::Initialize2(Initialize2Instruction::try_from_slice(payload)?),
            MONITOR_STEP => Self::MonitorStep(MonitorStepInstruction::try_from_slice(payload)?),
            DEPOSIT => Self::Deposit(DepositInstruction::try_from_slice(payload)?),
            WITHDRAW => Self::Withdraw(WithdrawInstruction::try_from_slice(payload)?),
            MIGRATE_TO_OPENBOOK => Self::MigrateToOpenBook,
            SET_PARAMS => Self::SetParams(SetParamsInstruction::try_from_slice(payload)?),
            WITHDRAW_PNL => Self::WithdrawPnl,
            WITHDRAW_SRM => Self::WithdrawSrm(WithdrawSrmInstruction::try_from_slice(payload)?),
            SWAP_BASE_IN => Self::SwapBaseIn(SwapBaseInInstruction::try_from_slice(&payload[..SWAP_LEN])?),
            PRE_INITIALIZE => Self::PreInitialize(PreInitializeInstruction::try_from_slice(payload)?),
            SWAP_BASE_OUT => Self::SwapBaseOut(SwapBaseOutInstruction::try_from_slice(&payload[..SWAP_LEN])?),
            SIMULATE_INFO => Self::SimulateInfo(SimulateInstruction::try_from_slice(payload)?),
            ADMIN_CANCEL_ORDERS => Self::AdminCancelOrders(AdminCancelOrdersInstruction::try_from_slice(payload)?),
            CREATE_CONFIG_ACCOUNT => Self::CreateConfigAccount,
            UPDATE_CONFIG_ACCOUNT => Self::UpdateConfigAccount(ConfigArgs::try_from_slice(payload)?),
            other => return Err(ParseError::RaydiumUnknown(other)),
        })
    }
}

// Convenience function retaining the old name; forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<RaydiumV4Instruction, ParseError> {
    RaydiumV4Instruction::try_from(data)
}
