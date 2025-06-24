#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::pumpfun;

    #[test]
    fn unpack_trade_event_v1() {
        // https://solscan.io/tx/285JxcUbpFMiBQkB3GQGdykYEyAxy4tswTrLFuctTPcdiCdtkSiBDkpX97VbP2Dibw657PVakGt4h6qz3NZqgstP
        let bytes = hex!("e445a52e51cb9a1dbddb7fd34ee661eed1eccc2b2d1ae4e554365084d2e506be491327246a3eb013188a1f7904a7cfbf64079b5d0000000080bd8289782e000000f9e4e232fe0ef74bd1a1c1c3d678748dcc382ae18715020e2e58a0469a91dd9de31952670000000023ba9b24070000009cc72a594aba0300230e7828000000009c2f180db9bb0200");

        match pumpfun::events::unpack(&bytes).expect("decode PumpFunEvent") {
            pumpfun::events::PumpFunEvent::TradeV1(event) => {
                assert_eq!(
                    event,
                    pumpfun::events::TradeEventV1 {
                        mint: "F8Tdm1Qo1HXiHAnCm6e3XpjDa2MejBxWn2RWK7nvpump".parse().unwrap(),
                        sol_amount: 1570441060,
                        token_amount: 51095238000000,
                        is_buy: false,
                        user: "HpUwfjtNscqm8zFpkmhtGoPuA2MxSRy5mDWHaaHndwkQ".parse().unwrap(),
                        timestamp: 1733433827,
                        virtual_sol_reserves: 30678956579,
                        virtual_token_reserves: 1049253416454044,
                        real_sol_reserves: 678956579,
                        real_token_reserves: 769353416454044,
                    }
                );
            }
            _ => panic!("Expected a TradeEvent"),
        }
    }

    #[test]
    fn unpack_trade_event_v2() {
        // https://solscan.io/tx/sK44CDg4qzi9jvTgA32dCTNh6Y3CgXki2kj9XtpaXRr83BipzpWPjnENzJR3TjLegAfDfPDG5Z8GZDkbrXDQk3w
        let bytes = hex!("e445a52e51cb9a1dbddb7fd34ee661ee1506533f60e64c4528916a7b404296ac72a1b1b65e817505f94b02b46d1969be46b1f416000000000076c04f9a040000013cd2f118afae78c0d0a91f649ff048b4a16749e82caac521286d90a6085cdd1026b2466800000000e7329a910b0000009d11034a374d0200e7867695040000009d79f0fda54e0100ad11e6a4fc2944a4fa8251bef815426e1bfb28c6b6646677607c6ad9f566a6465f000000000000001ed4370000000000deed67d04e125b1a2d7c6933fec6c7b08bce5763a576c88dcd36697d38298183050000000000000038f0020000000000");

        match pumpfun::events::unpack(&bytes).expect("decode PumpFunEvent") {
            pumpfun::events::PumpFunEvent::TradeV2(event) => {
                assert_eq!(
                    event,
                    pumpfun::events::TradeEventV2 {
                        mint: "2R5A2hvHqKUQE2sDpBQhPpCS5psiFFurPWfKazAnE8oX".parse().unwrap(),
                        sol_amount: 385_134_918,
                        token_amount: 5_060_809_487_872,
                        is_buy: true,
                        user: "56S29mZ3wqvw8hATuUUFqKhGcSGYFASRRFNT38W8q7G3".parse().unwrap(),
                        timestamp: 1_749_463_590,
                        virtual_sol_reserves: 49_687_442_151,
                        virtual_token_reserves: 647_849_813_676_445,
                        real_sol_reserves: 19_687_442_151,
                        real_token_reserves: 367_949_813_676_445,
                        fee_recipient: "CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM".parse().unwrap(),
                        fee_basis_points: 95,
                        fee: 3_658_782,
                        creator: "G1DUMJ8japRCHHxWkyVwsrzRJC4DSHCFD7NefPi6kZg6".parse().unwrap(),
                        creator_fee_basis_points: 5,
                        creator_fee: 192_568,
                    }
                );
            }
            _ => panic!("Expected a TradeEvent"),
        }
    }

    #[test]
    fn unpack_buy_instruction() {
        // https://solscan.io/tx/sK44CDg4qzi9jvTgA32dCTNh6Y3CgXki2kj9XtpaXRr83BipzpWPjnENzJR3TjLegAfDfPDG5Z8GZDkbrXDQk3w
        let bytes = hex!("66063d1201daebea0076c04f9a040000960d301700000000");

        match pumpfun::instructions::unpack(&bytes).expect("decode PumpFunInstruction") {
            pumpfun::instructions::PumpFunInstruction::Buy(event) => {
                assert_eq!(
                    event,
                    pumpfun::instructions::BuyInstruction {
                        amount: 5_060_809_487_872,
                        max_sol_cost: 389_025_174,
                    }
                );
            }
            _ => panic!("Expected a BuyInstruction"),
        }
    }

    #[test]
    fn unpack_create_instruction() {
        // https://solscan.io/tx/XzPzYUbKHHrG6gyKciKW2yHWqzjF1cbuiJzN6bwYQy3oQLwaFwc5ErKpMc1QHr9BLxuLzfqKaSosetgXgREyqc9
        let bytes = hex!("181ec828051c077704000000474f424f04000000474f424f5000000068747470733a2f2f697066732e696f2f697066732f6261666b7265696470357362746f346d76757472367463646b713574763262357a70336f727a706264646d767a3262786279747463336b6969326d77b855e4b353e62f466ee300057240ca099fc08b16e82dc80ab252b6d0c3b8bc");

        match pumpfun::instructions::unpack(&bytes).expect("decode PumpFunInstruction") {
            pumpfun::instructions::PumpFunInstruction::Create(event) => {
                assert_eq!(
                    event,
                    pumpfun::instructions::CreateInstruction {
                        name: "GOBO".to_string(),
                        symbol: "GOBO".to_string(),
                        uri: "https://ipfs.io/ipfs/bafkreidp5sbto4mvutr6tcdkq5tv2b5zp3orzpbddmvz2bxbyttc3kii2m".to_string(),
                        creator: "94LYWfthmeLjQ4ypgi9qjQNxeZdrZiL36i9RfDBLCFCX".parse().unwrap(),
                    }
                );
            }
            _ => panic!("Expected a CreateInstruction"),
        }
    }
}
