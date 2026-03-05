#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::saros;

    #[test]
    fn swap_instruction() {
        // https://solscan.io/tx/5q8HEhbmxBmo73AFTDfynCCRtcTfCRBBV3CaBsRCsAKfYj5JLXJzaCGH59a5rn72onddJR1oVXmyx9AfU2BPdsTR
        let bytes = hex!("017660741a000000000000000000000000");
        match substreams_solana_idls::saros::instructions::unpack(&bytes).expect("decode instruction") {
            substreams_solana_idls::saros::instructions::SarosInstruction::Swap(ix) => {
                assert_eq!(
                    ix,
                    substreams_solana_idls::saros::instructions::SwapInstruction {
                        amount_in: 443_834_486,
                        minimum_amount_out: 0,
                    }
                );
            }
            _ => panic!("Expected Swap instruction"),
        }
    }
}
