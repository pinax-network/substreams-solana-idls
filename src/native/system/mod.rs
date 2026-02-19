use substreams_solana::b58;
pub mod instructions;

/// System Program
///
/// https://solscan.io/account/11111111111111111111111111111111
pub const PROGRAM_ID: [u8; 32] = b58!("11111111111111111111111111111111");
