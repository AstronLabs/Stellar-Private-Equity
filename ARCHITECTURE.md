# Architecture: Private Equity Platform on Stellar

## Core Principle: Separate Compliance from Settlement

The architecture splits into two layers: a **Compliance Layer** that establishes investor identity and eligibility off-chain, and a **Settlement Layer** (Stellar/Soroban) that handles LP token issuance, capital calls, fund logic, distributions, and secondary market trading.

---

## 1. Compliance Layer (Off-Chain)

**Identity verification**: The investor submits their legal identity (NIN, BVN, passport, or CAC certificate for entities). Smile ID runs document checks and liveness detection to confirm authenticity.

**Accreditation check**: A compliance officer reviews the investor's source of funds declaration and minimum investment threshold to confirm they qualify as an accredited LP under SEC Nigeria rules.

**Wallet authorization**: Once approved, the investor's Stellar wallet address is whitelisted by the GP signing an authorization on the LP token account — enforcing at the token level that only KYC-cleared wallets can hold or receive LP tokens.

**Output**: An approved investor record stored in the platform database, with the Stellar wallet trustline authorized. A hash of the KYC approval can be anchored on Stellar for auditability, without sensitive identity data living on-chain.

---

## 2. Settlement Layer (Stellar / Soroban)

### Fund Contract — Master Fund Logic

A Soroban smart contract governs the fund lifecycle: it holds fund metadata, tracks the capital call schedule, mints LP tokens proportional to each investor's contribution, and enforces transfer restrictions so LP tokens can only move between authorized wallets.

### LP Token — Stellar Asset Contract (SAC)

LP tokens are issued as a Stellar custom asset via a Stellar Asset Contract. Two flags are set on the issuing account at creation:

- **AUTH_REQUIRED**: every wallet must be explicitly approved before it can receive LP tokens
- **CLAWBACK_ENABLED**: the GP or compliance officer can recover tokens if required by a regulatory order or AML action

### Escrow / Capital Call Contract

Holds incoming USDC contributions from LPs during each capital call drawdown period. Tracks individual LP contributions within a pooled structure. Releases accumulated capital to the GP treasury wallet once the call is fully subscribed — or refunds LPs proportionally if a call is cancelled.

### Milestone Disbursement Contract

Releases capital from the GP treasury to whitelisted portfolio companies and suppliers as investment milestones are completed (e.g., initial tranche, expansion tranche, follow-on round). Each release requires an IPFS-hashed evidence submission (board resolution, term sheet, valuation report) referenced on-chain, plus multisig approval from the investment committee.

### Distribution Contract — Waterfall Logic

Executes the LP Agreement waterfall automatically when the GP triggers a distribution event:

1. Return of capital to LPs
2. Preferred return (typically 8% p.a.) to LPs
3. GP catch-up to carried interest percentage
4. Remaining gains split 80% LP / 20% GP (carried interest)

USDC is distributed directly from the fund treasury to each LP's Stellar wallet in a single batched transaction.

### Governance / Multisig

Stellar accounts support **native multi-signature** — a single account can require multiple signatures with configurable weights and thresholds before authorizing a transaction. The GP treasury wallet is configured as a multisig account requiring GP + compliance co-signature for capital movements above a defined threshold. For more conditional logic (e.g., requiring both signature thresholds *and* a valid evidence hash before a milestone release), a thin Soroban contract wraps this logic on top of the multisig account.

### Secondary Market Contract

Manages LP stake listings and trade settlement via the Stellar DEX. A selling LP lists their tokens at an asking price in USDC. The contract verifies the buyer is an authorized wallet before allowing the trade to settle. Once both conditions are met, the Stellar DEX executes an atomic swap — USDC moves from buyer to seller, LP tokens move from seller to buyer, simultaneously, with no counterparty risk.

### Stablecoin Settlement

All contributions, disbursements, distributions, and secondary trades move in USDC on Stellar, benefiting from sub-cent transaction fees and 3–5 second settlement finality.

---

## 3. Evidence Storage

IPFS (via Pinata or a similar pinning service) stores investment milestone documents, board resolutions, valuation reports, and supplier receipts. The protocol records the resulting content hashes on-chain as references for governance review and LP audit access.

---

## 4. Frontend & Wallet Integration

A Next.js frontend integrates Stellar wallets (Freighter) for all settlement-side interactions — capital call contributions, viewing portfolio NAV, receiving distributions, and listing stakes on the secondary market. The GP dashboard connects the same way, with role-based views for fund management, portfolio reporting, and compliance actions.

---

## 5. Backend / Orchestration

A Node.js/TypeScript backend coordinates the system:

- **Stellar SDK** (js-stellar-sdk) for querying Horizon transaction history, monitoring capital call receipts, and interacting with Soroban contracts
- **PostgreSQL** for off-chain records: investor profiles, KYC status, cap table mirror, NAV history, quarterly reports
- **Smile ID API** for KYC/AML identity verification
- **Flutterwave / Paystack** for naira on/off-ramp (NGN ↔ USDC conversion)
- **IPFS pinning service** (Pinata) for milestone evidence uploads
- **SendGrid** for capital call notices, NAV report alerts, and distribution notifications
- **DocuSign** for LP Agreement and subscription document e-signatures

---

## End-to-End Flow Summary

1. GP deploys the fund contract and LP token on Stellar, configuring AUTH_REQUIRED and CLAWBACK_ENABLED on the issuing account
2. Investor completes KYC via Smile ID; compliance officer approves their Stellar wallet trustline
3. GP issues a capital call; LPs transfer USDC into the escrow contract proportional to their commitment; LP tokens are minted to their wallets
4. GP deploys capital to portfolio companies in milestone-based tranches, each gated by IPFS evidence and multisig approval
5. Each quarter, the GP submits portfolio company financials; the NAV engine calculates fund value; LP reports (TVPI, DPI, MOIC, IRR) are generated and anchored on Stellar
6. When the fund realises a gain, the distribution contract executes the waterfall and settles USDC directly to LP wallets
7. LPs wishing to exit early list their LP tokens on the secondary market; the contract verifies buyer authorization and the Stellar DEX settles the trade atomically
