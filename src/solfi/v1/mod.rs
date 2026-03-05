use substreams_solana::b58;

pub mod events;
pub mod instructions;

/// Solfi V1 program
///
/// https://solscan.io/account/SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe
pub const PROGRAM_ID: [u8; 32] = b58!("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe");
