use humidifi::instructions::*;
use solana_program::pubkey::Pubkey;

#[test]
fn parse_mystery() {
    let instr = MysteryInstruction {
        unknown_pubkey: Pubkey::new_unique(),
        field1: 100, field2: 200, field3: 300, flag: 1,
    };
    let mut data = MYSTERY_INSTRUCTION.to_vec();
    data.extend(borsh::to_vec(&instr).unwrap());
    assert!(matches!(unpack(&data).unwrap(), HumidiFiInstruction::Mystery(m) if m.field1 == 100));
}

#[test]
fn unknown() { assert!(unpack(&[0u8; 16]).is_err()); }
#[test]
fn too_short() { assert!(unpack(&[0u8; 4]).is_err()); }
