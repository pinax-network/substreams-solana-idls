use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// SPL Token Lending Program
///
/// https://solscan.io/account/LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi
pub const PROGRAM_ID: [u8; 32] = b58!("LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi");
