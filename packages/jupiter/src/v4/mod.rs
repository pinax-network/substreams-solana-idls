use substreams_solana::b58;
pub mod accounts;
pub mod events;
pub mod instructions;

/// Jupiter Aggregator v4
///
/// https://solscan.io/account/JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB
pub const PROGRAM_ID: [u8; 32] = b58!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");
