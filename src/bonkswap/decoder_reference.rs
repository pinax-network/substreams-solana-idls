use common::solana::{get_fee_payer, get_signers};
use proto::pb::bonk::swap::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

// BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p
const BONKSWAP_PROGRAM_ID: [u8; 32] = [
    155, 58, 93, 153, 133, 205, 99, 220, 200, 98, 145, 180, 142, 67, 70, 214, 157, 171, 54, 99, 242, 49, 240, 199, 95, 111, 196, 132, 118, 236, 111, 125,
];

const SWAP_DISCRIMINATOR: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

struct SwapData {
    delta_in: u64,
    price_limit: u128,
    x_to_y: bool,
}

fn decode_swap_instruction(data: &[u8]) -> Option<SwapData> {
    if data.len() < 33 || data[..8] != SWAP_DISCRIMINATOR {
        return None;
    }
    let delta_in = u64::from_le_bytes(data[8..16].try_into().ok()?);
    let price_limit = u128::from_le_bytes(data[16..32].try_into().ok()?);
    let x_to_y = data.get(32).copied()? != 0;
    Some(SwapData { delta_in, price_limit, x_to_y })
}

fn get_swap_accounts(instruction: &InstructionView) -> pb::SwapAccounts {
    pb::SwapAccounts {
        pool: instruction.accounts()[2 - 1].0.to_vec(),
        token_x: instruction.accounts()[3 - 1].0.to_vec(),
        token_y: instruction.accounts()[4 - 1].0.to_vec(),
        pool_x_account: instruction.accounts()[5 - 1].0.to_vec(),
        pool_y_account: instruction.accounts()[6 - 1].0.to_vec(),
        swapper_x_account: instruction.accounts()[7 - 1].0.to_vec(),
        swapper_y_account: instruction.accounts()[8 - 1].0.to_vec(),
        swapper: instruction.accounts()[9 - 1].0.to_vec(),
        referrer_x_account: instruction.accounts().get(10 - 1).map(|a| a.0.to_vec()),
        referrer_y_account: instruction.accounts().get(11 - 1).map(|a| a.0.to_vec()),
        referrer: instruction.accounts().get(12 - 1).map(|a| a.0.to_vec()),
    }
}
