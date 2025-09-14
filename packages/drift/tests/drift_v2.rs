#[cfg(test)]
mod tests {
    use base64::Engine;
    use drift::v2;
    use substreams::hex;

    #[test]
    fn decode_begin_swap() {
        let bytes = hex!("ae6de401f269e86921002200cace200c00000000");
        match v2::instructions::unpack(&bytes).expect("decode instruction") {
            v2::instructions::DriftInstruction::BeginSwap(ix) => {
                assert_eq!(ix.in_market_index, 33);
                assert_eq!(ix.out_market_index, 34);
                assert_eq!(ix.amount_in, 203_476_682);
            }
            _ => panic!("expected BeginSwap"),
        }

        let base64 = "t7rLuuG7X4Kz78ZoAAAAACIAlBzegf+f7gAAAAAAAAAAANpNL28CAAAAAAAAAAAAAAAsF72HcZTGAAAAAAAAAAAABI3KfgIAAAAAAAAAAAAAAAA1DAAUzQAAoLsNAA==";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");
        match v2::events::unpack(&bytes).expect("decode event") {
            v2::events::DriftEvent::SpotInterestRecord(event) => {
                assert_eq!(event.ts, 1_757_867_955);
                assert_eq!(event.market_index, 34);
                assert_eq!(event.deposit_balance, 67_166_964_201_430_164u128);
                assert_eq!(event.cumulative_deposit_interest, 10_455_305_690u128);
                assert_eq!(event.borrow_balance, 55_895_260_718_241_580u128);
                assert_eq!(event.cumulative_borrow_interest, 10_717_138_180u128);
                assert_eq!(event.optimal_utilization, 800_000);
                assert_eq!(event.optimal_borrow_rate, 52_500);
                assert_eq!(event.max_borrow_rate, 900_000);
            }
            _ => panic!("expected SpotInterestRecord"),
        }
    }

    #[test]
    fn decode_end_swap() {
        let bytes = hex!("b1b81bc1220dd291210022000000");
        match v2::instructions::unpack(&bytes).expect("decode instruction") {
            v2::instructions::DriftInstruction::EndSwap(ix) => {
                assert_eq!(ix.in_market_index, 33);
                assert_eq!(ix.out_market_index, 34);
                assert!(ix.limit_price.is_none());
                assert!(ix.reduce_only.is_none());
            }
            _ => panic!("expected EndSwap"),
        }

        let base64 = "ort7woo4+vGz78ZoAAAAAM+cjY9Ys484qMhhiydb7T65y+yoAqScK1eP11GYiHBWAIyGRwAAAADBtB0MAAAAACIAIQDZQQ8AAAAAADcJWgAAAAAAAAAAAAAAAAA=";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");
        match v2::events::unpack(&bytes).expect("decode event") {
            v2::events::DriftEvent::SwapRecord(event) => {
                assert_eq!(event.ts, 1_757_867_955);
                assert_eq!(event.user.to_string(), "EyRrHJuMqqiE2yMQSecHJ4hK1Y9MM7QUB39coLpc2RLq");
                assert_eq!(event.amount_out, 1_200_000_000);
                assert_eq!(event.amount_in, 203_273_409);
                assert_eq!(event.out_market_index, 34);
                assert_eq!(event.in_market_index, 33);
                assert_eq!(event.out_oracle_price, 999_897);
                assert_eq!(event.in_oracle_price, 5_900_599);
                assert_eq!(event.fee, 0);
            }
            _ => panic!("expected SwapRecord"),
        }
    }
}
