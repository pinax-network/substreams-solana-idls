use substreams_solana::b58;
pub mod accounts;
pub mod events;
pub mod instructions;
pub mod logs;

/// PumpFun AMM program
///
/// https://solscan.io/account/pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
pub const PROGRAM_ID: [u8; 32] = b58!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");
