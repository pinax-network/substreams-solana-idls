#[cfg(test)]
mod tests {
    use base64::Engine;
    use solfi::v2;
    use substreams::hex;

    #[test]
    fn decode_swap_instruction() {
        let bytes = hex!("075ceb260200000000000000000000000001");
        match v2::instructions::unpack(&bytes).expect("decode instruction") {
            v2::instructions::SolfiInstruction::Swap(ix) => {
                assert_eq!(ix.amount_in, 36_105_052);
                assert_eq!(ix.minimum_out, 0);
                assert_eq!(ix.direction, 1);
            }
            _ => panic!("expected Swap"),
        }
    }

    #[test]
    fn decode_swap_event() {
        let base64 = "S3VCwUhV8CXSyrcV3EtPUNCsQJvXpBqCGUobEJZVRnnjiR0AAAAAAEob3RUAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");
        match v2::events::unpack(&bytes).expect("decode event") {
            v2::events::SolfiEvent::Swap(event) => {
                assert_eq!(event.user.to_string(), "FBqu8mSDS32rbTLC3SUA17PLPLpXJb5qc8NBwLtmymjm");
                assert_eq!(event.amount_in, 366_811_978);
                assert_eq!(event.amount_out, 0);
                assert_eq!(event.unknown, 0);
            }
            _ => panic!("expected Swap"),
        }
    }
}
