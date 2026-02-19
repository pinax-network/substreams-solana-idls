use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// Metaplex Bubblegum (compressed NFTs)
///
/// https://solscan.io/account/BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY
pub const PROGRAM_ID: [u8; 32] = b58!("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY");
