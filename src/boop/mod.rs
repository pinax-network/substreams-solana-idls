use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Boop token launcher with fee sharing
///
/// https://solscan.io/account/boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4
pub const PROGRAM_ID: [u8; 32] = b58!("boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4");
