use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// SPL Token Program
///
/// https://solscan.io/account/TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
pub const PROGRAM_ID: [u8; 32] = b58!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
