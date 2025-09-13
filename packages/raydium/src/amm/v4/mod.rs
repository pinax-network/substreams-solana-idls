use substreams_solana::b58;
pub mod accounts;
pub mod instructions;
pub mod logs;

/// Raydium Liquidity Pool V4
///
/// https://solscan.io/account/675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
pub const PROGRAM_ID: [u8; 32] = b58!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
