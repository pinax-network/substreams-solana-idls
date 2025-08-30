use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_id, parse_raydium_log};
use proto::pb::raydium::clmm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta};

// CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK
const RAYDIUM_CLMM_PROGRAM_ID: [u8; 32] = [
    165, 213, 202, 158, 4, 207, 93, 181, 144, 183, 20, 186, 47, 227, 44, 177, 89, 19, 63, 193, 193, 146, 183, 34, 87, 253, 7, 211, 156, 176, 64, 30,
];

const SWAP_EVENT_DISCRIMINATOR: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];

fn decode_swap_event(data: &[u8]) -> Option<pb::SwapLog> {
    let mut idx = 0;
    fn take<'a>(data: &'a [u8], idx: &mut usize, len: usize) -> Option<&'a [u8]> {
        if *idx + len > data.len() {
            None
        } else {
            let slice = &data[*idx..*idx + len];
            *idx += len;
            Some(slice)
        }
    }

    let pool_state = take(data, &mut idx, 32)?.to_vec();
    let sender = take(data, &mut idx, 32)?.to_vec();
    let token_account_0 = take(data, &mut idx, 32)?.to_vec();
    let token_account_1 = take(data, &mut idx, 32)?.to_vec();
    let amount_0 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let transfer_fee_0 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let amount_1 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let transfer_fee_1 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let zero_for_one = take(data, &mut idx, 1)?[0] != 0;
    let sqrt_price_x64 = u128::from_le_bytes(take(data, &mut idx, 16)?.try_into().ok()?);
    let liquidity = u128::from_le_bytes(take(data, &mut idx, 16)?.try_into().ok()?);
    let tick = i32::from_le_bytes(take(data, &mut idx, 4)?.try_into().ok()?);

    Some(pb::SwapLog {
        pool_state,
        sender,
        token_account_0,
        token_account_1,
        amount_0,
        transfer_fee_0,
        amount_1,
        transfer_fee_1,
        zero_for_one,
        sqrt_price_x64: sqrt_price_x64.to_string(),
        liquidity: liquidity.to_string(),
        tick,
    })
}
