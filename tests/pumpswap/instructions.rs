use borsh::BorshSerialize;
use substreams_solana_idls::pumpswap::instructions::*;

#[test]
fn test_buy() {
    let ix = BuyInstruction {
        base_amount_out: 1_000_000,
        max_quote_amount_in: 500_000,
    };
    let mut data = BUY.to_vec();
    data.extend_from_slice(&borsh::to_vec(&ix).unwrap());
    match unpack(&data).unwrap() {
        PumpSwapInstruction::Buy(parsed) => {
            assert_eq!(parsed.base_amount_out, 1_000_000);
            assert_eq!(parsed.max_quote_amount_in, 500_000);
        }
        other => panic!("expected Buy, got {:?}", other),
    }
}

#[test]
fn test_sell() {
    let ix = SellInstruction {
        base_amount_in: 2_000_000,
        min_quote_amount_out: 900_000,
    };
    let mut data = SELL.to_vec();
    data.extend_from_slice(&borsh::to_vec(&ix).unwrap());
    match unpack(&data).unwrap() {
        PumpSwapInstruction::Sell(parsed) => {
            assert_eq!(parsed.base_amount_in, 2_000_000);
            assert_eq!(parsed.min_quote_amount_out, 900_000);
        }
        other => panic!("expected Sell, got {:?}", other),
    }
}

#[test]
fn test_create_pool() {
    let ix = CreatePoolInstruction {
        index: 42,
        base_amount_in: 1_000_000_000,
        quote_amount_in: 500_000_000,
        coin_creator: solana_program::pubkey::Pubkey::new_unique(),
        is_mayhem_mode: false,
    };
    let mut data = CREATE_POOL.to_vec();
    data.extend_from_slice(&borsh::to_vec(&ix).unwrap());
    match unpack(&data).unwrap() {
        PumpSwapInstruction::CreatePool(parsed) => {
            assert_eq!(parsed.index, 42);
            assert_eq!(parsed.base_amount_in, 1_000_000_000);
            assert!(!parsed.is_mayhem_mode);
        }
        other => panic!("expected CreatePool, got {:?}", other),
    }
}

#[test]
fn test_deposit() {
    let ix = DepositInstruction {
        lp_token_amount_out: 100,
        max_base_amount_in: 200,
        max_quote_amount_in: 300,
    };
    let mut data = DEPOSIT.to_vec();
    data.extend_from_slice(&borsh::to_vec(&ix).unwrap());
    match unpack(&data).unwrap() {
        PumpSwapInstruction::Deposit(parsed) => {
            assert_eq!(parsed.lp_token_amount_out, 100);
        }
        other => panic!("expected Deposit, got {:?}", other),
    }
}

#[test]
fn test_withdraw() {
    let ix = WithdrawInstruction {
        lp_token_amount_in: 50,
        min_base_amount_out: 40,
        min_quote_amount_out: 30,
    };
    let mut data = WITHDRAW.to_vec();
    data.extend_from_slice(&borsh::to_vec(&ix).unwrap());
    match unpack(&data).unwrap() {
        PumpSwapInstruction::Withdraw(parsed) => {
            assert_eq!(parsed.lp_token_amount_in, 50);
        }
        other => panic!("expected Withdraw, got {:?}", other),
    }
}

#[test]
fn test_no_arg_instructions() {
    // ClaimTokenIncentives has no args
    let data = CLAIM_TOKEN_INCENTIVES.to_vec();
    assert!(matches!(unpack(&data).unwrap(), PumpSwapInstruction::ClaimTokenIncentives));

    let data = EXTEND_ACCOUNT.to_vec();
    assert!(matches!(unpack(&data).unwrap(), PumpSwapInstruction::ExtendAccount));
}

#[test]
fn test_disable() {
    let ix = DisableInstruction {
        disable_create_pool: true,
        disable_deposit: false,
        disable_withdraw: false,
        disable_buy: true,
        disable_sell: false,
    };
    let mut data = DISABLE.to_vec();
    data.extend_from_slice(&borsh::to_vec(&ix).unwrap());
    match unpack(&data).unwrap() {
        PumpSwapInstruction::Disable(parsed) => {
            assert!(parsed.disable_create_pool);
            assert!(parsed.disable_buy);
            assert!(!parsed.disable_sell);
        }
        other => panic!("expected Disable, got {:?}", other),
    }
}

#[test]
fn test_unknown() {
    let data = [0u8; 8];
    assert!(unpack(&data).is_err());
}

#[test]
fn test_too_short() {
    let data = [0u8; 4];
    assert!(unpack(&data).is_err());
}
