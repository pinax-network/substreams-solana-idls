use substreams_solana_idls::pumpfun::bonding_curve::instructions::*;

#[test]
fn parse_buy_instruction() {
    let disc: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
    let buy = BuyInstruction {
        amount: 1_000_000,
        max_sol_cost: 500_000,
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&buy).unwrap());

    let parsed = unpack(&data).unwrap();
    match parsed {
        PumpFunInstruction::Buy(b) => {
            assert_eq!(b.amount, 1_000_000);
            assert_eq!(b.max_sol_cost, 500_000);
        }
        other => panic!("expected Buy, got {:?}", other),
    }
}

#[test]
fn parse_sell_instruction() {
    let disc: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
    let sell = SellInstruction {
        amount: 2_000_000,
        min_sol_output: 100_000,
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&sell).unwrap());

    let parsed = unpack(&data).unwrap();
    assert!(matches!(parsed, PumpFunInstruction::Sell(s) if s.amount == 2_000_000));
}

#[test]
fn parse_initialize() {
    let disc: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    assert!(matches!(unpack(&disc).unwrap(), PumpFunInstruction::Initialize));
}

#[test]
fn parse_withdraw() {
    let disc: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
    assert!(matches!(unpack(&disc).unwrap(), PumpFunInstruction::Withdraw));
}

#[test]
fn parse_create_instruction() {
    use solana_program::pubkey::Pubkey;
    let disc: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
    let create = CreateInstruction {
        name: "TestToken".into(),
        symbol: "TEST".into(),
        uri: "https://example.com/meta.json".into(),
        creator: Pubkey::new_unique(),
    };
    let mut data = disc.to_vec();
    data.extend(borsh::to_vec(&create).unwrap());

    let parsed = unpack(&data).unwrap();
    assert!(matches!(parsed, PumpFunInstruction::Create(c) if c.name == "TestToken"));
}

#[test]
fn too_short_returns_error() {
    assert!(unpack(&[0u8; 4]).is_err());
}

#[test]
fn unknown_discriminator_returns_error() {
    assert!(unpack(&[0u8; 8]).is_err());
}

#[test]
fn parse_current_and_legacy_buy_arguments() {
    for discriminator in [[102, 6, 61, 18, 1, 218, 235, 234], BUY_EXACT_SOL_IN] {
        for flag in [None, Some(0), Some(1)] {
            let mut data = discriminator.to_vec();
            data.extend_from_slice(&1_000_000u64.to_le_bytes());
            data.extend_from_slice(&500_000u64.to_le_bytes());
            if let Some(flag) = flag {
                data.push(flag);
            }
            match unpack(&data).unwrap() {
                PumpFunInstruction::Buy(ix) => {
                    assert_eq!(ix.amount, 1_000_000);
                    assert_eq!(ix.max_sol_cost, 500_000);
                }
                PumpFunInstruction::BuyExactSolIn(ix) => {
                    assert_eq!(ix.spendable_sol_in, 1_000_000);
                    assert_eq!(ix.min_tokens_out, 500_000);
                }
                other => panic!("expected buy instruction, got {other:?}"),
            }
            data.truncate(24);
            data.push(2);
            assert!(unpack(&data).is_err());
            data.truncate(24);
            data.extend_from_slice(&[1, 0]);
            assert!(unpack(&data).is_err());
        }
    }
}
