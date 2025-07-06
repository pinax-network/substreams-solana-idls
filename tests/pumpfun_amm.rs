#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::pumpfun;

    #[test]
    fn unpack_sell_event_v2() {
        // https://solscan.io/tx/5UWUzgvMGRrEBKqaPYDnSpSKTYsVYaaBsThoPVbSTMvGFTFa6P3ykQodcp3ShL9o1yUuuqix3wgHR74fHAj5q2HB
        let bytes = hex!("e445a52e51cb9a1d3e2f370aa503dc2a4545656800000000af28f004000000003c07deae14000000af28f00400000000ca938dcb04000000af0bdef97f000000ee1ef9689423020007103a201500000014000000000000001b03d10a000000000500000000000000c740b40200000000ec0c69151500000025ccb41215000000c40660072ddc79037ded460ba50e7db938f85e3cffdee2a1a443e0a544331255bd1f448d9dae5b6c1de2d05b043b7de51c4f56eedda06a074cc97e3db2700bbf8556d31ff821ffe7f4b72a466984b85eb850c5e389e07ac038836ac8ef37a9921072df16318f56050eab0292c03aa99d531d627d4c5147d5d403a9f109c665a7608ccc1dfce961b43b779c191505a6e2d3bf45d5a4db4618ad76c82d61754535ac96cbb25e6d405dfc5b494cc53c574d9e2e9e513c0f1216f5108edf7f3c82ec000000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000");

        match pumpfun::amm::anchor_self_cpi::unpack(&bytes).expect("decode event") {
            pumpfun::amm::anchor_self_cpi::PumpFunAmmEvent::SellEventV2(event) => {
                assert_eq!(event.timestamp, 1751467333, "timestamp");
                assert_eq!(event.base_amount_in, 82847919, "base_amount_in");
                assert_eq!(event.min_quote_amount_out, 88833132348, "min_quote_amount_out");
                assert_eq!(event.user_base_token_reserves, 82847919, "user_base_token_reserves");
                assert_eq!(event.user_quote_token_reserves, 20594922442, "user_quote_token_reserves");
                assert_eq!(event.pool_base_token_reserves, 549652925359, "pool_base_token_reserves");
                assert_eq!(event.pool_quote_token_reserves, 602070276710126, "pool_quote_token_reserves");
                assert_eq!(event.quote_amount_out, 90734989319, "quote_amount_out");
                assert_eq!(event.lp_fee_basis_points, 20, "lp_fee_basis_points");
                assert_eq!(event.lp_fee, 181469979, "lp_fee");
                assert_eq!(event.protocol_fee_basis_points, 5, "protocol_fee_basis_points");
                assert_eq!(event.protocol_fee, 45367495, "protocol_fee");
                assert_eq!(event.quote_amount_out_without_lp_fee, 90553519340, "quote_amount_out_without_lp_fee");
                assert_eq!(event.user_quote_amount_out, 90508151845, "user_quote_amount_out");
                assert_eq!(event.pool, "ECCYiudtxv9UaqGT2bDijsxDSMbmpYBx3vZ4SYqrYfXA".parse().unwrap(), "pool");
                assert_eq!(event.user, "DjFi5c3ZtY8kRzU4sXDese83tU48nab2qqzCeeVtaJN2".parse().unwrap(), "user");
                assert_eq!(
                    event.user_base_token_account,
                    "9yW1hQrgE8qy4pZtJnEK5GCGeRpuGX5Gcz7QCM39EhMb".parse().unwrap(),
                    "user_base_token_account"
                );
                assert_eq!(
                    event.user_quote_token_account,
                    "27D7jN48BA1hkqimrcqUEeQfRajm9QsoKoskUSkcB58a".parse().unwrap(),
                    "user_quote_token_account"
                );
                assert_eq!(
                    event.protocol_fee_recipient,
                    "7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ".parse().unwrap(),
                    "protocol_fee_recipient"
                );
                assert_eq!(
                    event.protocol_fee_recipient_token_account,
                    "CciVLGsXnAAPhP1BH7HKvHMoAC2Lu75embEe2D3CdsNK".parse().unwrap(),
                    "protocol_fee_recipient_token_account"
                );
                assert_eq!(event.coin_creator, "11111111111111111111111111111111".parse().unwrap(), "coin_creator");
                assert_eq!(event.coin_creator_fee_basis_points, 5, "coin_creator_fee_basis_points");
                assert_eq!(event.coin_creator_fee, 0, "coin_creator_fee");
            }
            _ => panic!("Expected a TradeEvent"),
        }
    }

    #[test]
    fn unpack_buy_event_v2() {
        // https://solscan.io/tx/3AvQDp1ZTdCPLLaf6zMC49gT6JwP7k21wPsTiitpGAS9kL6cKPTCSCKUtbv7No13i3XrbcRHWBLa2D2zrUjY9H5s
        let bytes = hex!("e445a52e51cb9a1d67f4521f2cf57777454565680000000068a95a0200000000fb2bd87a0a0000000000000000000000ebc54a1a1a0000005e34cefe7f000000021290537f23020045e90e120a000000140000000000000041f72705000000000500000000000000d1fd49010000000086e036170a00000057de80180a000000c40660072ddc79037ded460ba50e7db938f85e3cffdee2a1a443e0a544331255f771b015b5c0053e9f90450062b4af29076439be95be43461bacf21986db6c27b102664f7e7e0b85a6cd023a1af27c7fba5b3960e2730a11e5a9c25f014b72b874daca6974a5f667fd13d7c1edaa1a0e7105ee3e3691641ef09baaeee8c8b72d608ccc1dfce961b43b779c191505a6e2d3bf45d5a4db4618ad76c82d61754535ac96cbb25e6d405dfc5b494cc53c574d9e2e9e513c0f1216f5108edf7f3c82ec000000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000");

        match pumpfun::amm::anchor_self_cpi::unpack(&bytes).expect("decode event") {
            pumpfun::amm::anchor_self_cpi::PumpFunAmmEvent::BuyEventV2(event) => {
                assert_eq!(event.timestamp, 1751467333, "timestamp");
                assert_eq!(event.base_amount_out, 39496040, "base_amount_out");
                assert_eq!(event.max_quote_amount_in, 45010660347, "max_quote_amount_in");
                assert_eq!(event.user_base_token_reserves, 0, "user_base_token_reserves");
                assert_eq!(event.user_quote_token_reserves, 112110257643, "user_quote_token_reserves");
                assert_eq!(event.pool_base_token_reserves, 549735773278, "pool_base_token_reserves");
                assert_eq!(event.pool_quote_token_reserves, 601979723190786, "pool_quote_token_reserves");
                assert_eq!(event.quote_amount_in, 43252640069, "quote_amount_in");
                assert_eq!(event.lp_fee_basis_points, 20, "lp_fee_basis_points");
                assert_eq!(event.lp_fee, 86505281, "lp_fee");
                assert_eq!(event.protocol_fee_basis_points, 5, "protocol_fee_basis_points");
                assert_eq!(event.protocol_fee, 21626321, "protocol_fee");
                assert_eq!(event.quote_amount_in_with_lp_fee, 43339145350, "quote_amount_in_with_lp_fee");
                assert_eq!(event.user_quote_amount_in, 43360771671, "user_quote_amount_in");
                assert_eq!(event.pool, "ECCYiudtxv9UaqGT2bDijsxDSMbmpYBx3vZ4SYqrYfXA".parse().unwrap(), "pool");
                assert_eq!(event.user, "HevFMPmoWJE6WCDDpyC2ZWtyvFbkrLkEGPofYNbw3xjx".parse().unwrap(), "user");
                assert_eq!(
                    event.user_base_token_account,
                    "CuyHWxRHHhDehLoCJD53CA9QKp68rHTG2VyyNdLfcrCX".parse().unwrap(),
                    "user_base_token_account"
                );
                assert_eq!(
                    event.user_quote_token_account,
                    "8s9ny33tQZjpyvwtvEdBmG8z5pED8TKRniNRKwmVTf9v".parse().unwrap(),
                    "user_quote_token_account"
                );
                assert_eq!(
                    event.protocol_fee_recipient,
                    "7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ".parse().unwrap(),
                    "protocol_fee_recipient"
                );
                assert_eq!(
                    event.protocol_fee_recipient_token_account,
                    "CciVLGsXnAAPhP1BH7HKvHMoAC2Lu75embEe2D3CdsNK".parse().unwrap(),
                    "protocol_fee_recipient_token_account"
                );
                assert!(event.coin_creator == "11111111111111111111111111111111".parse().unwrap(), "coin_creator");
                assert_eq!(event.coin_creator_fee_basis_points, 5, "coin_creator_fee_basis_points");
                assert_eq!(event.coin_creator_fee, 0, "coin_creator_fee");
            }
            _ => panic!("Expected a TradeEvent"),
        }
    }

    #[test]
    fn unpack_buy_instruction() {
        // https://solscan.io/tx/3AvQDp1ZTdCPLLaf6zMC49gT6JwP7k21wPsTiitpGAS9kL6cKPTCSCKUtbv7No13i3XrbcRHWBLa2D2zrUjY9H5s
        let bytes = hex!("66063d1201daebea68a95a0200000000fb2bd87a0a000000");

        match pumpfun::amm::instructions::unpack(&bytes).expect("decode instruction") {
            pumpfun::amm::instructions::PumpFunAmmInstruction::Buy(event) => {
                assert_eq!(event.base_amount_out, 39496040, "base_amount_out");
                assert_eq!(event.max_quote_amount_in, 45010660347, "max_quote_amount_in");
            }
            _ => panic!("Expected a BuyInstruction"),
        }
    }

    #[test]
    fn unpack_sell_instruction() {
        // https://solscan.io/tx/3AvQDp1ZTdCPLLaf6zMC49gT6JwP7k21wPsTiitpGAS9kL6cKPTCSCKUtbv7No13i3XrbcRHWBLa2D2zrUjY9H5s
        let bytes = hex!("33e685a4017f83adaf28f004000000003c07deae14000000");

        match pumpfun::amm::instructions::unpack(&bytes).expect("decode instruction") {
            pumpfun::amm::instructions::PumpFunAmmInstruction::Sell(event) => {
                assert_eq!(event.base_amount_in, 82847919, "base_amount_in");
                assert_eq!(event.min_quote_amount_out, 88833132348, "min_quote_amount_out");
            }
            _ => panic!("Expected a BuyInstruction"),
        }
    }

    #[test]
    fn unpack_create_pool_v2_instruction() {
        // https://solscan.io/tx/bm6VNDbKhrZqXsrC297FNWX9EscSiFawEg2zKHmnBPT8WLJW92xdrh1Xhrb2KNEgRNSTPV9myHDWkxLoWe1voyx
        let bytes = hex!("e992d18ecf6840bc00000024ca94270000000080c6a47e8d03000000000000000000000000000000000000000000000000000000000000000000");

        match pumpfun::amm::instructions::unpack(&bytes).expect("decode instruction") {
            pumpfun::amm::instructions::PumpFunAmmInstruction::CreatePoolV2(event) => {
                assert_eq!(event.index, 0, "index");
                assert_eq!(event.base_amount_in, 170000000000, "base_amount_in");
                assert_eq!(event.quote_amount_in, 1000000000000000, "quote_amount_in");
                assert_eq!(event.coin_creator, "11111111111111111111111111111111".parse().unwrap(), "coin_creator");
            }
            _ => panic!("Expected a BuyInstruction"),
        }
    }

    #[test]
    fn unpack_create_pool_v2_event() {
        // https://solscan.io/tx/bm6VNDbKhrZqXsrC297FNWX9EscSiFawEg2zKHmnBPT8WLJW92xdrh1Xhrb2KNEgRNSTPV9myHDWkxLoWe1voyx
        let bytes = hex!("e445a52e51cb9a1db1310cd2a076a7749969656800000000000096c4e2ab2ea0b7c2f093a905be19705a6a761f8d4c3115cb43338a314f8ae05b069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f00000000001915f49b31a963278b23a8674bf218caeb2b0dfdc7c1c04c2623f9a742485535f09060024ca94270000000080c6a47e8d03000024ca94270000000080c6a47e8d03006400000000000000a5b67cbddb0b000041b67cbddb0b0000fec430179d65d3520de4fb25593a097803004bd24f9f7e44a1fc3b1cff3a82698e62656ca724fa17df29301afa7dbdef55588a2c09f270d1db5f8c99f5ce47b8643bc49a393fb897bf94f3c4a295b1be1dbf18752a2d84fd6f7bacbe66f63be840093a5ad265034d3e6bb2c7bffbae0478b359bca217afb634cd172d292c5827260000000000000000000000000000000000000000000000000000000000000000");

        match pumpfun::amm::anchor_self_cpi::unpack(&bytes).expect("decode event") {
            pumpfun::amm::anchor_self_cpi::PumpFunAmmEvent::CreatePoolEventV2(event) => {
                // assert_eq!(event.timestamp, 1751476633, "timestamp");
                assert_eq!(event.index, 0, "index");
                assert_eq!(event.creator, "B9YHJjB71MuL8aPjFtWc9kpaKH4KJD4CFgovEh5xaAVg".parse().unwrap(), "creator");
                assert_eq!(event.base_mint, "So11111111111111111111111111111111111111112".parse().unwrap(), "base_mint");
                assert_eq!(event.quote_mint, "AnUPaVnGeVbAPfBqHDmP7uZ6rQu1BdJtBveftnHEpump".parse().unwrap(), "quote_mint");
                assert_eq!(event.base_mint_decimals, 9, "base_mint_decimals");
                assert_eq!(event.quote_mint_decimals, 6, "quote_mint_decimals");
                assert_eq!(event.base_amount_in, 170000000000, "base_amount_in");
                assert_eq!(event.quote_amount_in, 1000000000000000, "quote_amount_in");
                assert_eq!(event.pool_base_amount, 170000000000, "pool_base_amount");
                assert_eq!(event.pool_quote_amount, 1000000000000000, "pool_quote_amount");
                assert_eq!(event.minimum_liquidity, 100, "minimum_liquidity");
                assert_eq!(event.initial_liquidity, 13038404810405, "initial_liquidity");
                assert_eq!(event.lp_token_amount_out, 13038404810305, "lp_token_amount_out");
                assert_eq!(event.pool_bump, 254, "pool_bump");
                assert_eq!(event.pool, "ECqSdAvSDuwBHDsfChLqkyHEbah6TH3weL5JZ4F7zGMb".parse().unwrap(), "pool");
                assert_eq!(event.lp_mint, "7d6eyGMqwHp6wUnTQsihqUKhynbRsrrmKXkviyDL9U67".parse().unwrap(), "lp_mint");
                assert_eq!(
                    event.user_base_token_account,
                    "52JvxjCzUe3JGKr7ZcLShCLQerhCrsjmQvikmEsn3uFd".parse().unwrap(),
                    "user_base_token_account"
                );
                assert_eq!(
                    event.user_quote_token_account,
                    "d2H9zsYnvBBJnqSYcCTuwYx6EJarNRb1F3bK6b7sjt1".parse().unwrap(),
                    "user_quote_token_account"
                );
                assert_eq!(event.coin_creator, "11111111111111111111111111111111".parse().unwrap(), "coin_creator");
                // assert_eq!(event.coin_creator_fee_bps, 5, "coin_creator_fee_bps");
                // assert_eq!(event.coin_creator_fee, 0, "coin_creator_fee");
            }
            _ => panic!("Expected a event"),
        }
    }
}
