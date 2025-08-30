use substreams_solana::b58;

pub mod anchor_self_cpi;
pub mod instructions;
pub mod logs;

/// Raydium Launchpad program
///
/// https://solscan.io/account/LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
pub const PROGRAM_ID: [u8; 32] = b58!("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");
