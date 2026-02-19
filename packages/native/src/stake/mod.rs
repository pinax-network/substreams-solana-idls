use substreams_solana::b58;
pub mod instructions;

/// Stake Program
///
/// https://solscan.io/account/Stake11111111111111111111111111111111111111
pub const PROGRAM_ID: [u8; 32] = b58!("Stake11111111111111111111111111111111111111");
