use substreams_solana_macro::b58;

pub mod accounts;
pub mod instructions;

/// Aldrin AMM V2. Legacy CLOB-shaped decoder APIs remain for compatibility.
///
/// https://solscan.io/account/CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4
pub const PROGRAM_ID: [u8; 32] = b58!("CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4");
