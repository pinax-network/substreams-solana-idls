use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// SPL Token Swap Program
///
/// https://solscan.io/account/SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8
pub const PROGRAM_ID: [u8; 32] = b58!("SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8");
