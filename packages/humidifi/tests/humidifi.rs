#[cfg(test)]
mod tests {
    use humidifi::instructions;
    use substreams::hex;

    #[test]
    fn decode_mystery_instruction() {
        let bytes = hex!("5f7f5705ece557b9d98b7923eb12a946eac9f67609e09d6e6f2a54412ae39d6ee0f1190bb3e29d6e492f841e93e0dd6e8474c1b7338119d351f04825a25641503f");
        match instructions::unpack(&bytes).expect("decode instruction") {
            instructions::HumidiFiInstruction::Mystery(ix) => {
                assert_eq!(ix.unknown_pubkey.to_string(), "FeCpdbVJaMCBD7vvK5umuMtmk9ajySgY9PP46XYVz8sP");
                assert_eq!(ix.field1, 7_988_788_236_501_921_609);
                assert_eq!(ix.field2, 15_211_331_275_546_784_900);
                assert_eq!(ix.field3, 5_782_998_650_930_655_313);
                assert_eq!(ix.flag, 63);
            }
            _ => panic!("expected Mystery"),
        }
    }
}
