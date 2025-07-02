#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::raydium;

    #[test]
    fn unpack_amm_v4_swap_event() {
        // https://solscan.io/tx/57d3uDBdPyrHX44aPzWVznDn39qx3ixFdRVieEfvhKtYErtgbYUpetApiZaYDSHCsfQWmqJryjknyFYT2U21oqrU
        // let base64 = "AwBOclMAAAAAmOUFnQ4AAAACAAAAAAAAAABOclMAAAAAfTDHRyEBAAAR7JIChTUAAIVC62EPAAAA";
        let bytes = hex!("03004e72530000000098e5059d0e0000000200000000000000004e7253000000007d30c7472101000011ec9202853500008542eb610f000000");

        match raydium::amm::v4::events::unpack(&bytes).expect("decode event") {
            raydium::amm::v4::events::RaydiumV4Event::SwapBaseIn(event) => {
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
}
