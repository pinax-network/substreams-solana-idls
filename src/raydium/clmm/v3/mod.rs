use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Raydium CLMM v3 program
///
/// https://solscan.io/account/CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK
pub const PROGRAM_ID: [u8; 32] = b58!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
