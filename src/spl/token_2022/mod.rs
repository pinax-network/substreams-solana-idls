use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// SPL Token 2022 Program
///
/// https://solscan.io/account/TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
pub const PROGRAM_ID: [u8; 32] = b58!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
