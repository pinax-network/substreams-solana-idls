#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::raydium;

    #[test]
    fn unpack_amm_v4_swap_event() {
        // https://solscan.io/tx/57d3uDBdPyrHX44aPzWVznDn39qx3ixFdRVieEfvhKtYErtgbYUpetApiZaYDSHCsfQWmqJryjknyFYT2U21oqrU
        // let base64 = "AwBOclMAAAAAmOUFnQ4AAAACAAAAAAAAAABOclMAAAAAfTDHRyEBAAAR7JIChTUAAIVC62EPAAAA";
        let bytes = hex!("03004e72530000000098e5059d0e0000000200000000000000004e7253000000007d30c7472101000011ec9202853500008542eb610f000000");

        match raydium::amm::v4::logs::unpack(&bytes).expect("decode event") {
            raydium::amm::v4::logs::RaydiumV4Log::SwapBaseIn(event) => {
                assert_eq!(event.log_type, 3, "log_type");
                assert_eq!(event.amount_in, 1400000000, "amount_in");
                assert_eq!(event.minimum_out, 62763951512, "minimum_out");
                assert_eq!(event.direction, 2, "direction");
                assert_eq!(event.user_source, 1400000000, "user_source");
                assert_eq!(event.pool_coin, 1242449784957, "pool_coin");
                assert_eq!(event.pool_pc, 58845390105617, "pool_pc");
                assert_eq!(event.out_amount, 66067317381, "out_amount");
            }
            _ => panic!("Expected a Event"),
        }
    }

    #[test]
    fn unpack_amm_v4_swap_event_2() {
        // https://solscan.io/tx/3GTW94fNv4JF4LeWNXCd6mQhxcZjNic9D1UoY9Qenf2S2t7gcgj7z58KwmHMHAYPqKVe812rrnpf2SKw8MdbNW7m
        // let base64 = "AwL/AQAAAAAAAAAAAAAAAAACAAAAAAAAAAL/AQAAAAAA4VSaQlgAAADuOE6EHwAAAAS2AAAAAAAA";
        let bytes = hex!("0302ff0100000000000000000000000000020000000000000002ff010000000000e1549a4258000000ee384e841f00000004b6000000000000");

        match raydium::amm::v4::logs::unpack(&bytes).expect("decode event") {
            raydium::amm::v4::logs::RaydiumV4Log::SwapBaseIn(event) => {
                assert_eq!(event.log_type, 3, "log_type");
                assert_eq!(event.amount_in, 130818, "amount_in");
                assert_eq!(event.minimum_out, 0, "minimum_out");
                assert_eq!(event.direction, 2, "direction");
                assert_eq!(event.user_source, 130818, "user_source");
                assert_eq!(event.pool_coin, 379074532577, "pool_coin");
                assert_eq!(event.pool_pc, 135363705070, "pool_pc");
                assert_eq!(event.out_amount, 46596, "out_amount");
            }
            _ => panic!("Expected a Event"),
        }
    }

    #[test]
    fn unpack_amm_v4_swap_instruction() {
        // https://solscan.io/tx/2mCvqGCwJszrr3BWB1Jen6gtEpS1xMoakxD4KRAAqFka76WJhQDCMFRZX7VqKGPNqfGXUzvyrLDENoqizBcBGCWN
        match raydium::amm::v4::instructions::unpack(&hex!("0926ed3c0000000000c637450000000000")).expect("decode event") {
            raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event) => {
                assert_eq!(event.amount_in, 3992870, "amount_in");
                assert_eq!(event.minimum_amount_out, 4536262, "minimum_amount_out");
            }
            _ => panic!("Expected an Instruction"),
        }
    }

    #[test]
    fn unpack_amm_v4_swap_instruction_2() {
        // https://solscan.io/tx/3GTW94fNv4JF4LeWNXCd6mQhxcZjNic9D1UoY9Qenf2S2t7gcgj7z58KwmHMHAYPqKVe812rrnpf2SKw8MdbNW7m
        match raydium::amm::v4::instructions::unpack(&hex!("0902ff010000000000000000000000000040")).expect("decode event") {
            raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event) => {
                assert_eq!(event.amount_in, 130818, "amount_in");
                assert_eq!(event.minimum_amount_out, 0, "minimum_amount_out");
            }
            _ => panic!("Expected an Instruction"),
        }
    }

    #[test]
    fn unpack_amm_v4_swap_base_in_instruction() {
        // https://solscan.io/tx/4NHdxiefrFEDazhdrcTjmG73G3TewYjhexKvpnNHgnzG5NK53A5otzHcE3xrzohGVAjvxQnYL3ysLD8xH89dVr3
        match raydium::amm::v4::instructions::unpack(&hex!("092ec29212000000000000000000000000")).expect("decode event") {
            raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event) => {
                assert_eq!(event.amount_in, 311607854, "amount_in");
                assert_eq!(event.minimum_amount_out, u64::MIN, "minimum_amount_out");
            }
            _ => panic!("Expected an Instruction"),
        }
    }
    #[test]
    fn unpack_amm_v4_swap_base_out_instruction() {
        // https://solscan.io/tx/3DHK2CAoTWeBoyM4Yzfsd5zSAPnN1kPXpbDp42YXKh2gE5K4Un28xu5qEeCpq7f85hUC3cCVkivhCARffSLARXZf
        match raydium::amm::v4::instructions::unpack(&hex!("0bffffffffffffffff0b6f14a60c010000")).expect("decode event") {
            raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseOut(event) => {
                assert_eq!(event.max_amount_in, u64::MAX, "max_amount_in");
                assert_eq!(event.amount_out, 1153837592331, "amount_out");
            }
            _ => panic!("Expected a SwapBaseOut Instruction 1"),
        }
    }
}
