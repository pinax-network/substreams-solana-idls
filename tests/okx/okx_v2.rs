#[cfg(test)]
mod tests {
    use base64::Engine;
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;
    use substreams::hex;
    use substreams_solana::pb::sf::solana::r#type::v1 as pb;
    use substreams_solana_idls::okx::v2::{
        accounts,
        events::{self, OkxV2Event, SwapEvent, SWAP_EVENT},
        instructions::DEX_VARIANT_NAMES,
        instructions::{self, Dex, Route, SwapArgs, SwapV3Instruction},
        logs,
    };

    #[test]
    fn swap_v3_instruction() {
        let bytes = hex!("f0e02621b01ff1af78e8692600000000add6d4007a00000063cd6f09730000000100000078e86926000000000100000003000000010000000501000000640200000012120200000050140100000049010000006420b3818010270000000000000000");
        match instructions::unpack(&bytes).expect("decode instruction") {
            instructions::OkxV2Instruction::SwapV3(ix) => {
                assert_eq!(
                    ix,
                    SwapV3Instruction {
                        args: SwapArgs {
                            amount_in: 644_475_000,
                            expect_amount_out: 523_999_958_701,
                            min_return: 494_079_561_059,
                            amounts: vec![644_475_000],
                            routes: vec![vec![
                                Route {
                                    dexes: vec![Dex::RaydiumStableSwap],
                                    weights: vec![100],
                                },
                                Route {
                                    dexes: vec![Dex::ObricV2, Dex::ObricV2],
                                    weights: vec![80, 20],
                                },
                                Route {
                                    dexes: vec![Dex::PumpfunammBuy3],
                                    weights: vec![100],
                                },
                            ]],
                        },
                        commission_info: 2_155_983_648,
                        platform_fee_rate: 10_000,
                        order_id: 0,
                    }
                );
            }
            _ => panic!("expected SwapV3"),
        }
    }

    #[test]
    fn swap_v3_tessera_instruction() {
        let bytes = hex!("f0e02621b01ff1afbf775f3b05000000db7706a03a000000db7706a03a00000001000000bf775f3b050000000100000001000000010000003f0100000064000000000000000000000000000000000000");
        match instructions::unpack(&bytes).expect("decode instruction") {
            instructions::OkxV2Instruction::SwapV3(ix) => {
                assert_eq!(
                    ix,
                    SwapV3Instruction {
                        args: SwapArgs {
                            amount_in: 22_470_948_799,
                            expect_amount_out: 251_792_881_627,
                            min_return: 251_792_881_627,
                            amounts: vec![22_470_948_799],
                            routes: vec![vec![Route {
                                dexes: vec![Dex::Tessera],
                                weights: vec![100],
                            }]],
                        },
                        commission_info: 0,
                        platform_fee_rate: 0,
                        order_id: 0,
                    }
                );
            }
            _ => panic!("expected SwapV3"),
        }
    }

    #[test]
    fn swap_v3_sanctum_alphaq_instruction() {
        let bytes = hex!("f0e02621b01ff1af7cc51c7f010000003bcc0fe901000000bfcc2be401000000010000007cc51c7f01000000010000000200000001000000150100000064010000005301000000640000000000000000000000000000");
        match instructions::unpack(&bytes).expect("decode instruction") {
            instructions::OkxV2Instruction::SwapV3(ix) => {
                assert_eq!(
                    ix,
                    SwapV3Instruction {
                        args: SwapArgs {
                            amount_in: 6_427_559_292,
                            expect_amount_out: 8_205_093_947,
                            min_return: 8_123_043_007,
                            amounts: vec![6_427_559_292],
                            routes: vec![vec![
                                Route {
                                    dexes: vec![Dex::SanctumNonWsolSwap],
                                    weights: vec![100],
                                },
                                Route {
                                    dexes: vec![Dex::AlphaQ],
                                    weights: vec![100],
                                },
                            ]],
                        },
                        commission_info: 0,
                        platform_fee_rate: 0,
                        order_id: 0,
                    }
                );
            }
            _ => panic!("expected SwapV3"),
        }
    }

    #[test]
    fn dex_variant_names_are_parseable() {
        assert_eq!(DEX_VARIANT_NAMES.len(), 110);

        for name in DEX_VARIANT_NAMES {
            Dex::from_str(name).unwrap_or_else(|_| panic!("missing Dex parser for {name}"));
        }
    }

