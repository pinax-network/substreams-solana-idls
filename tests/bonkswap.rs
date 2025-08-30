#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::bonkswap;

    #[test]
    fn unpack_swap_event() {
        // https://solscan.io/tx/V8eadAratmZPPwZP8aJJDCS6ZBQkNMN9KqVQefVubkCZKyw7ofT1DnunVJRTk8yiwkyyuGMjTv5MJzrAfAT9VxX
        let bytes = hex!("f8c69e91e17587c8726a450900000000ffffffffffffffffffffffffffffffff00");

        match bonkswap::instructions::unpack(&bytes).expect("decode instruction") {
            bonkswap::instructions::BonkSwapInstruction::Swap(event) => {
                assert_eq!(
                    event,
                    bonkswap::instructions::SwapInstruction {
                        delta_in: 155544178,
                        price_limit: 340282366920938463463374607431768211455,
                        x_to_y: false
                    }
                );
            }
            _ => panic!("Expected a SwapInstruction"),
        }
    }
}
