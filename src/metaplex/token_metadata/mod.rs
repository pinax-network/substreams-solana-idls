use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// Metaplex Token Metadata Program
///
/// https://solscan.io/account/metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s
pub const PROGRAM_ID: [u8; 32] = b58!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
