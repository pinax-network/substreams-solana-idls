//! Wire and distinct-account fixtures checked against the pinned source
//! contracts listed in docs/pool-initialization.md, including captured Aldrin
//! creation bytes. These tests decode inputs; they do not prove execution.
use sha2::Digest;
use substreams_solana::pb::sf::solana::r#type::v1 as pb;
use substreams_solana_idls::{
    aldrin, boop, byreal::clmm as byreal, darklake, dumpfun, moonshot, pancakeswap, phoenix, plasma, pumpfun::bonding_curve as pump, raydium::launchpad, saros,
    serum, spl::token_swap, stabble,
};

fn transaction(count: usize) -> pb::ConfirmedTransaction {
    pb::ConfirmedTransaction {
        transaction: pb::Transaction {
            message: pb::Message {
                account_keys: (0..=count).map(|n| vec![n as u8; 32]).collect(),
                instructions: vec![pb::CompiledInstruction {
                    program_id_index: 0,
                    accounts: (1..=count as u8).collect(),
                    data: vec![],
                }],
                ..Default::default()
            }
            .into(),
            ..Default::default()
        }
        .into(),
        ..Default::default()
    }
}

macro_rules! account_layout {
    ($name:ident, $helper:path, $count:expr, $pool:ident, $index:expr) => {
        #[test]
        fn $name() {
            let tx = transaction($count);
            let ix = tx.walk_instructions().next().unwrap();
            let decoded = $helper(&ix).unwrap();
            assert_eq!(decoded.$pool.to_bytes(), [$index; 32]);
            let expected: Vec<u8> = (1..=$count).flat_map(|n| [n as u8; 32]).collect();
            assert_eq!(borsh::to_vec(&decoded).unwrap(), expected);
            // Every required account must be present and exactly 32 bytes.
            for n in 0..$count {
                let tx = transaction(n);
                assert!($helper(&tx.walk_instructions().next().unwrap()).is_err());
            }
            for n in 1..=$count {
                let mut tx = transaction($count);
                tx.transaction.as_option_mut().unwrap().message.as_option_mut().unwrap().account_keys[n].pop();
                assert!($helper(&tx.walk_instructions().next().unwrap()).is_err());
            }
        }
    };
}
account_layout!(byreal_accounts, byreal::accounts::get_create_pool_accounts, 16, pool_state, 5);
account_layout!(pancake_accounts, pancakeswap::accounts::get_create_pool_accounts, 13, pool_state, 3);
account_layout!(launchpad_accounts, launchpad::accounts::get_initialize_accounts, 18, pool_state, 6);
account_layout!(launchpad_v2_accounts, launchpad::accounts::get_initialize_v2_accounts, 18, pool_state, 6);
account_layout!(
    launchpad_token2022_accounts,
    launchpad::accounts::get_initialize_with_token_2022_accounts,
    15,
    pool_state,
    6
);
account_layout!(phoenix_accounts, phoenix::accounts::get_initialize_market_accounts, 10, market, 3);
account_layout!(moonshot_accounts, moonshot::accounts::get_token_mint_accounts, 11, curve_account, 3);
account_layout!(darklake_accounts, darklake::accounts::get_initialize_pool_accounts, 25, pool, 2);
account_layout!(plasma_accounts, plasma::accounts::get_initialize_pool_accounts, 10, pool, 3);
account_layout!(saros_accounts, saros::accounts::get_initialize_accounts, 8, swap_account, 1);
account_layout!(spl_swap_accounts, token_swap::accounts::get_initialize_accounts, 8, swap_account, 1);
account_layout!(stabble_accounts, stabble::accounts::get_initialize_accounts, 6, pool, 3);
account_layout!(pump_v2_accounts, pump::accounts::get_create_v2_accounts, 16, bonding_curve, 3);

fn exact_rejections<T>(bytes: &[u8], parse: impl Fn(&[u8]) -> Result<T, substreams_solana_idls::common::ParseError>) {
    for length in 0..bytes.len() {
        assert!(parse(&bytes[..length]).is_err(), "accepted length {length}");
    }
    let mut extra = bytes.to_vec();
    extra.push(0);
    assert!(parse(&extra).is_err());
    let mut wrong = bytes.to_vec();
    wrong[0] ^= 0x80;
    assert!(parse(&wrong).is_err());
}

