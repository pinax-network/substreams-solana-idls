#[macro_use]
extern crate common;
use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Moonshot token launchpad program
///
/// https://solscan.io/account/MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG
pub const PROGRAM_ID: [u8; 32] = b58!("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG");
