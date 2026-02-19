use pumpfun::amm::instructions::*;

#[test]
fn parse_amm_buy() {
    let disc: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
    let buy = BuyInstruction { base_amount_out: 5_000_000, max_quote_amount_in: 1_000_000 };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&buy).unwrap());
    assert!(matches!(unpack(&data).unwrap(), PumpFunAmmInstruction::Buy(b) if b.base_amount_out == 5_000_000));
}

#[test]
fn parse_amm_sell() {
    let disc: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
    let sell = SellInstruction { base_amount_in: 3_000_000, min_quote_amount_out: 100_000 };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&sell).unwrap());
    assert!(matches!(unpack(&data).unwrap(), PumpFunAmmInstruction::Sell(s) if s.base_amount_in == 3_000_000));
}

#[test]
fn amm_too_short() { assert!(unpack(&[0u8; 4]).is_err()); }

#[test]
fn amm_unknown() { assert!(unpack(&[0u8; 16]).is_err()); }
