use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;
use substreams_solana::pb::sf::solana::r#type::v1::{
    CompiledInstruction, ConfirmedTransaction, Message, MessageHeader, Transaction, TransactionStatusMeta,
};
use substreams_solana_idls::common::accounts::AccountsError;
use substreams_solana_idls::pumpswap::accounts::*;

#[test]
fn test_pool_account() {
    let pool = Pool {
        pool_bump: 254,
        index: 7,
        creator: Pubkey::new_unique(),
        base_mint: Pubkey::new_unique(),
        quote_mint: Pubkey::new_unique(),
        lp_mint: Pubkey::new_unique(),
        pool_base_token_account: Pubkey::new_unique(),
        pool_quote_token_account: Pubkey::new_unique(),
        lp_supply: 1_000_000,
        coin_creator: Pubkey::new_unique(),
        is_mayhem_mode: true,
    };
    let mut data = POOL_DISC.to_vec();
    data.extend_from_slice(&borsh::to_vec(&pool).unwrap());
    match unpack_account(&data).unwrap() {
        PumpSwapAccount::Pool(parsed) => {
            assert_eq!(parsed.pool_bump, 254);
            assert_eq!(parsed.index, 7);
            assert_eq!(parsed.lp_supply, 1_000_000);
            assert!(parsed.is_mayhem_mode);
        }
        other => panic!("expected Pool, got {:?}", other),
    }
}

#[test]
fn test_bonding_curve_account() {
    let bc = BondingCurve {
        virtual_token_reserves: 1_000_000_000,
        virtual_sol_reserves: 30_000_000_000,
        real_token_reserves: 500_000_000,
        real_sol_reserves: 15_000_000_000,
        token_total_supply: 1_000_000_000_000,
        complete: false,
        creator: Pubkey::new_unique(),
    };
    let mut data = BONDING_CURVE_DISC.to_vec();
    data.extend_from_slice(&borsh::to_vec(&bc).unwrap());
    match unpack_account(&data).unwrap() {
        PumpSwapAccount::BondingCurve(parsed) => {
            assert_eq!(parsed.virtual_token_reserves, 1_000_000_000);
            assert!(!parsed.complete);
        }
        other => panic!("expected BondingCurve, got {:?}", other),
    }
}

#[test]
fn test_unknown_account() {
    let data = [0u8; 16];
    assert!(unpack_account(&data).is_err());
}

// -----------------------------------------------------------------------------
// Trade-instruction account resolution (Buy / Sell / BuyExactQuoteIn)
// -----------------------------------------------------------------------------
//
// These tests pin the account layout to the canonical `pumpswap/idl.json`
// ordering (`pool, user, global_config, base_mint, quote_mint, …`). They
// catch any future re-ordering and also document the bug that motivated this
// helper: the previous `dex-swaps/src/pumpfun_amm.rs` decoder hand-indexed
// `accounts.get(4)` / `accounts.get(5)` for `base_mint` / `quote_mint`, which
// is off by one (those positions are actually `quote_mint` and
// `user_base_token_account`). Production data showed ~165 fake `(mint0,
// mint1)` pairs per pool because the user-specific token account was leaking
// into the `mint` slot.

