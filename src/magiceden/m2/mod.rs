use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// Magic Eden M2 NFT Marketplace
///
/// https://solscan.io/account/M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K
pub const PROGRAM_ID: [u8; 32] = b58!("M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K");
