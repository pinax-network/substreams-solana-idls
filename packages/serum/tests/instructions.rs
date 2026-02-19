use serum::instructions::{self, SerumInstruction};

#[test]
fn parse_initialize_market() {
    let mut data = vec![0u8, 0, 0, 0]; // disc=0 LE u32
    data.extend_from_slice(&[1, 2, 3]);
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::InitializeMarket(_)));
}

#[test]
fn parse_new_order() {
    let data = [1u8, 0, 0, 0, 0xAA];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::NewOrder(_)));
}

#[test]
fn parse_match_orders() {
    let data = [2u8, 0, 0, 0];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::MatchOrders(_)));
}

#[test]
fn parse_consume_events() {
    let data = [3u8, 0, 0, 0];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::ConsumeEvents(_)));
}

#[test]
fn parse_cancel_order() {
    let data = [4u8, 0, 0, 0, 1, 2];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::CancelOrder(_)));
}

#[test]
fn parse_settle_funds() {
    let data = [5u8, 0, 0, 0];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::SettleFunds(_)));
}

#[test]
fn parse_new_order_v3() {
    let data = [10u8, 0, 0, 0, 0xFF];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::NewOrderV3(_)));
}

#[test]
fn parse_cancel_order_v2() {
    let data = [11u8, 0, 0, 0, 1];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::CancelOrderV2(_)));
}

#[test]
fn parse_close_open_orders() {
    let data = [14u8, 0, 0, 0];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::CloseOpenOrders(_)));
}

#[test]
fn parse_consume_events_permissioned() {
    let data = [17u8, 0, 0, 0, 5];
    let parsed = instructions::unpack(&data).unwrap();
    assert!(matches!(parsed, SerumInstruction::ConsumeEventsPermissioned(_)));
}

#[test]
fn unknown_discriminator() {
    let data = [99u8, 0, 0, 0];
    assert!(instructions::unpack(&data).is_err());
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[0u8, 0]).is_err());
    assert!(instructions::unpack(&[]).is_err());
}
