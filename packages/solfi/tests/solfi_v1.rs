#[cfg(test)]
mod tests {
    use base64::Engine;
    use solfi::v1;
    use substreams::hex;

    #[test]
    fn decode_swap_instruction() {
        let bytes = hex!("06faffffffffffffffad99f644000000004322dd1500000000d1e2534999010000e8030000000000000b23dd150000000001060000000000000000000000000000665ad1c1110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        match v1::instructions::unpack(&bytes).expect("decode instruction") {
            v1::instructions::SolfiInstruction::Swap(ix) => {
                assert_eq!(ix.data.len(), bytes.len() - 1);
                assert_eq!(&ix.data[0..8], &hex!("faffffffffffffff"));
            }
            _ => panic!("expected Swap"),
        }
    }

    #[test]
    fn decode_swap_event() {
        let base64 = "TzZmUrzTPrLxizcYaVilIUxKK9ig82ou11DM0Gbvv3U02w8BAAAAAEIi3RUAAAAAAAAAAAAAAAA=";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");
        match v1::events::unpack(&bytes).expect("decode event") {
            v1::events::SolfiEvent::Swap(event) => {
                assert_eq!(event.user.to_string(), "HFtNuvz6jj2FbLp43M6Ean4mn3imJrmLzdgVQQNikAvK");
                assert_eq!(event.amount_in, 366_813_762);
                assert_eq!(event.amount_out, 0);
            }
            _ => panic!("expected Swap"),
        }
    }
}
