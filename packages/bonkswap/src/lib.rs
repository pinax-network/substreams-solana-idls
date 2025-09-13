#[macro_use]
extern crate idls_common;
use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// BonkSwap AMM program
///
/// https://solscan.io/account/BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p
pub const PROGRAM_ID: [u8; 32] = b58!("BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p");
