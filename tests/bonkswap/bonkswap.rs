#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::bonkswap;

    #[test]
    fn unpack_swap_event() {
        // https://solscan.io/tx/V8eadAratmZPPwZP8aJJDCS6ZBQkNMN9KqVQefVubkCZKyw7ofT1DnunVJRTk8yiwkyyuGMjTv5MJzrAfAT9VxX
        let bytes = hex!("f8c69e91e17587c8726a450900000000ffffffffffffffffffffffffffffffff00");

        match substreams_solana_idls::bonkswap::instructions::unpack(&bytes).expect("decode instruction") {
            substreams_solana_idls::bonkswap::instructions::BonkSwapInstruction::Swap(event) => {
                assert_eq!(
                    event,
                    substreams_solana_idls::bonkswap::instructions::SwapInstruction {
                        delta_in: substreams_solana_idls::bonkswap::instructions::Token { v: 155544178 },
                        price_limit: substreams_solana_idls::bonkswap::instructions::FixedPoint {
                            v: 340282366920938463463374607431768211455,
                        },
                        x_to_y: false
                    }
                );
            }
            _ => panic!("Expected a SwapInstruction"),
        }
    }
}
