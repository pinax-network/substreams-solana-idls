use substreams_solana::b58;

pub mod events;
pub mod instructions;

/// Solfi V2 program
///
/// https://solscan.io/account/SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF
pub const PROGRAM_ID: [u8; 32] = b58!("SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF");
