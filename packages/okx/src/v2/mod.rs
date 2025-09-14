use substreams_solana::b58;

pub mod instructions;

/// OKX Dex program
///
/// https://solscan.io/account/6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma
pub const PROGRAM_ID: [u8; 32] = b58!("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma");
