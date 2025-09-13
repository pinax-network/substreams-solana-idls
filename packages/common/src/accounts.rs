use solana_program::pubkey::Pubkey;
use thiserror::Error;

// -----------------------------------------------------------------------------
// Error type shared across account parsers
// -----------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum AccountsError {
    #[error("missing required account `{name}` at index {index}")]
    Missing { name: &'static str, index: usize },
    #[error("invalid key length for `{name}` at index {index}: got {got}, want 32")]
    InvalidLen { name: &'static str, index: usize, got: usize },
}

#[inline]
pub fn to_pubkey(name: &'static str, index: usize, bytes: &[u8]) -> Result<Pubkey, AccountsError> {
    let arr: [u8; 32] = bytes.try_into().map_err(|_| AccountsError::InvalidLen { name, index, got: bytes.len() })?;
    Ok(Pubkey::new_from_array(arr))
}

// -----------------------------------------------------------------------------
// Helper macro for required-only account structs
// -----------------------------------------------------------------------------
#[macro_export]
macro_rules! accounts {
    ($name:ident, $getter:ident, { $( $(#[$doc:meta])* $field:ident ),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
        pub struct $name {
            $( $(#[$doc])* pub $field: Pubkey,)+
        }

        impl<'ix> TryFrom<&InstructionView<'ix>> for $name {
            type Error = $crate::accounts::AccountsError;
            fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
                let accounts = ix.accounts();
                let mut idx = 0;
                $(
                    let $field = {
                        let name = stringify!($field);
                        let a = accounts
                            .get(idx)
                            .ok_or($crate::accounts::AccountsError::Missing { name, index: idx })?;
                        let pk = $crate::accounts::to_pubkey(name, idx, a.0)?;
                        idx += 1;
                        pk
                    };
                )+
                let _ = idx;
                Ok(Self { $($field,)+ })
            }
        }

        pub fn $getter(ix: &InstructionView) -> Result<$name, $crate::accounts::AccountsError> {
            $name::try_from(ix)
        }
    };
}
