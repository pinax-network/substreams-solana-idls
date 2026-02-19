use openbook::instructions;

#[test]
fn decode_deposit() {
    // DEPOSIT discriminator + base_amount(100) + quote_amount(200)
    let mut data = vec![];
    data.extend_from_slice(&instructions::DEPOSIT);
    data.extend_from_slice(&100u64.to_le_bytes());
    data.extend_from_slice(&200u64.to_le_bytes());

    let ix = instructions::unpack(&data).unwrap();
    match ix {
        instructions::OpenbookInstruction::Deposit(d) => {
            assert_eq!(d.base_amount, 100);
            assert_eq!(d.quote_amount, 200);
        }
        other => panic!("expected Deposit, got {:?}", other),
    }
}

#[test]
fn decode_sweep_fees() {
    let mut data = vec![];
    data.extend_from_slice(&instructions::SWEEP_FEES);
    let ix = instructions::unpack(&data).unwrap();
    assert!(matches!(ix, instructions::OpenbookInstruction::SweepFees));
}

#[test]
fn unknown_discriminator() {
    assert!(instructions::unpack(&[0u8; 16]).is_err());
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[0u8; 4]).is_err());
}
