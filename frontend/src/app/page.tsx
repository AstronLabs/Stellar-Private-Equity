"use client";

import React, { useState } from "react";
import Image from "next/image";
import styles from "./page.module.css";

export default function Home() {
  const [activeStep, setActiveStep] = useState(0);

  const steps = [
    {
      title: "1. Fund Governance",
      contract: "pe-fund",
      description: "Establish the fund, set the target size, and manage the investor compliance whitelist.",
      code: `// GP whitelists an investor and sets commitment
fund_client.whitelist_investor(&gp, &investor);
fund_client.commit_capital(&investor, &100_000);`,
      metric: "Compliance Gated"
    },
    {
      title: "2. Capital Call Escrow",
      contract: "pe-capital-call",
      description: "Drawdown commitments. LPs contribute USDC. On subscription, USDC is released and LP tokens are minted.",
      code: `// LP contributes to call; GP draws down & mints LP tokens
cc_client.contribute(&lp, &call_id, &50_000);
cc_client.drawdown(&gp, &call_id);`,
      metric: "Atomic Minting"
    },
    {
      title: "3. Milestone Payouts",
      contract: "pe-milestone-disbursement",
      description: "Fund portfolio companies conditionally. LPs vote with their LP tokens to approve disbursements.",
      code: `// LP votes with token weight; GP executes if approved
md_client.vote(&lp, &proposal_id, true); // weight = LP balance
md_client.execute_disbursement(&gp, &proposal_id);`,
      metric: "Token-Weighted Voting"
    },
    {
      title: "4. Yield Distribution",
      contract: "pe-distribution",
      description: "Distribute returns to LPs. USDC is split proportionally based on LP token balances at distribution time.",
      code: `// GP deposits returns; LP claims pro-rata share
dist_client.create_distribution(&gp, &500_000);
dist_client.claim_distribution(&lp, &dist_id);`,
      metric: "Pro-rata Claims"
    },
    {
      title: "5. Secondary Liquidity",
      contract: "pe-secondary-market",
      description: "Trade LP tokens peer-to-peer. The market enforces whitelisting on the buyer for absolute compliance.",
      code: `// Seller escrows LP tokens; Buyer swaps atomically
market_client.create_listing(&seller, &10_000, &12_000);
market_client.buy_listing(&buyer, &listing_id); // checks whitelist`,
      metric: "Atomic & Compliant"
    }
  ];

  return (
    <div className={styles.container}>
      {/* Header */}
      <header className={styles.header}>
        <div className={styles.logoContainer}>
          <div className={styles.logoGlow}></div>
          <span className={styles.logoText}>STELLAR <span className={styles.accentText}>PE</span></span>
        </div>
        <nav className={styles.nav}>
          <a href="#features" className={styles.navLink}>Features</a>
          <a href="#lifecycle" className={styles.navLink}>Lifecycle</a>
          <a href="#metrics" className={styles.navLink}>Metrics</a>
        </nav>
        <button className={styles.connectButton}>
          <span>Launch Platform</span>
          <div className={styles.buttonGlow}></div>
        </button>
      </header>

      {/* Hero Section */}
      <section className={styles.hero}>
        <div className={styles.heroContent}>
          <div className={styles.badge}>
            <span className={styles.badgeDot}></span>
            <span>Powered by Soroban Smart Contracts</span>
          </div>
          <h1 className={styles.heroTitle}>
            The Future of <br />
            <span className={styles.gradientText}>Private Equity</span> <br />
            is On-Chain.
          </h1>
          <p className={styles.heroSubtitle}>
            An automated, compliant, and end-to-end investment suite for General Partners and Limited Partners. Build, fund, and trade private assets with cryptographic certainty.
          </p>
          <div className={styles.heroActions}>
            <button className={styles.primaryButton}>
              Get Started
              <div className={styles.buttonGlow}></div>
            </button>
            <a href="https://github.com/AstronLabs/Stellar-Private-Equity" target="_blank" rel="noopener noreferrer" className={styles.secondaryButton}>
              View GitHub
            </a>
          </div>
        </div>

        <div className={styles.heroVisual}>
          <div className={styles.visualGlow}></div>
          <div className={styles.imageWrapper}>
            <Image
              src="/hero_network.png"
              alt="Private Equity Network Visualization"
              width={600}
              height={600}
              priority
              className={styles.heroImage}
            />
          </div>
        </div>
      </section>

      {/* Metrics Board */}
      <section id="metrics" className={styles.metricsSection}>
        <div className={styles.metricsGrid}>
          <div className={styles.metricCard}>
            <div className={styles.metricHeader}>
              <span className={styles.metricTitle}>Total Value Locked</span>
              <span className={styles.metricIcon}>🔒</span>
            </div>
            <div className={styles.metricValue}>$48,250,000</div>
            <div className={styles.metricTrend}>+12.4% this month</div>
          </div>
          <div className={styles.metricCard}>
            <div className={styles.metricHeader}>
              <span className={styles.metricTitle}>Active Funds</span>
              <span className={styles.metricIcon}>📈</span>
            </div>
            <div className={styles.metricValue}>12</div>
            <div className={styles.metricTrend}>Global deployments</div>
          </div>
          <div className={styles.metricCard}>
            <div className={styles.metricHeader}>
              <span className={styles.metricTitle}>Milestone Disbursements</span>
              <span className={styles.metricIcon}>⚡</span>
            </div>
            <div className={styles.metricValue}>$14,620,000</div>
            <div className={styles.metricTrend}>100% LP voted</div>
          </div>
          <div className={styles.metricCard}>
            <div className={styles.metricHeader}>
              <span className={styles.metricTitle}>Secondary Volume</span>
              <span className={styles.metricIcon}>🔄</span>
            </div>
            <div className={styles.metricValue}>$3,450,000</div>
            <div className={styles.metricTrend}>Atomic compliant trades</div>
          </div>
        </div>
      </section>

      {/* Interactive Lifecycle */}
      <section id="lifecycle" className={styles.lifecycleSection}>
        <div className={styles.sectionHeader}>
          <h2 className={styles.sectionTitle}>Fund Lifecycle Explorer</h2>
          <p className={styles.sectionSubtitle}>Click through the five core smart contracts that govern the fund from creation to exit.</p>
        </div>

        <div className={styles.lifecycleGrid}>
          <div className={styles.lifecycleNav}>
            {steps.map((step, index) => (
              <button
                key={index}
                className={`${styles.lifecycleStepBtn} ${activeStep === index ? styles.activeStepBtn : ""}`}
                onClick={() => setActiveStep(index)}
              >
                <div className={styles.stepBtnIndicator}></div>
                <div className={styles.stepBtnContent}>
                  <span className={styles.stepBtnTitle}>{step.title}</span>
                  <span className={styles.stepBtnSub}>{step.contract}.wasm</span>
                </div>
              </button>
            ))}
          </div>

          <div className={styles.lifecycleDisplay}>
            <div className={styles.displayCard}>
              <div className={styles.displayHeader}>
                <span className={styles.contractBadge}>{steps[activeStep].contract}</span>
                <span className={styles.statusBadge}>{steps[activeStep].metric}</span>
              </div>
              <h3 className={styles.displayTitle}>{steps[activeStep].title}</h3>
              <p className={styles.displayDesc}>{steps[activeStep].description}</p>
              
              <div className={styles.codeContainer}>
                <div className={styles.codeHeader}>
                  <span className={styles.codeDot}></span>
                  <span className={styles.codeDot}></span>
                  <span className={styles.codeDot}></span>
                  <span className={styles.codeTitle}>Soroban Integration</span>
                </div>
                <pre className={styles.codeBlock}>
                  <code>{steps[activeStep].code}</code>
                </pre>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Core Features */}
      <section id="features" className={styles.featuresSection}>
        <div className={styles.sectionHeader}>
          <h2 className={styles.sectionTitle}>Built for Compliance & Speed</h2>
          <p className={styles.sectionSubtitle}>Leveraging Stellar's native assets and Soroban's smart contract engine.</p>
        </div>

        <div className={styles.featuresGrid}>
          <div className={styles.featureCard}>
            <div className={styles.featureIcon}>🛡️</div>
            <h3 className={styles.featureTitle}>Stellar Asset Clawbacks</h3>
            <p className={styles.featureDesc}>
              Full compliance enforcement. The GP can claw back LP tokens from non-compliant addresses directly via the Stellar Asset Contract (SAC).
            </p>
          </div>
          <div className={styles.featureCard}>
            <div className={styles.featureIcon}>⚙️</div>
            <h3 className={styles.featureTitle}>Zero-Counterparty Escrows</h3>
            <p className={styles.featureDesc}>
              No middleman. Capital calls, milestone proposals, and secondary market trades are held in secure, dedicated contract-owned escrows.
            </p>
          </div>
          <div className={styles.featureCard}>
            <div className={styles.featureIcon}>🌐</div>
            <h3 className={styles.featureTitle}>Atomic Swaps</h3>
            <p className={styles.featureDesc}>
              LP tokens are traded atomically for USDC on the secondary market. Trades succeed or fail in a single transaction, eliminating settlement risk.
            </p>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className={styles.ctaSection}>
        <div className={styles.ctaCard}>
          <div className={styles.ctaGlow}></div>
          <h2 className={styles.ctaTitle}>Ready to Deploy Your Fund?</h2>
          <p className={styles.ctaSubtitle}>
            Launch your private equity fund on-chain. Automate whitelisting, drawdowns, disbursements, and distributions today.
          </p>
          <button className={styles.primaryButton}>
            Launch Application
            <div className={styles.buttonGlow}></div>
          </button>
        </div>
      </section>

      {/* Footer */}
      <footer className={styles.footer}>
        <p className={styles.footerText}>&copy; 2026 Stellar Private Equity Platform. Built on Stellar & Soroban.</p>
      </footer>
    </div>
  );
}
