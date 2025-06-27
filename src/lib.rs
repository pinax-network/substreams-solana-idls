pub mod jupiter;
pub mod pumpfun;

use thiserror::Error;

// -----------------------------------------------------------------------------
// Error handling
// -----------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("payload too short: {0} bytes (need at least 16)")]
    TooShort(usize),
    #[error("unknown discriminator {0:?}")]
    Unknown([u8; 8]),
    #[error("invalid payload length: expected {expected} bytes, got {got}")]
    InvalidLength { expected: usize, got: usize },
    #[error("Borsh decode error: {0}")]
    Decode(#[from] borsh::io::Error),
}