    #[test]
    fn swap_event() {
        let event = SwapEvent {
            dex: Dex::RaydiumStableSwap,
            amount_in: 644_475_000,
            amount_out: 523_999_958_701,
        };
        let mut data = SWAP_EVENT.to_vec();
        data.extend(borsh::to_vec(&event).unwrap());

        match events::unpack(&data).expect("decode event") {
            OkxV2Event::Swap(decoded) => assert_eq!(decoded, event),
            _ => panic!("expected SwapEvent"),
        }
    }

    #[test]
    fn swap_event_from_program_data_log() {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode("QMbN6CYIceI/v3dfOwUAAACk+K2gOgAAAA==")
            .expect("decode base64 program data");

        match events::unpack(&bytes).expect("decode event") {
            OkxV2Event::Swap(event) => {
                assert_eq!(event.dex, Dex::Tessera);
                assert_eq!(event.amount_in, 22_470_948_799);
                assert_eq!(event.amount_out, 251_803_859_108);
            }
            _ => panic!("expected SwapEvent"),
        }
    }

    #[test]
    fn swap_event_from_program_log_line() {
        match logs::unpack("SwapEvent { dex: Tessera, amount_in: 22470948799, amount_out: 251803859108 }").expect("decode log") {
            logs::OkxV2Log::Swap(event) => {
                assert_eq!(event.dex, Dex::Tessera);
                assert_eq!(event.amount_in, 22_470_948_799);
                assert_eq!(event.amount_out, 251_803_859_108);
            }
            _ => panic!("expected SwapEvent log"),
        }
    }

    #[test]
    fn route_amm_pool_from_program_log_pair() {
        match logs::unpack_route_amm_pool(
            "Program log: Dex::Tessera amount_in: 13320946404, offset: 0",
            "Program log: FLckHLGMJy5gEoXWwcE68Nprde1D4araK4TGLw4pQq2n",
        )
        .expect("decode route pool")
        {
            logs::OkxV2Log::RouteAmmPool(log) => {
                assert_eq!(log.dex, Dex::Tessera);
                assert_eq!(log.amount_in, 13_320_946_404);
                assert_eq!(log.offset, 0);
                assert_eq!(log.amm_pool.to_string(), "FLckHLGMJy5gEoXWwcE68Nprde1D4araK4TGLw4pQq2n");
            }
            _ => panic!("expected route pool log"),
        }
    }

