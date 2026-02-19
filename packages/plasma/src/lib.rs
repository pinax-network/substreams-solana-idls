#[macro_use]
extern crate common;
use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// Plasma DEX
///
/// https://solscan.io/account/srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi
pub const PROGRAM_ID: [u8; 32] = b58!("srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi");
