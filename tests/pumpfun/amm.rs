use substreams_solana_idls::pumpfun::amm::instructions::*;

#[test]
fn parse_amm_buy() {
    let disc: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
    let buy = BuyInstruction {
        base_amount_out: 5_000_000,
        max_quote_amount_in: 1_000_000,
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&buy).unwrap());
    assert!(matches!(unpack(&data).unwrap(), PumpFunAmmInstruction::Buy(b) if b.base_amount_out == 5_000_000));
}

#[test]
fn parse_amm_buy_exact_quote_in() {
    // Real on-chain bytes from a Jupiter-routed PumpSwap call: tx
    // 61heGfiqm9enY7MEfqjvmADrnRyD1oMUjAdMTUa8u1iXMKz8ecgBXEgX2xnoZuUor6dyJ8ni8KbNxBrY37NgWPdo
    // (pumpfun amm program pAMMBay6...). Discriminator c62e1552b4d9e870
    // followed by spendable_quote_in=11904075 and min_base_amount_out=1.
    let bytes = substreams::hex!("c62e1552b4d9e8704ba4b500000000000100000000000000");
    match unpack(&bytes).expect("decode buy_exact_quote_in") {
        PumpFunAmmInstruction::BuyExactQuoteIn(b) => {
            assert_eq!(b.spendable_quote_in, 11_904_075);
            assert_eq!(b.min_base_amount_out, 1);
        }
        other => panic!("expected BuyExactQuoteIn, got {:?}", other),
    }
}

#[test]
fn parse_amm_buy_exact_quote_in_roundtrip() {
    let disc: [u8; 8] = [198, 46, 21, 82, 180, 217, 232, 112];
    let ix = BuyExactQuoteInInstruction {
        spendable_quote_in: 7_500_000,
        min_base_amount_out: 250_000,
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&ix).unwrap());
    assert!(matches!(
        unpack(&data).unwrap(),
        PumpFunAmmInstruction::BuyExactQuoteIn(b) if b.spendable_quote_in == 7_500_000 && b.min_base_amount_out == 250_000,
    ));
}

#[test]
fn parse_amm_sell() {
    let disc: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
    let sell = SellInstruction {
        base_amount_in: 3_000_000,
        min_quote_amount_out: 100_000,
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&sell).unwrap());
    assert!(matches!(unpack(&data).unwrap(), PumpFunAmmInstruction::Sell(s) if s.base_amount_in == 3_000_000));
}

#[test]
fn amm_too_short() {
    assert!(unpack(&[0u8; 4]).is_err());
}

#[test]
fn amm_unknown() {
    assert!(unpack(&[0u8; 16]).is_err());
}
