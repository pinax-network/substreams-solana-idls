use substreams_solana::b58;

pub mod accounts;
pub mod events;
pub mod instructions;

/// Meteora DLMM Program
///
/// https://solscan.io/account/LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo
pub const PROGRAM_ID: [u8; 32] = b58!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");
