// ── Recurring Interval ──────────────────────────────────────────
export enum RecurringInterval {
  Weekly = 'Weekly',
  Monthly = 'Monthly',
}

// ── Recurring Payment ───────────────────────────────────────────
export interface RecurringPayment {
  id: number;
  owner: string;
  recipient: string;
  token: string;
  amount: bigint;
  interval: RecurringInterval;
  nextExecution: number; // unix timestamp
  totalExecuted: number;
  maxExecutions: number; // 0 = unlimited
  isActive: boolean;
}

// ── Spending Limit ──────────────────────────────────────────────
export interface SpendingLimit {
  owner: string;
  token: string;
  maxAmount: bigint;
  periodSeconds: number;
  currentSpent: bigint;
  periodStart: number;
}

// ── Multisig Config ─────────────────────────────────────────────
export interface MultisigConfig {
  signers: string[];
  threshold: number;
}

// ── Multisig Proposal ───────────────────────────────────────────
export interface MultisigProposal {
  id: number;
  proposer: string;
  recipient: string;
  token: string;
  amount: bigint;
  approvals: string[];
  executed: boolean;
  createdAt: number;
  expiresAt: number;
}

// ── Scheduled Transfer ──────────────────────────────────────────
export interface ScheduledTransfer {
  id: number;
  owner: string;
  recipient: string;
  token: string;
  amount: bigint;
  executeAfter: number;
  executed: boolean;
}

// ── Smart Account Dashboard State ───────────────────────────────
export interface SmartAccountState {
  recurringPayments: RecurringPayment[];
  spendingLimits: SpendingLimit[];
  multisigProposals: MultisigProposal[];
  scheduledTransfers: ScheduledTransfer[];
  multisigConfig: MultisigConfig | null;
}
