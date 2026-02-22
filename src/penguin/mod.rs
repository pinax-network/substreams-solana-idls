use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// Penguin Finance (PSwap) program
///
/// https://solscan.io/account/PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP
pub const PROGRAM_ID: [u8; 32] = b58!("PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP");