#[test]
fn clmm_creation_wire_is_exact_and_protocol_accounts_are_distinct() {
    let mut bytes = vec![233, 146, 209, 142, 207, 104, 64, 188];
    bytes.extend(123456789012345678901234567890u128.to_le_bytes());
    bytes.extend(987654321u64.to_le_bytes());
    let byreal::instructions::ByrealClmmInstruction::CreatePool(a) = byreal::instructions::unpack(&bytes).unwrap() else {
        panic!()
    };
    let pancakeswap::instructions::PancakeSwapInstruction::CreatePool(b) = pancakeswap::instructions::unpack(&bytes).unwrap() else {
        panic!()
    };
    assert_eq!(a.sqrt_price_x64, 123456789012345678901234567890);
    assert_eq!(a.open_time, 987654321);
    assert_eq!(a.sqrt_price_x64, b.sqrt_price_x64);
    assert_eq!(a.open_time, b.open_time);
    exact_rejections(&bytes, byreal::instructions::unpack);
    exact_rejections(&bytes, pancakeswap::instructions::unpack);
}

fn string(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend((value.len() as u32).to_le_bytes());
    bytes.extend(value.as_bytes());
}
fn launchpad_args(curve: u8) -> Vec<u8> {
    let mut bytes = vec![6];
    string(&mut bytes, "Example");
    string(&mut bytes, "EX");
    string(&mut bytes, "https://example.invalid/meta");
    bytes.push(curve);
    bytes.extend(1_000_000u64.to_le_bytes());
    if curve == 0 {
        bytes.extend(500_000u64.to_le_bytes());
    }
    bytes.extend(100u64.to_le_bytes());
    bytes.push(1);
    for value in [11u64, 22, 33] {
        bytes.extend(value.to_le_bytes());
    }
    bytes
}
#[test]
fn launchpad_creation_variants_decode_all_curves_and_optional_transfer_fee() {
    use launchpad::instructions::{self, RaydiumLaunchpadInstruction as I};
    for curve in 0..3 {
        let args = launchpad_args(curve);
        let bytes = [instructions::INITIALIZE.as_slice(), &args].concat();
        let I::Initialize(v) = instructions::unpack(&bytes).unwrap() else { panic!() };
        assert_eq!(v.base_mint_param.decimals, 6);
        assert_eq!(v.base_mint_param.name, "Example");
        assert_eq!(v.vesting_param.unlock_period, 33);
        assert_eq!(borsh::to_vec(&v).unwrap(), args);
        exact_rejections(&bytes, instructions::unpack);
        let mut bytes = [instructions::INITIALIZE_V2.as_slice(), &args].concat();
        bytes.push(1);
        let I::InitializeV2(v) = instructions::unpack(&bytes).unwrap() else {
            panic!()
        };
        assert_eq!(v.amm_fee_on, instructions::AmmCreatorFeeOn::BothToken);
        exact_rejections(&bytes, instructions::unpack);
        for some in [false, true] {
            let mut bytes = [instructions::INITIALIZE_WITH_TOKEN_2022.as_slice(), &args].concat();
            bytes.extend([0, some as u8]);
            if some {
                bytes.extend(250u16.to_le_bytes());
                bytes.extend(123u64.to_le_bytes());
            }
            let I::InitializeWithToken2022(v) = instructions::unpack(&bytes).unwrap() else {
                panic!()
            };
            assert_eq!(v.transfer_fee_extension_param.is_some(), some);
            exact_rejections(&bytes, instructions::unpack);
            let suffix = 8 + args.len();
            bytes[suffix] = 2;
            assert!(instructions::unpack(&bytes).is_err());
        }
    }
}
#[test]
fn phoenix_creation_parses_versioned_option_and_exact_amounts() {
    use phoenix::instructions::{self, PhonenixInstruction};
    for some in [false, true] {
        let mut bytes = vec![100];
        for n in 1u64..=6 {
            bytes.extend(n.to_le_bytes());
        }
        bytes.extend(7u16.to_le_bytes());
        bytes.extend([8; 32]);
        bytes.push(some as u8);
        if some {
            bytes.extend(9u32.to_le_bytes());
        }
        let PhonenixInstruction::InitializeMarket(v) = instructions::unpack(&bytes).unwrap() else {
            panic!()
        };
        assert_eq!(v.market_size_params.bids_size, 1);
        assert_eq!(v.market_size_params.num_seats, 3);
        assert_eq!(v.num_base_lots_per_base_unit, 6);
        assert_eq!(v.taker_fee_bps, 7);
        assert_eq!(v.fee_collector.to_bytes(), [8; 32]);
        assert_eq!(v.raw_base_units_per_base_unit, if some { Some(9) } else { None });
        exact_rejections(&bytes, instructions::unpack);
    }
}
#[test]
fn spl_and_saros_initialization_share_exact_fee_curve_wire_contract() {
    let mut bytes = vec![0];
    for n in 1u64..=8 {
        bytes.extend(n.to_le_bytes());
    }
    bytes.push(2);
    bytes.extend([3; 32]);
    let spl = token_swap::instructions::unpack_initialize(&bytes).unwrap();
    let saros::instructions::SarosInstruction::Initialize(saros) = saros::instructions::unpack(&bytes).unwrap() else {
        panic!()
    };
    assert_eq!(spl, saros);
    assert_eq!(spl.fees.host_fee_denominator, 8);
    assert_eq!(spl.curve_type, 2);
    assert_eq!(spl.curve_parameters, [3; 32]);
    exact_rejections(&bytes, token_swap::instructions::unpack_initialize);
    exact_rejections(&bytes, saros::instructions::unpack);
}
#[test]
fn serum_creation_requires_canonical_version_and_exact_payload() {
    let mut bytes = vec![0; 5];
    bytes.extend(11u64.to_le_bytes());
    bytes.extend(22u64.to_le_bytes());
    bytes.extend(33u16.to_le_bytes());
    bytes.extend(44u64.to_le_bytes());
    bytes.extend(55u64.to_le_bytes());
    let v = serum::instructions::unpack_initialize_market(&bytes).unwrap();
    assert_eq!(v.coin_lot_size, 11);
    assert_eq!(v.pc_lot_size, 22);
    assert_eq!(v.fee_rate_bps, 33);
    assert_eq!(v.vault_signer_nonce, 44);
    assert_eq!(v.pc_dust_threshold, 55);
    exact_rejections(&bytes, serum::instructions::unpack_initialize_market);
    assert!(serum::instructions::unpack_initialize_market(&bytes[1..]).is_err());
    for n in 0..5 {
        let mut invalid = bytes.clone();
        invalid[n] = 1;
        assert!(serum::instructions::unpack_initialize_market(&invalid).is_err());
    }
}
#[test]
fn current_pump_create_v2_preserves_all_official_fields() {
    use pump::instructions::{self, PumpFunInstruction};
    let mut bytes = instructions::CREATE_V2.to_vec();
    string(&mut bytes, "Current");
    string(&mut bytes, "NOW");
    string(&mut bytes, "uri");
    bytes.extend([7; 32]);
    bytes.extend([1, 0]);
    bytes.extend(100u64.to_le_bytes());
    bytes.push(1);
    let PumpFunInstruction::CreateV2(v) = instructions::unpack(&bytes).unwrap() else {
        panic!()
    };
    assert_eq!(v.name, "Current");
    assert_eq!(v.creator.to_bytes(), [7; 32]);
    assert!(v.is_mayhem_mode);
    assert!(!v.is_cashback_enabled.0);
    assert_eq!(v.creator_fee_bps.0, 100);
    assert!(v.is_holder_reward.0);
    assert_eq!(borsh::to_vec(&v).unwrap(), bytes[8..]);
    for length in 0..bytes.len() {
        if [bytes.len() - 9, bytes.len() - 1].contains(&length) {
            let PumpFunInstruction::CreateV2(short) = instructions::unpack(&bytes[..length]).unwrap() else {
                panic!()
            };
            assert_eq!(short.creator_fee_bps.0, if length == bytes.len() - 9 { 0 } else { 100 });
            assert!(!short.is_holder_reward.0);
        } else {
            assert!(instructions::unpack(&bytes[..length]).is_err(), "accepted length {length}");
        }
    }
    let mut extra = bytes.clone();
    extra.push(0);
    assert!(instructions::unpack(&extra).is_err());
    for n in [bytes.len() - 11, bytes.len() - 10, bytes.len() - 1] {
        let mut invalid = bytes.clone();
        invalid[n] = 2;
        assert!(instructions::unpack(&invalid).is_err());
    }
    assert_eq!(borsh::to_vec(&PumpFunInstruction::Unknown).unwrap(), vec![6]);
}
#[test]
fn stabble_initialization_preserves_multi_asset_weights_and_caps() {
    let mut bytes = stabble::instructions::INITIALIZE.to_vec();
    bytes.extend(123u64.to_le_bytes());
    for values in [[1u64, 2, 3], [4u64, 5, 6]] {
        bytes.extend(3u32.to_le_bytes());
        for n in values {
            bytes.extend(n.to_le_bytes());
        }
    }
    let stabble::instructions::StabbleInstruction::Initialize(v) = stabble::instructions::unpack(&bytes).unwrap() else {
        panic!()
    };
    assert_eq!(v.swap_fee, 123);
    assert_eq!(v.weights, vec![1, 2, 3]);
    assert_eq!(v.max_caps, vec![4, 5, 6]);
    exact_rejections(&bytes, stabble::instructions::unpack);
}

