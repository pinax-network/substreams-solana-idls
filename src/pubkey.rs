use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use substreams_solana::base58;

/// Number of bytes in a pubkey
pub const PUBKEY_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Pubkey(pub(crate) [u8; 32]);

impl Pubkey {
    pub fn to_bytes(self) -> [u8; 32] {
        self.0
    }
    pub fn to_string(&self) -> String {
        base58::encode(self.0).to_string()
    }
    pub fn default() -> Self {
        Pubkey([0u8; 32])
    }
}

impl AsRef<[u8]> for Pubkey {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl From<[u8; 32]> for Pubkey {
    #[inline]
    fn from(from: [u8; 32]) -> Self {
        Self(from)
    }
}

impl TryFrom<&[u8]> for Pubkey {
    type Error = std::array::TryFromSliceError;

    #[inline]
    fn try_from(pubkey: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(pubkey).map(Self::from)
    }
}

impl TryFrom<Vec<u8>> for Pubkey {
    type Error = Vec<u8>;

    #[inline]
    fn try_from(pubkey: Vec<u8>) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(pubkey).map(Self::from)
    }
}