const PROGRAM: [u8; 32] = [0xfe; 32];
const POOL: [u8; 32] = [0x01; 32];
const USER: [u8; 32] = [0x02; 32];
const GLOBAL_CONFIG: [u8; 32] = [0x03; 32];
const BASE_MINT: [u8; 32] = [0x04; 32];
const QUOTE_MINT: [u8; 32] = [0x05; 32];
const USER_BASE_TA: [u8; 32] = [0x06; 32];
const USER_QUOTE_TA: [u8; 32] = [0x07; 32];
const POOL_BASE_TA: [u8; 32] = [0x08; 32];
const POOL_QUOTE_TA: [u8; 32] = [0x09; 32];
const PROTOCOL_FEE_RECIPIENT: [u8; 32] = [0x0a; 32];
const PROTOCOL_FEE_RECIPIENT_TA: [u8; 32] = [0x0b; 32];
// Trailing required-by-IDL accounts that we don't surface in TradeAccounts but
// still need to populate so the V2 `coin_creator_vault_*` end up at indices
// 17/18.
const BASE_TOKEN_PROGRAM: [u8; 32] = [0x0c; 32];
const QUOTE_TOKEN_PROGRAM: [u8; 32] = [0x0d; 32];
const SYSTEM_PROGRAM: [u8; 32] = [0x0e; 32];
const ASSOCIATED_TOKEN_PROGRAM: [u8; 32] = [0x0f; 32];
const EVENT_AUTHORITY: [u8; 32] = [0x10; 32];
const PUMP_PROGRAM: [u8; 32] = [0x11; 32];
const COIN_CREATOR_VAULT_ATA: [u8; 32] = [0x12; 32];
const COIN_CREATOR_VAULT_AUTHORITY: [u8; 32] = [0x13; 32];

fn pubkey(bytes: [u8; 32]) -> Pubkey {
    Pubkey::new_from_array(bytes)
}

fn make_tx(accounts: &[[u8; 32]]) -> ConfirmedTransaction {
    let mut keys: Vec<Vec<u8>> = vec![[0xff; 32].to_vec(), PROGRAM.to_vec()];
    let mut ix_accounts = Vec::new();
    for (i, a) in accounts.iter().enumerate() {
        keys.push(a.to_vec());
        ix_accounts.push((i + 2) as u8);
    }
    ConfirmedTransaction {
        transaction: Some(Transaction {
            signatures: vec![vec![0; 64]],
            message: Some(Message {
                header: Some(MessageHeader {
                    num_required_signatures: 1,
                    num_readonly_signed_accounts: 0,
                    num_readonly_unsigned_accounts: 0,
                }),
                account_keys: keys,
                recent_blockhash: vec![0; 32],
                instructions: vec![CompiledInstruction {
                    program_id_index: 1,
                    accounts: ix_accounts,
                    data: vec![],
                }],
                versioned: false,
                address_table_lookups: vec![],
            }),
        }),
        meta: Some(TransactionStatusMeta::default()),
    }
}

/// Full required+optional account list for a V2 buy/sell. Indices 0..=16 are
/// the IDL-required accounts; 17/18 are the V2 trailing optional accounts we
/// expose in `TradeAccounts`.
fn full_v2_accounts() -> [[u8; 32]; 19] {
    [
        POOL,
        USER,
        GLOBAL_CONFIG,
        BASE_MINT,
        QUOTE_MINT,
        USER_BASE_TA,
        USER_QUOTE_TA,
        POOL_BASE_TA,
        POOL_QUOTE_TA,
        PROTOCOL_FEE_RECIPIENT,
        PROTOCOL_FEE_RECIPIENT_TA,
        BASE_TOKEN_PROGRAM,
        QUOTE_TOKEN_PROGRAM,
        SYSTEM_PROGRAM,
        ASSOCIATED_TOKEN_PROGRAM,
        EVENT_AUTHORITY,
        PUMP_PROGRAM,
        COIN_CREATOR_VAULT_ATA,
        COIN_CREATOR_VAULT_AUTHORITY,
    ]
}

