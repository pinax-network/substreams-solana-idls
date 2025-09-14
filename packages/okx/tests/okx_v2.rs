#[cfg(test)]
mod tests {
    use okx::v2::instructions::{self, Dex, Route, SwapArgs, SwapV3Instruction};
    use substreams::hex;

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
}
