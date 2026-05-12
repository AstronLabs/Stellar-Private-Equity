'use client';

import React, { useState } from 'react';
import type {
  RecurringPayment,
  SpendingLimit,
  MultisigProposal,
  ScheduledTransfer,
} from '@/types/smart-account.types';
import { RecurringInterval } from '@/types/smart-account.types';
import RecurringPaymentCard from './RecurringPaymentCard';
import RecurringPaymentForm from './RecurringPaymentForm';
import SpendingLimitPanel from './SpendingLimitPanel';
import MultisigSetup from './MultisigSetup';
import MultisigProposalCard from './MultisigProposalCard';
import ScheduledTransferCard from './ScheduledTransferCard';
import ScheduledTransferForm from './ScheduledTransferForm';
import styles from './SmartAccount.module.css';

type Tab = 'recurring' | 'limits' | 'multisig' | 'scheduled';

const TABS: { key: Tab; label: string; icon: string }[] = [
  { key: 'recurring', label: 'Recurring', icon: '🔄' },
  { key: 'limits', label: 'Limits', icon: '🛡️' },
  { key: 'multisig', label: 'Multisig', icon: '🔐' },
  { key: 'scheduled', label: 'Scheduled', icon: '⏰' },
];

// ── Demo data for UI development ─────────────────────────────
const DEMO_RECURRING: RecurringPayment[] = [
  {
    id: 0,
    owner: 'GABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUVWX',
    recipient: 'GZYXWVUTSRQPONMLKJIHGFEDCBA765432ZYXWVUTSRQPONMLKJIHGFEDC',
    token: 'GCTOKEN1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGH',
    amount: BigInt(50_000_000),
    interval: RecurringInterval.Weekly,
    nextExecution: Math.floor(Date.now() / 1000) + 86400,
    totalExecuted: 4,
    maxExecutions: 52,
    isActive: true,
  },
  {
    id: 1,
    owner: 'GABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUVWX',
    recipient: 'GPOOL9876543210ZYXWVUTSRQPONMLKJIHGFEDCBA765432ZYXWVUTSRQ',
    token: 'GCTOKEN1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGH',
    amount: BigInt(100_000_000),
    interval: RecurringInterval.Monthly,
    nextExecution: Math.floor(Date.now() / 1000) - 3600,
    totalExecuted: 2,
    maxExecutions: 0,
    isActive: true,
  },
];

const DEMO_LIMITS: SpendingLimit[] = [
  {
    owner: 'GABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUVWX',
    token: 'GCTOKEN1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGH',
    maxAmount: BigInt(500_000_000),
    periodSeconds: 604_800,
    currentSpent: BigInt(320_000_000),
    periodStart: Math.floor(Date.now() / 1000) - 259200,
  },
];

const DEMO_PROPOSALS: MultisigProposal[] = [
  {
    id: 0,
    proposer: 'GABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUVWX',
    recipient: 'GZYXWVUTSRQPONMLKJIHGFEDCBA765432ZYXWVUTSRQPONMLKJIHGFEDC',
    token: 'GCTOKEN1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGH',
    amount: BigInt(1_000_000_000),
    approvals: ['GSIGNER1ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOP'],
    executed: false,
    createdAt: Math.floor(Date.now() / 1000) - 3600,
    expiresAt: Math.floor(Date.now() / 1000) + 604800,
  },
];

const DEMO_SCHEDULED: ScheduledTransfer[] = [
  {
    id: 0,
    owner: 'GABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUVWX',
    recipient: 'GZYXWVUTSRQPONMLKJIHGFEDCBA765432ZYXWVUTSRQPONMLKJIHGFEDC',
    token: 'GCTOKEN1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGH',
    amount: BigInt(250_000_000),
    executeAfter: Math.floor(Date.now() / 1000) + 172800,
    executed: false,
  },
];

