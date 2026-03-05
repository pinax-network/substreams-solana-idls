use solana_program::pubkey::Pubkey;
use substreams_solana_idls::pumpfun::bonding_curve::events::*;

#[test]
fn parse_create_event() {
    let outer_disc = [0u8; 8];
    let event = CreateEvent {
        name: "TestCoin".into(),
        symbol: "TC".into(),
        uri: "https://example.com".into(),
        mint: Pubkey::new_unique(),
        bonding_curve: Pubkey::new_unique(),
        user: Pubkey::new_unique(),
        creator: Pubkey::new_unique(),
        timestamp: 1700000000,
        virtual_token_reserves: 1_000_000_000,
        virtual_sol_reserves: 30_000_000_000,
        real_token_reserves: 793_100_000_000_000,
        token_total_supply: 1_000_000_000_000_000,
    };
    let mut data = outer_disc.to_vec();
    data.extend_from_slice(&CREATE);
    data.extend(borsh::to_vec(&event).unwrap());

    let parsed = unpack(&data).unwrap();
    assert!(matches!(parsed, PumpFunEvent::Create(e) if e.name == "TestCoin"));
}

#[test]
fn unknown_anchor_discriminator() {
    assert!(unpack(&[255u8; 24]).is_err());
}

#[test]
fn too_short() {
    assert!(unpack(&[0u8; 8]).is_err());
}
