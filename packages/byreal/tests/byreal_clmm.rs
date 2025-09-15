#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use base64::prelude::*;
    use byreal::clmm;
    use substreams::hex;

    #[test]
    fn unpack_swap_v2() {
        // https://solscan.io/tx/3HXXgbu4qogXV4Wdd45hWjMkXaozRtZ4q218GwsQwcxQZrphXFGGbfLH5TEJYrGL8rXnqHMunvGGBVquaftv6rmr
        let bytes = hex!("2b04ed0b1ac91e6280a7e50b000000003a60ba0a000000000000000000000000000000000000000001");
        match clmm::instructions::unpack(&bytes).expect("decode instruction") {
            clmm::instructions::ByrealClmmInstruction::SwapV2(ix) => {
                assert_eq!(
                    ix,
                    clmm::instructions::SwapInstruction {
                        amount: 199_600_000,
                        other_amount_threshold: 179_986_490,
                        sqrt_price_limit_x64: 0,
                        is_base_input: true,
                    }
                );
            }
            _ => panic!("Expected SwapV2 instruction"),
        }
    }

    #[test]
    fn swap_event() {
        let bytes = BASE64_STANDARD.decode("QMbN6CYIceJpwRq4TEEH+giluk1y+E+UaLJ185GNtH22OJZcsFWHwD6CCOZhd0gD9+ObJSAGxAE8YubkjOww95H7zH5K87vTttClBpkojyKfAHiVwxyUIGpcmQF2OBKLhR+VjMEWxrVmc8JsiuoEZbAdSicLaeRkq3RJ9BF+f9NoRzI8DNdMK4Cn5QsAAAAAAAAAAAAAAADMo8IKAAAAAAAAAAAAAAAAAa4KaOmv/IXzAAAAAAAAAACF7myvI6rkAwAAAAAAAAAAGPz//w==").expect("base64 decode");
        match clmm::events::unpack(&bytes).expect("decode event") {
            clmm::events::ByrealClmmEvent::SwapEvent(event) => {
                assert_eq!(
                    event,
                    clmm::events::SwapEvent {
                        pool_state: "87pbGHxigtjdMovzkAAFEe8XFVTETjDomoEFfpSFd2yD".parse().unwrap(),
                        sender: "5D1HHAUJeRHqoajJHsAYNHXfUjdGauKe8EdCGvwkxPE2".parse().unwrap(),
                        token_account_0: "DJdjBrCPJekuaAzfy5njZ6dFxQ5MryBoMuUsPWNdvdpQ".parse().unwrap(),
                        token_account_1: "7tvxszGPiND7uLE4dFjcoCYxZKE1MzMYByeo2FB6Pdcn".parse().unwrap(),
                        amount_0: 199_600_000,
                        transfer_fee_0: 0,
                        amount_1: 180_528_076,
                        transfer_fee_1: 0,
                        zero_for_one: true,
                        sqrt_price_x64: 17_547_709_355_584_391_854,
                        liquidity: 280_536_147_047_542_405,
                        tick: -1000,
                    }
                );
            }
            _ => panic!("Expected SwapEvent"),
        }
    }
}
