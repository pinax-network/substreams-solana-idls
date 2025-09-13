#[cfg(test)]
mod tests {
    use substreams::hex;
    use jupiter;

    #[test]
    fn unpack_v4_swap_event() {
        // https://solscan.io/tx/5xQncr8APuvFEzZVnBDJf7BKdFB5MRQ4nFpmCPZxUgqriAn4gLUHhRe6hXwiEsqxWCmATHQraV7Rx6E7gci2CAP
        let bytes = hex!("516ce3becdd00ac47e54771a57a6f14ca9e402d54aee45f7378aca365c7b169a7ec83f5182b298f0069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f0000000000103e727000000000006a1ec5bd82ad9c032a9f7d466ba2c728b0ef36a8b773ed219d69650d3472bd665ba6d3901000000");

        match jupiter::v4::events::unpack(&bytes).expect("decode jupiterEvent") {
            jupiter::v4::events::JupiterV4Event::Swap(event) => {
                assert_eq!(
                    event,
                    jupiter::v4::events::SwapEvent {
                        amm: "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP".parse().expect("parse Pubkey"),
                        input_mint: "So11111111111111111111111111111111111111112".parse().expect("parse Pubkey"),
                        input_amount: 2615043,
                        output_mint: "StepAscQoEioFxxWGnh2sLBDFp9d8rvKz2Yp39iDpyT".parse().expect("parse Pubkey"),
                        output_amount: 5258459749,
                    }
                );
            }
            _ => panic!("Expected a TradeEvent"),
        }
    }
}