#[test]
fn stabble_assets_are_ordered_remaining_mints_not_lp_mint_or_vault() {
    let tx = transaction(9);
    let ix = tx.walk_instructions().next().unwrap();
    let mints = stabble::accounts::get_initialize_mints(&ix).unwrap();
    assert_eq!(mints.iter().map(|x| x.to_bytes()).collect::<Vec<_>>(), vec![[7; 32], [8; 32], [9; 32]]);
    let short = transaction(5);
    assert!(stabble::accounts::get_initialize_mints(&short.walk_instructions().next().unwrap()).is_err());
}

account_layout!(boop_accounts, boop::accounts::get_deploy_bonding_curve_accounts, 10, bonding_curve, 3);
account_layout!(
    boop_fallback_accounts,
    boop::accounts::get_deploy_bonding_curve_fallback_accounts,
    10,
    bonding_curve,
    3
);
account_layout!(dumpfun_accounts, dumpfun::accounts::get_mint_accounts, 11, liquidity_pool, 4);

#[test]
fn pump_create_v2_quote_is_native_only_when_no_remaining_accounts() {
    assert!(
        pump::accounts::get_create_v2_quote_accounts(&transaction(16).walk_instructions().next().unwrap())
            .unwrap()
            .is_none()
    );
    for n in 0..16 {
        assert!(pump::accounts::get_create_v2_quote_accounts(&transaction(n).walk_instructions().next().unwrap()).is_err());
    }
    for n in [17, 18] {
        assert!(pump::accounts::get_create_v2_quote_accounts(&transaction(n).walk_instructions().next().unwrap()).is_err());
    }
    for n in 19..=24 {
        let tx = transaction(n);
        let v = pump::accounts::get_create_v2_quote_accounts(&tx.walk_instructions().next().unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(v.quote_mint.to_bytes(), [17; 32]);
        assert_eq!(v.associated_quote_bonding_curve.to_bytes(), [18; 32]);
        assert_eq!(v.quote_token_program.to_bytes(), [19; 32]);
    }
    for n in 17..=19 {
        let mut tx = transaction(19);
        tx.transaction.as_option_mut().unwrap().message.as_option_mut().unwrap().account_keys[n].pop();
        assert!(pump::accounts::get_create_v2_quote_accounts(&tx.walk_instructions().next().unwrap()).is_err());
    }
}

#[test]
fn existing_creation_wires_match_new_account_layout_sources() {
    for some in [false, true] {
        let mut d = darklake::instructions::INITIALIZE_POOL.to_vec();
        d.extend(11u64.to_le_bytes());
        d.extend(22u64.to_le_bytes());
        d.push(some as u8);
        if some {
            d.extend([3; 21]);
        }
        let darklake::instructions::DarklakeInstruction::InitializePool(v) = darklake::instructions::unpack(&d).unwrap() else {
            panic!()
        };
        assert_eq!(v.amount_x, 11);
        assert_eq!(v.amount_y, 22);
        assert_eq!(v.label.is_some(), some);
        exact_rejections(&d, darklake::instructions::unpack);
        let mut d = vec![6];
        d.extend(1u64.to_le_bytes());
        d.extend(2u64.to_le_bytes());
        for n in 3..6 {
            d.extend([n; 32]);
            d.extend((n as u64).to_le_bytes());
        }
        d.push(some as u8);
        if some {
            d.extend(7u64.to_le_bytes());
        }
        let plasma::instructions::PlasmaInstruction::InitializePool(v) = plasma::instructions::unpack(&d).unwrap() else {
            panic!()
        };
        assert_eq!(v.lp_fee_in_bps, 1);
        assert_eq!(v.fee_recipients_params[2].recipient, [5; 32]);
        assert_eq!(v.num_slots_to_vest_lp_shares, if some { Some(7) } else { None });
        exact_rejections(&d, plasma::instructions::unpack);
    }
    for discriminator in [boop::instructions::DEPLOY_BONDING_CURVE, boop::instructions::DEPLOY_BONDING_CURVE_FALLBACK] {
        let mut d = discriminator.to_vec();
        d.extend([7; 32]);
        d.extend(8u64.to_le_bytes());
        match boop::instructions::unpack(&d).unwrap() {
            boop::instructions::BoopInstruction::DeployBondingCurve(v) => {
                assert_eq!(v.creator.to_bytes(), [7; 32]);
                assert_eq!(v.salt, 8);
            }
            boop::instructions::BoopInstruction::DeployBondingCurveFallback(v) => {
                assert_eq!(v.creator.to_bytes(), [7; 32]);
                assert_eq!(v.salt, 8);
            }
            _ => panic!(),
        }
        exact_rejections(&d, boop::instructions::unpack);
    }
    let mut d = dumpfun::instructions::MINT.to_vec();
    for s in ["Token", "TKN", "uri"] {
        string(&mut d, s);
    }
    d.extend((-9i64).to_le_bytes());
    d.extend(10u64.to_le_bytes());
    d.extend(1u32.to_le_bytes());
    d.extend(11i64.to_le_bytes());
    d.extend(12i64.to_le_bytes());
    d.extend(13u16.to_le_bytes());
    let dumpfun::instructions::DumpfunInstruction::Mint(v) = dumpfun::instructions::unpack(&d).unwrap() else {
        panic!()
    };
    assert_eq!(v.token_name, "Token");
    assert_eq!(v.sell_lock_period, -9);
    assert_eq!(v.ramping_limits[0].limit_bps, 13);
    exact_rejections(&d, dumpfun::instructions::unpack);
    let mut d = moonshot::instructions::TOKEN_MINT.to_vec();
    for s in ["Token", "TKN", "uri"] {
        string(&mut d, s);
    }
    d.extend([9, 0]);
    d.extend(123u64.to_le_bytes());
    d.push(1);
    let moonshot::instructions::MoonshotInstruction::TokenMint(v) = moonshot::instructions::unpack(&d).unwrap() else {
        panic!()
    };
    assert_eq!(v.name, "Token");
    assert_eq!(v.decimals, 9);
    assert_eq!(v.amount, 123);
    exact_rejections(&d, moonshot::instructions::unpack);
}

account_layout!(aldrin_accounts, aldrin::accounts::get_initialize_accounts, 14, pool, 1);

#[test]
fn captured_aldrin_pool_initializer_is_distinct_from_legacy_clob_api() {
    use aldrin::instructions::{self, AldrinInstruction};
    let data = include_bytes!("fixtures/pool-initialization/aldrin-initialize-data.bin");
    let accounts = include_bytes!("fixtures/pool-initialization/aldrin-initialize-accounts.bin");
    let AldrinInstruction::Initialize(v) = instructions::unpack(data).unwrap() else {
        panic!()
    };
    assert_eq!(v.signer_nonce, 254);
    assert_eq!(v.curve_type, 0);
    assert_ne!(instructions::INITIALIZE, instructions::INITIALIZE_MARKET);
    assert_eq!(&sha2::Sha256::digest(b"global:initialize")[..8], instructions::INITIALIZE);
    exact_rejections(data, instructions::unpack);
    let mut tx = transaction(14);
    let message = tx.transaction.as_option_mut().unwrap().message.as_option_mut().unwrap();
    message.account_keys = std::iter::once(aldrin::PROGRAM_ID.to_vec())
        .chain(accounts.chunks_exact(32).map(|v| v.to_vec()))
        .collect();
    message.instructions[0].data = data.to_vec();
    let view = tx.walk_instructions().next().unwrap();
    let parsed = aldrin::accounts::get_initialize_accounts(&view).unwrap();
    assert_eq!(borsh::to_vec(&parsed).unwrap(), accounts);
    assert_eq!(parsed.pool.to_string(), "931bAjQx4RtbBjreP8HTNEu7ykozk4QXyYj3hiaJNdH");
    assert_eq!(parsed.base_token_vault.to_string(), "GBN31V2iuAf4vVszJopaeWtncQpaR6yCqiQZxrKxQNNc");
    assert_eq!(parsed.quote_token_vault.to_string(), "A24aR7uX7bZmyTQqHoADthNdc9nCru25oc7uafw18EWk");
}
