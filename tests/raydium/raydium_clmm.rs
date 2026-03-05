#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::raydium::clmm;

    #[test]
    fn unpack_swap_v1_deprecated() {
        // https://solscan.io/tx/36pQ8ChCf4Jje7YWM22BBMszvix6HoK6S4zH31p3nJn3eiVGLPEEhUKTEqhdFjDbVm5dQ8djXfDH3qwzQUZUFXfL
        let bytes = hex!("f8c69e91e17587c80065cd1d0000000000000000000000009a57694ea91a5c84b1c4feff0000000001");

        match clmm::v3::instructions::unpack(&bytes).expect("decode instruction") {
            clmm::v3::instructions::RaydiumClmmInstruction::Swap(event) => {
                assert_eq!(
                    event,
                    clmm::v3::instructions::SwapInstruction {
                        amount: 500000000,
                        other_amount_threshold: 0,
                        sqrt_price_limit_x64: 79226673521066979257578248090,
                        is_base_input: true
                    }
                );
            }
            _ => panic!("Expected a SwapInstruction"),
        }
    }

    #[test]
    fn unpack_swap_v2() {
        // https://solscan.io/tx/3iuj2NSLKVwLx5GA3gK8JBKGcE4x4w6dTzmsFXGJRrF5R6pYLJXZ59xTh6awypDW5LSAMJvcRxxMbpxqprjYkECN
        let bytes = hex!("2b04ed0b1ac91e6280af2a230000000087cdca4621070000513b010001000000000000000000000001");

        match clmm::v3::instructions::unpack(&bytes).expect("decode instruction") {
            clmm::v3::instructions::RaydiumClmmInstruction::SwapV2(event) => {
                assert_eq!(
                    event,
                    clmm::v3::instructions::SwapInstruction {
                        amount: 590000000,
                        other_amount_threshold: 7839503011207,
                        sqrt_price_limit_x64: 4295048017,
                        is_base_input: true
                    }
                );
            }
            _ => panic!("Expected a SwapInstruction"),
        }
    }
}