#[test]
fn buy_accounts_resolve_to_idl_layout() {
    let tx = make_tx(&full_v2_accounts());
    let ix = tx.walk_instructions().next().unwrap();
    let accounts = get_buy_accounts(&ix).unwrap();

    assert_eq!(accounts.pool, pubkey(POOL));
    assert_eq!(accounts.user, pubkey(USER));
    assert_eq!(accounts.global_config, pubkey(GLOBAL_CONFIG));
    // Critical: base_mint is index 3 and quote_mint is index 4 — *not* 4 / 5
    // (the off-by-one that produced the production bug).
    assert_eq!(accounts.base_mint, pubkey(BASE_MINT));
    assert_eq!(accounts.quote_mint, pubkey(QUOTE_MINT));
    assert_ne!(
        accounts.base_mint,
        pubkey(USER_BASE_TA),
        "regression: base_mint must not read accounts[5] (user_base_token_account)"
    );
    assert_ne!(
        accounts.quote_mint,
        pubkey(USER_BASE_TA),
        "regression: quote_mint must not read accounts[5] (user_base_token_account)"
    );
    assert_eq!(accounts.user_base_token_account, pubkey(USER_BASE_TA));
    assert_eq!(accounts.user_quote_token_account, pubkey(USER_QUOTE_TA));
    assert_eq!(accounts.pool_base_token_account, pubkey(POOL_BASE_TA));
    assert_eq!(accounts.pool_quote_token_account, pubkey(POOL_QUOTE_TA));
    assert_eq!(accounts.protocol_fee_recipient, pubkey(PROTOCOL_FEE_RECIPIENT));
    assert_eq!(accounts.protocol_fee_recipient_token_account, pubkey(PROTOCOL_FEE_RECIPIENT_TA));
    assert_eq!(accounts.coin_creator_vault_ata, Some(pubkey(COIN_CREATOR_VAULT_ATA)));
    assert_eq!(
        accounts.coin_creator_vault_authority,
        Some(pubkey(COIN_CREATOR_VAULT_AUTHORITY))
    );
}

#[test]
fn sell_accounts_share_buy_layout() {
    // Sell instruction has the same first 17 accounts as Buy.
    let tx = make_tx(&full_v2_accounts());
    let ix = tx.walk_instructions().next().unwrap();
    let accounts = get_sell_accounts(&ix).unwrap();

    assert_eq!(accounts.pool, pubkey(POOL));
    assert_eq!(accounts.base_mint, pubkey(BASE_MINT));
    assert_eq!(accounts.quote_mint, pubkey(QUOTE_MINT));
    assert_eq!(accounts.coin_creator_vault_ata, Some(pubkey(COIN_CREATOR_VAULT_ATA)));
}

#[test]
fn buy_exact_quote_in_accounts_share_buy_layout() {
    let tx = make_tx(&full_v2_accounts());
    let ix = tx.walk_instructions().next().unwrap();
    let accounts = get_buy_exact_quote_in_accounts(&ix).unwrap();

    assert_eq!(accounts.pool, pubkey(POOL));
    assert_eq!(accounts.base_mint, pubkey(BASE_MINT));
    assert_eq!(accounts.quote_mint, pubkey(QUOTE_MINT));
}

#[test]
fn trade_accounts_v1_omits_coin_creator_fields() {
    // V1 buy instructions stop at index 16 (program). The V2 coin_creator_*
    // accounts are absent; TradeAccounts must surface them as None instead of
    // erroring.
    let v2 = full_v2_accounts();
    let tx = make_tx(&v2[..17]);
    let ix = tx.walk_instructions().next().unwrap();
    let accounts = get_buy_accounts(&ix).unwrap();

    assert_eq!(accounts.coin_creator_vault_ata, None);
    assert_eq!(accounts.coin_creator_vault_authority, None);
    assert_eq!(accounts.base_mint, pubkey(BASE_MINT));
}

#[test]
fn trade_accounts_reports_missing_required_account() {
    // Truncate to 9 accounts so `protocol_fee_recipient` (index 9) is missing.
    let v2 = full_v2_accounts();
    let tx = make_tx(&v2[..9]);
    let ix = tx.walk_instructions().next().unwrap();

    let err = get_buy_accounts(&ix).unwrap_err();
    assert!(matches!(
        err,
        AccountsError::Missing {
            name: "protocol_fee_recipient",
            index: 9
        }
    ));
}
