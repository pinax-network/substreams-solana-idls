use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// Obric V2
///
/// https://solscan.io/account/coUnmi3oBUtwtd9fU42p6jg75UmdnNMgBQMedGNYEAs
pub const PROGRAM_ID: [u8; 32] = b58!("coUnmi3oBUtwtd9fU42p6jg75UmdnNMgBQMedGNYEAs");
