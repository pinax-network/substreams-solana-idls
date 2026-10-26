use borsh::BorshSerialize;
use substreams_solana_idls::pumpfun::amm::{
    events::{self, PumpFunAmmEvent},
    instructions::{self, PumpFunAmmInstruction},
};

const CURRENT_INSTRUCTION: &[u8] = include_bytes!("fixtures/create-pool-current.instruction.bin");
const CURRENT_EVENT: &[u8] = include_bytes!("fixtures/create-pool-current.event.bin");
const V2_INSTRUCTION: &[u8] = include_bytes!("fixtures/create-pool-v2.instruction.bin");
const V2_EVENT: &[u8] = include_bytes!("fixtures/create-pool-v2.event.bin");

#[test]
fn captured_current_creation_preserves_every_official_suffix_field() {
    let PumpFunAmmInstruction::CreatePoolV3(init) = instructions::unpack(CURRENT_INSTRUCTION).unwrap() else {
        panic!("wrong current instruction variant")
    };
    let PumpFunAmmEvent::CreatePoolEventV3(event) = events::unpack(CURRENT_EVENT).unwrap() else {
        panic!("wrong current event variant")
    };
    assert_eq!(CURRENT_INSTRUCTION.len(), 70);
    assert_eq!(CURRENT_EVENT.len(), 352);
    assert_eq!(init.base.index, event.base.index);
    assert_eq!(init.base.base_amount_in, event.base.base_amount_in);
    assert_eq!(init.base.quote_amount_in, event.base.quote_amount_in);
    assert_eq!(init.base.coin_creator, event.base.coin_creator);
    assert_eq!(init.creator_fee_bps.0, 100);
    assert_eq!(event.creator_fee_bps, 100);
    assert!(!init.is_mayhem_mode && !event.is_mayhem_mode);
    assert!(!init.is_cashback_coin.0 && !init.can_edit_creator_fee.0 && !event.can_edit_creator_fee);
    assert!(init.is_holder_reward.0 && event.is_holder_reward);
    assert_ne!(event.base.pool.to_bytes(), [0; 32]);
    assert_ne!(event.base.base_mint, event.base.quote_mint);
    // Exact payload roundtrip checks all old and new fields, not just suffixes.
    assert_eq!(borsh::to_vec(&init).unwrap(), CURRENT_INSTRUCTION[8..]);
    assert_eq!(borsh::to_vec(&event).unwrap(), CURRENT_EVENT[16..]);
}

#[test]
fn captured_v2_creation_retains_existing_types_and_payloads() {
    let PumpFunAmmInstruction::CreatePoolV2(init) = instructions::unpack(V2_INSTRUCTION).unwrap() else {
        panic!("wrong historical instruction variant")
    };
    let PumpFunAmmEvent::CreatePoolEventV2(event) = events::unpack(V2_EVENT).unwrap() else {
        panic!("wrong historical event variant")
    };
    assert_eq!(V2_INSTRUCTION.len(), 58);
    assert_eq!(V2_EVENT.len(), 341);
    assert_eq!(init.coin_creator, event.coin_creator);
    assert_eq!(borsh::to_vec(&init).unwrap(), V2_INSTRUCTION[8..]);
    assert_eq!(borsh::to_vec(&event).unwrap(), V2_EVENT[16..]);
    // Public instruction enum's pre-existing Borsh discriminants do not shift.
    assert_eq!(borsh::to_vec(&PumpFunAmmInstruction::Unknown).unwrap(), vec![12]);
    assert_eq!(borsh::to_vec(&PumpFunAmmInstruction::CreatePoolV2(init)).unwrap()[0], 4);
}

#[test]
fn only_complete_supported_creation_lengths_are_admitted() {
    for length in 0..CURRENT_INSTRUCTION.len() {
        if length == 58 {
            continue;
        } // Exact V2 prefix is intentionally supported.
        assert!(instructions::unpack(&CURRENT_INSTRUCTION[..length]).is_err(), "instruction length {length}");
    }
    for length in 0..CURRENT_EVENT.len() {
        if length == 341 {
            continue;
        }
        assert!(events::unpack(&CURRENT_EVENT[..length]).is_err(), "event length {length}");
    }
    for extra in [1, 2, 8, 12] {
        let mut init = CURRENT_INSTRUCTION.to_vec();
        init.extend(vec![0; extra]);
        assert!(instructions::unpack(&init).is_err());
        let mut event = CURRENT_EVENT.to_vec();
        event.extend(vec![0; extra]);
        assert!(events::unpack(&event).is_err());
    }
    // V1's 18-byte argument payload still follows the previous rejection policy.
    assert!(instructions::unpack(&V2_INSTRUCTION[..26]).is_err());
    assert!(events::unpack(&V2_EVENT[..309]).is_err());
}

#[test]
fn malformed_bool_bytes_and_discriminators_are_rejected() {
    for index in [58, 59, 68, 69] {
        let mut init = CURRENT_INSTRUCTION.to_vec();
        init[index] = 2;
        assert!(instructions::unpack(&init).is_err(), "init bool {index}");
    }
    for index in [341, 350, 351] {
        let mut event = CURRENT_EVENT.to_vec();
        event[index] = 2;
        assert!(events::unpack(&event).is_err(), "event bool {index}");
    }
    let mut init = CURRENT_INSTRUCTION.to_vec();
    init[0] ^= 255;
    assert!(instructions::unpack(&init).is_err());
    for index in [0, 8] {
        let mut event = CURRENT_EVENT.to_vec();
        event[index] ^= 255;
        assert!(events::unpack(&event).is_err());
    }
}

#[test]
fn official_option_wrappers_have_no_borsh_option_tag() {
    assert_eq!(borsh::to_vec(&instructions::OptionBool(true)).unwrap(), vec![1]);
    let mut bytes = Vec::new();
    instructions::OptionU64(123).serialize(&mut bytes).unwrap();
    assert_eq!(bytes, 123u64.to_le_bytes());
}
