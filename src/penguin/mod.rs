use substreams_solana::b58;

/// Penguin Finance (PSwap) program — an SPL Token Swap fork.
///
/// Re-exports instruction and account types from [`crate::spl::token_swap`].
///
/// https://solscan.io/account/PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP
pub const PROGRAM_ID: [u8; 32] = b58!("PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP");

pub use crate::spl::token_swap::accounts;
pub use crate::spl::token_swap::instructions;
