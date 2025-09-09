#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::raydium::cpmm;

    #[test]
    fn unpack_cpmm_swap_event() {
        // https://solscan.io/tx/4kKAK8GFTrdmqsMqny7Bvh4Ume5vWqsw9BHeVUwEiPefbdxAYSnzWF38QV4iV1Y7Q3WnddGkfKbCyxtn4NoqoKuD
        // let base64 = "QMbN6CYIceJwUd/DEGbRPw2+ldrbHD1h/PUjKCkbb/k/AK44Td3RrLH9rdkNAAAA/rbodtcmAQDAD7ZMAAAAAFw7UYM6BgAAAAAAAAAAAAAAAAAAAAAAAAE=";
        let bytes = hex!("40c6cde8260871e27051dfc31066d13f0dbe95dadb1c3d61fcf52328291b6ff93f00ae384dddd1acb1fdadd90d000000feb6e876d7260100c00fb64c000000005c3b51833a0600000000000000000000000000000000000001");

        match cpmm::events::unpack_event(&bytes).expect("decode event") {
            cpmm::events::RaydiumCpmmEvent::SwapEvent(event) => {
                assert_eq!(event.base_input, true, "base_input");
                assert_eq!(event.input_amount, 1287000000, "input_amount");
            }
            _ => panic!("Expected a Event"),
        }
    }
}
