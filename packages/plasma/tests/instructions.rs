use plasma::instructions;

#[test]
fn unknown_discriminator() {
    assert!(instructions::unpack(&[255u8, 0, 0, 0]).is_err());
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[]).is_err());
}

#[test]
fn decode_swap() {
    // discriminator 0 (Swap) + Side::Buy (0) + SwapType::ExactIn (0) + amount_in + min_amount_out
    let mut data = vec![0u8]; // Swap discriminant
    data.push(0); // Side::Buy
    data.push(0); // SwapType::ExactIn
    data.extend_from_slice(&1000u64.to_le_bytes()); // amount_in
    data.extend_from_slice(&500u64.to_le_bytes()); // min_amount_out
    let ix = instructions::unpack(&data).unwrap();
    match ix {
        instructions::PlasmaInstruction::Swap(s) => {
            assert_eq!(s.side, instructions::Side::Buy);
            assert_eq!(
                s.swap_type,
                instructions::SwapType::ExactIn {
                    amount_in: 1000,
                    min_amount_out: 500
                }
            );
        }
        _ => panic!("expected Swap"),
    }
}
