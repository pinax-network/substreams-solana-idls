pub mod instructions;

use substreams_solana::b58;

/// Sanctum Router program — LST swap routing via stake accounts
///
/// https://solscan.io/account/5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx
pub const PROGRAM_ID: [u8; 32] = b58!("5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx");
