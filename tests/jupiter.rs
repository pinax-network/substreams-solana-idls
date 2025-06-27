#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::jupiter;

    #[test]
    fn unpack_v6_swap_event() {
        // https://solscan.io/tx/5xQncr8APuvFEzZVnBDJf7BKdFB5MRQ4nFpmCPZxUgqriAn4gLUHhRe6hXwiEsqxWCmATHQraV7Rx6E7gci2CAP
        let bytes = hex!("e445a52e51cb9a1d40c6cde8260871e20c14defc825ec67694250818bb654065f4298d3156d571b4d4f8090c18e9a863fd51dc4c913e7ca8e73cd3cbb45769ac41aaade3696540dcf482a5fe57383a8f28cac2eb06000000069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f000000000019bdc520300000000");

        match jupiter::v6::events::unpack(&bytes).expect("decode jupiterEvent") {
            jupiter::v6::events::JupiterV6Event::Swap(event) => {
                assert_eq!(
                    event,
                    jupiter::v6::events::SwapEvent {
                        amm: "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".parse().expect("parse Pubkey"),
                        input_mint: "J3rYdme789g1zAysfbH9oP4zjagvfVM2PX7KJgFDpump".parse().expect("parse Pubkey"),
                        input_amount: 29725215272,
                        output_mint: "So11111111111111111111111111111111111111112".parse().expect("parse Pubkey"),
                        output_amount: 55762075,
                    }
                );
            }
            _ => panic!("Expected a TradeEvent"),
        }
    }

    #[test]
    fn unpack_v6_swap_fee_event() {
        // https://solscan.io/tx/3tNX6MzVEnsDCJWSnM5xqBQNWXh22T3q2PkF1hys35P9kpxVuut6chgPbnFs5Cyp9ygU9EidQgvqbuak26u1bmQM
        let bytes = hex!("e445a52e51cb9a1d494f4e7fb8d50ddc71337d91df2be75ff1c1ce88c1f5f0293ce7ee4562f6ee3e045fa07faad311dd069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f0000000000108b5000000000000");

        match jupiter::v6::events::unpack(&bytes).expect("decode jupiterEvent") {
            jupiter::v6::events::JupiterV6Event::Fee(event) => {
                assert_eq!(
                    event,
                    jupiter::v6::events::FeeEvent {
                        account: "8ctcHN52LY21FEipCjr1MVWtoZa1irJQTPyAaTj72h7S".parse().expect("parse Pubkey"),
                        mint: "So11111111111111111111111111111111111111112".parse().expect("parse Pubkey"),
                        amount: 46344,
                    }
                );
            }
            _ => panic!("Expected a TradeEvent"),
        }
    }
}
