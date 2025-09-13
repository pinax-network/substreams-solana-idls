extern crate idls_common;
use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Stabble weighted swap program
///
/// https://solscan.io/account/swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW
pub const PROGRAM_ID: [u8; 32] = b58!("swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW");
