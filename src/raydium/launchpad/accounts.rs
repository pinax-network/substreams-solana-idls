use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use substreams_solana::block_view::InstructionView;
use thiserror::Error;

// -----------------------------------------------------------------------------
// Error type
// -----------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum AccountsError {
    #[error("missing required account `{name}` at index {index}")]
    Missing { name: &'static str, index: usize },
    #[error("invalid key length for `{name}` at index {index}: got {got}, want 32")]
    InvalidLen { name: &'static str, index: usize, got: usize },
}

#[inline]
fn to_pubkey(name: &'static str, index: usize, bytes: &[u8]) -> Result<Pubkey, AccountsError> {
    let arr: [u8; 32] = bytes.try_into().map_err(|_| AccountsError::InvalidLen { name, index, got: bytes.len() })?;
    Ok(Pubkey::new_from_array(arr))
}

// -----------------------------------------------------------------------------
// Helper macro with doc comments
// -----------------------------------------------------------------------------
macro_rules! accounts {
    ($name:ident, $getter:ident, { $( $(#[$meta:meta])* $field:ident ),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
        pub struct $name {
            $( $(#[$meta])* pub $field: Pubkey,)+
        }

        impl<'ix> TryFrom<&InstructionView<'ix>> for $name {
            type Error = AccountsError;

            fn try_from(ix: &InstructionView<'ix>) -> Result<Self, Self::Error> {
                let accounts = ix.accounts();
                let mut idx = 0;
                $(
                    let $field = {
                        let name = stringify!($field);
                        let a = accounts
                            .get(idx)
                            .ok_or(AccountsError::Missing { name, index: idx })?;
                        let pk = to_pubkey(name, idx, &a.0)?;
                        idx += 1;
                        pk
                    };
                )+
                let _ = idx;
                Ok(Self { $( $field, )+ })
            }
        }

        pub fn $getter(ix: &InstructionView) -> Result<$name, AccountsError> {
            $name::try_from(ix)
        }
    };
}

// -----------------------------------------------------------------------------
// Buy exact in accounts
// -----------------------------------------------------------------------------
accounts!(
    BuyExactInAccounts,
    get_buy_exact_in_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being bought)
        user_base_token,
        /// The user's token account for quote tokens (tokens being sold)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);

// -----------------------------------------------------------------------------
// Buy exact out accounts
// -----------------------------------------------------------------------------
accounts!(
    BuyExactOutAccounts,
    get_buy_exact_out_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being bought)
        user_base_token,
        /// The user's token account for quote tokens (tokens being sold)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);

// -----------------------------------------------------------------------------
// Sell exact in accounts
// -----------------------------------------------------------------------------
accounts!(
    SellExactInAccounts,
    get_sell_exact_in_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being sold)
        user_base_token,
        /// The user's token account for quote tokens (tokens being bought)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);

// -----------------------------------------------------------------------------
// Sell exact out accounts
// -----------------------------------------------------------------------------
accounts!(
    SellExactOutAccounts,
    get_sell_exact_out_accounts,
    {
        /// The user performing the swap operation
        payer,
        /// PDA that acts as the authority for pool vault operations
        authority,
        /// Global configuration account containing protocol-wide settings
        global_config,
        /// Platform configuration account containing platform-wide settings
        platform_config,
        /// The pool state account where the swap will be performed
        pool_state,
        /// The user's token account for base tokens (tokens being sold)
        user_base_token,
        /// The user's token account for quote tokens (tokens being bought)
        user_quote_token,
        /// The pool's vault for base tokens
        base_vault,
        /// The pool's vault for quote tokens
        quote_vault,
        /// The mint of the base token
        base_token_mint,
        /// The mint of the quote token
        quote_token_mint,
        /// SPL Token program for base token transfers
        base_token_program,
        /// SPL Token program for quote token transfers
        quote_token_program,
        /// Program-derived address authorising event emissions
        event_authority,
        /// The raydium launchpad program id
        program
    }
);
