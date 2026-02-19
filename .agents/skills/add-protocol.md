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

### Option 2: Source code
If no IDL is available (non-Anchor programs), you'll need to:
- Read the program source code
- Identify instruction discriminators (first 8 bytes of `sha256("global:<instruction_name>")`)
- Define structs matching the instruction/event data layout
- Implement `borsh` deserialization manually

### Option 3: Dead/closed-source programs
Mark as 💀 in the README. Skip or implement based on reverse-engineering transaction data.

## Steps

### 1. Create package directory

```bash
mkdir -p packages/{protocol}/src
```

### 2. Create Cargo.toml

```toml
[package]
name = "{protocol}"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
common = { path = "../common" }
substreams = { workspace = true }
substreams-solana = { workspace = true }
solana-program = { workspace = true }
borsh = { workspace = true }

[dev-dependencies]
base64 = { workspace = true }
```

### 3. Implement decoders

Standard file structure:
```
packages/{protocol}/src/
├── lib.rs              — Module declarations + re-exports
├── instructions.rs     — Instruction enum + borsh deserialization
├── events.rs           — Event structs + deserialization
└── accounts.rs         — Account structs (if needed)
```

#### Instruction decoder pattern
```rust
use borsh::BorshDeserialize;

#[derive(Debug, BorshDeserialize)]
pub struct SwapArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

pub fn decode_instruction(data: &[u8]) -> Option<Instruction> {
    let discriminator = &data[..8];
    let args_data = &data[8..];
    
    match discriminator {
        SWAP_DISCRIMINATOR => {
            let args = SwapArgs::try_from_slice(args_data).ok()?;
            Some(Instruction::Swap(args))
        }
        _ => None,
    }
}
```

#### Event decoder pattern (Anchor events)
```rust
// Anchor events have an 8-byte discriminator: sha256("event:<EventName>")[..8]
pub fn decode_event(data: &[u8]) -> Option<Event> {
    let discriminator = &data[..8];
    let event_data = &data[8..];
    
    match discriminator {
        SWAP_EVENT_DISCRIMINATOR => {
            let event = SwapEvent::try_from_slice(event_data).ok()?;
            Some(Event::Swap(event))
        }
        _ => None,
    }
}
```

### 4. Add to workspace

In root `Cargo.toml`:
```toml
[workspace]
members = [
    # ... existing members
    "packages/{protocol}",
]

[dependencies]
{protocol} = { version = "0.1.0", path = "packages/{protocol}" }
```

### 5. Add to root re-exports

In `src/lib.rs`:
```rust
pub use {protocol};
```

### 6. Create README.md

**Required:** Create `packages/{protocol}/README.md` with:
- Protocol name and description
- Program ID(s)
- Key instructions decoded
- Key events decoded
- Links to protocol docs/source

### 7. Add tests

Create tests with real base64-encoded transaction data:
```rust
#[cfg(test)]
mod tests {
    use base64::Engine;
    
    #[test]
    fn test_decode_swap() {
        let data = base64::engine::general_purpose::STANDARD
            .decode("BASE64_INSTRUCTION_DATA")
            .unwrap();
        let instruction = decode_instruction(&data).unwrap();
        // assert fields...
    }
}
```

### 8. Build and Test

```bash
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

### 9. Update root README.md

Add the protocol to the coverage table in the root `README.md`.

### 10. Commit and PR

- Branch naming: `feat/{protocol}-decoder`
- Add `DenisCarriere` as reviewer
- Use `pax` label for issues

## Common patterns

### Discriminator calculation
```rust
use solana_program::hash::hash;

// Anchor instruction discriminator
let disc = &hash(b"global:swap").to_bytes()[..8];

// Anchor event discriminator  
let disc = &hash(b"event:SwapEvent").to_bytes()[..8];
```

### Using the common crate
The `common` crate provides shared utilities. Check `packages/common/src/` for helpers like discriminator macros, account parsing, etc.
