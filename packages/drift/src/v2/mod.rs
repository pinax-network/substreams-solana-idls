use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Drift v2 program
///
/// https://solscan.io/account/dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH
pub const PROGRAM_ID: [u8; 32] = b58!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");
