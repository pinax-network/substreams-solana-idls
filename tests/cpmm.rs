#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use solana_program::pubkey::Pubkey;
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
        use cpmm::events::{SwapEventV2, SWAP_EVENT};

        let event = SwapEventV2 {
            pool_id: Pubkey::default(),
            input_vault_before: 1,
            output_vault_before: 2,
            input_amount: 3,
            output_amount: 4,
            input_transfer_fee: 5,
            output_transfer_fee: 6,
            base_input: true,
            input_mint: Pubkey::default(),
            output_mint: Pubkey::default(),
            trade_fee: 7,
            creator_fee: 8,
            creator_fee_on_input: false,
        };

        let mut bytes = SWAP_EVENT.to_vec();
        bytes.extend(borsh::to_vec(&event).expect("encode"));

        match cpmm::events::unpack_event(&bytes).expect("decode event") {
            cpmm::events::RaydiumCpmmEvent::SwapEventV2(decoded) => {
                assert_eq!(decoded.trade_fee, 7, "trade_fee");
                assert_eq!(decoded.creator_fee, 8, "creator_fee");
                assert!(!decoded.creator_fee_on_input, "creator_fee_on_input");
            }
            _ => panic!("Expected SwapEventV2"),
        }
    }
}

