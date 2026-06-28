# Contributing to Stellar Private Equity Platform

Thank you for your interest in contributing to the Stellar Private Equity Platform! We welcome contributions from developers, designers, security auditors, and community members. 

By participating in this project, you help build a more transparent, compliant, and automated private equity fund management protocol on the Stellar network.

---

## Table of Contents

1. [Code of Conduct](#1-code-of-conduct)
2. [How Can I Contribute?](#2-how-can-i-contribute)
3. [Local Development Setup](#3-local-development-setup)
4. [Smart Contract Guidelines](#4-smart-contract-guidelines)
5. [Frontend Guidelines](#5-frontend-guidelines)
6. [Commit & Branching Style](#6-commit--branching-style)
7. [Submitting a Pull Request](#7-submitting-a-pull-request)

---

## 1. Code of Conduct

We are committed to providing a welcoming, inclusive, and harassment-free environment for everyone. Please be respectful and constructive in all communication, including GitHub issues, pull requests, and community chat channels.

---

## 2. How Can I Contribute?

### Reporting Bugs
If you find a bug, please open an issue on GitHub. Include:
*   A clear, descriptive title.
*   Steps to reproduce the issue.
*   Expected vs. actual behavior.
*   Relevant logs, screenshots, or transaction hashes (if on-chain).

### Suggesting Enhancements
Have an idea for a new feature, a custom drawdown pattern, or an improved secondary market mechanism? Open an issue describing:
*   The goal of the enhancement.
*   How it should behave.
*   Why it adds value to the platform.

### Code Contributions
1.  Search our open issues for tasks labeled `good first issue` or `help wanted`.
2.  Comment on the issue to let us know you are working on it.
3.  Follow the [Local Development Setup](#3-local-development-setup) to get the project running.

---

## 3. Local Development Setup

### Prerequisites
To build and run the Stellar Private Equity Platform, you need:
*   **Node.js** (v20.0.0 or higher) and `npm`.
*   **Rust** (latest stable version).
*   **WASM Target**: Add the WebAssembly target via:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
*   **Stellar CLI**:
    ```bash
    cargo install --locked stellar-cli --version 22.0.1
    ```
*   **Freighter Wallet**: Install the extension in your browser from [freighter.app](https://www.freighter.app/).

### Cloned Repository Structure
*   `/contracts`: Rust-based Soroban smart contracts.
    *   `/fund`: Core fund governance, whitelisting, and LP token mechanics.
    *   `/capital_call`: Escrow contract for LP commitments, drawdowns, and refunds.
    *   `/milestone_disbursement`: Milestone-based voting and portfolio disbursement.
    *   `/distribution`: Pro-rata returns distribution to LPs.
    *   `/secondary_market`: Compliant, atomic secondary trades of LP tokens.
*   `/frontend`: Next.js web application.
*   `/Docs`: Technical specifications and diagrams.

---

## 4. Smart Contract Guidelines

All smart contracts are located in `/contracts`.

### Building Contracts
Compile the contracts to WASM using the following commands:
```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

### Running Tests
We enforce strict test coverage. To execute the smart contract test suites:
```bash
cd contracts
cargo test
```

### Deferring and Formatting
Before committing your code, format it and run clippy:
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 5. Frontend Guidelines

The frontend is built with Next.js 14, React, TailwindCSS, and `@stellar/stellar-sdk`.

### Installing Dependencies
```bash
cd frontend
npm install
```

### Environment Configuration
Copy the template and configure your local settings:
```bash
cp .env.local.example .env.local
```

### Running Locally
```bash
npm run dev
```
Open `http://localhost:3000` to view the application in your browser. Ensure your Freighter wallet is set to **Testnet**.

---

## 6. Commit & Branching Style

To maintain a clean and searchable history, we use **Semantic Branching** and **Conventional Commits**.

### Branch Naming Conventions
Always create a branch off of `main` for your work. Use the following prefixes:
*   `feature/` (e.g., `feature/fundraising-cap-rules`)
*   `bugfix/` (e.g., `bugfix/whitelisting-validation`)
*   `docs/` (e.g., `docs/add-api-endpoints`)
*   `chore/` (e.g., `chore/bump-deps`)

### Commit Messages
We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

Format: `<type>(<scope>): <description>`

Examples:
*   `feat(contracts): add capital call escrow and automated minting`
*   `fix(frontend): resolve wallet disconnect edge-case`
*   `docs(readme): add troubleshooting section for docker`
*   `style(frontend): adjust button padding in portfolio view`

---

## 7. Submitting a Pull Request

When you are ready to submit your work:
1.  **Sync your branch**: Rebase or merge the latest `main` branch into your feature branch.
2.  **Verify build & tests**: Run `cargo test` (for contracts) and `npm run build` (for frontend) to ensure everything compiles and tests pass.
3.  **Submit PR**: Open a Pull Request against our `main` branch.
4.  **Describe changes**: Use our PR template to detail:
    *   What issue does this resolve?
    *   How was it tested?
    *   Any breaking changes?
5.  **Review cycle**: Code must be reviewed and approved by at least one maintainer before merging.
