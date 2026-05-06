//! OKX Dex v2 human-readable program log parsing helpers.

use crate::{
    common::ParseError,
    okx::v2::{events::SwapEvent, instructions::Dex},
};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OkxV2Log {
    Swap(SwapEvent),
    RouteAmmPool(RouteAmmPoolLog),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteAmmPoolLog {
    pub dex: Dex,
    pub amount_in: u64,
    pub offset: u64,
    pub amm_pool: Pubkey,
}

pub fn unpack(line: &str) -> Result<OkxV2Log, ParseError> {
    let line = strip_program_log_prefix(line);
    let Some(event) = line.strip_prefix("SwapEvent { ").and_then(|value| value.strip_suffix(" }")) else {
        return Ok(OkxV2Log::Unknown);
    };

    let mut dex = None;
    let mut amount_in = None;
    let mut amount_out = None;

    for part in event.split(", ") {
        let Some((name, value)) = part.split_once(": ") else {
            return Ok(OkxV2Log::Unknown);
        };

        match name {
            "dex" => dex = Some(Dex::from_str(value)?),
            "amount_in" => amount_in = value.parse::<u64>().ok(),
            "amount_out" => amount_out = value.parse::<u64>().ok(),
            _ => {}
        }
    }

    match (dex, amount_in, amount_out) {
        (Some(dex), Some(amount_in), Some(amount_out)) => Ok(OkxV2Log::Swap(SwapEvent { dex, amount_in, amount_out })),
        _ => Ok(OkxV2Log::Unknown),
    }
}

pub fn unpack_route_amm_pool(dex_line: &str, pool_line: &str) -> Result<OkxV2Log, ParseError> {
    let dex_line = strip_program_log_prefix(dex_line);
    let pool_line = strip_program_log_prefix(pool_line);

    let Some(route) = dex_line.strip_prefix("Dex::") else {
        return Ok(OkxV2Log::Unknown);
    };
    let Some((dex, fields)) = route.split_once(" amount_in: ") else {
        return Ok(OkxV2Log::Unknown);
    };
    let Some((amount_in, offset)) = fields.split_once(", offset: ") else {
        return Ok(OkxV2Log::Unknown);
    };

    let Ok(amm_pool) = Pubkey::from_str(pool_line) else {
        return Ok(OkxV2Log::Unknown);
    };

    Ok(OkxV2Log::RouteAmmPool(RouteAmmPoolLog {
        dex: Dex::from_str(dex)?,
        amount_in: amount_in.parse().map_err(|_| ParseError::InvalidLength {
            expected: 0,
            got: amount_in.len(),
        })?,
        offset: offset.parse().map_err(|_| ParseError::InvalidLength {
            expected: 0,
            got: offset.len(),
        })?,
        amm_pool,
    }))
}

pub fn unpack_route_amm_pools(lines: &[&str]) -> Result<Vec<RouteAmmPoolLog>, ParseError> {
    let mut pools = Vec::new();

    for pair in lines.windows(2) {
        if let Ok(OkxV2Log::RouteAmmPool(log)) = unpack_route_amm_pool(pair[0], pair[1]) {
            pools.push(log);
        }
    }

    Ok(pools)
}

fn strip_program_log_prefix(line: &str) -> &str {
    line.strip_prefix("Program log: ").unwrap_or(line)
}
