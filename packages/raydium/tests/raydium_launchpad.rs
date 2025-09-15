#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use raydium::launchpad;
    use substreams::hex;

    #[test]
    fn unpack_launchpad_trade_event() {
        let bytes = hex!("e445a52e51cb9a1dbddb7fd34ee661eef5e31a347d6c0ffb5c1019f78d2c2244f6c99c6e873d3928bf94d281f65888bf0078c5fb51d10200de740e3ee9cf0300d7af30fc060000005ba7ef1db2a002006c872c7f0f000000577b4148b3a00200f2a5427f0f0000001c69160000000000fcd3512a01000000590e0000000000005f39000000000000de020000000000000000000000000000000001");
        match launchpad::anchor_cpi_event::unpack(&bytes).expect("decode event") {
            launchpad::anchor_cpi_event::RaydiumLaunchpadAnchorCpiEvent::TradeEventV1(event) => {
                assert_eq!(event.pool_state.to_string(), "HYqjiMv2jVE41qMoBDXAE5SEeNcyaEQMaEuVHCuisB4A", "pool_state");
                assert_eq!(event.total_base_sell, 793_100_000_000_000, "total_base_sell");
                assert_eq!(event.virtual_base, 1_073_025_605_596_382, "virtual_base");
                assert_eq!(event.virtual_quote, 30_000_852_951, "virtual_quote");
                assert_eq!(event.real_base_before, 739_636_820_289_371, "real_base_before");
                assert_eq!(event.real_quote_before, 66_558_134_124, "real_quote_before");
                assert_eq!(event.real_base_after, 739_641_825_262_423, "real_base_after");
                assert_eq!(event.real_quote_after, 66_559_583_730, "real_quote_after");
                assert_eq!(event.amount_in, 1_468_700, "amount_in");
                assert_eq!(event.amount_out, 5_004_973_052, "amount_out");
                assert_eq!(event.protocol_fee, 3_673, "protocol_fee");
                assert_eq!(event.platform_fee, 14_687, "platform_fee");
                assert_eq!(event.creator_fee, 734, "creator_fee");
                assert_eq!(event.share_fee, 0, "share_fee");
                assert!(
                    matches!(event.trade_direction, launchpad::anchor_cpi_event::TradeDirection::Buy),
                    "trade_direction"
                );
                assert!(matches!(event.pool_status, launchpad::anchor_cpi_event::PoolStatus::Fund), "pool_status");
                assert!(event.exact_in, "exact_in");
            }
            _ => panic!("Expected TradeEventV1"),
        }
    }

    #[test]
    fn unpack_launchpad_trade_event_2() {
        let bytes = hex!("e445a52e51cb9a1dbddb7fd34ee661ee98f7bd21e067c6d32836f1fedec18b74080417ae1639b740c2ae2cf1ed4d166b0078c5fb51d10200de740e3ee9cf0300d7af30fc06000000bd19242496bc0200960933c61100000035f94e2496bc0200710d33c611000000e80300000000000078df2a000000000003000000000000000a0000000000000000000000000000000000");
        match launchpad::anchor_cpi_event::unpack(&bytes).expect("decode event") {
            launchpad::anchor_cpi_event::RaydiumLaunchpadAnchorCpiEvent::TradeEventV2(event) => {
                assert_eq!(event.pool_state.to_string(), "BJ859ZCoKvCrq6Fv9aJGChHy9UnG7o2ZhLVpsYw8wyPp", "pool_state");
                assert_eq!(event.total_base_sell, 793_100_000_000_000, "total_base_sell");
                assert_eq!(event.virtual_base, 1_073_025_605_596_382, "virtual_base");
                assert_eq!(event.virtual_quote, 30_000_852_951, "virtual_quote");
                assert_eq!(event.real_base_before, 770_302_990_883_261, "real_base_before");
                assert_eq!(event.real_quote_before, 76_339_677_590, "real_quote_before");
                assert_eq!(event.real_base_after, 770_302_993_692_981, "real_base_after");
                assert_eq!(event.real_quote_after, 76_339_678_577, "real_quote_after");
                assert_eq!(event.amount_in, 1_000, "amount_in");
                assert_eq!(event.amount_out, 2_809_720, "amount_out");
                assert_eq!(event.protocol_fee, 3, "protocol_fee");
                assert_eq!(event.platform_fee, 10, "platform_fee");
                assert_eq!(event.share_fee, 0, "share_fee");
                assert!(
                    matches!(event.trade_direction, launchpad::anchor_cpi_event::TradeDirection::Buy),
                    "trade_direction"
                );
                assert!(matches!(event.pool_status, launchpad::anchor_cpi_event::PoolStatus::Fund), "pool_status");
            }
            _ => panic!("Expected TradeEventV2"),
        }
    }

    #[test]
    fn instruction_sell_exact_in() {
        // https://solscan.io/tx/2Yy4JHRCZbxfupUJymVwMnvGmvfcZHFewJAtbBQFzqB1BLxK2cyYFXB7qFQdR9rL6EhjUKUXrtYE7FwfHXWU1JFj
        let bytes = hex!("9527de9bd37c981af1e1139f101c00002c3cd31f000000000000000000000000");

        match launchpad::instructions::unpack(&bytes).expect("decode instruction") {
            launchpad::instructions::RaydiumLaunchpadInstruction::SellExactIn(ix) => {
                assert_eq!(
                    ix,
                    launchpad::instructions::SellExactInInstruction {
                        amount_in: 30_857_713_934_833,
                        minimum_amount_out: 533_937_196,
                        share_fee_rate: 0,
                    }
                );
            }
            _ => panic!("Expected a SellExactIn instruction"),
        }
    }
}
