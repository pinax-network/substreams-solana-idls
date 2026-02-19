# Skill: Add New Solana Protocol

How to add a new Solana program decoder to the repo.

## Prerequisites

- The program's IDL (Anchor IDL JSON) or source code
- The program's on-chain Program ID

## Finding the IDL

### Option 1: Anchor IDL (preferred)
If the program uses Anchor, the IDL is usually:
- Published on-chain: `anchor idl fetch <PROGRAM_ID>`
- In the program's GitHub repo under `target/idl/` or `idl/`
- On the protocol's docs site
- In `bitquery/solana-idl-lib` community collection

### Option 2: Source code / reverse engineering
If no IDL is available, reverse-engineer swap events from Solscan transaction data.

## Steps

### 1. Create module directory

```bash
mkdir -p src/{protocol}
```

### 2. Implement decoders

Standard file structure:
```
src/{protocol}/
├── mod.rs              — Module declarations + PROGRAM_ID
├── instructions.rs     — Instruction enum + `unpack(&[u8])`
├── events.rs           — Event enum + `unpack_event(&[u8])`
└── accounts.rs         — Account structs (if needed)
```

#### mod.rs pattern
```rust
use substreams_solana::b58;

pub mod instructions;
pub mod events;
pub mod accounts;

pub const PROGRAM_ID: [u8; 32] = b58!("PROGRAM_ID_HERE");
```

#### Instruction decoder pattern
```rust
use borsh::{BorshDeserialize, BorshSerialize};
use crate::common::ParseError;

pub const SWAP: [u8; 8] = [/* discriminator bytes */];

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SwapInstruction {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

pub fn unpack(data: &[u8]) -> Result<MyInstruction, ParseError> {
    if data.len() < 8 { return Err(ParseError::TooShort(data.len())); }
    let (disc, rest) = data.split_at(8);
    let disc: [u8; 8] = disc.try_into().unwrap();
    Ok(match disc {
        SWAP => MyInstruction::Swap(SwapInstruction::try_from_slice(rest)?),
        _ => return Err(ParseError::Unknown(disc)),
    })
}
```

### 3. Register in lib.rs

In `src/lib.rs`:
```rust
pub mod {protocol};
```

### 4. Add tests

Create `tests/{protocol}/instructions.rs` (and events.rs, accounts.rs as needed).
Create `tests/{protocol}.rs` wrapper:
```rust
#[path = "{protocol}/instructions.rs"]
mod {protocol}_instructions;
```

Tests use `use substreams_solana_idls::{protocol}::...`.

### 5. Build and Test

```bash
cargo test
```

### 6. Update root README.md

Add the protocol to the coverage table.

### 7. Commit and PR

- Branch naming: `feat/{protocol}`
- Add `DenisCarriere` as reviewer

## Discriminator calculation

```rust
// Anchor instruction: sha256("global:swap")[..8]
// Anchor event: sha256("event:SwapEvent")[..8]
// New Anchor IDL format (spec 0.1.0): discriminators are pre-computed in the IDL
```
