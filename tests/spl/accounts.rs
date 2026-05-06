use solana_program::pubkey::Pubkey;
use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction, ConfirmedTransaction, Message, MessageHeader, Transaction, TransactionStatusMeta};

const PROGRAM: [u8; 32] = [0xfe; 32];
const SWAP_ACCOUNT: [u8; 32] = [1; 32];
const AUTHORITY: [u8; 32] = [2; 32];
const USER_TRANSFER_AUTHORITY: [u8; 32] = [3; 32];
const SOURCE: [u8; 32] = [4; 32];
const SWAP_SOURCE: [u8; 32] = [5; 32];
const SWAP_DESTINATION: [u8; 32] = [6; 32];
const DESTINATION: [u8; 32] = [7; 32];
const POOL_MINT: [u8; 32] = [8; 32];
const FEE_ACCOUNT: [u8; 32] = [9; 32];
const HOST_FEE_ACCOUNT: [u8; 32] = [10; 32];

fn pubkey(bytes: [u8; 32]) -> Pubkey {
    Pubkey::new_from_array(bytes)
}

fn make_tx(accounts: &[[u8; 32]]) -> ConfirmedTransaction {
    let mut keys: Vec<Vec<u8>> = vec![[0xff; 32].to_vec(), PROGRAM.to_vec()];
    let mut instruction_accounts = Vec::new();

    for (index, account) in accounts.iter().enumerate() {
        keys.push(account.to_vec());
        instruction_accounts.push((index + 2) as u8);
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
                    accounts: instruction_accounts,
                    data: vec![],
                }],
                versioned: false,
                address_table_lookups: vec![],
            }),
        }),
        meta: Some(TransactionStatusMeta::default()),
    }
}

#[test]
fn token_swap_get_swap_accounts_base_form() {
    let tx = make_tx(&[
        SWAP_ACCOUNT,
        AUTHORITY,
        USER_TRANSFER_AUTHORITY,
        SOURCE,
        SWAP_SOURCE,
        SWAP_DESTINATION,
        DESTINATION,
        POOL_MINT,
        FEE_ACCOUNT,
    ]);
    let ix = tx.walk_instructions().next().unwrap();

    let accounts = substreams_solana_idls::spl::token_swap::accounts::get_swap_accounts(&ix).unwrap();

    assert_eq!(accounts.swap_account, pubkey(SWAP_ACCOUNT));
    assert_eq!(accounts.authority, pubkey(AUTHORITY));
    assert_eq!(accounts.user_transfer_authority, pubkey(USER_TRANSFER_AUTHORITY));
    assert_eq!(accounts.source, pubkey(SOURCE));
    assert_eq!(accounts.swap_source, pubkey(SWAP_SOURCE));
    assert_eq!(accounts.swap_destination, pubkey(SWAP_DESTINATION));
    assert_eq!(accounts.destination, pubkey(DESTINATION));
    assert_eq!(accounts.pool_mint, pubkey(POOL_MINT));
    assert_eq!(accounts.fee_account, pubkey(FEE_ACCOUNT));
    assert_eq!(accounts.host_fee_account, None);
}

#[test]
fn token_swap_get_swap_accounts_with_host_fee_account() {
    let tx = make_tx(&[
        SWAP_ACCOUNT,
        AUTHORITY,
        USER_TRANSFER_AUTHORITY,
        SOURCE,
        SWAP_SOURCE,
        SWAP_DESTINATION,
        DESTINATION,
        POOL_MINT,
        FEE_ACCOUNT,
        HOST_FEE_ACCOUNT,
    ]);
    let ix = tx.walk_instructions().next().unwrap();

    let accounts = substreams_solana_idls::spl::token_swap::accounts::get_swap_accounts(&ix).unwrap();

    assert_eq!(accounts.host_fee_account, Some(pubkey(HOST_FEE_ACCOUNT)));
}

#[test]
fn token_swap_get_swap_accounts_reports_missing_required_account() {
    let tx = make_tx(&[
        SWAP_ACCOUNT,
        AUTHORITY,
        USER_TRANSFER_AUTHORITY,
        SOURCE,
        SWAP_SOURCE,
        SWAP_DESTINATION,
        DESTINATION,
        POOL_MINT,
    ]);
    let ix = tx.walk_instructions().next().unwrap();

    let err = substreams_solana_idls::spl::token_swap::accounts::get_swap_accounts(&ix).unwrap_err();

    assert!(matches!(
        err,
        substreams_solana_idls::common::accounts::AccountsError::Missing { name: "fee_account", index: 8 }
    ));
}
