#[cfg(test)]
mod tests {
    use base64::prelude::*;
    use pancakeswap;
    use substreams::hex;

    #[test]
    fn swap_instruction() {
        let bytes = hex!("f8c69e91e17587c8001bb70000000000ffffffffffffffff0000000000000000000000000000000000");
        match pancakeswap::instructions::unpack(&bytes).expect("decode instruction") {
            pancakeswap::instructions::PancakeSwapInstruction::Swap(ix) => {
                assert_eq!(
                    ix,
                    pancakeswap::instructions::SwapInstruction {
                        amount: 12_000_000,
                        other_amount_threshold: u64::MAX,
                        sqrt_price_limit_x64: 0,
                        is_base_input: false,
                    }
                );
            }
            _ => panic!("Expected SwapInstruction"),
        }
    }

    #[test]
    fn swap_event() {
        let bytes = BASE64_STANDARD
            .decode("QMbN6CYIceJ7JcKWCueJDroD6zdLF7C8Scs5WVmrkFzhFFxokePKiSMs7ScTXdfDnZnirgrntvRpZY8iIJA0U4ZGOOkDB1xy01xyr7Wn83b59FUflt8S+3YR7TDsa5HnmGozMQoBxqOKe+XwJSlYektM0QaH8s/H6HLIQhCF7BhBgqiQQbr8zC4EHwAAAAAAAAAAAAAAAAAAG7cAAAAAAAAAAAAAAAAAATY2amoJdgFuAgAAAAAAAAB65SHmCgAAAAAAAAAAAAAAXEUAAA==")
            .expect("base64 decode");

        match pancakeswap::events::unpack(&bytes).expect("decode event") {
            pancakeswap::events::PancakeSwapEvent::Swap(event) => {
                assert_eq!(
                    event,
                    pancakeswap::events::SwapEvent {
                        pool_state: "9HiYSTWK5otBxVc9e9bTNhpRDiDKjki65o9YxRhPWyQC".parse().unwrap(),
                        sender: "3NK19Y8KQyr3HmJVEmn1gjaXDDDpsVVQtBWmHkwZXkAm".parse().unwrap(),
                        token_account_0: "FE4na6rdS6xsCf2WbebozF4tizMW6JXUMX2tdBsyKmsG".parse().unwrap(),
                        token_account_1: "AKaqdf9yFtgArvdXG9LNx6dsR8dC7WAVjRzmJD4gSMUf".parse().unwrap(),
                        amount_0: 2_032_686,
                        transfer_fee_0: 0,
                        amount_1: 12_000_000,
                        transfer_fee_1: 0,
                        zero_for_one: true,
                        sqrt_price_x64: 44_820_234_749_380_015_670,
                        liquidity: 46_810_654_074,
                        tick: 17_756,
                    }
                );
            }
            _ => panic!("Expected SwapEvent"),
        }
    }
}
