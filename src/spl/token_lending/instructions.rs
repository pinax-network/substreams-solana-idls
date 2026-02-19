//! SPL Token Lending on-chain instructions.

use crate::common::ParseError;

// -----------------------------------------------------------------------------
// Discriminators (single-byte index)
// -----------------------------------------------------------------------------
pub const INIT_LENDING_MARKET: u8 = 0;
pub const SET_LENDING_MARKET_OWNER: u8 = 1;
pub const INIT_RESERVE: u8 = 2;
pub const REFRESH_RESERVE: u8 = 3;
pub const DEPOSIT_RESERVE_LIQUIDITY: u8 = 4;
pub const REDEEM_RESERVE_COLLATERAL: u8 = 5;
pub const INIT_OBLIGATION: u8 = 6;
pub const REFRESH_OBLIGATION: u8 = 7;
pub const DEPOSIT_OBLIGATION_COLLATERAL: u8 = 8;
pub const WITHDRAW_OBLIGATION_COLLATERAL: u8 = 9;
pub const BORROW_OBLIGATION_LIQUIDITY: u8 = 10;
pub const REPAY_OBLIGATION_LIQUIDITY: u8 = 11;
pub const LIQUIDATE_OBLIGATION: u8 = 12;
pub const FLASH_LOAN: u8 = 13;
pub const DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL: u8 = 14;

// -----------------------------------------------------------------------------
// Instruction enumeration
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum TokenLendingInstruction {
    InitLendingMarket,
    SetLendingMarketOwner,
    InitReserve,
    RefreshReserve,
    DepositReserveLiquidity { liquidity_amount: u64 },
    RedeemReserveCollateral { collateral_amount: u64 },
    InitObligation,
    RefreshObligation,
    DepositObligationCollateral { collateral_amount: u64 },
    WithdrawObligationCollateral { collateral_amount: u64 },
    BorrowObligationLiquidity { liquidity_amount: u64 },
    RepayObligationLiquidity { liquidity_amount: u64 },
    LiquidateObligation { liquidity_amount: u64 },
    FlashLoan { amount: u64 },
    DepositReserveLiquidityAndObligationCollateral { liquidity_amount: u64 },
}

// -----------------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------------
impl TryFrom<&[u8]> for TokenLendingInstruction {
    type Error = ParseError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ParseError::TooShort(0));
        }
        let rest = &data[1..];
        match data[0] {
            INIT_LENDING_MARKET => Ok(Self::InitLendingMarket),
            SET_LENDING_MARKET_OWNER => Ok(Self::SetLendingMarketOwner),
            INIT_RESERVE => Ok(Self::InitReserve),
            REFRESH_RESERVE => Ok(Self::RefreshReserve),
            DEPOSIT_RESERVE_LIQUIDITY => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::DepositReserveLiquidity {
                    liquidity_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            REDEEM_RESERVE_COLLATERAL => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::RedeemReserveCollateral {
                    collateral_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            INIT_OBLIGATION => Ok(Self::InitObligation),
            REFRESH_OBLIGATION => Ok(Self::RefreshObligation),
            DEPOSIT_OBLIGATION_COLLATERAL => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::DepositObligationCollateral {
                    collateral_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            WITHDRAW_OBLIGATION_COLLATERAL => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::WithdrawObligationCollateral {
                    collateral_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            BORROW_OBLIGATION_LIQUIDITY => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::BorrowObligationLiquidity {
                    liquidity_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            REPAY_OBLIGATION_LIQUIDITY => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::RepayObligationLiquidity {
                    liquidity_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            LIQUIDATE_OBLIGATION => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::LiquidateObligation {
                    liquidity_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            FLASH_LOAN => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::FlashLoan {
                    amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL => {
                if rest.len() < 8 {
                    return Err(ParseError::TooShort(data.len()));
                }
                Ok(Self::DepositReserveLiquidityAndObligationCollateral {
                    liquidity_amount: u64::from_le_bytes(rest[..8].try_into().unwrap()),
                })
            }
            other => Err(ParseError::SplUnknown(other)),
        }
    }
}

pub fn unpack(data: &[u8]) -> Result<TokenLendingInstruction, ParseError> {
    TokenLendingInstruction::try_from(data)
}
