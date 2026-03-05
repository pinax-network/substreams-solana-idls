use substreams_solana::b58;

pub mod accounts;
pub mod anchor_cpi_event;
pub mod instructions;

/// Meteora DAMM v2
///
/// https://solscan.io/account/cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG
pub const PROGRAM_ID: [u8; 32] = b58!("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG");
