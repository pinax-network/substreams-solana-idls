extern crate common;
use substreams_solana::b58;

pub mod instructions;
pub mod logs;

/// Heaven DEX program
///
/// https://solscan.io/account/HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o
pub const PROGRAM_ID: [u8; 32] = b58!("HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o");
