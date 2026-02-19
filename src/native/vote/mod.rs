use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// Vote Program
///
/// https://solscan.io/account/Vote111111111111111111111111111111111111111
pub const PROGRAM_ID: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");
