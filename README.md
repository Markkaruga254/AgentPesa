# AgentPesa

**Programmable spending controls for autonomous AI agents on Solana.**

AgentPesa is an experimental, non-custodial payment-control layer. A user funds a program-derived vault, authorizes a delegated agent session, and defines on-chain spending limits. The agent can request provider payments, but the Solana program—not the AI model—enforces the authorization rules.

> **Status:** Early development scaffold. Not audited. Devnet only. Do not use real funds.

## The problem

AI agents can choose tools and services, but giving an agent an unrestricted private key is unsafe. AgentPesa explores a safer model: give an agent a narrowly scoped capability to spend from a vault under rules enforced by a Solana program.

## Intended flow

1. A wallet owner creates an AgentPesa account and its vault.
2. The owner funds the vault with Devnet SOL.
3. The owner registers approved service providers.
4. The owner creates a time-limited session for a delegated agent signer, with a total budget and per-payment cap.
5. The agent requests a payment to a registered provider.
6. The program checks the session, provider, expiry, per-payment limit, remaining budget, and vault balance before transferring SOL.
7. A receipt account records the authorized payment on-chain.
8. The owner can revoke a session and withdraw unspent funds.

The model is **AI proposes; the program disposes**. The model never receives the owner's wallet secret.

## Repository layout

```text
programs/agent_pesa/   Anchor program (Rust)
tests/                Anchor integration tests (to be added)
app/                  Wallet-connected web dashboard (planned)
agent/                Agent runtime and tool/payment client (planned)
docs/                 Architecture and security notes
Anchor.toml           Anchor workspace configuration
Cargo.toml            Rust workspace
package.json          JavaScript tooling
```

## Current implementation

- [x] Initial repository and Anchor workspace scaffold
- [x] Agent account initialization
- [x] PDA vault funding and owner withdrawal
- [ ] Provider allowlist
- [ ] Delegated spending sessions
- [ ] Policy-enforced provider payments and on-chain receipts
- [ ] Rust/Anchor and TypeScript integration tests
- [ ] Devnet deployment and verified program address
- [ ] Agent runtime and dashboard

No deployment address is published yet. A program ID will be generated locally and added here after the program is built and deployed.

## Prerequisites

The initial development environment targets:

- Rust stable
- Solana CLI 2.3.0
- Anchor CLI 0.32.1
- Node.js 20+
- npm

## Local setup

Clone the repository:

```bash
git clone https://github.com/Markkaruga254/AgentPesa.git
cd AgentPesa
```

Install JavaScript dependencies and build the Anchor program:

```bash
npm install
anchor build
```

The checked-in program ID is a temporary placeholder. Before building for deployment, generate a program keypair and update **both** `declare_id!` in `programs/agent_pesa/src/lib.rs` and the `[programs.devnet]` entry in `Anchor.toml` to the generated public key. Keep the generated keypair in the local Anchor build directory; never commit it.

Configure Devnet and request test SOL:

```bash
solana config set --url devnet
solana address
solana airdrop 2
solana balance
```

Build and test instructions will be expanded as the integration tests are added. Do not deploy until the program has been built, tested, and reviewed.

## Security model and limitations

- The owner wallet remains separate from the delegated agent signer.
- The vault is a PDA; the owner does not hand the agent their wallet key.
- Spending limits and provider authorization must be checked by the on-chain program on every payment.
- Session revocation must be enforced on-chain, not merely hidden in the UI.
- Off-chain API delivery is not inherently verified by Solana. A payment receipt proves that the program authorized and executed a transfer; it does **not** prove that an external API returned a useful result.
- This project is experimental and unaudited. Use Devnet only.

See [docs/architecture.md](docs/architecture.md) and [SECURITY.md](SECURITY.md).

## License

MIT. See [LICENSE](LICENSE).
