use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Darklake - Dark pool DEX with confidential trading
///
/// https://solscan.io/account/darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1
pub const PROGRAM_ID: [u8; 32] = b58!("darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1");
