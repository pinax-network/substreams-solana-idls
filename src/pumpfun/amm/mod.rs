use substreams_solana::b58;
pub mod anchor_self_cpi;
pub mod instructions;
pub mod logs;

// -----------------------------------------------------------------------------
// Program ID
// -----------------------------------------------------------------------------
pub const PROGRAM_ID: [u8; 32] = b58!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");
