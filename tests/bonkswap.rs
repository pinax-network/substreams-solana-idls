#[cfg(test)]
mod tests {
    use borsh::BorshSerialize;
    use substreams_solana_idls::bonkswap;

    #[test]
    fn unpack_swap_instruction() {
        use bonkswap::instructions::{unpack, BonkSwapInstruction, SwapInstruction, SWAP};

        let payload = SwapInstruction {
            delta_in: 1,
            price_limit: 2,
            x_to_y: true,
        };
        let mut data = SWAP.to_vec();
        payload.serialize(&mut data).unwrap();

        match unpack(&data).expect("decode") {
            BonkSwapInstruction::Swap(ix) => {
                assert_eq!(ix.delta_in, 1);
                assert_eq!(ix.price_limit, 2);
                assert!(ix.x_to_y);
            }
            _ => panic!("expected swap"),
        }
    }
}
