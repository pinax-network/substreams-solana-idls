# Native Programs

Solana native program instruction parsers for:

- **System** (`11111111111111111111111111111111`) — 12 instructions
- **Stake** (`Stake11111111111111111111111111111111111111`) — 13 instructions
- **Vote** (`Vote111111111111111111111111111111111111111`) — 16 instructions, 3 account types

All native programs use little-endian u32 index discriminators (first 4 bytes of instruction data).
