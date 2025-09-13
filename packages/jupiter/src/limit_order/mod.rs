use substreams_solana::b58;
pub mod accounts;
pub mod events;
pub mod instructions;

/// Jupiter Limit Order
///
/// https://solscan.io/account/jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu
pub const PROGRAM_ID: [u8; 32] = b58!("jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu");
