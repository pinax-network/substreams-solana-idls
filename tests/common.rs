use substreams_solana_idls::common::ParseError;

#[test]
fn too_short_display_is_generic() {
    assert_eq!(ParseError::TooShort(5).to_string(), "payload too short: got 5 bytes");
}
