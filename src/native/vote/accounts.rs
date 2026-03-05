//! Vote Program account state types.
//!
//! The Vote program defines three on-chain account state variants:
//! - Uninitialized
//! - VoteState
//! - VoteStateWithLatency

/// Vote account state discriminator.
///
/// The first 4 bytes of a vote account's data indicate its variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoteAccountState {
    Uninitialized,
    VoteState,
    VoteStateWithLatency,
}
