#![cfg(test)]
#![allow(deprecated)]

mod tests {
    use meteora::daam;
    use substreams::hex;

    #[test]
    fn unpack_daam_swap_event() {
        // https://solscan.io/tx/<unknown> contains this event payload
        let bytes = hex!("e445a52e51cb9a1d1b3c15d58aaabb9361eb363b6c0f73437a18891ec65e5a1f20fbbdd8870656657d3167e04d4830a5010000c2eb0b00000000ccc0a13a8c7f0000ee64bf1c6f8d0000e83e7db166ee49000000000000000000544b1d0000000000d4520700000000000000000000000000000000000000000000c2eb0b0000000053f2bb6800000000");
        match daam::anchor_cpi_event::unpack(&bytes).expect("decode event") {
            daam::anchor_cpi_event::MeteoraDammAnchorCpiEvent::EvtSwap(event) => {
                assert_eq!(event.pool.to_string(), "7bEa1teiLKdRbEopnCWjDdcvAAAnaHuiRiPErUKhLybS", "pool",);
                assert_eq!(event.trade_direction, 1, "trade_direction");
                assert!(!event.has_referral, "has_referral");
                assert_eq!(event.params.amount_in, 200_000_000, "amount_in");
                assert_eq!(event.params.minimum_amount_out, 140_240_255_828_172, "minimum_amount_out",);
                assert_eq!(event.swap_result.output_amount, 155_508_363_191_534, "output_amount",);
                assert_eq!(event.swap_result.next_sqrt_price, 20_809_798_131_728_104, "next_sqrt_price",);
                assert_eq!(event.swap_result.lp_fee, 1_919_828, "lp_fee");
                assert_eq!(event.swap_result.protocol_fee, 479_956, "protocol_fee");
                assert_eq!(event.swap_result.partner_fee, 0, "partner_fee");
                assert_eq!(event.swap_result.referral_fee, 0, "referral_fee");
                assert_eq!(event.actual_amount_in, 200_000_000, "actual_amount_in");
                assert_eq!(event.current_timestamp, 1_757_147_731, "current_timestamp");
            }
            _ => panic!("Expected Swap event"),
        }
    }
}
