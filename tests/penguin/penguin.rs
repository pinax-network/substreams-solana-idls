#[cfg(test)]
mod tests {
    use substreams::hex;

    #[test]
    fn swap_instruction() {
        // https://solscan.io/tx/oihkvvj7j5cUa9fzHjt1yG69fKzya2f5hNZ4MPs7ST83pVpPYAVzT2DWunXpJM69B6pa1yWPJ8puub6VRGHwVwJ
        let bytes = hex!("0152ea1600000000000000000000000000");
        match substreams_solana_idls::penguin::instructions::unpack(&bytes).expect("decode instruction") {
            substreams_solana_idls::penguin::instructions::PenguinInstruction::Swap(ix) => {
                assert_eq!(
                    ix,
                    substreams_solana_idls::penguin::instructions::SwapInstruction {
                        amount_in: 1_501_778,
                        minimum_amount_out: 0,
                    }
                );
            }
            _ => panic!("Expected Swap instruction"),
        }
    }
}
