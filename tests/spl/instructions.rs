// ---- token ----

#[test]
fn token_transfer() {
    let mut data = vec![substreams_solana_idls::spl::token::instructions::TRANSFER];
    data.extend_from_slice(&100u64.to_le_bytes());
    let ix = substreams_solana_idls::spl::token::instructions::unpack(&data).unwrap();
    assert_eq!(ix, substreams_solana_idls::spl::token::instructions::TokenInstruction::Transfer { amount: 100 });
}

#[test]
fn token_empty() {
    assert!(substreams_solana_idls::spl::token::instructions::unpack(&[]).is_err());
}

#[test]
fn token_unknown() {
    assert!(substreams_solana_idls::spl::token::instructions::unpack(&[255]).is_err());
}

#[test]
fn token_mint_to() {
    let mut data = vec![substreams_solana_idls::spl::token::instructions::MINT_TO];
    data.extend_from_slice(&500u64.to_le_bytes());
    let ix = substreams_solana_idls::spl::token::instructions::unpack(&data).unwrap();
    assert_eq!(ix, substreams_solana_idls::spl::token::instructions::TokenInstruction::MintTo { amount: 500 });
}

// ---- token_2022 ----

#[test]
fn token_2022_transfer() {
    let mut data = vec![substreams_solana_idls::spl::token_2022::instructions::TRANSFER];
    data.extend_from_slice(&200u64.to_le_bytes());
    let ix = substreams_solana_idls::spl::token_2022::instructions::unpack(&data).unwrap();
    assert_eq!(
        ix,
        substreams_solana_idls::spl::token_2022::instructions::Token2022Instruction::Transfer { amount: 200 }
    );
}

#[test]
fn token_2022_empty() {
    assert!(substreams_solana_idls::spl::token_2022::instructions::unpack(&[]).is_err());
}

#[test]
fn token_2022_unknown() {
    assert!(substreams_solana_idls::spl::token_2022::instructions::unpack(&[255]).is_err());
}

#[test]
fn token_2022_transfer_checked() {
    let mut data = vec![substreams_solana_idls::spl::token_2022::instructions::TRANSFER_CHECKED];
    data.extend_from_slice(&1000u64.to_le_bytes());
    data.push(9); // decimals
    let ix = substreams_solana_idls::spl::token_2022::instructions::unpack(&data).unwrap();
    assert_eq!(
        ix,
        substreams_solana_idls::spl::token_2022::instructions::Token2022Instruction::TransferChecked { amount: 1000, decimals: 9 }
    );
}

// ---- token_swap ----

#[test]
fn token_swap_swap() {
    let mut data = vec![substreams_solana_idls::spl::token_swap::instructions::SWAP];
    data.extend_from_slice(&50u64.to_le_bytes());
    data.extend_from_slice(&10u64.to_le_bytes());
    let ix = substreams_solana_idls::spl::token_swap::instructions::unpack(&data).unwrap();
    assert_eq!(
        ix,
        substreams_solana_idls::spl::token_swap::instructions::TokenSwapInstruction::Swap {
            amount_in: 50,
            minimum_amount_out: 10,
        }
    );
}

#[test]
fn token_swap_empty() {
    assert!(substreams_solana_idls::spl::token_swap::instructions::unpack(&[]).is_err());
}

#[test]
fn token_swap_unknown() {
    assert!(substreams_solana_idls::spl::token_swap::instructions::unpack(&[255]).is_err());
}

// ---- token_lending ----

#[test]
fn token_lending_deposit() {
    let mut data = vec![substreams_solana_idls::spl::token_lending::instructions::DEPOSIT_RESERVE_LIQUIDITY];
    data.extend_from_slice(&1000u64.to_le_bytes());
    let ix = substreams_solana_idls::spl::token_lending::instructions::unpack(&data).unwrap();
    assert_eq!(
        ix,
        substreams_solana_idls::spl::token_lending::instructions::TokenLendingInstruction::DepositReserveLiquidity { liquidity_amount: 1000 }
    );
}

#[test]
fn token_lending_empty() {
    assert!(substreams_solana_idls::spl::token_lending::instructions::unpack(&[]).is_err());
}

#[test]
fn token_lending_unknown() {
    assert!(substreams_solana_idls::spl::token_lending::instructions::unpack(&[255]).is_err());
}

#[test]
fn token_lending_flash_loan() {
    let mut data = vec![substreams_solana_idls::spl::token_lending::instructions::FLASH_LOAN];
    data.extend_from_slice(&5000u64.to_le_bytes());
    let ix = substreams_solana_idls::spl::token_lending::instructions::unpack(&data).unwrap();
    assert_eq!(
        ix,
        substreams_solana_idls::spl::token_lending::instructions::TokenLendingInstruction::FlashLoan { amount: 5000 }
    );
}
