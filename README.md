# Stellar Private Equity 
A decentralized, compliant, and automated private equity fund management protocol built on Stellar.

The Stellar Private Equity Protocol streamlines the lifecycle of private equity funds—from whitelisting and capital commitments to drawdowns, milestone-based disbursements, pro-rata returns distributions, and secondary market liquidity. By utilizing Soroban smart contracts, USDC escrows, token-weighted milestone voting, and compliant secondary trading, it replaces slow, paper-based fund administration with a transparent, on-chain alternative.

It's the bridge between institutional private equity compliance and blockchain efficiency.

## Motivation
Traditional private equity is plagued by high operational overhead, slow manual capital calls, paper-based compliance checks, and a complete lack of secondary liquidity for Limited Partners (LPs). Deals are slow to close, and capital remains locked up for years with minimal transparency.

Stellar Private Equity Protocol makes fund management efficient and liquid:

*   **Automate Compliance:** Enforce investor whitelisting on-chain. Only verified, whitelisted investors can commit capital, receive distributions, or buy LP tokens.
*   **Streamline Capital Calls:** Escrow LP contributions securely and trigger atomic LP token minting as soon as the drawdown is funded, eliminating manual reconciliation.
*   **Govern Disbursements:** Empower LPs with token-weighted voting on portfolio milestones before capital is released to portfolio companies.
*   **Inject Secondary Liquidity:** Enable LPs to trade their fund shares peer-to-peer via atomic swaps, while maintaining absolute compliance gating.

## Features
*   **Fund Governance Contract** — Manages fund lifecycle stages, investor whitelisting, and LP token minting/clawbacks via the Stellar Asset Contract (SAC).
*   **Capital Call Escrow Contract** — Escrows USDC contributions during drawdowns, handles automatic LP token minting on success, and manages refunds on cancellation.
*   **Milestone Disbursement Contract** — Milestone-based voting where LPs use their LP token balances as voting weight to approve or reject USDC payouts to portfolio companies.
*   **Distribution Contract** — Snapshots LP token supply and automates pro-rata USDC yield distributions to LPs.
*   **Secondary Market Contract** — Facilitates atomic, compliant swaps of LP tokens for USDC, enforcing that the buyer is whitelisted on the Fund contract.
*   **Stablecoin Settlement** — All contributions, payouts, and trades settle in USDC on Stellar with sub-cent fees and 3–5 second finality.

## Stack
*   **Frontend:** Next.js, TypeScript, Vanilla CSS Modules
*   **Wallet:** Freighter + `@stellar/freighter-api`
*   **Smart Contracts:** Rust, Soroban SDK v22
*   **Backend:** Node.js, Express, TypeScript, Stellar SDK (`@stellar/stellar-sdk` v13)
*   **Database:** In-memory / mock database (for KYC records)

## Running it locally

You can run and build the entire full-stack application (smart contracts, frontend, and backend) from the root directory of the project.

### Prerequisites
*   Node.js ≥ 20.0.0
*   Rust (latest stable) + `wasm32-unknown-unknown` target
*   Stellar CLI — `cargo install stellar-cli`
*   [Freighter Wallet](https://www.freighter.app/) browser extension

### 1. Setup Frontend & Backend
Install dependencies for both folders:
```bash
npm --prefix frontend install
npm --prefix backend install
```

Configure `backend/.env` (use `backend/.env.example` as a template):
```env
PORT=4000
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
FUND_CONTRACT_ID=your_deployed_fund_contract_id
GP_SECRET_KEY=your_gp_secret_key
```

### 2. Run the Application
Start the development servers from the root directory:
```bash
# Start the Next.js frontend (http://localhost:3000)
npm run frontend:dev

# Start the Express backend (http://localhost:4000)
npm run backend:dev
```

### 3. Build & Test Contracts
To build and test the Soroban smart contracts:
```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
cargo test
```

## How the Private Equity Protocol Works
The protocol coordinates the entire fund lifecycle on-chain through five phases:

1.  **Whitelisting:** The General Partner (GP) whitelists approved investors on the Fund contract after KYC verification.
2.  **Commitment & Capital Call:** The GP issues a capital call. Whitelisted LPs deposit USDC into the capital call escrow contract.
3.  **Drawdown:** Once the call is fully subscribed, the GP draws down the USDC to their treasury, and LP tokens are automatically minted to the LPs.
4.  **Milestone Voting:** The GP proposes a disbursement to a portfolio company. LPs vote on the milestone using their LP tokens. If approved, USDC is released to the portfolio company.
5.  **Distribution & Secondary Trade:** Portfolio exits are deposited by the GP and distributed pro-rata. LPs can also list their LP tokens on the secondary market for atomic, compliant sales to other whitelisted LPs.

## Roadmap
*   **Automated KYC Oracle Integration:** Connect the backend to identity verification services to automate the whitelisting trigger.
*   **Yield-Bearing Treasuries:** Integrate idle capital call or distribution escrows with Soroban-based lending protocols (e.g. Blend) to earn yield.
*   **Dynamic Voting Quorums:** Allow the fund setup to customize voting thresholds and quorums based on the fund size or asset class.
*   **Historical Event Indexer:** Set up a Mercury or custom indexer to cache and query historical secondary market trades and milestone votes.

## Documentation
*   [Architecture](ARCHITECTURE.md): Core design principles, contract interactions, and system architecture.
*   [Contributing Guide](CONTRIBUTING.md): Code formatting, branching style, and pull request guidelines.

## License
MIT
