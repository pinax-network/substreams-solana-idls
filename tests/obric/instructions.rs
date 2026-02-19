use substreams_solana_idls::obric::v2;
use substreams_solana_idls::obric::v3;

#[test]
fn v2_unknown_discriminator() {
    assert!(v2::instructions::unpack(&[0u8; 16]).is_err());
}

#[test]
fn v2_too_short() {
    assert!(v2::instructions::unpack(&[0u8; 4]).is_err());
}

#[test]
fn v2_decode_swap_x_to_y() {
    let mut data = vec![226, 74, 41, 166, 87, 155, 41, 75];
    data.extend_from_slice(&1000u64.to_le_bytes());
    data.extend_from_slice(&500u64.to_le_bytes());
    let ix = v2::instructions::unpack(&data).unwrap();
    match ix {
        v2::instructions::ObricV2Instruction::SwapXToY(s) => {
            assert_eq!(s.input_x, 1000);
            assert_eq!(s.min_output_amt, 500);
        }
        _ => panic!("expected SwapXToY"),
    }
}

#[test]
fn v3_unknown_discriminator() {
    assert!(v3::instructions::unpack(&[0u8; 16]).is_err());
}

#[test]
fn v3_too_short() {
    assert!(v3::instructions::unpack(&[0u8; 4]).is_err());
}

#[test]
fn v3_decode_swap_x_to_y() {
    let mut data = vec![226, 74, 41, 166, 87, 155, 41, 75];
    data.extend_from_slice(&2000u64.to_le_bytes());
    data.extend_from_slice(&1000u64.to_le_bytes());
    let ix = v3::instructions::unpack(&data).unwrap();
    match ix {
        v3::instructions::ObricV3Instruction::SwapXToY(s) => {
            assert_eq!(s.input_x, 2000);
            assert_eq!(s.min_output_amt, 1000);
        }
        _ => panic!("expected SwapXToY"),
    }
}
