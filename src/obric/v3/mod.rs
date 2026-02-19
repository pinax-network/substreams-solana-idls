use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// Obric V3
///
/// https://solscan.io/account/ob2wXVFGiWXkPwYv8CSpPMm5m3NR7DjSrVJAHGSb9Gu
pub const PROGRAM_ID: [u8; 32] = b58!("ob2wXVFGiWXkPwYv8CSpPMm5m3NR7DjSrVJAHGSb9Gu");
