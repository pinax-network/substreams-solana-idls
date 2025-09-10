#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use base64::Engine;
    use substreams::hex;
    use substreams_solana_idls::raydium::cpmm;

    #[test]
    fn unpack_cpmm_swap_event_v1() {
        // https://solscan.io/tx/4kKAK8GFTrdmqsMqny7Bvh4Ume5vWqsw9BHeVUwEiPefbdxAYSnzWF38QV4iV1Y7Q3WnddGkfKbCyxtn4NoqoKuD
        // let base64 = "QMbN6CYIceJwUd/DEGbRPw2+ldrbHD1h/PUjKCkbb/k/AK44Td3RrLH9rdkNAAAA/rbodtcmAQDAD7ZMAAAAAFw7UYM6BgAAAAAAAAAAAAAAAAAAAAAAAE=";
        let bytes = hex!("40c6cde8260871e27051dfc31066d13f0dbe95dadb1c3d61fcf52328291b6ff93f00ae384dddd1acb1fdadd90d000000feb6e876d7260100c00fb64c000000005c3b51833a0600000000000000000000000000000000000001");

        match cpmm::events::unpack_event(&bytes).expect("decode event") {
            cpmm::events::RaydiumCpmmEvent::SwapEventV1(event) => {
                assert_eq!(event.base_input, true, "base_input");
                assert_eq!(event.input_amount, 1_287_000_000, "input_amount");
            }
            _ => panic!("Expected a Event"),
        }
    }

    #[test]
    fn unpack_cpmm_swap_event_v2() {
        // https://solscan.io/tx/gz8KEqNmnNpthq31nQogLWkLNMm3eT3FzQKcNwoJ6wAhLxUVk6qCbGvRPhFw5et8dxrS6psBnMFcbuAdtbGFQta
        let base64 = "QMbN6CYIceLDaOKRwjNXP6I2Olk7UZ+dmtkrhhO8crfwIq0BUi5EJ1h+UMWwFgAABwYi+rIBAAApSLsuAAAAAJiTfQMAAAAAAAAAAAAAAAAAAAAAAAAAAAFapfStVf3ObMBLO2gYyo3hrXzhGzL3/H46j2BlPPT19QabiFf+q4GE+2h/Y0YYwDXaxDncGus7VZig8AAAAAABgegdAAAAAAAAAAAAAAAAAAE=";
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base64)
            .expect("decode base64");

        match cpmm::events::unpack_event(&bytes).expect("decode event") {
            cpmm::events::RaydiumCpmmEvent::SwapEventV2(event) => {
                assert_eq!(event.base_input, true, "base_input");
                assert_eq!(event.input_amount, 784_025_641, "input_amount");
                assert_eq!(event.output_amount, 58_561_432, "output_amount");
                assert_eq!(event.trade_fee, 1_960_065, "trade_fee");
                assert_eq!(event.creator_fee, 0, "creator_fee");
                assert!(event.creator_fee_on_input, "creator_fee_on_input");
                assert_eq!(
                    event.input_mint.to_string(),
                    "76rTxzztXjJe7AUaBi7jQ5J61MFgpQgB4Cc934sWbonk",
                    "input_mint",
                );
                assert_eq!(
                    event.output_mint.to_string(),
                    "So11111111111111111111111111111111111111112",
                    "output_mint",
                );
            }
            _ => panic!("Expected SwapEventV2"),
        }
    }
}

