#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use base64::Engine;
    use substreams_solana_idls::raydium::stable;

    #[test]
    fn unpack_stable_swap_event() {
        // https://provided program logs event example
        let base64 = "QMbN6CYIceIF0asNAwAAAABJXQ0DAAAAAA==";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");

        match stable::events::unpack(&bytes).expect("decode event") {
            stable::events::RaydiumStableEvent::SwapEvent(event) => {
                assert_eq!(event.dex, 5, "dex");
                assert_eq!(event.amount_in, 51_227_601, "amount_in");
                assert_eq!(event.amount_out, 51_207_497, "amount_out");
            }
            _ => panic!("Expected SwapEvent"),
        }
    }
}
