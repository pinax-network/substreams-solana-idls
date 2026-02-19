use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// DumpFun token launcher
///
/// https://solscan.io/account/DumpFunGAgW6kPHzWMA3Nnqecyrd6SGnLZvNGp2aHwEa
pub const PROGRAM_ID: [u8; 32] = b58!("DumpFunGAgW6kPHzWMA3Nnqecyrd6SGnLZvNGp2aHwEa");
