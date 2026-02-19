use tensor::instructions::*;

#[test]
fn parse_buy() {
    let instr = BuyInstruction { max_amount: 5_000_000_000 };
    let mut data = BUY.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(
        unpack(&data).unwrap(),
        TensorInstruction::Buy(b) if b.max_amount == 5_000_000_000
    ));
}

#[test]
fn parse_delist() {
    assert!(matches!(unpack(&DELIST).unwrap(), TensorInstruction::Delist));
}

#[test]
fn parse_cancel_bid() {
    assert!(matches!(unpack(&CANCEL_BID).unwrap(), TensorInstruction::CancelBid));
}

#[test]
fn parse_buy_legacy() {
    let instr = BuyInstruction { max_amount: 1_000_000_000 };
    let mut data = BUY_LEGACY.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(
        unpack(&data).unwrap(),
        TensorInstruction::BuyLegacy(b) if b.max_amount == 1_000_000_000
    ));
}

#[test]
fn parse_list_core() {
    let instr = ListInstruction {
        amount: 2_000_000_000,
        expire_in_sec: Some(86400),
        currency: None,
        private_taker: None,
        maker_broker: None,
    };
    let mut data = LIST_CORE.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(
        unpack(&data).unwrap(),
        TensorInstruction::ListCore(l) if l.amount == 2_000_000_000
    ));
}

#[test]
fn unknown_discriminator() {
    assert!(unpack(&[0u8; 16]).is_err());
}

#[test]
fn too_short() {
    assert!(unpack(&[0u8; 4]).is_err());
}
