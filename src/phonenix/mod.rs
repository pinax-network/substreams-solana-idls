use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Phonenix program
///
/// https://solscan.io/account/PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY
pub const PROGRAM_ID: [u8; 32] = b58!("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");
