use aldrin::instructions;

#[test]
fn unknown_discriminator() {
    assert!(instructions::unpack(&[0u8; 16]).is_err());
}

#[test]
fn too_short() {
    assert!(instructions::unpack(&[0u8; 4]).is_err());
}
