#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_solana_idls::pumpfun;

    #[test]
    fn unpack_trade() {
        // --------------------------------------------------------------------
        // raw Pump.fun Trade event (hex)
        // --------------------------------------------------------------------
        // https://solscan.io/tx/sK44CDg4qzi9jvTgA32dCTNh6Y3CgXki2kj9XtpaXRr83BipzpWPjnENzJR3TjLegAfDfPDG5Z8GZDkbrXDQk3w
        let bytes = hex!("e445a52e51cb9a1dbddb7fd34ee661ee1506533f60e64c4528916a7b404296ac72a1b1b65e817505f94b02b46d1969be46b1f416000000000076c04f9a040000013cd2f118afae78c0d0a91f649ff048b4a16749e82caac521286d90a6085cdd1026b2466800000000e7329a910b0000009d11034a374d0200e7867695040000009d79f0fda54e0100ad11e6a4fc2944a4fa8251bef815426e1bfb28c6b6646677607c6ad9f566a6465f000000000000001ed4370000000000deed67d04e125b1a2d7c6933fec6c7b08bce5763a576c88dcd36697d38298183050000000000000038f0020000000000").to_vec();

        // --------------------------------------------------------------------
        // decode and make sure we got the right variant
        // --------------------------------------------------------------------
        match pumpfun::logs::unpack(&bytes).expect("decode PumpfunLog") {
            pumpfun::logs::PumpFunEvent::Trade(trade) => {
                assert_eq!(trade.mint.to_string(), "2R5A2hvHqKUQE2sDpBQhPpCS5psiFFurPWfKazAnE8oX");
                assert_eq!(trade.sol_amount, 385_134_918);
                assert_eq!(trade.token_amount, 5_060_809_487_872);
                assert!(trade.is_buy);
                assert_eq!(trade.user.to_string(), "56S29mZ3wqvw8hATuUUFqKhGcSGYFASRRFNT38W8q7G3");
                assert_eq!(trade.timestamp, 1_749_463_590);
                assert_eq!(trade.virtual_sol_reserves, 49_687_442_151);
                assert_eq!(trade.virtual_token_reserves, 647_849_813_676_445);
                assert_eq!(trade.real_sol_reserves, 19_687_442_151);
                assert_eq!(trade.real_token_reserves, 367_949_813_676_445);
                assert_eq!(trade.fee_recipient.to_string(), "CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");
                assert_eq!(trade.fee_basis_points, 95);
                assert_eq!(trade.fee, 3_658_782);
                assert_eq!(trade.creator.to_string(), "G1DUMJ8japRCHHxWkyVwsrzRJC4DSHCFD7NefPi6kZg6");
                assert_eq!(trade.creator_fee_basis_points, 5);
                assert_eq!(trade.creator_fee, 192_568);
            }
            _ => panic!("Expected a Trade event"),
        }
    }
}