    #[test]
    fn route_amm_pool_from_raw_program_logs() {
        let raw_logs = r#"Program ComputeBudget111111111111111111111111111111 invoke [1]
Program ComputeBudget111111111111111111111111111111 success
Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma invoke [1]
Program log: Instruction: SwapV3
Program log: before_source_balance: 104910092088, before_destination_balance: 3422088026285, amount_in: 13320946404, expect_amount_out: 149665924142, min_return: 149665924142
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program log: Instruction: TransferChecked
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
Program log: Dex::Tessera amount_in: 13320946404, offset: 0
Program log: FLckHLGMJy5gEoXWwcE68Nprde1D4araK4TGLw4pQq2n
Program TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH invoke [2]
Program TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH success
Program data: QMbN6CYIceI/5IL9GQMAAABSAdzYIgAAAA==
Program log: SwapEvent { dex: Tessera, amount_in: 13320946404, amount_out: 149667184978 }
Program 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma success"#;
        let lines = raw_logs.lines().collect::<Vec<_>>();
        let pools = logs::unpack_route_amm_pools(&lines).expect("decode route pools");

        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].dex, Dex::Tessera);
        assert_eq!(pools[0].amount_in, 13_320_946_404);
        assert_eq!(pools[0].offset, 0);
        assert_eq!(pools[0].amm_pool.to_string(), "FLckHLGMJy5gEoXWwcE68Nprde1D4araK4TGLw4pQq2n");
    }

    #[test]
    fn route_amm_pools_from_sanctum_alphaq_program_logs() {
        let raw_logs = r#"Program log: Dex::SanctumSwapWithoutWsol amount_in: 6427559292, offset: 0
Program log: AYhux5gJzCoeoc1PoJ1VxwPDe22RwcvpHviLDD1oCGvW
Program 5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx invoke [2]
Program data: QMbN6CYIceIVfMUcfwEAAADarlWaAQAAAA==
Program log: SwapEvent { dex: SanctumNonWsolSwap, amount_in: 6427559292, amount_out: 6884273882 }
Program log: Dex::AlphaQ amount_in: 6884273882, offset: 25
Program log: C2GdMFGp2vSZHnU76pH2ukEWxuhoJBuaA54Ftzcvv4z5
Program ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA invoke [2]
Program data: QMbN6CYIceJT2q5VmgEAAAB1lizmAQAAAA==
Program log: SwapEvent { dex: AlphaQ, amount_in: 6884273882, amount_out: 8156649077 }"#;
        let lines = raw_logs.lines().collect::<Vec<_>>();
        let pools = logs::unpack_route_amm_pools(&lines).expect("decode route pools");

        assert_eq!(pools.len(), 2);
        assert_eq!(pools[0].dex, Dex::SanctumNonWsolSwap);
        assert_eq!(pools[0].amount_in, 6_427_559_292);
        assert_eq!(pools[0].offset, 0);
        assert_eq!(pools[0].amm_pool.to_string(), "AYhux5gJzCoeoc1PoJ1VxwPDe22RwcvpHviLDD1oCGvW");
        assert_eq!(pools[1].dex, Dex::AlphaQ);
        assert_eq!(pools[1].amount_in, 6_884_273_882);
        assert_eq!(pools[1].offset, 25);
        assert_eq!(pools[1].amm_pool.to_string(), "C2GdMFGp2vSZHnU76pH2ukEWxuhoJBuaA54Ftzcvv4z5");
    }

    #[test]
    fn swap_v3_accounts_include_mints() {
        let payer = [1u8; 32];
        let source_token_account = [2u8; 32];
        let destination_token_account = [3u8; 32];
        let source_mint = [4u8; 32];
        let destination_mint = [5u8; 32];
        let program = [6u8; 32];

        let trx = pb::ConfirmedTransaction {
            transaction: Some(pb::Transaction {
                message: Some(pb::Message {
                    account_keys: vec![
                        payer.to_vec(),
                        source_token_account.to_vec(),
                        destination_token_account.to_vec(),
                        source_mint.to_vec(),
                        destination_mint.to_vec(),
                        program.to_vec(),
                    ],
                    instructions: vec![pb::CompiledInstruction {
                        program_id_index: 5,
                        accounts: vec![0, 1, 2, 3, 4],
                        data: vec![],
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            }),
            meta: Some(pb::TransactionStatusMeta::default()),
        };

        let ix = trx.compiled_instructions().next().expect("instruction view");
        let decoded = accounts::get_swap_v3_accounts(&ix).expect("decode accounts");

        assert_eq!(decoded.source_mint.to_bytes(), source_mint);
        assert_eq!(decoded.destination_mint.to_bytes(), destination_mint);
    }

    #[test]
    fn swap_v3_fixture_accounts_include_usdc_wsol_mints() {
        let payer = Pubkey::from_str("BpEBKwah2sGwa9zNtdoUgdmEDdms8L2NkFLfXShrz1ac").unwrap();
        let source_token_account = Pubkey::from_str("6KiH81VTVqNGqwkDgtk2qV7rRjWfSXrMaRD45KW5VG8K").unwrap();
        let destination_token_account = Pubkey::from_str("4NYrbAm1jfjgnV9JUTQbZ5VJoXYGHGKZcZqGLupCwJDU").unwrap();
        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let okx_program = Pubkey::from_str("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma").unwrap();

        let trx = pb::ConfirmedTransaction {
            transaction: Some(pb::Transaction {
                message: Some(pb::Message {
                    account_keys: vec![
                        payer.to_bytes().to_vec(),
                        source_token_account.to_bytes().to_vec(),
                        destination_token_account.to_bytes().to_vec(),
                        usdc_mint.to_bytes().to_vec(),
                        wsol_mint.to_bytes().to_vec(),
                        [6u8; 32].to_vec(),
                        [7u8; 32].to_vec(),
                        [8u8; 32].to_vec(),
                        [9u8; 32].to_vec(),
                        [10u8; 32].to_vec(),
                        [11u8; 32].to_vec(),
                        [12u8; 32].to_vec(),
                        [13u8; 32].to_vec(),
                        [14u8; 32].to_vec(),
                        okx_program.to_bytes().to_vec(),
                    ],
                    instructions: vec![pb::CompiledInstruction {
                        program_id_index: 14,
                        accounts: (0..14).collect(),
                        data: vec![],
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            }),
            meta: Some(pb::TransactionStatusMeta::default()),
        };

        let ix = trx.compiled_instructions().next().expect("instruction view");
        let decoded = accounts::get_swap_v3_accounts(&ix).expect("decode accounts");

        assert_eq!(decoded.payer, payer);
        assert_eq!(decoded.source_token_account, source_token_account);
        assert_eq!(decoded.destination_token_account, destination_token_account);
        assert_eq!(decoded.source_mint, usdc_mint);
        assert_eq!(decoded.destination_mint, wsol_mint);
    }
}
