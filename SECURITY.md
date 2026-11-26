# Security

AgentPesa is experimental software under active development. It has not been audited and must only be used with Devnet SOL.

## Never commit secrets

Do not commit:
- Solana keypair JSON files or seed phrases
- Private keys, API keys, access tokens, or service credentials
- Local environment files containing secrets

Use environment variables or an untracked local `.env` file for development credentials.

## Reporting a vulnerability

Do not publish an exploit or sensitive vulnerability details in a public issue. Contact the repository owner privately through GitHub to coordinate a responsible report.

## Security design requirements

- AI model output is untrusted input.
- The program must enforce every spending rule on-chain.
- Delegated signers must never receive the owner's wallet private key.
- Session revocation and expiry must be checked by the payment instruction.
- Provider destination accounts must be bound to owner-approved provider records.
- Arithmetic must use checked operations.
- Duplicate payment requests must be rejected.
- Mainnet use is out of scope until the program has tests, independent review, and an audit appropriate to the risk.
