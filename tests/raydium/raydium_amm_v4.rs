#[cfg(test)]
mod tests {
    use base64::Engine;
    use substreams::hex;
    use substreams_solana_idls::common::ParseError;
    #[test]
    fn unpack_amm_v4_swap_event() {
        // https://solscan.io/tx/57d3uDBdPyrHX44aPzWVznDn39qx3ixFdRVieEfvhKtYErtgbYUpetApiZaYDSHCsfQWmqJryjknyFYT2U21oqrU
        let base64 = "AwBOclMAAAAAmOUFnQ4AAAACAAAAAAAAAABOclMAAAAAfTDHRyEBAAAR7JIChTUAAIVC62EPAAAA";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");

        match substreams_solana_idls::raydium::amm::v4::logs::unpack(&bytes).expect("decode event") {
            substreams_solana_idls::raydium::amm::v4::logs::RaydiumV4Log::SwapBaseIn(event) => {
                assert_eq!(event.log_type, 3, "log_type");
                assert_eq!(event.amount_in, 1400000000, "amount_in");
                assert_eq!(event.minimum_out, 62763951512, "minimum_out");
                assert_eq!(event.direction, 2, "direction");
                assert_eq!(event.user_source, 1400000000, "user_source");
                assert_eq!(event.pool_coin, 1242449784957, "pool_coin");
                assert_eq!(event.pool_pc, 58845390105617, "pool_pc");
                assert_eq!(event.out_amount, 66067317381, "out_amount");
            }
            _ => panic!("Expected a Event"),
        }
    }

    #[test]
    fn unpack_amm_v4_swap_event_2() {
        // https://solscan.io/tx/3GTW94fNv4JF4LeWNXCd6mQhxcZjNic9D1UoY9Qenf2S2t7gcgj7z58KwmHMHAYPqKVe812rrnpf2SKw8MdbNW7m
        let base64 = "AwL/AQAAAAAAAAAAAAAAAAACAAAAAAAAAAL/AQAAAAAA4VSaQlgAAADuOE6EHwAAAAS2AAAAAAAA";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");

        match substreams_solana_idls::raydium::amm::v4::logs::unpack(&bytes).expect("decode event") {
            substreams_solana_idls::raydium::amm::v4::logs::RaydiumV4Log::SwapBaseIn(event) => {
                assert_eq!(event.log_type, 3, "log_type");
                assert_eq!(event.amount_in, 130818, "amount_in");
                assert_eq!(event.minimum_out, 0, "minimum_out");
                assert_eq!(event.direction, 2, "direction");
                assert_eq!(event.user_source, 130818, "user_source");
                assert_eq!(event.pool_coin, 379074532577, "pool_coin");
                assert_eq!(event.pool_pc, 135363705070, "pool_pc");
                assert_eq!(event.out_amount, 46596, "out_amount");
            }
            _ => panic!("Expected a Event"),
        }
    }

    #[test]
    fn unpack_amm_v4_empty_event_reports_actual_length() {
        assert!(matches!(
            substreams_solana_idls::raydium::amm::v4::logs::unpack(&[]),
            Err(ParseError::TooShort(0))
        ));
    }

