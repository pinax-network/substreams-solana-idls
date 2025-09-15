use substreams_solana::b58;

pub mod accounts;
pub mod anchor_cpi_event;
pub mod instructions;

/// Raydium Launchpad program
///
/// https://solscan.io/account/LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
pub const PROGRAM_ID: [u8; 32] = b58!("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");
