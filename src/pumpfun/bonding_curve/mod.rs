use substreams_solana::b58;
pub mod anchor_self_cpi;
pub mod instructions;

// -----------------------------------------------------------------------------
// Program ID
// -----------------------------------------------------------------------------
pub const PROGRAM_ID: [u8; 32] = b58!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
