# Agent Skills

Instructions for AI agents working on this repository.

## Skills

| Skill | Description |
|-------|-------------|
| [add-protocol.md](add-protocol.md) | How to add a new Solana protocol decoder |
| [build-and-test.md](build-and-test.md) | Build system, testing, common failures |
| [pr-workflow.md](pr-workflow.md) | Branch naming, PR process, releases |

## Quick Reference

### Adding a new protocol
1. Create `packages/{protocol}/` with `Cargo.toml` and `src/`
2. Get the IDL (from anchor, program repo, or reverse-engineer from source)
3. Implement instruction + event decoders using `borsh` deserialization
4. Add to workspace in root `Cargo.toml`
5. Create `packages/{protocol}/README.md` with Program ID and key instructions
6. `cargo clippy --workspace && cargo test --workspace`
7. Open PR with `DenisCarriere` as reviewer

### Project structure
```
packages/
├── common/        — Shared utilities (discriminator helpers, etc.)
├── raydium/       — Raydium AMM V4, CPMM, CLMM, StableSwap, LaunchPad
├── jupiter/       — Jupiter Aggregator V4, V6, Limit Orders, DCA
├── pumpfun/       — Pump.fun Bonding Curve + PumpSwap AMM
├── orca/          — Orca Whirlpool (CLMM)
├── meteora/       — Meteora Pools, DAMM V2, DLMM
├── phoenix/       — Phoenix on-chain CLOB
├── drift/         — Drift V2 Perpetuals (🚧)
├── pancakeswap/   — PancakeSwap AMM (🚧)
├── ...            — More protocols
└── src/           — Root crate re-exports
```
