# AgentPesa architecture

## Product boundary

AgentPesa is a payment authorization program, not an AI wallet. The owner retains control of their wallet. The program owns the authorization state and a PDA-controlled vault. A separately generated delegated signer is intended to authorize narrowly scoped agent sessions.

## Planned on-chain accounts

| Account | Purpose |
| --- | --- |
| `AgentAccount` | Owner, vault bump, accounting and agent configuration |
| `Vault` | System-owned, data-free PDA holding native SOL |
| `ProviderAccount` | Owner-approved provider identity and payout destination |
| `SessionAccount` | Delegated signer, expiry, total budget, per-payment cap, spent amount and revocation state |
| `PaymentReceipt` | Immutable record of a successful authorized payment |

## Planned instruction flow

1. `initialize_agent`: create the owner configuration PDA.
2. `fund_vault`: transfer native SOL from the owner to the vault PDA.
3. `register_provider`: owner authorizes a provider and its payout address.
4. `create_session`: owner authorizes a delegated signer with an expiry and spending limits.
5. `pay_provider`: delegated signer requests payment; the program validates session and provider state, enforces limits, transfers from the vault and records a receipt.
6. `revoke_session`: owner invalidates a session.
7. `withdraw_vault`: owner withdraws available funds.

## Invariants for provider payments

Every payment must validate all of the following on-chain:

- The AgentAccount belongs to the owner recorded in its PDA.
- The session belongs to that agent and is active.
- The transaction signer is exactly the delegated signer recorded in the session.
- The session has not expired and has not been revoked.
- The provider account belongs to the same agent and is enabled.
- The destination account equals the provider's recorded payout address.
- The requested amount is non-zero and does not exceed the per-payment cap.
- Checked arithmetic confirms cumulative spend remains within the session budget.
- The vault has sufficient spendable balance.
- A unique receipt/request identifier prevents duplicate request processing.

The payment instruction must update session spend and create the receipt atomically with the transfer. A failed check must revert the entire transaction.

## Important distinction: payment vs service delivery

A successful on-chain receipt proves that the program authorized and executed a transfer to a configured provider address. It does not prove that an off-chain API was available, returned correct data, or fulfilled a service. Service delivery verification would require a separate mechanism (for example, signed provider attestations or a dispute/escrow design) and is outside the initial MVP.

## Current implementation

The first scaffold contains agent initialization, owner deposits, and owner withdrawals. Provider registration, delegated sessions, payment enforcement, receipts, and integration tests are not implemented yet.
