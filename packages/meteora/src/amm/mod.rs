use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Meteora Pools Program
///
/// https://solscan.io/account/Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB
pub const PROGRAM_ID: [u8; 32] = b58!("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB");
