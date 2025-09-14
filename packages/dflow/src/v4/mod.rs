use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// DFlow Aggregator v4
///
/// https://solscan.io/account/DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH
pub const PROGRAM_ID: [u8; 32] = b58!("DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH");
