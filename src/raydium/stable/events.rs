//! Raydium Stable AMM events.

use crate::ParseError;
use serde::{Deserialize, Serialize};

// The Raydium stable AMM program does not currently emit structured events.
// This module defines a placeholder enum for interface completeness.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaydiumStableEvent {
    /// No known events
    Unknown,
}

impl<'a> TryFrom<&'a [u8]> for RaydiumStableEvent {
    type Error = ParseError;

    fn try_from(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self::Unknown)
    }
}

pub fn unpack(data: &[u8]) -> Result<RaydiumStableEvent, ParseError> {
    RaydiumStableEvent::try_from(data)
}

