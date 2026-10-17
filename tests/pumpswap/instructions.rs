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
fn test_buy_volume_flag_and_legacy_arguments() {
    for discriminator in [BUY, BUY_EXACT_QUOTE_IN] {
        for flag in [None, Some(0), Some(1)] {
            let mut data = discriminator.to_vec();
            data.extend_from_slice(&576_275_924_281u64.to_le_bytes());
            data.extend_from_slice(&55_920_000u64.to_le_bytes());
            if let Some(flag) = flag {
                data.push(flag);
            }
            match unpack(&data).unwrap() {
                PumpSwapInstruction::Buy(ix) => {
                    assert_eq!(ix.base_amount_out, 576_275_924_281);
                    assert_eq!(ix.max_quote_amount_in, 55_920_000);
                }
                PumpSwapInstruction::BuyExactQuoteIn(ix) => {
                    assert_eq!(ix.spendable_quote_in, 576_275_924_281);
                    assert_eq!(ix.min_base_amount_out, 55_920_000);
                }
                other => panic!("expected buy instruction, got {other:?}"),
            }
        }
    }
}

#[test]
fn test_buy_rejects_invalid_or_extra_volume_flag_bytes() {
    for discriminator in [BUY, BUY_EXACT_QUOTE_IN] {
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&[0; 16]);
        data.push(2);
        assert!(unpack(&data).is_err(), "invalid boolean must fail");
        data.pop();
        data.extend_from_slice(&[1, 0]);
        assert!(unpack(&data).is_err(), "extra bytes must fail");
        data.truncate(23);
        assert!(unpack(&data).is_err(), "truncated amounts must fail");
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
