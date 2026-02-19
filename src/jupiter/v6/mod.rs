use substreams_solana::b58;
pub mod accounts;
pub mod events;
pub mod instructions;

/// Jupiter Aggregator v6
///
/// https://solscan.io/account/JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4
pub const PROGRAM_ID: [u8; 32] = b58!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
