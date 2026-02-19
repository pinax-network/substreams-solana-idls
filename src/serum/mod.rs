use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// Serum DEX V3 program
///
/// https://solscan.io/account/9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin
pub const PROGRAM_ID: [u8; 32] = b58!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
