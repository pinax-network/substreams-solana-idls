use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Jupiter DCA
///
/// https://solscan.io/account/DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M
pub const PROGRAM_ID: [u8; 32] = b58!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M");
