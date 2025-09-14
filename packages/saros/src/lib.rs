extern crate common;
use substreams_solana::b58;

pub mod instructions;

/// Saros swap program
///
/// https://solscan.io/account/SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr
pub const PROGRAM_ID: [u8; 32] = b58!("SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr");
