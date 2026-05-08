# TDP Solana

A token distribution protocol (vesting/streaming) built on Solana using the Anchor framework.

## Overview

TDP Solana enables creators to lock SPL tokens in an escrow account and release them to a recipient on a customizable vesting schedule — with optional cliff, cancelability, and full closure support.

### Instructions

| Instruction | Description |
|---|---|
| `create_stream` | Lock tokens into a PDA escrow with a vesting schedule |
| `withdraw` | Recipient claims vested tokens after the cliff |
| `cancel_stream` | Creator cancels (if cancelable), splits vested vs unvested |
| `close_stream` | Clean up empty escrow and stream accounts, reclaim rent |

## Prerequisites

- [Rust](https://rustup.rs) (stable, `1.75+`)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (`1.18+`)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (`0.32+`)
- [Node.js](https://nodejs.org) (`18+`) & [Yarn](https://yarnpkg.com)

Verify your installation:

```bash
rustc --version
solana --version
anchor --version
node --version
```

## Setup

```bash
# Clone the repository
git clone https://github.com/mancer-team2/programs.git
cd programs

# Install dependencies
yarn install
```

## Build

```bash
anchor build
```

A successful build produces:
- `target/deploy/tdp_solana.so` — the compiled BPF program
- `target/idl/tdp_solana.json` — the IDL for client generation

## Run Tests

```bash
anchor test
```

Or directly via Cargo:

```bash
cargo test
```

## Deploy to Devnet

```bash
# Configure Solana CLI to devnet
solana config set --url devnet

# Generate a deploy wallet if you don't have one
solana-keygen new -o ~/.config/solana/id.json

# Airdrop devnet SOL
solana airdrop 2

# Deploy
anchor deploy --provider.cluster devnet
```

After deploying, copy the program ID from the output and update it in:
- `programs/tdp-solana/src/lib.rs` (`declare_id!`)
- `Anchor.toml` (`[programs.devnet]` section)

## Project Structure

```
programs/tdp-solana/src/
├── lib.rs                          # Entry point with 4 instruction handlers
├── constants.rs                    # Seed constants
├── error.rs                        # VestingError enum
├── state.rs                        # Module re-exports
├── state/
│   └── stream.rs                   # Stream account struct (PDA)
└── instructions/
    ├── mod.rs                      # Module re-exports
    ├── create_stream.rs            # Lock tokens + init escrow
    ├── withdraw.rs                 # Claim vested tokens
    ├── cancel_stream.rs            # Cancel stream (creator-only)
    └── close_stream.rs             # Close empty accounts
```

## Stream PDA Seeds

```
["stream", creator.key, recipient.key, stream_id.to_le_bytes]
```

## Escrow Authority PDA

```
["escrow_authority", stream.key]
```
