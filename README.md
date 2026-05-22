# tdp-solana

## 1. Project Overview

**tdp-solana** is a token vesting program on Solana. It lets a project lock tokens on-chain for a recipient (an investor, team member, or partner) and automatically releases those tokens over time according to a schedule — with an optional cliff period before any tokens unlock and linear vesting after that. This removes the need to trust either party: the smart contract holds the tokens and enforces the schedule.

It supports three distribution patterns: **cliff + linear vesting** (time-based), **milestone-based vesting** (tokens unlock all at once when the creator marks a milestone as reached, not by time), and **cancellation** (the creator can revoke a stream — vested tokens go to the recipient, still-locked tokens return to the creator).

**Tech stack:**
- **Rust + [Anchor](https://www.anchor-lang.com/)** — on-chain program
- **SPL Token** — standard Solana token interface
- **TypeScript** — integration tests (Mocha + Chai)

---

## 2. Prerequisites

You'll need the following installed on your machine:

- **Rust** (with `cargo`) — install via [rustup.rs](https://rustup.rs)
- **Solana CLI** — [install guide](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor CLI** `v0.31.x` (project uses `anchor-lang = "1.0.2"`) — install via [avm](https://www.anchor-lang.com/docs/installation)
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
git clone https://github.com/mancer-team2/programs.git
cd programs
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

The test suite is written in Rust: pure-logic unit tests plus LiteSVM integration tests.

```bash
cargo test
```

The LiteSVM integration tests in `tests/test_vesting_flow.rs` exercise the program end-to-end and
need the compiled program, so build it first:

```bash
anchor build   # generates target/deploy/tdp_solana.so
cargo test
```

If the `.so` is not present, the LiteSVM tests skip themselves automatically and the pure-logic unit
tests still run. `anchor test` also works — it runs the configured `cargo test` script (see
`[scripts]` in `Anchor.toml`).

---

## 6. Project Structure

The on-chain program lives under `programs/tdp-solana/src/`:

```
programs/tdp-solana/src/
├── lib.rs                  # Program entry point — declares program ID and registers the 5 instructions
├── error.rs                # Custom error codes returned by the program
├── instructions/
│   ├── mod.rs              # Module index for all instruction handlers
│   ├── create_stream.rs    # Creates a vesting stream: locks tokens in a PDA escrow (cliff + linear, or milestone)
│   ├── withdraw.rs         # Recipient claims vested tokens (time-based or milestone)
│   ├── cancel_stream.rs    # Creator cancels: vested → recipient, still-locked → creator
│   ├── close_stream.rs     # Closes the drained escrow + stream accounts and reclaims rent
│   └── set_milestone.rs    # Creator flips the milestone flag to unlock a milestone-based stream
└── state/
    ├── mod.rs              # Module index for account state types
    └── stream.rs           # `Stream` account — recipient, amounts, schedule, cliff, milestone flags, withdrawn amount
```

### Instructions (5 total)

| Instruction       | What it does                                                                                  |
| ----------------- | --------------------------------------------------------------------------------------------- |
| `create_stream`   | Sender locks tokens in a PDA-owned escrow and defines the schedule: cliff + linear vesting, or milestone-based. |
| `withdraw`        | Recipient withdraws whatever has vested — by elapsed time, or all-at-once after the milestone is reached. |
| `set_milestone`   | Creator marks a milestone-based stream as reached, unlocking the full amount for withdrawal.   |
| `cancel_stream`   | Creator cancels the stream; vested tokens go to the recipient, still-locked tokens return to the creator. |
| `close_stream`    | Closes the drained escrow and stream accounts and reclaims their rent lamports once fully settled. |

The escrow is a **PDA (Program Derived Address)** — meaning only this program can move the locked tokens, and the release schedule is enforced by program logic rather than by any private key.
