//! Synthetic wire/account fixtures from the official Raydium CLMM source and SDK.
//! Sources are pinned in docs/pool-initialization.md; these are not chain captures.
use sha2::{Digest, Sha256};
use substreams_solana::pb::sf::solana::r#type::v1 as pb;
use substreams_solana_idls::raydium::clmm::v3::{accounts as a, instructions as i};

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

macro_rules! accounts {
    ($test:ident, $helper:path, $count:expr, $pool_index:expr, $mint_index:expr, $program_index:expr) => {
        #[test]
        fn $test() {
            let tx = transaction($count);
            let decoded = $helper(&tx.walk_instructions().next().unwrap()).unwrap();
            assert_eq!(decoded.pool_state.to_bytes(), [$pool_index; 32]);
            assert_eq!(decoded.token_mint_0.to_bytes(), [$mint_index; 32]);
            assert_eq!(decoded.token_mint_1.to_bytes(), [$mint_index + 1; 32]);
            assert_eq!(decoded.token_vault_0.to_bytes(), [$mint_index + 2; 32]);
            assert_eq!(decoded.token_vault_1.to_bytes(), [$mint_index + 3; 32]);
            assert_eq!(decoded.token_program_0.to_bytes(), [$program_index; 32]);
            assert_eq!(decoded.token_program_1.to_bytes(), [$program_index + 1; 32]);
            let expected: Vec<_> = (1..=$count).flat_map(|n| [n as u8; 32]).collect();
            assert_eq!(borsh::to_vec(&decoded).unwrap(), expected);
            for count in 0..$count {
                let tx = transaction(count);
                assert!($helper(&tx.walk_instructions().next().unwrap()).is_err());
            }
            for index in 1..=$count {
                let mut tx = transaction($count);
                tx.transaction.as_option_mut().unwrap().message.as_option_mut().unwrap().account_keys[index].pop();
                assert!($helper(&tx.walk_instructions().next().unwrap()).is_err());
            }
            // Support-mint and dynamic-fee accounts are allowed as remaining
            // accounts; they must not shift any declared identity account.
            let tx = transaction($count + 2);
            assert_eq!($helper(&tx.walk_instructions().next().unwrap()).unwrap(), decoded);
        }
    };
}
accounts!(legacy_create_pool_accounts, a::get_create_pool_accounts, 13, 3, 4, 10);
accounts!(customizable_accounts, a::get_create_customizable_pool_accounts, 13, 3, 4, 10);
accounts!(customizable_v2_accounts, a::get_create_customizable_pool_v2_accounts, 14, 4, 5, 11);
accounts!(historical_permissioned_accounts, a::get_create_permissioned_pool_accounts, 15, 5, 6, 12);

fn rejects_incomplete_or_extended(data: &[u8]) {
    for length in 0..data.len() {
        assert!(i::unpack(&data[..length]).is_err(), "accepted length {length}");
    }
    let mut extra = data.to_vec();
    extra.push(0);
    assert!(i::unpack(&extra).is_err());
}

#[test]
fn current_and_historical_customizable_payloads_match_the_official_sdk_layout() {
    for (name, discriminator, seed) in [
        ("create_customizable_pool", i::CREATE_CUSTOMIZABLE_POOL, None),
        ("create_customizable_pool_v2", i::CREATE_CUSTOMIZABLE_POOL_V2, Some(513u16)),
        ("create_permissioned_pool", i::CREATE_PERMISSIONED_POOL, Some(513u16)),
    ] {
        let digest = Sha256::digest(format!("global:{name}").as_bytes());
        assert_eq!(discriminator, digest[..8]);
        // SDK instrument.ts uses u128, u8, bool, then (V2 only) u16;
        // historical permissioned source declares the same trailing u16.
        let mut wire = discriminator.to_vec();
        wire.extend(123456789012345678901234567890u128.to_le_bytes());
        wire.extend([2, 1]);
        if let Some(seed) = seed {
            wire.extend(seed.to_le_bytes());
        }
        let (params, decoded_seed, payload) = match i::unpack(&wire).unwrap() {
            i::RaydiumClmmInstruction::CreateCustomizablePool(value) => {
                let bytes = borsh::to_vec(&value).unwrap();
                (value.customizable_params, None, bytes)
            }
            i::RaydiumClmmInstruction::CreateCustomizablePoolV2(value) => {
                let bytes = borsh::to_vec(&value).unwrap();
                (value.customizable_params, Some(value.seed_index), bytes)
            }
            i::RaydiumClmmInstruction::CreatePermissionedPool(value) => {
                let bytes = borsh::to_vec(&value).unwrap();
                (value.customizable_params, Some(value.seed_index), bytes)
            }
            other => panic!("unexpected {other:?}"),
        };
        assert_eq!(params.sqrt_price_x64, 123456789012345678901234567890);
        assert_eq!(params.collect_fee_on, i::CollectFeeOn::Token1Only);
        assert!(params.enable_dynamic_fee);
        assert_eq!(decoded_seed, seed);
        assert_eq!(payload, wire[8..]);
        rejects_incomplete_or_extended(&wire);
        let mut invalid_fee = wire.clone();
        invalid_fee[24] = 3;
        assert!(i::unpack(&invalid_fee).is_err());
        let mut invalid_bool = wire;
        invalid_bool[25] = 2;
        assert!(i::unpack(&invalid_bool).is_err());
    }
}

#[test]
fn legacy_create_pool_payload_is_unchanged() {
    let args = i::CreatePoolInstruction {
        sqrt_price_x64: 1 << 64,
        open_time: 123,
    };
    let mut wire = i::CREATE_POOL.to_vec();
    wire.extend(borsh::to_vec(&args).unwrap());
    assert_eq!(i::unpack(&wire).unwrap(), i::RaydiumClmmInstruction::CreatePool(args));
    rejects_incomplete_or_extended(&wire);
}
