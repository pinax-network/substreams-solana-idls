use substreams_solana::b58;
pub mod accounts;
pub mod events;
pub mod instructions;

/// PumpFun Bonding Curve
///
/// https://solscan.io/account/6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
pub const PROGRAM_ID: [u8; 32] = b58!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
