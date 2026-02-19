extern crate common;

pub mod accounts;
pub mod events;
pub mod instructions;

use substreams_solana::b58;

/// Marinade Finance liquid staking program
///
/// https://solscan.io/account/MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD
pub const PROGRAM_ID: [u8; 32] = b58!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");
