//! Jupiter v6 on-chain instructions.

use idls_common::ParseError;
use borsh::{BorshDeserialize, BorshSerialize};

// -----------------------------------------------------------------------------
// Payload structs
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimInstruction {
    pub id: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimTokenInstruction {
    pub id: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CloseTokenInstruction {
    pub id: u8,
    pub burn_all: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateProgramOpenOrdersInstruction {
    pub id: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateTokenAccountInstruction {
    pub bump: u8,
}

/// Route by using program owned token accounts and open orders accounts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedAccountsExactOutRouteInstruction {
    pub id: u8,
    /// Raw remaining instruction data (route plan and amounts).
    pub data: Vec<u8>,
}

/// Route by using program owned token accounts and open orders accounts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedAccountsRouteInstruction {
    pub id: u8,
    /// Raw remaining instruction data (route plan and amounts).
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedAccountsRouteWithTokenLedgerInstruction {
    pub id: u8,
    /// Raw remaining instruction data (route plan and amounts).
    pub data: Vec<u8>,
}

/// `route_plan` Topologically sorted trade DAG
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteInstruction {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactOutRouteInstruction {
    /// Raw route plan and arguments.
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteWithTokenLedgerInstruction {
    /// Raw route plan and arguments.
    pub data: Vec<u8>,
}

// -----------------------------------------------------------------------------
// Discriminators
// -----------------------------------------------------------------------------
pub const CLAIM: [u8; 8] = [62, 198, 214, 193, 213, 159, 108, 210];
pub const CLAIM_TOKEN: [u8; 8] = [116, 206, 27, 191, 166, 19, 0, 73];
pub const CLOSE_TOKEN: [u8; 8] = [26, 74, 236, 151, 104, 64, 183, 249];
pub const CREATE_OPEN_ORDERS: [u8; 8] = [229, 194, 212, 172, 8, 10, 134, 147];
pub const CREATE_PROGRAM_OPEN_ORDERS: [u8; 8] = [28, 226, 32, 148, 188, 136, 113, 171];
pub const CREATE_TOKEN_LEDGER: [u8; 8] = [232, 242, 197, 253, 240, 143, 129, 52];
pub const CREATE_TOKEN_ACCOUNT: [u8; 8] = [147, 241, 123, 100, 244, 132, 174, 118];
pub const EXACT_OUT_ROUTE: [u8; 8] = [208, 51, 239, 151, 123, 43, 237, 92];
pub const ROUTE: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];
pub const ROUTE_WITH_TOKEN_LEDGER: [u8; 8] = [150, 86, 71, 116, 167, 93, 14, 104];
pub const SET_TOKEN_LEDGER: [u8; 8] = [228, 85, 185, 112, 78, 79, 77, 2];
pub const SHARED_ACCOUNTS_EXACT_OUT_ROUTE: [u8; 8] = [176, 209, 105, 168, 154, 125, 69, 62];
pub const SHARED_ACCOUNTS_ROUTE: [u8; 8] = [193, 32, 155, 51, 65, 214, 156, 129];
pub const SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER: [u8; 8] = [230, 121, 143, 80, 119, 159, 106, 170];

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JupiterV6Instruction {
    Claim(ClaimInstruction),
    ClaimToken(ClaimTokenInstruction),
    CloseToken(CloseTokenInstruction),
    CreateOpenOrders,
    CreateProgramOpenOrders(CreateProgramOpenOrdersInstruction),
    CreateTokenLedger,
    CreateTokenAccount(CreateTokenAccountInstruction),
    ExactOutRoute(ExactOutRouteInstruction),
    /// route_plan Topologically sorted trade DAG
    Route(RouteInstruction),
    RouteWithTokenLedger(RouteWithTokenLedgerInstruction),
    SetTokenLedger,
    SharedAccountsExactOutRoute(SharedAccountsExactOutRouteInstruction),
    SharedAccountsRoute(SharedAccountsRouteInstruction),
    SharedAccountsRouteWithTokenLedger(SharedAccountsRouteWithTokenLedgerInstruction),
    Unknown,
}

// -----------------------------------------------------------------------------
// Borsh deserialisation helper
// -----------------------------------------------------------------------------
impl<'a> TryFrom<&'a [u8]> for JupiterV6Instruction {
    type Error = ParseError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::TooShort(data.len()));
        }

        let (disc, payload) = data.split_at(8);
        let discriminator: [u8; 8] = disc.try_into().expect("slice len 8");

        Ok(match discriminator {
            CLAIM => Self::Claim(ClaimInstruction::try_from_slice(payload)?),
            CLAIM_TOKEN => Self::ClaimToken(ClaimTokenInstruction::try_from_slice(payload)?),
            CLOSE_TOKEN => Self::CloseToken(CloseTokenInstruction::try_from_slice(payload)?),
            CREATE_OPEN_ORDERS => Self::CreateOpenOrders,
            CREATE_PROGRAM_OPEN_ORDERS => Self::CreateProgramOpenOrders(CreateProgramOpenOrdersInstruction::try_from_slice(payload)?),
            CREATE_TOKEN_LEDGER => Self::CreateTokenLedger,
            CREATE_TOKEN_ACCOUNT => Self::CreateTokenAccount(CreateTokenAccountInstruction::try_from_slice(payload)?),
            EXACT_OUT_ROUTE => Self::ExactOutRoute(ExactOutRouteInstruction { data: payload.to_vec() }),
            ROUTE => Self::Route(RouteInstruction { data: payload.to_vec() }),
            ROUTE_WITH_TOKEN_LEDGER => Self::RouteWithTokenLedger(RouteWithTokenLedgerInstruction { data: payload.to_vec() }),
            SET_TOKEN_LEDGER => Self::SetTokenLedger,
            SHARED_ACCOUNTS_EXACT_OUT_ROUTE => {
                if payload.is_empty() {
                    return Err(ParseError::InvalidLength { expected: 1, got: 0 });
                }
                let id = payload[0];
                let data = payload[1..].to_vec();
                Self::SharedAccountsExactOutRoute(SharedAccountsExactOutRouteInstruction { id, data })
            }
            SHARED_ACCOUNTS_ROUTE => {
                if payload.is_empty() {
                    return Err(ParseError::InvalidLength { expected: 1, got: 0 });
                }
                let id = payload[0];
                let data = payload[1..].to_vec();
                Self::SharedAccountsRoute(SharedAccountsRouteInstruction { id, data })
            }
            SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER => {
                if payload.is_empty() {
                    return Err(ParseError::InvalidLength { expected: 1, got: 0 });
                }
                let id = payload[0];
                let data = payload[1..].to_vec();
                Self::SharedAccountsRouteWithTokenLedger(SharedAccountsRouteWithTokenLedgerInstruction { id, data })
            }
            other => return Err(ParseError::Unknown(other)),
        })
    }
}

/// Convenience wrapper that forwards to `TryFrom`.
pub fn unpack(data: &[u8]) -> Result<JupiterV6Instruction, ParseError> {
    JupiterV6Instruction::try_from(data)
}
