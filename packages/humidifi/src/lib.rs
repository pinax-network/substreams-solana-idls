#[macro_use]
extern crate common;
use substreams_solana::b58;

pub mod instructions;

/// HumidiFi program
///
/// https://solscan.io/account/9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp
pub const PROGRAM_ID: [u8; 32] = b58!("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp");
