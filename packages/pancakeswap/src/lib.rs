extern crate common;
use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// PancakeSwap concentrated liquidity AMM program
///
/// https://solscan.io/account/HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq
pub const PROGRAM_ID: [u8; 32] = b58!("HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq");
