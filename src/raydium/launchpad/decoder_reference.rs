use proto::pb::raydium::launchpad::v1 as pb;
use substreams_solana::block_view::InstructionView;

// LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
pub const RAYDIUM_LAUNCHPAD_PROGRAM_ID: [u8; 32] = [
    5, 4, 59, 149, 77, 202, 38, 225, 239, 145, 181, 44, 79, 143, 137, 175, 138, 111, 90, 200, 198, 33, 86, 241, 113, 207, 15, 33, 172, 81, 201, 34,
];

pub const BUY_EXACT_IN_DISCRIMINATOR: [u8; 8] = [250, 234, 13, 123, 213, 156, 19, 236];
pub const BUY_EXACT_OUT_DISCRIMINATOR: [u8; 8] = [24, 211, 116, 40, 105, 3, 153, 56];
pub const SELL_EXACT_IN_DISCRIMINATOR: [u8; 8] = [149, 39, 222, 155, 211, 124, 152, 26];
pub const SELL_EXACT_OUT_DISCRIMINATOR: [u8; 8] = [95, 200, 71, 34, 8, 9, 11, 166];
pub const TRADE_EVENT_DISCRIMINATOR: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];

pub fn decode_buy_exact_in(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    if data.len() < 32 || data[..8] != BUY_EXACT_IN_DISCRIMINATOR {
        return None;
    }
    let amount_in = u64::from_le_bytes(data[8..16].try_into().ok()?);
    let minimum_amount_out = u64::from_le_bytes(data[16..24].try_into().ok()?);
    let share_fee_rate = u64::from_le_bytes(data[24..32].try_into().ok()?);
    Some(pb::instruction::Instruction::BuyExactIn(pb::BuyExactInInstruction {
        accounts: Some(get_trade_accounts(ix)),
        amount_in,
        minimum_amount_out,
        share_fee_rate,
    }))
}

pub fn decode_buy_exact_out(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    if data.len() < 32 || data[..8] != BUY_EXACT_OUT_DISCRIMINATOR {
        return None;
    }
    let amount_out = u64::from_le_bytes(data[8..16].try_into().ok()?);
    let maximum_amount_in = u64::from_le_bytes(data[16..24].try_into().ok()?);
    let share_fee_rate = u64::from_le_bytes(data[24..32].try_into().ok()?);
    Some(pb::instruction::Instruction::BuyExactOut(pb::BuyExactOutInstruction {
        accounts: Some(get_trade_accounts(ix)),
        amount_out,
        maximum_amount_in,
        share_fee_rate,
    }))
}

pub fn decode_sell_exact_in(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    if data.len() < 32 || data[..8] != SELL_EXACT_IN_DISCRIMINATOR {
        return None;
    }
    let amount_in = u64::from_le_bytes(data[8..16].try_into().ok()?);
    let minimum_amount_out = u64::from_le_bytes(data[16..24].try_into().ok()?);
    let share_fee_rate = u64::from_le_bytes(data[24..32].try_into().ok()?);
    Some(pb::instruction::Instruction::SellExactIn(pb::SellExactInInstruction {
        accounts: Some(get_trade_accounts(ix)),
        amount_in,
        minimum_amount_out,
        share_fee_rate,
    }))
}

pub fn decode_sell_exact_out(data: &[u8], ix: &InstructionView) -> Option<pb::instruction::Instruction> {
    if data.len() < 32 || data[..8] != SELL_EXACT_OUT_DISCRIMINATOR {
        return None;
    }
    let amount_out = u64::from_le_bytes(data[8..16].try_into().ok()?);
    let maximum_amount_in = u64::from_le_bytes(data[16..24].try_into().ok()?);
    let share_fee_rate = u64::from_le_bytes(data[24..32].try_into().ok()?);
    Some(pb::instruction::Instruction::SellExactOut(pb::SellExactOutInstruction {
        accounts: Some(get_trade_accounts(ix)),
        amount_out,
        maximum_amount_in,
        share_fee_rate,
    }))
}

pub fn decode_trade_event(data: &[u8]) -> Option<pb::instruction::Instruction> {
    if data.len() < 8 || data[..8] != TRADE_EVENT_DISCRIMINATOR {
        return None;
    }
    let mut idx = 8; // skip discriminator

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
    let total_base_sell = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let virtual_base = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let virtual_quote = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let real_base_before = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let real_quote_before = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let real_base_after = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let real_quote_after = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let amount_in = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let amount_out = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let protocol_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let platform_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let creator_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let share_fee = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let trade_direction = match take(data, &mut idx, 1)?[0] {
        0 => pb::TradeDirection::Buy as i32,
        1 => pb::TradeDirection::Sell as i32,
        _ => return None,
    };
    let pool_status = match take(data, &mut idx, 1)?[0] {
        0 => pb::PoolStatus::Fund as i32,
        1 => pb::PoolStatus::Migrate as i32,
        2 => pb::PoolStatus::Trade as i32,
        _ => return None,
    };
    let exact_in = take(data, &mut idx, 1)?[0] != 0;

    Some(pb::instruction::Instruction::TradeEvent(pb::TradeEvent {
        pool_state,
        total_base_sell,
        virtual_base,
        virtual_quote,
        real_base_before,
        real_quote_before,
        real_base_after,
        real_quote_after,
        amount_in,
        amount_out,
        protocol_fee,
        platform_fee,
        creator_fee,
        share_fee,
        trade_direction,
        pool_status,
        exact_in,
    }))
}

pub fn get_trade_accounts(ix: &InstructionView) -> pb::TradeAccounts {
    pb::TradeAccounts {
        payer: ix.accounts()[0].0.to_vec(),
        authority: ix.accounts()[1].0.to_vec(),
        global_config: ix.accounts()[2].0.to_vec(),
        platform_config: ix.accounts()[3].0.to_vec(),
        pool_state: ix.accounts()[4].0.to_vec(),
        user_base_token: ix.accounts()[5].0.to_vec(),
        user_quote_token: ix.accounts()[6].0.to_vec(),
        base_vault: ix.accounts()[7].0.to_vec(),
        quote_vault: ix.accounts()[8].0.to_vec(),
        base_token_mint: ix.accounts()[9].0.to_vec(),
        quote_token_mint: ix.accounts()[10].0.to_vec(),
        base_token_program: ix.accounts()[11].0.to_vec(),
        quote_token_program: ix.accounts()[12].0.to_vec(),
        event_authority: ix.accounts()[13].0.to_vec(),
        program: ix.accounts()[14].0.to_vec(),
    }
}
