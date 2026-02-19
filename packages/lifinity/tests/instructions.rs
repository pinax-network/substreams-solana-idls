use lifinity::instructions;

#[test]
fn unknown_discriminator() {
    assert!(instructions::unpack(&[0u8; 16]).is_err());
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[0u8; 4]).is_err());
}

#[test]
fn decode_swap() {
    // SWAP discriminator + 2x u64
    let mut data = vec![248, 198, 158, 145, 225, 117, 135, 200];
    data.extend_from_slice(&1000u64.to_le_bytes());
    data.extend_from_slice(&500u64.to_le_bytes());
    let ix = instructions::unpack(&data).unwrap();
    match ix {
        instructions::LifinityInstruction::Swap(s) => {
            assert_eq!(s.amount_in, 1000);
            assert_eq!(s.minimum_amount_out, 500);
        }
        _ => panic!("expected Swap"),
    }
}
