pub mod events;
pub mod instructions;

use substreams_solana::b58;

/// Tensor Marketplace program (cNFT, pNFT, Token-2022, WNS, Core)
///
/// https://solscan.io/account/TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp
pub const PROGRAM_ID: [u8; 32] = b58!("TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp");
