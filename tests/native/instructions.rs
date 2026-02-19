use substreams_solana_idls::native::stake::instructions::{self as stake_ix, StakeInstruction};
use substreams_solana_idls::native::system::instructions::{self as system_ix, SystemInstruction};
use substreams_solana_idls::native::vote::instructions::{self as vote_ix, VoteInstruction};

// ---- System ----

#[test]
fn system_create_account() {
    let data = [0u8, 0, 0, 0, 1, 2, 3];
    let parsed = system_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, SystemInstruction::CreateAccount(_)));
}

#[test]
fn system_assign() {
    let data = [1u8, 0, 0, 0];
    let parsed = system_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, SystemInstruction::Assign(_)));
}

#[test]
fn system_transfer() {
    let data = [2u8, 0, 0, 0, 0xAA];
    let parsed = system_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, SystemInstruction::Transfer(_)));
}

#[test]
fn system_allocate() {
    let data = [8u8, 0, 0, 0];
    let parsed = system_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, SystemInstruction::Allocate(_)));
}

#[test]
fn system_transfer_with_seed() {
    let data = [11u8, 0, 0, 0, 0xFF];
    let parsed = system_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, SystemInstruction::TransferWithSeed(_)));
}

#[test]
fn system_too_short() {
    let data = [0u8, 0];
    assert!(system_ix::unpack(&data).is_err());
}

#[test]
fn system_unknown() {
    let data = [255u8, 0, 0, 0];
    assert!(system_ix::unpack(&data).is_err());
}

// ---- Stake ----

#[test]
fn stake_initialize() {
    let data = [0u8, 0, 0, 0, 1];
    let parsed = stake_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, StakeInstruction::Initialize(_)));
}

#[test]
fn stake_delegate() {
    let data = [2u8, 0, 0, 0];
    let parsed = stake_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, StakeInstruction::DelegateStake(_)));
}

#[test]
fn stake_withdraw() {
    let data = [4u8, 0, 0, 0, 0xBB];
    let parsed = stake_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, StakeInstruction::Withdraw(_)));
}

#[test]
fn stake_deactivate() {
    let data = [5u8, 0, 0, 0];
    let parsed = stake_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, StakeInstruction::Deactivate(_)));
}

#[test]
fn stake_set_lockup_checked() {
    let data = [12u8, 0, 0, 0];
    let parsed = stake_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, StakeInstruction::SetLockupChecked(_)));
}

#[test]
fn stake_too_short() {
    let data = [0u8];
    assert!(stake_ix::unpack(&data).is_err());
}

// ---- Vote ----

#[test]
fn vote_initialize() {
    let data = [0u8, 0, 0, 0, 1, 2];
    let parsed = vote_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, VoteInstruction::Initialize(_)));
}

#[test]
fn vote_vote() {
    let data = [2u8, 0, 0, 0, 0xCC];
    let parsed = vote_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, VoteInstruction::Vote(_)));
}

#[test]
fn vote_update_commission() {
    let data = [5u8, 0, 0, 0];
    let parsed = vote_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, VoteInstruction::UpdateCommission(_)));
}

#[test]
fn vote_tower_sync() {
    let data = [14u8, 0, 0, 0, 0xDD];
    let parsed = vote_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, VoteInstruction::TowerSync(_)));
}

#[test]
fn vote_tower_sync_switch() {
    let data = [15u8, 0, 0, 0];
    let parsed = vote_ix::unpack(&data).unwrap();
    assert!(matches!(parsed, VoteInstruction::TowerSyncSwitch(_)));
}

#[test]
fn vote_too_short() {
    let data = [0u8, 0, 0];
    assert!(vote_ix::unpack(&data).is_err());
}

#[test]
fn vote_unknown() {
    let data = [99u8, 0, 0, 0];
    assert!(vote_ix::unpack(&data).is_err());
}
