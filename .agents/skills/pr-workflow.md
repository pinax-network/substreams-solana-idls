# Skill: PR Workflow

## Branch Naming

- `feat/{protocol}-decoder` — New protocol decoders
- `fix/{description}` — Bug fixes
- `docs/{description}` — Documentation only
- `refactor/{description}` — Code reorganization

## Before Opening a PR

1. **Rebase on latest main:**
   ```bash
   git fetch origin main
   git rebase origin/main
   ```

2. **Lint, build, and test:**
   ```bash
   cargo clippy --workspace -- -D warnings
   cargo test --workspace
   cargo check --workspace --target wasm32-unknown-unknown
   ```

## Opening a PR

- Always add `DenisCarriere` as reviewer
- Use descriptive PR titles: `feat: add Raydium LaunchPad decoder`
- Include a summary of what's decoded (instructions, events)
- List the Program ID(s)

## After Opening a PR

- Wait for merge before opening the next PR
- If CI fails, fix and force-push to the same branch

## Issues

- Use `pax` label for issues that should be auto-picked up by AI agents
- Assign `DenisCarriere` on issues needing human feedback

## Releases

- Tag format: `v{major}.{minor}.{patch}`
- Releases auto-publish to crates.io via GitHub Actions
- Update the coverage table in root `README.md` when adding protocols
