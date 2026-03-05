use substreams_solana::b58;
pub mod accounts;
pub mod instructions;

/// Magic Eden M3/MMM NFT AMM
///
/// https://solscan.io/account/mmm3XBJg5gk8XJxEKBvdgptZz6SgK4tXvn36sodowMc
pub const PROGRAM_ID: [u8; 32] = b58!("mmm3XBJg5gk8XJxEKBvdgptZz6SgK4tXvn36sodowMc");
