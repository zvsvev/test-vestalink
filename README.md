# Solana Distribution Protocol

Anchor scaffold for a Solana token distribution protocol that combines vesting, streaming, milestone unlocks, self-service withdrawals, and administrative cancellation into one program.

The Week 2 architecture uses one isolated `VestingState` PDA per funder, recipient, and token mint. Each state account points at its own vault token account, which avoids a global shared vault and keeps recipient schedules independently addressable.

## Major Concepts

- `VestingState`: on-chain schedule state for one recipient allocation.
- `vault_token_account`: isolated token custody account for the recipient allocation.
- `authority_funder`: wallet that creates and funds the schedule.
- `authority_milestone`: authority that will later unlock milestone-based allocations.
- `authority_revoker`: authority that will later cancel a schedule and claw back unvested funds.
- `treasury_return_address`: fixed destination for future cancellation clawbacks.
- `DistributionKind`: enum covering stream, linear vesting, cliff vesting, and milestone distributions.

The current Week 4 scaffold intentionally keeps instruction handlers empty while preserving the account structure and IDL surface developers need next:

- `create_stream`
- `withdraw`
- `cancel`

## Prerequisites

- Node.js 20 or newer
- npm
- Rust stable
- Solana CLI / Agave CLI
- Anchor CLI `0.32.0`

Check local versions:

```bash
node --version
npm --version
rustc --version
solana --version
anchor --version
```

Create a local Solana wallet if you do not already have one:

```bash
solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
```

## Setup

```bash
npm install
```

The Rust dependency lockfile pins a few transitive crates to versions compatible with the Solana SBPF Rust toolchain used by Anchor `0.32.0`.

## Build

```bash
npm run build
```

This runs:

```bash
anchor build
```

## Test

```bash
npm test
```

The test suite starts an Anchor local validator on port `8900` to avoid conflicts with any validator already running on the default `8899` port. The included integration test calls `create_stream` and verifies the `VestingState` PDA is created.

## Deploy To Devnet

Set your Solana CLI to devnet and make sure the wallet has devnet SOL:

```bash
solana config set --url devnet
solana airdrop 2
```

Build and deploy:

```bash
npm run build
anchor keys sync
npm run build
npm run deploy:devnet
```

`anchor keys sync` keeps `declare_id!`, `Anchor.toml`, and the generated local program keypair aligned. Run it whenever a new `target/deploy/*-keypair.json` file is generated.

## CI

GitHub Actions runs on every push and pull request:

```bash
npm ci
npm run build
npm test
```

The workflow also creates a CI-only Solana keypair before running Anchor tests.