    #[test]
    fn unpack_amm_v4_swap_instruction() {
        // https://solscan.io/tx/2mCvqGCwJszrr3BWB1Jen6gtEpS1xMoakxD4KRAAqFka76WJhQDCMFRZX7VqKGPNqfGXUzvyrLDENoqizBcBGCWN
        match substreams_solana_idls::raydium::amm::v4::instructions::unpack(&hex!("0926ed3c0000000000c637450000000000")).expect("decode event") {
            substreams_solana_idls::raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event) => {
                assert_eq!(event.amount_in, 3992870, "amount_in");
                assert_eq!(event.minimum_amount_out, 4536262, "minimum_amount_out");
            }
            _ => panic!("Expected an Instruction"),
        }
    }

    #[test]
    fn unpack_amm_v4_swap_instruction_2() {
        // https://solscan.io/tx/3GTW94fNv4JF4LeWNXCd6mQhxcZjNic9D1UoY9Qenf2S2t7gcgj7z58KwmHMHAYPqKVe812rrnpf2SKw8MdbNW7m
        match substreams_solana_idls::raydium::amm::v4::instructions::unpack(&hex!("0902ff010000000000000000000000000040")).expect("decode event") {
            substreams_solana_idls::raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event) => {
                assert_eq!(event.amount_in, 130818, "amount_in");
                assert_eq!(event.minimum_amount_out, 0, "minimum_amount_out");
            }
            _ => panic!("Expected an Instruction"),
        }
    }

    #[test]
    fn unpack_amm_v4_swap_base_in_instruction() {
        // https://solscan.io/tx/4NHdxiefrFEDazhdrcTjmG73G3TewYjhexKvpnNHgnzG5NK53A5otzHcE3xrzohGVAjvxQnYL3ysLD8xH89dVr3
        match substreams_solana_idls::raydium::amm::v4::instructions::unpack(&hex!("092ec29212000000000000000000000000")).expect("decode event") {
            substreams_solana_idls::raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event) => {
                assert_eq!(event.amount_in, 311607854, "amount_in");
                assert_eq!(event.minimum_amount_out, u64::MIN, "minimum_amount_out");
            }
            _ => panic!("Expected an Instruction"),
        }
    }
    #[test]
    fn unpack_amm_v4_swap_base_out_instruction() {
        // https://solscan.io/tx/3DHK2CAoTWeBoyM4Yzfsd5zSAPnN1kPXpbDp42YXKh2gE5K4Un28xu5qEeCpq7f85hUC3cCVkivhCARffSLARXZf
        match substreams_solana_idls::raydium::amm::v4::instructions::unpack(&hex!("0bffffffffffffffff0b6f14a60c010000")).expect("decode event") {
            substreams_solana_idls::raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseOut(event) => {
                assert_eq!(event.max_amount_in, u64::MAX, "max_amount_in");
                assert_eq!(event.amount_out, 1153837592331, "amount_out");
            }
            _ => panic!("Expected a SwapBaseOut Instruction 1"),
        }
    }

    // -------------------------------------------------------------------------
    // SwapBaseAccounts — typed account-list resolver.
    //
    // Locks down the account layout against `idl.json` and exercises both the
    // current 18-account form (with `amm_target_orders`) and the legacy
    // 17-account form (without). The `pool_coin_token_account` /
    // `pool_pc_token_account` slots are the most consequential to get right —
    // downstream `dex-swaps` adapters resolve those via `TokenMintLookup` to
    // produce the swap's `(input_mint, output_mint)`.
    // -------------------------------------------------------------------------
    use substreams_solana::pb::sf::solana::r#type::v1::{
        CompiledInstruction, ConfirmedTransaction, Message, MessageHeader, Transaction, TransactionStatusMeta,
    };
    use substreams_solana_idls::common::accounts::AccountsError;
    use substreams_solana_idls::raydium::amm::v4::accounts::{get_swap_base_in_accounts, SwapBaseAccounts};

    fn pubkey(bytes: [u8; 32]) -> solana_program::pubkey::Pubkey {
        solana_program::pubkey::Pubkey::new_from_array(bytes)
    }

    fn make_tx(accounts: &[[u8; 32]]) -> ConfirmedTransaction {
        let fee_payer = [0xfe; 32];
        let program = [0xfd; 32];
        let mut keys: Vec<Vec<u8>> = vec![fee_payer.to_vec(), program.to_vec()];
        let mut acc_idx = Vec::new();
        for (i, a) in accounts.iter().enumerate() {
            keys.push(a.to_vec());
            acc_idx.push((i + 2) as u8);
        }
        ConfirmedTransaction {
            transaction: Some(Transaction {
                signatures: vec![vec![0u8; 64]],
                message: Some(Message {
                    header: Some(MessageHeader {
                        num_required_signatures: 1,
                        num_readonly_signed_accounts: 0,
                        num_readonly_unsigned_accounts: 0,
                    }),
                    account_keys: keys,
                    recent_blockhash: vec![0u8; 32],
                    instructions: vec![CompiledInstruction {
                        program_id_index: 1,
                        accounts: acc_idx,
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
    fn swap_base_accounts_post_fork_18_accounts() {
        let token_program = [0x00; 32];
        let amm = [0x01; 32];
        let amm_authority = [0x02; 32];
        let amm_open_orders = [0x03; 32];
        let amm_target_orders = [0x04; 32];
        let pool_coin = [0x05; 32];
        let pool_pc = [0x06; 32];
        let serum_program = [0x07; 32];
        let serum_market = [0x08; 32];
        let serum_bids = [0x09; 32];
        let serum_asks = [0x0a; 32];
        let serum_event_queue = [0x0b; 32];
        let serum_coin_vault = [0x0c; 32];
        let serum_pc_vault = [0x0d; 32];
        let serum_vault_signer = [0x0e; 32];
        let user_src = [0x0f; 32];
        let user_dst = [0x10; 32];
        let user_owner = [0x11; 32];

        let tx = make_tx(&[
            token_program,
            amm,
            amm_authority,
            amm_open_orders,
            amm_target_orders,
            pool_coin,
            pool_pc,
            serum_program,
            serum_market,
            serum_bids,
            serum_asks,
            serum_event_queue,
            serum_coin_vault,
            serum_pc_vault,
            serum_vault_signer,
            user_src,
            user_dst,
            user_owner,
        ]);
        let ix = tx.walk_instructions().next().unwrap();
        let a: SwapBaseAccounts = (&ix).try_into().unwrap();

        assert_eq!(a.amm, pubkey(amm));
        assert_eq!(a.amm_target_orders, Some(pubkey(amm_target_orders)));
        assert_eq!(a.pool_coin_token_account, pubkey(pool_coin));
        assert_eq!(a.pool_pc_token_account, pubkey(pool_pc));
        assert_eq!(a.uer_source_token_account, pubkey(user_src));
        assert_eq!(a.uer_destination_token_account, pubkey(user_dst));
        assert_eq!(a.user_source_owner, pubkey(user_owner));

        // Regression: pool vaults must NOT come from a shifted index.
        assert_ne!(a.pool_coin_token_account, pubkey(amm_target_orders));
    }

    #[test]
    fn swap_base_accounts_legacy_17_accounts() {
        // Legacy pre-fork pools omit `amm_target_orders`. Every index from
        // `pool_coin_token_account` onward shifts down by 1.
        let token_program = [0x00; 32];
        let amm = [0x01; 32];
        let amm_authority = [0x02; 32];
        let amm_open_orders = [0x03; 32];
        let pool_coin = [0x05; 32];
        let pool_pc = [0x06; 32];
        let serum_program = [0x07; 32];
        let serum_market = [0x08; 32];
        let serum_bids = [0x09; 32];
        let serum_asks = [0x0a; 32];
        let serum_event_queue = [0x0b; 32];
        let serum_coin_vault = [0x0c; 32];
        let serum_pc_vault = [0x0d; 32];
        let serum_vault_signer = [0x0e; 32];
        let user_src = [0x0f; 32];
        let user_dst = [0x10; 32];
        let user_owner = [0x11; 32];

        let tx = make_tx(&[
            token_program,
            amm,
            amm_authority,
            amm_open_orders,
            pool_coin,
            pool_pc,
            serum_program,
            serum_market,
            serum_bids,
            serum_asks,
            serum_event_queue,
            serum_coin_vault,
            serum_pc_vault,
            serum_vault_signer,
            user_src,
            user_dst,
            user_owner,
        ]);
        let ix = tx.walk_instructions().next().unwrap();
        let a = get_swap_base_in_accounts(&ix).expect("legacy 17-account form must resolve");

        assert_eq!(a.amm, pubkey(amm));
        assert_eq!(a.amm_target_orders, None);
        // pool_coin/pc end up at indices 4/5 in the legacy form, but the
        // typed helper still names them correctly.
        assert_eq!(a.pool_coin_token_account, pubkey(pool_coin));
        assert_eq!(a.pool_pc_token_account, pubkey(pool_pc));
        assert_eq!(a.user_source_owner, pubkey(user_owner));
    }

    #[test]
    fn swap_base_accounts_reports_missing_required() {
        // 16 accounts — short of even the legacy 17-account layout.
        let accounts: Vec<[u8; 32]> = (0u8..16u8).map(|i| [i + 1; 32]).collect();
        let tx = make_tx(&accounts);
        let ix = tx.walk_instructions().next().unwrap();

        let err = get_swap_base_in_accounts(&ix).expect_err("must error on too-few accounts");
        assert!(matches!(err, AccountsError::Missing { .. }));
    }

    // -------------------------------------------------------------------------
    // SwapV2Accounts — orderbook-disabled `SwapBaseInV2` / `SwapBaseOutV2`.
    //
    // V2 ships an 8-account layout in this fixed order: token_program, amm,
    // amm_authority, pool_coin_token_account, pool_pc_token_account,
    // uer_source_token_account, uer_destination_token_account,
    // user_source_owner. These tests pin the layout against the canonical
    // Raydium AMM v4 source.
    // -------------------------------------------------------------------------
    use substreams_solana_idls::raydium::amm::v4::accounts::{get_swap_base_in_v2_accounts, get_swap_base_out_v2_accounts, SwapV2Accounts};

    #[test]
    fn swap_v2_accounts_resolve_8_account_layout() {
        let token_program = [0x00; 32];
        let amm = [0x01; 32];
        let amm_authority = [0x02; 32];
        let pool_coin = [0x03; 32];
        let pool_pc = [0x04; 32];
        let user_src = [0x05; 32];
        let user_dst = [0x06; 32];
        let user_owner = [0x07; 32];

        let tx = make_tx(&[token_program, amm, amm_authority, pool_coin, pool_pc, user_src, user_dst, user_owner]);
        let ix = tx.walk_instructions().next().unwrap();
        let a = get_swap_base_in_v2_accounts(&ix).expect("V2 8-account layout must resolve");

        assert_eq!(a.token_program, pubkey(token_program));
        assert_eq!(a.amm, pubkey(amm));
        assert_eq!(a.amm_authority, pubkey(amm_authority));
        assert_eq!(a.pool_coin_token_account, pubkey(pool_coin));
        assert_eq!(a.pool_pc_token_account, pubkey(pool_pc));
        assert_eq!(a.uer_source_token_account, pubkey(user_src));
        assert_eq!(a.uer_destination_token_account, pubkey(user_dst));
        assert_eq!(a.user_source_owner, pubkey(user_owner));

        // Regression: pool vault slots must not pick up amm/authority indices.
        assert_ne!(a.pool_coin_token_account, pubkey(amm));
        assert_ne!(a.pool_coin_token_account, pubkey(amm_authority));
    }

    #[test]
    fn swap_v2_accounts_get_base_out_uses_same_layout() {
        // `get_swap_base_out_v2_accounts` shares the same struct/layout —
        // this test pins the parity so a future divergence in V2 base-out
        // surfaces here.
        let accounts: Vec<[u8; 32]> = (0u8..8u8).map(|i| [i + 0x10; 32]).collect();
        let tx = make_tx(&accounts);
        let ix = tx.walk_instructions().next().unwrap();
        let a: SwapV2Accounts = get_swap_base_out_v2_accounts(&ix).expect("V2 base-out must resolve");
        assert_eq!(a.amm, pubkey(accounts[1]));
        assert_eq!(a.pool_coin_token_account, pubkey(accounts[3]));
        assert_eq!(a.pool_pc_token_account, pubkey(accounts[4]));
        assert_eq!(a.user_source_owner, pubkey(accounts[7]));
    }

    #[test]
    fn swap_v2_accounts_reports_missing_required() {
        // Only 5 accounts — short of the 8-account V2 layout.
        let accounts: Vec<[u8; 32]> = (0u8..5u8).map(|i| [i + 1; 32]).collect();
        let tx = make_tx(&accounts);
        let ix = tx.walk_instructions().next().unwrap();
        let err = get_swap_base_in_v2_accounts(&ix).expect_err("must error on too-few accounts");
        assert!(matches!(err, AccountsError::Missing { .. }));
    }
}
