use substreams_solana::b58;

pub mod instructions;

/// GoonFi DEX program
///
/// https://solscan.io/account/goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j
pub const PROGRAM_ID: [u8; 32] = b58!("goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j");
