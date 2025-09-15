use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Byreal CLMM program
///
/// https://solscan.io/account/REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2
pub const PROGRAM_ID: [u8; 32] = b58!("REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2");
