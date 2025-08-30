use substreams_solana::b58;

pub mod instructions;

/// Raydium Concentrated Pool Market Maker program
///
/// https://solscan.io/account/CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
pub const PROGRAM_ID: [u8; 32] = b58!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