export default function SmartAccountDashboard() {
  const [activeTab, setActiveTab] = useState<Tab>('recurring');

  // Demo handlers
  const handleExecuteRecurring = (id: number) => {
    console.log('Execute recurring payment:', id);
  };
  const handleCancelRecurring = (id: number) => {
    console.log('Cancel recurring payment:', id);
  };
  const handleCreateRecurring = (data: {
    recipient: string;
    amount: string;
    interval: RecurringInterval;
    maxExecutions: number;
  }) => {
    console.log('Create recurring:', data);
  };
  const handleSetLimit = (data: {
    token: string;
    maxAmount: string;
    periodSeconds: number;
  }) => {
    console.log('Set limit:', data);
  };
  const handleSetupMultisig = (data: {
    signers: string[];
    threshold: number;
  }) => {
    console.log('Setup multisig:', data);
  };
  const handleApproveMultisig = (id: number) => {
    console.log('Approve multisig:', id);
  };
  const handleExecuteScheduled = (id: number) => {
    console.log('Execute scheduled:', id);
  };
  const handleCancelScheduled = (id: number) => {
    console.log('Cancel scheduled:', id);
  };
  const handleScheduleTransfer = (data: {
    recipient: string;
    amount: string;
    executeAfter: number;
  }) => {
    console.log('Schedule transfer:', data);
  };

  return (
    <div className={styles.dashboard} id="smart-account-dashboard">
      {/* Header */}
      <div className={styles.dashboardHeader}>
        <div>
          <h1 className={styles.pageTitle}>Stellar Smart Accounts</h1>
          <p className={styles.pageSubtitle}>
            Automate your insurance contributions, set spending limits, and protect your funds
          </p>
        </div>
      </div>

      {/* Tab Navigation */}
      <nav className={styles.tabNav} id="smart-account-tabs">
        {TABS.map((tab) => (
          <button
            key={tab.key}
            className={`${styles.tab} ${activeTab === tab.key ? styles.tabActive : ''}`}
            onClick={() => setActiveTab(tab.key)}
            id={`tab-${tab.key}`}
          >
            <span className={styles.tabIcon}>{tab.icon}</span>
            {tab.label}
          </button>
        ))}
      </nav>

      {/* Tab Content */}
      <div className={styles.tabContent}>
        {/* ── Recurring Payments ──────────────────────────────── */}
        {activeTab === 'recurring' && (
          <div className={styles.tabPanel}>
            <div className={styles.sectionHeader}>
              <h2 className={styles.sectionTitle}>Recurring Payments</h2>
              <p className={styles.sectionDesc}>
                Authorize automatic weekly or monthly insurance contributions
              </p>
            </div>
            <div className={styles.gridCards}>
              {DEMO_RECURRING.map((payment) => (
                <RecurringPaymentCard
                  key={payment.id}
                  payment={payment}
                  onExecute={handleExecuteRecurring}
                  onCancel={handleCancelRecurring}
                />
              ))}
            </div>
            <RecurringPaymentForm onSubmit={handleCreateRecurring} />
          </div>
        )}

        {/* ── Spending Limits ────────────────────────────────── */}
        {activeTab === 'limits' && (
          <div className={styles.tabPanel}>
            <div className={styles.sectionHeader}>
              <h2 className={styles.sectionTitle}>Spending Limits</h2>
              <p className={styles.sectionDesc}>
                Set daily, weekly, or monthly caps on token spending
              </p>
            </div>
            <SpendingLimitPanel
              limits={DEMO_LIMITS}
              onSetLimit={handleSetLimit}
            />
          </div>
        )}

        {/* ── Multisig ───────────────────────────────────────── */}
        {activeTab === 'multisig' && (
          <div className={styles.tabPanel}>
            <div className={styles.sectionHeader}>
              <h2 className={styles.sectionTitle}>Multisig Protection</h2>
              <p className={styles.sectionDesc}>
                Require multiple signatures for high-value transactions
              </p>
            </div>
            <MultisigSetup
              currentSigners={[]}
              currentThreshold={0}
              onSetup={handleSetupMultisig}
            />
            {DEMO_PROPOSALS.length > 0 && (
              <>
                <h3 className={styles.subSectionTitle}>Pending Proposals</h3>
                <div className={styles.gridCards}>
                  {DEMO_PROPOSALS.map((proposal) => (
                    <MultisigProposalCard
                      key={proposal.id}
                      proposal={proposal}
                      threshold={2}
                      onApprove={handleApproveMultisig}
                    />
                  ))}
                </div>
              </>
            )}
          </div>
        )}

        {/* ── Scheduled Transfers ────────────────────────────── */}
        {activeTab === 'scheduled' && (
          <div className={styles.tabPanel}>
            <div className={styles.sectionHeader}>
              <h2 className={styles.sectionTitle}>Scheduled Transfers</h2>
              <p className={styles.sectionDesc}>
                Schedule future one-time transfers that execute automatically
              </p>
            </div>
            <div className={styles.gridCards}>
              {DEMO_SCHEDULED.map((transfer) => (
                <ScheduledTransferCard
                  key={transfer.id}
                  transfer={transfer}
                  onExecute={handleExecuteScheduled}
                  onCancel={handleCancelScheduled}
                />
              ))}
            </div>
            <ScheduledTransferForm onSubmit={handleScheduleTransfer} />
          </div>
        )}
      </div>
    </div>
  );
}
