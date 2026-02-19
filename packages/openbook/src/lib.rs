#[macro_use]
extern crate common;
use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Openbook V2 program
///
/// https://solscan.io/account/opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb
pub const PROGRAM_ID: [u8; 32] = b58!("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb");
