use substreams_solana::b58;

pub mod accounts;
pub mod instructions;

/// Aldrin CLOB DEX
///
/// https://solscan.io/account/CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4
pub const PROGRAM_ID: [u8; 32] = b58!("CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4");
