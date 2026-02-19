use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Raydium Stable AMM program
///
/// https://solscan.io/account/5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h
pub const PROGRAM_ID: [u8; 32] = b58!("5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h");
