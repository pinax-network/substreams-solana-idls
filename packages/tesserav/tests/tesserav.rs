#[cfg(test)]
mod tests {
    use substreams::hex;
    use tesserav;

    #[test]
    fn unpack_swap_instruction() {
        // https://solscan.io/tx/5EnwvjH197dogrBjyiVmCHniUPoYRQksTwUTczLL568ukQQY5wK2JX3iDwMJ3yEyQgEB3MLJYF6D79FAM2fkbDZM
        let bytes = hex!("1000ec930d00000000000000000000000000");
        match tesserav::instructions::unpack(&bytes).expect("decode instruction") {
            tesserav::instructions::TesseraVInstruction::Swap(ix) => {
                assert!(!ix.is_a_to_b);
                assert_eq!(ix.amount_in, 889_836);
                assert_eq!(ix.min_amount_out, 0);
            }
            _ => panic!("Expected a Swap instruction"),
        }
    }
}
