# tdp-solana

## 1. Project Overview

**tdp-solana** is a token vesting program on Solana. It lets a project lock tokens on-chain for a recipient (an investor, team member, or partner) and automatically releases those tokens over time according to a schedule — with an optional cliff period before any tokens unlock and linear vesting after that. This removes the need to trust either party: the smart contract holds the tokens and enforces the schedule.

**Tech stack:**
- **Rust + [Anchor](https://www.anchor-lang.com/)** — on-chain program
- **SPL Token** — standard Solana token interface
- **TypeScript** — integration tests (Mocha + Chai)

---

## 2. Prerequisites

You'll need the following installed on your machine:

- **Rust** (with `cargo`) — install via [rustup.rs](https://rustup.rs)
- **Solana CLI** — [install guide](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor CLI** `v0.31.x` (project uses `anchor-lang = "1.0.0"`) — install via [avm](https://www.anchor-lang.com/docs/installation)
- **Node.js** (v18+) and **yarn** (or npm)

Verify each tool:

```bash
rustc --version
solana --version
anchor --version
node --version
yarn --version
```

---

## 3. Setup Steps

### Clone the repository

```bash
git clone https://github.com/mancer-team2/tdp-solana.git
cd tdp-solana
```

### Install JavaScript dependencies

```bash
yarn install
```

### Build the program

```bash
anchor build
```

This compiles the Rust program and generates a keypair at `target/deploy/tdp_solana-keypair.json`.

### Sync the program ID

After the first build, copy the generated program ID into the source code:

```bash
anchor keys sync
```

Or do it manually:

```bash
solana address -k target/deploy/tdp_solana-keypair.json
```

Then paste the resulting address into both:

- `programs/tdp-solana/src/lib.rs` → inside `declare_id!("...")`
- `Anchor.toml` → under `[programs.localnet]` and `[programs.devnet]`

Rebuild after syncing:

```bash
anchor build
```

---

## 4. Deploying to Devnet

### Point the Solana CLI at devnet

```bash
solana config set --url devnet
```

### Fund your wallet with devnet SOL

```bash
solana airdrop 2
```

(If the airdrop is rate-limited, try [faucet.solana.com](https://faucet.solana.com).)

### Deploy

```bash
anchor deploy --provider.cluster devnet
```

The CLI will print the deployed program ID once it's confirmed.

---

## 5. Running Tests

```bash
anchor test
```

`anchor test` automatically spins up a local Solana validator (`solana-test-validator`), deploys the program to it, runs the TypeScript test suite against the local node, and shuts everything down when finished. You don't need to start a validator manually.

---

## 6. Project Structure

The on-chain program lives under `programs/tdp-solana/src/`:

```
programs/tdp-solana/src/
├── lib.rs              # Program entry point — declares program ID and registers the 4 instructions
├── constants.rs        # Shared constants (PDA seeds, etc.)
├── error.rs            # Custom error codes returned by the program
├── instructions.rs     # Re-exports the instructions module
├── instructions/
│   ├── mod.rs          # Module index for all instruction handlers
│   ├── initialize.rs   # Bootstrap helper (scaffolding)
│   └── create_stream.rs# Creates a vesting stream: locks tokens in a PDA escrow with a cliff + linear schedule
├── state.rs            # Re-exports the state module
└── state/
    ├── mod.rs          # Module index for account state types
    └── stream.rs       # `Stream` account — stores recipient, amounts, start time, cliff, duration, and withdrawn amount
```

### Instructions (4 total)

| Instruction       | What it does                                                                                  |
| ----------------- | --------------------------------------------------------------------------------------------- |
| `create_stream`   | Sender locks tokens in a PDA-owned escrow and defines the vesting schedule (cliff + duration). |
| `withdraw`        | Recipient withdraws whatever has vested so far, based on the current on-chain time.           |
| `cancel_stream`   | Sender cancels the stream; vested tokens go to the recipient, unvested tokens return to sender. |
| `close_stream`    | Closes the empty stream account and reclaims its rent lamports once fully drained.            |

The escrow is a **PDA (Program Derived Address)** — meaning only this program can move the locked tokens, and the release schedule is enforced by program logic rather than by any private key.
