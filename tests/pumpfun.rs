#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams_idls::pumpfun;

    #[test]
    fn parse_trade_log() {
        // --------------------------------------------------------------------
        // raw Pump.fun Trade event (hex)
        // --------------------------------------------------------------------
        // https://solscan.io/tx/sK44CDg4qzi9jvTgA32dCTNh6Y3CgXki2kj9XtpaXRr83BipzpWPjnENzJR3TjLegAfDfPDG5Z8GZDkbrXDQk3w
        let bytes = hex!("e445a52e51cb9a1dbddb7fd34ee661ee32e48e8b6ad77786c4e23fb007a1fb63f64412846ee10800244665850d8f918fecd4eb020000000052ee26d37800000001ed09a7fd83eda97b39a03f1bed6c3629d4bf3bd2c4a26f26c90fccb41e3c61db26b2466800000000484fe4d70c0000002592f509be12020048a3c0db0500000025fae2bd2c140100e004c87ceb98fa5ce47f803806fd2c7945d29524959aec00ded97814f38f78465f00000000000000bb1a0700000000005bc9df4aa79d20a724add3dae08e33ed59fb304bf1033a7f0e50777f5b87f86c0500000000000000b95f000000000000");

        // --------------------------------------------------------------------
        // decode and make sure we got the right variant
        // --------------------------------------------------------------------
        let pump_log = pumpfun::logs::parse_event(&bytes).expect("decode PumpfunLog");
        let trade = match pump_log {
            pumpfun::logs::PumpFunEvent::Trade(trade) => trade,
            _ => panic!("Expected a Trade event"),
        };

        // --------------------------------------------------------------------
        // assertions (values taken from the JSON you supplied)
        // --------------------------------------------------------------------
        assert_eq!(trade.mint.to_string(), "4RfXFyiDSGvKukvz5yYFZ6qAD8MrvXrcvyW11xSfpump");
        assert_eq!(trade.sol_amount, 49_009_900);
        assert_eq!(trade.token_amount, 518_938_619_474);
        assert!(trade.is_buy);
        assert_eq!(trade.user.to_string(), "GxJAXx1tgzUYLBMW47PZcibHJKh4nHcrrnJry1FQ9KSz");
        assert_eq!(trade.timestamp, 1_749_463_590);
        assert_eq!(trade.virtual_sol_reserves, 55_161_671_496);
        assert_eq!(trade.virtual_token_reserves, 583_557_373_596_197);
        assert_eq!(trade.real_sol_reserves, 25_161_671_496);
        assert_eq!(trade.real_token_reserves, 303_657_373_596_197);
        assert_eq!(trade.fee_recipient.to_string(), "G5UZAVbAf46s7cKWoyKu8kYTip9DGTpbLZ2qa9Aq69dP");
        assert_eq!(trade.fee_basis_points, 95);
        assert_eq!(trade.fee, 465_595);
        assert_eq!(trade.creator.to_string(), "7BJdxxFVo7jJXztNWjQfjdHQafXFJJxRa7AuMtXjowGX");
        assert_eq!(trade.creator_fee_basis_points, 5);
        assert_eq!(trade.creator_fee, 24_505);
    }
}
