#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::penguin::instructions;

    #[test]
    fn swap_instruction() {
        // https://solscan.io/tx/oihkvvj7j5cUa9fzHjt1yG69fKzya2f5hNZ4MPs7ST83pVpPYAVzT2DWunXpJM69B6pa1yWPJ8puub6VRGHwVwJ
        let bytes = hex!("0152ea1600000000000000000000000000");
        match instructions::unpack(&bytes).expect("decode instruction") {
            instructions::TokenSwapInstruction::Swap { amount_in, minimum_amount_out } => {
                assert_eq!(amount_in, 1_501_778);
                assert_eq!(minimum_amount_out, 0);
            }
            _ => panic!("Expected Swap instruction"),
        }
    }
}
