use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// Lifinity AMM V2
///
/// https://solscan.io/account/2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c
pub const PROGRAM_ID: [u8; 32] = b58!("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c");
