# Stellar Simple Voting Contract
<img width="2879" height="1559" alt="Screenshot from 2026-04-13 13-31-29" src="https://github.com/user-attachments/assets/a104ec0d-34b9-4131-adb6-13d298d52e35" />


[![Soroban](https://img.shields.io/badge/Soroban-Smart%20Contract-blue?logo=stellar)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green)](LICENSE)
[![Network](https://img.shields.io/badge/Network-Stellar%20Testnet-purple)](https://developers.stellar.org/docs/fundamentals-and-concepts/testnet-and-pubnet)

A minimalist, fully on-chain **Yes/No voting smart contract** built on the [Stellar](https://stellar.org) blockchain using [Soroban](https://soroban.stellar.org) — Stellar's smart contract platform. Each Stellar address can cast exactly one vote, and results are transparent and verifiable by anyone at any time.

---

## Deployed Contract Adress
[Click Here](https://stellar.expert/explorer/testnet/contract/CCZJZ3TAZ2J7HHY2723SGUEPW22EJK22ZTRQX4RDFQ64MVADWBT3NWDY)


---

## Table of Contents

- [Features](#features)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
- [Contract Functions](#contract-functions)
- [Contract Storage](#contract-storage)
- [Prerequisites](#prerequisites)
- [Installation & Build](#installation--build)
- [Running Tests](#running-tests)
- [Deployment](#deployment)
- [Usage](#usage)
- [Security Notes](#security-notes)
- [Dependencies](#dependencies)
- [License](#license)
- [Author](#author)

---

## Features

- **Simple Yes/No Voting**: Users vote `true` (Yes) or `false` (No) on a single proposal
- **One Vote Per Address**: The contract enforces a strict one-vote-per-address rule
- **Tamper-proof Results**: Vote counts are stored on-chain; anyone can query them
- **Authentication Required**: Uses Soroban's `require_auth()` so only the rightful account holder can cast their vote
- **Fully Permissionless**: No owner, no admin — any Stellar account can participate
- **Lightweight & Efficient**: Compiled to WASM, optimized for minimal execution cost

---

## Project Structure

```
stellar-simple-voting-contract/
├── contracts/
│   └── contract/
│       ├── src/
│       │   ├── lib.rs       # Core contract logic (vote & result functions)
│       │   └── test.rs      # Comprehensive unit tests
│       ├── Cargo.toml       # Contract package manifest
│       └── Makefile         # Build, test, and format shortcuts
├── Cargo.toml               # Workspace configuration & shared dependencies
└── README.md
```

---

## How It Works

```
  Voter (Stellar Address)
         │
         │  vote(voter, true/false)
         ▼
  ┌─────────────────────────────────┐
  │       VotingContract            │
  │                                 │
  │  1. require_auth(voter)         │  ← Ensures only the real account votes
  │  2. Check HasVoted(voter)       │  ← Prevent double-voting
  │  3. Increment YesVotes/NoVotes  │  ← Tally the vote
  │  4. Set HasVoted(voter) = true  │  ← Mark voter as done
  └─────────────────────────────────┘
         │
         │  result() → (yes: u32, no: u32)
         ▼
   Anyone can read the live tally
```

---

## Contract Functions

### `vote(env: Env, voter: Address, support: bool)`

Casts a vote on behalf of the given `voter` address.

| Parameter | Type      | Description                              |
|-----------|-----------|------------------------------------------|
| `voter`   | `Address` | The Stellar address casting the vote     |
| `support` | `bool`    | `true` = Yes vote · `false` = No vote    |

**Behaviour:**
- Calls `voter.require_auth()` — the transaction must be signed by `voter`'s keypair
- Panics with `"already voted"` if the address has already voted
- Atomically increments the correct counter and records the address

---

### `result(env: Env) -> (u32, u32)`

Returns the current vote tally. Read-only, costs minimal fees.

**Returns:** `(yes_votes, no_votes)` — a tuple of unsigned 32-bit integers.

---

## Contract Storage

All state lives in Soroban **instance storage** (tied to the contract instance lifetime):

| Key                   | Type   | Description                              |
|-----------------------|--------|------------------------------------------|
| `YesVotes`            | `u32`  | Running count of Yes votes               |
| `NoVotes`             | `u32`  | Running count of No votes                |
| `HasVoted(Address)`   | `bool` | Per-address flag — `true` if voted       |

---

## Prerequisites

| Tool            | Version  | Install                                                      |
|-----------------|----------|--------------------------------------------------------------|
| Rust + Cargo    | 1.80+    | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Stellar CLI     | latest   | `cargo install --locked stellar-cli --features opt`          |
| WASM target     | —        | `rustup target add wasm32v1-none`                            |

---

## Installation & Build

```bash
# 1. Clone the repository
git clone https://github.com/anindhabiswas25/Sellar-Simple-Voting-Contract.git
cd Sellar-Simple-Voting-Contract

# 2. Add the WASM compilation target (first time only)
rustup target add wasm32v1-none

# 3. Build the contract
stellar contract build
# or
make build
```

The compiled WASM binary will be at:
```
target/wasm32v1-none/release/contract.wasm
```

---

## Running Tests

```bash
# Run all unit tests
cargo test
# or
make test
```

**Test coverage:**

| Test                     | What it verifies                                          |
|--------------------------|-----------------------------------------------------------|
| `test_vote_yes`          | A single Yes vote increments the Yes counter              |
| `test_vote_no`           | A single No vote increments the No counter                |
| `test_double_vote_panics`| Voting twice with the same address panics correctly       |
| `test_multiple_voters`   | Multiple distinct voters are tallied independently        |
| `test_initial_result_is_zero` | Fresh contract starts at (0, 0)                     |

Expected output:
```
running 5 tests
test test_initial_result_is_zero ... ok
test test_vote_yes ... ok
test test_vote_no ... ok
test test_double_vote_panics ... ok
test test_multiple_voters ... ok

test result: ok. 5 passed; 0 failed
```

---

## Deployment

### 1. Generate / Fund a Test Key

```bash
stellar keys generate dev --network testnet --fund
```

### 2. Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source-account dev \
  --network testnet
```

This prints your contract address, e.g. `CXXXXXXXXXX...`. Save it — you'll need it for invocations.

---

## Usage

### Via Stellar CLI

```bash
export CONTRACT=CXXXXXXXXXX   # your deployed contract address
export VOTER=$(stellar keys address dev)

# Cast a Yes vote
stellar contract invoke \
  --id $CONTRACT \
  --source-account dev \
  --network testnet \
  -- vote \
  --voter $VOTER \
  --support true

# Cast a No vote (different account)
stellar contract invoke \
  --id $CONTRACT \
  --source-account alice \
  --network testnet \
  -- vote \
  --voter $(stellar keys address alice) \
  --support false

# Query current results  (no auth needed)
stellar contract invoke \
  --id $CONTRACT \
  --source-account dev \
  --network testnet \
  -- result
# Output: [2, 1]  (yes, no)
```

### Via JavaScript SDK

```typescript
import {
  Contract,
  SorobanRpc,
  TransactionBuilder,
  Networks,
  Keypair,
  Address,
  nativeToScVal,
} from "@stellar/stellar-sdk";

const rpcUrl = "https://soroban-testnet.stellar.org";
const server = new SorobanRpc.Server(rpcUrl);
const contract = new Contract("CXXXXXXXXXX");

// --- Cast a vote ---
const keypair = Keypair.fromSecret("S...");
const account = await server.getAccount(keypair.publicKey());

const tx = new TransactionBuilder(account, {
  fee: "100",
  networkPassphrase: Networks.TESTNET,
})
  .addOperation(
    contract.call(
      "vote",
      new Address(keypair.publicKey()).toScVal(),
      nativeToScVal(true, { type: "bool" })
    )
  )
  .setTimeout(30)
  .build();

const prepared = await server.prepareTransaction(tx);
prepared.sign(keypair);
const result = await server.sendTransaction(prepared);
console.log("Vote tx hash:", result.hash);

// --- Read results (no signing needed) ---
const resultTx = new TransactionBuilder(account, {
  fee: "100",
  networkPassphrase: Networks.TESTNET,
})
  .addOperation(contract.call("result"))
  .setTimeout(30)
  .build();

const sim = await server.simulateTransaction(resultTx);
console.log("Results (yes, no):", sim.result?.retval);
```

---

## Security Notes

| Concern | Mitigation |
|---|---|
| Double voting | `HasVoted(Address)` flag checked before every vote; panics on repeat |
| Impersonation | `require_auth()` enforces that only the key holder can vote as themselves |
| Admin takeover | No owner or admin role — the contract is fully immutable and permissionless |
| Integer overflow | `overflow-checks = true` in the release profile catches overflows at runtime |

---

## Dependencies

```toml
[workspace.dependencies]
soroban-sdk = "25"   # Soroban Smart Contract SDK (latest stable)
```

---

## License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

## Author

**Anindha Biswas** — [@anindhabiswas25](https://github.com/anindhabiswas25)

---

## Contributing

Contributions, issues and feature requests are welcome!
Feel free to open an [issue](https://github.com/anindhabiswas25/Sellar-Simple-Voting-Contract/issues) or submit a pull request.

1. Fork the repo
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request
