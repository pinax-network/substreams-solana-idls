use heaven::instructions::*;
use heaven::logs;
use solana_program::pubkey::Pubkey;

#[test]
fn parse_sell() {
    let disc: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
    let instr = SellInstruction {
        amount_in: 1_000_000,
        minimum_amount_out: 500_000,
        encoded_user_defined_event_data: "test".into(),
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), HeavenInstruction::Sell(s) if s.amount_in == 1_000_000));
}

#[test]
fn unknown() {
    assert!(unpack(&[0u8; 16]).is_err());
}
#[test]
fn too_short() {
    assert!(unpack(&[0u8; 4]).is_err());
}

#[test]
fn parse_sell_log() {
    let disc: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
    let log = logs::SellLog {
        user: Pubkey::new_unique(),
        mint: Pubkey::new_unique(),
        amount: 42,
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&log).unwrap());
    assert!(matches!(logs::unpack(&data).unwrap(), logs::HeavenLog::Sell(l) if l.amount == 42));
}

#[test]
fn log_unknown() {
    assert!(logs::unpack(&[0u8; 16]).is_err());
}
